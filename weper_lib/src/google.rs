use url::{self, Url};
use scraper::{Selector, Html};
use std::error::Error;
use std::io::Write;

const GOOGLE_URL: &str = "https://www.google.com/search";

// Googleの検索結果の最初の結果を返す。　クラス名は変更される可能性があるためメンテナンスが必要？
pub fn get_first_google_result_html_elemnt(html: &Html) -> Result<scraper::ElementRef, Box<dyn Error>> {
    let first_result_class_name = ".hlcw0c";//CSSセレクタが認証できるように<「.」をつけたクラス名にする。変更される恐れあり

    let search_result_selector = Selector::parse(&first_result_class_name)?;    
    let mut first_result_element = html.select(&search_result_selector).next();

    // 最初の試みで結果が見つからなかった場合
    if first_result_element.is_none() {
        // クラス名を変更して再度試みる
        let alternative_class_name = ".sATSHe"; // ここに変更するクラス名を指定
        let alternative_selector = Selector::parse(&alternative_class_name)?;
        first_result_element = html.select(&alternative_selector).next();
    }

    match first_result_element {
        Some(first_result_element) => Ok(first_result_element),
        None => {
            let mut file = crate::create_unique_file("error_output")?;

            // HTMLの内容を取得してファイルに書き込む
            let html_string = html.root_element().html();
            file.write_all(html_string.as_bytes())?;

            // 最終的にエラーを返す
            Err("Googleの最初の結果が取得できませんでした。".into())
        }
    }
}
// 引数serach_wordでGoogle検索を実行し、結果のHTMLを返す。
pub async fn search_google_for_html_with(search_word:&str) -> Result<Html, Box<dyn Error>> {
    // serach_wordをパースしてURLクエリに変更
    let url = Url::parse_with_params(&GOOGLE_URL, &[("q", &search_word)]);
    match url {
        Ok(url) => crate::get_html(&url.as_str()).await,
        Err(e) => Err(Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
#[test]
fn test_get_first_google_result_html_element_with_class() {
    let raw_html = r#"
    <div class="hlcw0c">
        <a href="https://example.com">Example</a>
    </div>
    <span>Some Other HTML tag</span>
    "#;

    let html = Html::parse_document(&raw_html);
    let result = get_first_google_result_html_elemnt(&html);

    assert!(result.is_ok());
        if let Ok(element) = result {
            assert_eq!(element.value().name(), "div");
            assert!(element.value().has_class("hlcw0c", scraper::CaseSensitivity::AsciiCaseInsensitive));
        }
}

#[test]
fn test_get_first_google_result_html_element_without_class() {
    let raw_html = r#"
    <div class="some_other_class"><a href="https://example.com">Example</a></div>
    <p class="some_other">Other Class Sample</p>
    "#;
    let html = Html::parse_document(&raw_html);

    let result = get_first_google_result_html_elemnt(&html);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "Googleの最初の結果が取得できませんでした");
    }
}

#[tokio::test]
async fn test_search_google_for_html() {
    let serach_word = "最高";
    let result = search_google_for_html_with(serach_word).await;
    assert!(result.is_ok());
}
}