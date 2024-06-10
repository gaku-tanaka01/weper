use config;
use csv_config;
use std::error::Error;
use std::io::BufWriter;
use std::io::Write;
use weper_lib;

const BASE_URL: &str = "https://jp.indeed.com";
const INDEED_OFFER_PER_PAGE: u32 = 15;

pub async fn run_indeed_scraper(args: &config::IndeedArgs) -> Result<(), Box<dyn Error>> {
    let mut area_name = args.area_word.clone();
    let job_name = &args.job_word;
    if area_name.is_none() && job_name.is_none() {
        area_name = Some("東京都".to_string());
    }
    let count = &args.count.unwrap_or(15) / INDEED_OFFER_PER_PAGE * INDEED_OFFER_PER_PAGE;
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

    println!("url:{}", url);
    for i in 0..(count / INDEED_OFFER_PER_PAGE) {
        let url_with_page_num = format!("{}&start={}", url, i * (INDEED_OFFER_PER_PAGE - 5));

        let html = weper_lib::get_html_with_headless_chrome(&url_with_page_num).await?;
        let mut offers_info = weper_lib::get_offer_info(
            &html,
            &weper_lib::GetOfferInfoParams {
                base_url: &BASE_URL,
                offer_wrapper_selector: ".job_seen_beacon",
                company_name_selector: ".css-92r8pb",
                job_title_selector: ".jcs-JobTitle",
            },
        )?;
        // println!("html:{:#?}", html);

        // println!("offers_info:{:#?}", offers_info);
        for offer_info in &mut offers_info {
            let hp_link = match weper_lib::get_hp_link(&offer_info.company_name).await {
                Ok(hp_link) => hp_link,
                Err(e) => {
                    println!("HPリンク取得失敗: {}", e);
                    continue;
                }
            };
            match hp_link {
                Some(hp_link) => {
                    offer_info.hp_link = Some(hp_link);
                }
                None => {
                    offer_info.hp_link = Some("URL取得失敗".to_string());
                }
            }
        }

        match csv_config::write_to_csv(&mut writer, &offers_info) {
            Ok(_) => {
                println!(
                    "{}件のデータ書き込み完了",
                    { i + 1 } * INDEED_OFFER_PER_PAGE
                );
            }
            Err(e) => {
                println!("CSV書き込み失敗: {}", e);
                return Err(Box::new(e));
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
            count: Some(30),
        };

        let result = run_indeed_scraper(&args).await;
        eprintln!("result:{:#?}", result);
        assert!(result.is_ok());
    }
}
