use headless_chrome;
use reqwest::header;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;

pub mod google;

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct OfferInfo {
    #[serde(rename = "会社名")]
    pub company_name: String,
    #[serde(rename = "求人タイトル")]
    pub job_title: String,
    #[serde(rename = "オファー詳細URL")]
    pub offer_link: String,
    #[serde(rename = "HPリンク")]
    pub hp_link: Option<String>,
}

pub async fn get_html(url: &str) -> Result<Html, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client.get(url)
                .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                .send().await?.text().await?;

    let html = Html::parse_document(&response);
    Ok(html)
}

pub async fn get_html_with_headless_chrome(url: &str) -> Result<Html, Box<dyn Error>> {
    let browser = headless_chrome::Browser::new(headless_chrome::LaunchOptions {
        headless: false, // ヘッドレスモードで実行
        ..Default::default()
    })?;

    // 新しいタブを開く
    let tab = browser.new_tab()?;

    // urlにアクセス
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    // ページのHTMLを取得
    let raw_html = tab.get_content()?;
    let html = Html::parse_document(&raw_html);
    Ok(html)
}

// 求人情報を取得するためのパラメータ　base_url以外はdot付きクラス名を入力
pub struct GetOfferInfoParams {
    pub base_url: &'static str,
    pub offer_wrapper_selector: &'static str,
    pub company_name_selector: &'static str,
    pub job_title_selector: &'static str,
}

// HTML内の会社名、求人タイトル、オファー詳細URLの配列を返す。
pub fn get_offer_info(
    html: &Html,
    params: &GetOfferInfoParams,
) -> Result<Vec<OfferInfo>, Box<dyn Error>> {
    let offer_wrapper_selector = Selector::parse(params.offer_wrapper_selector)?; //求人情報のリンク /company/{id}/job/{id}となっている。
    let company_name_selector = Selector::parse(params.company_name_selector)?; //会社名
    let job_title_selector = Selector::parse(params.job_title_selector)?; //求人タイトル

    let mut result = vec![];
    for element in html.select(&offer_wrapper_selector) {
        let offer_link = format!(
            "{}{}",
            params.base_url,
            extract_link_from_href(&element).unwrap_or("".to_string())
        );

        let company_name = extract_text_from_element(&element, &company_name_selector);
        let job_title = extract_text_from_element(&element, &job_title_selector);
        result.push(OfferInfo {
            company_name: company_name,
            job_title: job_title,
            offer_link: offer_link,
            hp_link: None,
        });
    }
    Ok(result)
}

//要素1つしか返さない。
pub fn extract_text_from_element(element: &scraper::ElementRef<'_>, selector: &Selector) -> String {
    element
        .select(&selector)
        .next()
        .map_or("".to_string(), |element| element.text().collect::<String>())
}

// 要素内の最初のaタグのhrefリンクを抽出
pub fn extract_link_from_href(element: &scraper::ElementRef<'_>) -> Option<String> {
    let href = element
        .select(&Selector::parse("a").ok()?)
        .next()
        .and_then(|element| element.attr("href").map(String::from));
    href
}

pub fn create_unique_file(filename: &str) -> Result<fs::File, Box<dyn Error>> {
    let mut index = 1;
    loop {
        let unique_filename = format!("{}({}).csv", &filename, index);
        let result = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&unique_filename);

        match result {
            Ok(file) => {
                println!("File created: {}", unique_filename);
                return Ok(file);
            }
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
                index += 1; // Increment the index and try the next file name
            }
            Err(e) => return Err(Box::new(e) as Box<dyn Error>), // Propagate other kinds of errors
        }
    }
}

pub async fn get_hp_link(company_name: &str) -> Result<Option<String>, Box<dyn Error>> {
    let result_hp_google =
        google::search_google_for_html_with(format!("{} HP", company_name).as_str()).await?;

    let first_element = google::get_first_google_result_html_element(&result_hp_google)?;
    let hp_link = extract_link_from_href(&first_element);
    Ok(hp_link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_html() {
        let url = "https://www.google.com";
        let title_selector = Selector::parse("title").unwrap();
        let result = get_html(url).await;
        let html = match result {
            Ok(html) => html,
            Err(e) => panic!("Failed to fetch HTML: {}", e),
        };
        let title = html
            .select(&title_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        assert_eq!(title, "Google");
    }

    #[test]
    fn test_extract_text_from_element() {
        let raw_html = r#"
    <div class="card-info__wrapper">
        <a href="/company/1/job/1" class="card-info">
            <div class="card-info__detail-area__box__title">Company Name</div>
            <div class="card-info__heading-area__title">Job Title</div>
        </a>
    </div>
    "#;
        let html = Html::parse_document(raw_html);
        let element_selector = Selector::parse(".card-info__wrapper").unwrap();
        let element = html.select(&element_selector).next().unwrap();
        let title_selector = Selector::parse(".card-info__heading-area__title").unwrap();
        let result = extract_text_from_element(&element, &title_selector);
        assert_eq!(result, "Job Title");
    }

    #[test]
    fn test_extract_link_from_href() {
        let raw_html = r#"
    <div class="card-info__wrapper">
        <a href="/company/1/job/1" class="card-info">
            <div class="card-info__detail-area__box__title">Company Name</div>
            <div class="card-info__heading-area__title">Job Title</div>
        </a>
    </div>
    "#;
        let html = Html::parse_document(raw_html);
        let element_selector = Selector::parse(".card-info__wrapper").unwrap();
        let element = html.select(&element_selector).next().unwrap();
        let result = extract_link_from_href(&element);
        assert_eq!(result.unwrap(), "/company/1/job/1".to_string());
    }

    #[tokio::test]
    async fn test_get_hp_link() {
        let company_name = "株式会社ヴァレイン";
        let result = get_hp_link(company_name).await;
        assert_eq!(
            result.unwrap().unwrap(),
            "https://valleyin.co.jp/".to_string()
        );
    }
}
