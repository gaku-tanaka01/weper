use config;
use csv_config;
use headless_chrome::browser::tab::element;
use std::error::Error;
use std::io::BufWriter;
use std::io::Write;
use weper_lib;
use tokio::time;
use std::time::Duration;
use headless_chrome;
use std::sync::Arc;

const BASE_URL: &str = "https://jp.indeed.com";
const INDEED_OFFER_PER_PAGE: u32 = 15;

fn with_close_tab<F,T>(browser: &headless_chrome::Browser, f: F) -> Result<T, Box<dyn Error>>
where
    F: FnOnce(Arc<&headless_chrome::Tab>) -> Result<T, Box<dyn Error>>,
{
    let tab = browser.new_tab()?;
    let result = f(Arc::new(&tab));
    tab.close_target()?;
    result

}

async fn get_hp_link_in_indeed(browser: &headless_chrome::Browser, indeed_company_url: &str) -> Result<Option<String>, Box<dyn Error>> {
    with_close_tab(browser, |tab|  {
        tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36", Some("ja-JP"), Some("Windows"))?;
        tab.navigate_to(&indeed_company_url)?;
        tab.wait_until_navigated()?;
        tab.set_default_timeout(std::time::Duration::from_secs(5));
        let element = match tab.wait_for_element("a.css-1rezcpd.e19afand0") {
            Ok(element) => element,
            Err(e) => {
                return Ok(None);
            }
        };
        let link = match element.get_attribute_value("href") {
            Ok(link) => link,
            Err(e) => {
                return Ok(None);
            }
        };
        Ok(link)
    })
}

fn extract_indeed_company_link(tab: &headless_chrome::Tab, element: &headless_chrome::Element) -> Result<Option<String>, Box<dyn Error>> {
    element.click()?;
    let parent_a = match tab.wait_for_element("[data-testid='inlineHeader-companyName']") {
        Ok(parent_a) => parent_a,
        Err(e) => {
            return Ok(None);
        }
    };
   let a_tag = match parent_a.find_element("a") {
        Ok(a_tag) => a_tag,
        Err(e) => {
            return Ok(None);
        }
    };
    let link = match a_tag.get_attribute_value("href") {
        Ok(link) => link,
        Err(e) => {
            return Ok(None);
        }
    };
    Ok(link)
}

async fn try_access_indeed_with_headless_chrome(browser: &headless_chrome::Browser) -> Result<Option<String>, Box<dyn Error>> {
    let tab = browser.new_tab()?;
    tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36", Some("ja-JP"), Some("Windows"))?;
    tab.navigate_to(&BASE_URL)?;
    tab.wait_until_navigated()?;
    // 会社リンクaタグの親要素を取得
    let parent_a = tab.wait_for_element("div.css-wnhbdt.e37uo190")?;
    // 指定のクラスを持つaタグを取得
    let a_tag = parent_a.find_element("a.gnav-Logo.gnav-header-19985ju.e19afand0")?;

    let attributes = a_tag.get_attributes()?;
    let link = a_tag.get_attribute_value("href")?;
    println!("attributes:{:#?}", attributes);
    println!("link:{:#?}", link);
    Ok(link)
}

