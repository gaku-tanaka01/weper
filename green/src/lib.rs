use chrono::Local;
use config;
use scraper::{Html, Selector};
use serde;
use std::error::Error;
use weper_lib;
pub mod id_collections;
use csv;
use csv_config;
use id_collections::{area_id_collection, major_job_type_collection, minor_job_type_collection};
use std::fs::File;
use std::io::{BufWriter, Write};
use weper_lib::google;

// green転職サービスのエンジニア職種から1ページ目を検索結果から、会社名と求人タイトルを出力
// 検索結果は1ページ10件表示される？？　これは数えたほうがいいかも.
pub const GREEN_OFFER_PER_PAGE: u32 = 10;

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct GreenOfferInfo {
    #[serde(rename = "会社名")]
    pub company_name: String,
    #[serde(rename = "求人タイトル")]
    pub job_title: String,
    #[serde(rename = "オファー詳細URL")]
    pub offer_link: String,
    #[serde(rename = "HPリンク")]
    pub hp_link: Option<String>,
}

pub enum JobType<'a> {
    Major(&'a str),
    Minor(&'a str),
}

impl<'a> JobType<'a> {
    pub fn unwrap(&self) -> &'a str {
        match self {
            JobType::Major(s) | JobType::Minor(s) => s,
        }
    }
}

//Green求人検索結果のHTMLから求人情報を抽出します.
pub fn get_offer_info(html: &Html) -> Result<Vec<GreenOfferInfo>, Box<dyn Error>> {
    let offer_wrapper_selector = Selector::parse(".card-info__wrapper")?; //求人情報のリンク /company/{id}/job/{id}となっている。
    let company_name_selector = Selector::parse(".card-info__detail-area__box__title")?; //会社名
    let job_title_selector = Selector::parse(".card-info__heading-area__title")?; //求人タイトル

    let mut result = vec![];
    for element in html.select(&offer_wrapper_selector) {
        let offer_link = weper_lib::extract_link_from_href(&element)?;
        let company_name = weper_lib::extract_text_from_element(&element, &company_name_selector);
        let job_title = weper_lib::extract_text_from_element(&element, &job_title_selector);
        result.push(GreenOfferInfo {
            company_name: company_name,
            job_title: job_title,
            offer_link: offer_link,
            hp_link: None,
        });
    }
    Ok(result)
}

