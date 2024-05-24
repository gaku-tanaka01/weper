use reqwest::header;
use scraper::{Html, Selector};
use std::error::Error;

pub async fn get_html(url: &str) -> Result<Html, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.get(url)
                .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                .send().await?.text().await?;
    let html = Html::parse_document(&response);
    Ok(html)
}

//要素1つしか返さない。
pub fn extract_text_from_element(element: &scraper::ElementRef<'_>, selector: &Selector) -> String {
    element.select(&selector).next()
        .map_or("".to_string(), |element| element.text().collect::<String>())
}

// aタグのhrefリンクを抽出
pub fn extract_link_from_href(element: &scraper::ElementRef<'_>) -> Result<String, Box<dyn Error>> {
    let href = element.select(&Selector::parse("a")?)
        .next().map_or("".to_string(), 
        |element| element.attr("href").unwrap_or("").to_string());
    Ok(href)
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
    let title = html.select(&title_selector).next().unwrap().text().collect::<String>();
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
}