pub async fn run_indeed_scraper(args: &config::IndeedArgs) -> Result<(), Box<dyn Error>> {
    let mut area_name = args.area_word.clone();
    let job_name = &args.job_word;
    if area_name.is_none() && job_name.is_none() {
        area_name = Some("東京都".to_string());
    }
    let count = &args.count.unwrap_or(15) / INDEED_OFFER_PER_PAGE * INDEED_OFFER_PER_PAGE;
    let interval = args.interval.unwrap_or(5);
    let filename = &format!(
        "indeed_{}_{}_{}件",
        &area_name.as_deref().unwrap_or(""),
        &job_name.as_deref().unwrap_or(""),
        count
    );

    let file = weper_lib::create_unique_file(&filename)?;
    //CSVにBOMを書き込みUTF-8 BOMにする excelのため
    let mut buf_writer = BufWriter::new(file);
    // UTF-8 BOMを書き込む
    match buf_writer.write_all(b"\xEF\xBB\xBF") {
        Ok(_) => {}
        Err(e) => panic!("BOM書き込み失敗\nerror:{}", e),
    };

    //BufWriterをCSVWriterに変換
    let mut writer = csv::Writer::from_writer(buf_writer);

    let url = format!(
        "{}/jobs?q={}&l={}",
        BASE_URL,
        &job_name.as_deref().unwrap_or(""),
        &area_name.as_deref().unwrap_or("")
    );

    let browser = headless_chrome::Browser::new(
        headless_chrome::LaunchOptions {
            headless: false,
            // port: Some(8000),
            // sandbox: false, //リリースの時はコメントアウト
            ..Default::default()
        }
    )?;
    let tab = browser.new_tab()?;
    tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36", Some("ja-JP"), Some("Windows"))?;

    println!("url:{}", url);
    for i in 0..(count / INDEED_OFFER_PER_PAGE + 1) {
        let html = if i == 0 {
            weper_lib::get_first_page_html_with_headless_chrome(&url, &tab).await?
        } else if i == 1 {
            continue;
        } else {
            weper_lib::get_next_page_html_with_headless_chrome(r#"[data-testid="pagination-page-next"]"#, &tab).await?
        };

        let mut offers_info = weper_lib::get_offer_info(
            &html,
            &weper_lib::GetOfferInfoParams {
                base_url: &BASE_URL,
                offer_wrapper_selector: ".job_seen_beacon",
                company_name_selector: ".css-92r8pb",
                job_title_selector: ".jcs-JobTitle",
            },
        )?;

        let elements = tab.wait_for_elements("div.cardOutline.tapItem.dd-privacy-allow.result")?;
        //indeed内の会社詳細リンクを取得
        let mut indeed_company_links = Vec::new();
        for element in elements {
            let link = extract_indeed_company_link(&tab, &element)?;
            indeed_company_links.push(link);
        }

        /* 会社HPリンク取得　まずindeed内で取得を試みる 失敗したらGoogle検索でHP取得 */
        for i in 0..INDEED_OFFER_PER_PAGE {
            if let Some(link) = indeed_company_links[i as usize].clone() {
                let hp_link = get_hp_link_in_indeed(&browser, &link).await?;
                offers_info[i as usize].hp_link = hp_link;
            }

            if offers_info[i as usize].hp_link.is_none() {
                //Indeed内で取得失敗した場合はGoogle検索でHP取得
                offers_info[i as usize].hp_link = match weper_lib::get_hp_link(&offers_info[i as usize].company_name).await {
                    Ok(hp_link) => {
                        println!("Google検索のHPリンク取得成功: {:#?}", &hp_link);
                        hp_link
                    },
                    Err(e) => {
                        eprintln!("Google検索のHPリンク取得失敗: {}", e);
                        continue;
                    }
                };
            } else {
                continue;
            }
        }

        match csv_config::write_to_csv(&mut writer, &offers_info) {
            Ok(_) => {
                if i < 1 {
                    println!("{}件のデータ書き込み完了", (i + 1) * INDEED_OFFER_PER_PAGE)
            }else if i > 1 {
                println!("{}件のデータ書き込み完了", i * INDEED_OFFER_PER_PAGE)
            } else {}
            }
            Err(e) => {
                println!("CSV書き込み失敗: {}", e);
                return Err(Box::new(e));
            }
        }
        // インターバル中にアクションを実行して接続を維持
        if interval > 0 {
            for _ in 0..(interval) {
                if let Err(e) = tab.evaluate("console.log('keep-alive')", false) {
                    println!("Failed to send keep-alive signal: {}", e);
                }
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_run_indeed_scraper() {
        let args = config::IndeedArgs {
            area_word: Some("東京都".to_string()),
            job_word: Some("プログラマー".to_string()),
            count: Some(15),
            interval: Some(5),
        };

        let result = run_indeed_scraper(&args).await;
        eprintln!("result:{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_try_access_indeed_with_headless_chrome() {
        let browser = headless_chrome::Browser::new(
            headless_chrome::LaunchOptions {
                headless: true,
                port: Some(8000),
                sandbox: false,
                ..Default::default()
            }
        ).unwrap();
        let result = try_access_indeed_with_headless_chrome(&browser).await;
        assert!(result.is_ok());
    }
}