//Green求人検索結果のHTMLから求人検索結果のページ数を取得します。
pub fn get_pages_count_from_html(html: &Html) -> Result<u32, Box<dyn Error>> {
    let count_number_selector = Selector::parse(".client_count")?;
    let count_str = html
        .select(&count_number_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let pages_count_num = count_str.parse::<u32>()? / GREEN_OFFER_PER_PAGE + 1;
    Ok(pages_count_num)
}

//Green求人サイトの条件検索指定済みのURLを作成する。
//条件指定がないとき自動的に13(Tokyo)が指定される。
//理由としては　なにか条件を指定しないとgreenのリザルト画面が異なるため。

pub fn create_search_url(area_id: Option<&str>, job_id: Option<&str>) -> String {
    let mut url = String::from("https://www.green-japan.com");
    match (area_id, job_id) {
        (Some(area_id), Some(job_id)) => {
            url.push_str(&format!("/search/area/{}/job/{}", area_id, job_id));
        }
        (Some(area_id), None) => {
            url.push_str(&format!("/area/{}", area_id));
        }

        //大職種の場合 jobtype-hになる。
        (None, Some(job_id)) if job_id.len() <= 3 => {
            url.push_str(&format!("/jobtype-h/{}", job_id));
        }

        //小職種の場合 jobtype-lになる
        (None, Some(job_id)) if job_id.len() > 3 => {
            url.push_str(&format!("/jobtype-l/{}", job_id));
        }
        _ => url.push_str("/area/13"),
    }
    url
}

pub async fn run_green_scraper(args: &config::RunArgs) -> Result<(), Box<dyn Error>> {
    //日付を取得
    let now = Local::now();
    let now_str = now.format("%Y年%m月%d日").to_string();

    let (area_name, count_num) = (&args.area, args.count);
    let job_name = match (&args.main_job, &args.sub_job) {
        (Some(job_name), _) => Some(JobType::Major(&job_name)),
        (None, Some(job_name)) => Some(JobType::Minor(&job_name)),
        (None, None) => None,
    };

    let area_id = match area_name {
        Some(area_name) => area_id_collection::get_area_id(&area_name),
        None => None,
    };

    let job_id = match job_name {
        Some(JobType::Major(job_name)) => major_job_type_collection::get_major_job_id(&job_name),
        Some(JobType::Minor(job_name)) => minor_job_type_collection::get_minor_job_id(&job_name),
        None => None,
    };

    //URLを作成
    let url = create_search_url(area_id, job_id);

    println!("URL:{}", url);

    //CSVファイル名
    let file_name = format!(
        "{}_{}_{}件_{}.csv",
        &area_name.as_deref().unwrap_or("東京"),
        &job_name.unwrap_or(JobType::Major("指定なし")).unwrap(),
        &count_num.unwrap_or(1 * GREEN_OFFER_PER_PAGE),
        now_str,
    );

    let file = File::create(&file_name)?;
    //CSVにBOMを書き込みUTF-8 BOMにする excelのため
    let mut buf_writer = BufWriter::new(file);
    // UTF-8 BOMを書き込む
    match buf_writer.write_all(b"\xEF\xBB\xBF") {
        Ok(_) => {}
        Err(e) => panic!("BOM書き込み失敗\nerror:{}", e),
    };

    //BufWriterをCSVWriterに変換
    let mut writer = csv::Writer::from_writer(buf_writer);

    /* CSVファイルに書き込みをする。 */
    for i in 0..count_num.map(|c| c / GREEN_OFFER_PER_PAGE).unwrap_or(1) {
        //page指定をURLの末尾に追加
        let url_with_page_num = url.to_string() + "?page=" + &(i + 1).to_string();

        //HTMLをURLから取得
        let html = weper_lib::get_html(&url_with_page_num).await?;

        let mut offers_info = get_offer_info(&html)?;

        for object in &mut offers_info {
            let html_for_hp =
                match google::search_google_for_html_with(&format!("{} HP", &object.company_name))
                    .await
                {
                    Ok(html) => html,
                    Err(e) => {
                        eprintln!("Google検索結果取得エラー：{}", e);
                        continue;
                    }
                };

            let first_element_for_hp =
                match google::get_first_google_result_html_elemnt(&html_for_hp) {
                    Ok(element) => element,
                    Err(e) => {
                        eprintln!("{}　HP要素取得エラー：{}", &object.company_name, e);
                        continue;
                    }
                };

            //offer_infoにHPリンクを追加
            match weper_lib::extract_link_from_href(&first_element_for_hp) {
                Ok(link) => object.hp_link = Some(link),
                Err(_) => object.hp_link = Some("URL取得失敗".to_string()),
            };
        }

        match csv_config::write_to_csv(&mut writer, &offers_info) {
            Ok(_) => println!("検索結果{}ページ目の書き込み完了", i + 1),
            Err(e) => {
                eprintln!("検索結果{}ページ目の書き込み失敗\nerror:{}", i + 1, e);
                continue;
            }
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_get_offer_info() {
        let raw_html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Test HTML</title>
        </head>
        <body>
            <div class="srch-rslt">
                <div class="card-info__wrapper">
                    <a class="card-info" href="/company/1/job/1">
                    <div class="card-info__detail-area__box__title">Company A</div>
                    <div class="card-info__heading-area__title">Software Engineer</div>
                    </a>
                </div>
                <div class="card-info__wrapper">
                    <a class="card-info" href="/company/2/job/2">
                    <div class="card-info__detail-area__box__title">Company B</div>
                    <div class="card-info__heading-area__title">Product Manager</div>
                    </a>
                </div>
            </div>
        </body>
        </html>
        "#;
        let html = Html::parse_document(raw_html);
        let result = get_offer_info(&html);

        let expected = vec![
            GreenOfferInfo {
                company_name: "Company A".to_string(),
                job_title: "Software Engineer".to_string(),
                offer_link: "/company/1/job/1".to_string(),
                hp_link: None,
            },
            GreenOfferInfo {
                company_name: "Company B".to_string(),
                job_title: "Product Manager".to_string(),
                offer_link: "/company/2/job/2".to_string(),
                hp_link: None,
            },
        ];

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_get_pages_count_from_html() {
        let raw_html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Test HTML</title>
        </head>
        <body>
            <div class="client_count">103</div>
        </body>
        </html>
        "#;
        let html = Html::parse_document(raw_html);
        let result = get_pages_count_from_html(&html);
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn test_create_search_url() {
        let url = create_search_url(Some("1"), Some("2"));
        assert_eq!(url, "https://www.green-japan.com/search/area/1/job/2");
    }

    #[test]
    fn test_get_area_id() {
        let area_id = area_id_collection::get_area_id("tokyo");
        assert_eq!(area_id, Some(area_id_collection::TOKYO));
    }

    #[test]
    fn test_get_major_job_id() {
        let job_id = major_job_type_collection::get_major_job_id("engineer");
        assert_eq!(job_id, Some(major_job_type_collection::ENGINEER));
    }

    #[test]
    fn test_get_minor_job_id() {
        let job_id = minor_job_type_collection::get_minor_job_id("Backend");
        assert_eq!(job_id, Some(minor_job_type_collection::BACKEND));
    }

    #[test]
    fn test_create_search_url_with_only_area_id() {
        let url = create_search_url(Some("1"), None);
        assert_eq!(url, "https://www.green-japan.com/area/1");
    }

    #[test]
    fn test_create_serach_url_with_only_maijor_job_id() {
        let url = create_search_url(None, Some("190"));
        assert_eq!(url, "https://www.green-japan.com/jobtype-h/190");
    }

    #[test]
    fn test_create_search_url_with_only_minor_job_id() {
        let url = create_search_url(None, Some("190110"));
        assert_eq!(url, "https://www.green-japan.com/jobtype-l/190110");
    }

    #[test]
    fn test_create_search_url_with_no_condition() {
        let url = create_search_url(None, None);
        assert_eq!(url, "https://www.green-japan.com/area/13");
    }

    #[tokio::test]
    async fn test_run_green_scraper() {
        let args = config::RunArgs {
            main_job: Some("Engineer".to_string()),
            sub_job: None,
            area: None,
            count: Some(10),
        };
        let result = run_green_scraper(&args).await;
        assert_eq!(result.is_ok(), true);
    }
}
