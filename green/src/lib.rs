use weper_lib;
use scraper::{Html, Selector};
use std::error::Error;

mod id_collections;
use id_collections::{area_id_collection, job_id_collection};
use serde;


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
    pub hp_link: Option<String>
}

//Green求人検索結果のHTMLから求人情報を抽出します.
pub fn get_offer_info(html: &Html) -> Result<Vec<GreenOfferInfo>, Box<dyn Error>> {
    
    let offer_wrapper_selector = Selector::parse(".card-info__wrapper")?; //求人情報のリンク /company/{id}/job/{id}となっている。
    let company_name_selector = Selector::parse(".card-info__detail-area__box__title")?; //会社名
    let job_title_selector = Selector::parse(".card-info__heading-area__title")?; //求人タイトル
    
    let mut result = vec![];
    for element in html.select(&offer_wrapper_selector) {
        let offer_link= weper_lib::extract_link_from_href(&element)?;
        let company_name = weper_lib::extract_text_from_element(&element, &company_name_selector);
        let job_title = weper_lib::extract_text_from_element(&element, &job_title_selector);
        result.push(GreenOfferInfo { company_name: company_name, job_title: job_title, offer_link: offer_link, hp_link:None });
    }
    Ok(result)
}


//Green求人検索結果のHTMLから求人検索結果のページ数を取得します。
pub fn get_pages_count_from_html(html: &Html) -> Result<u32, Box<dyn Error>> {
    let count_number_selector = Selector::parse(".client_count")?;
    let count_str = html.select(&count_number_selector).next().unwrap().text().collect::<String>();
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
        (None, Some(job_id)) => {
            url.push_str(&format!("/job_type-h/{}", job_id));
        }
        _ => {url.push_str("/area/13")}
    }
    url
}

pub fn get_area_id(area_name: &str) -> Option<u32> {
    match area_name.to_lowercase().as_str() {
        "tokyo" => Some(area_id_collection::TOKYO),
        "kanagawa" => Some(area_id_collection::KANAGAWA),
        "chiba" => Some(area_id_collection::CHIBA),
        "saitama" => Some(area_id_collection::SAITAMA),
        "ibaraki" => Some(area_id_collection::IBARAKI),
        "tochigi" => Some(area_id_collection::TOCHIGI),
        "gunma" => Some(area_id_collection::GUNMA),
        "hokkaido" => Some(area_id_collection::HOKKAIDO),
        "aomori" => Some(area_id_collection::AOMORI),
        "iwate" => Some(area_id_collection::IWATE),
        "miyagi" => Some(area_id_collection::MIYAGI),
        "akita" => Some(area_id_collection::AKITA),
        "yamagata" => Some(area_id_collection::YAMAGATA),
        "fukushima" => Some(area_id_collection::FUKUSHIMA),
        "niigata" => Some(area_id_collection::NIIGATA),
        "toyama" => Some(area_id_collection::TOYAMA),
        "ishikawa" => Some(area_id_collection::ISHIKAWA),
        "fukui" => Some(area_id_collection::FUKUI),
        "yamanashi" => Some(area_id_collection::YAMANASHI),
        "nagano" => Some(area_id_collection::NAGANO),
        "aichi" => Some(area_id_collection::AICHI),
        "gifu" => Some(area_id_collection::GIFU),
        "shizuoka" => Some(area_id_collection::SHIZUOKA),
        "mie" => Some(area_id_collection::MIE),
        "osaka" => Some(area_id_collection::OSAKA),
        "hyogo" => Some(area_id_collection::HYOGO),
        "kyoto" => Some(area_id_collection::KYOTO),
        "shiga" => Some(area_id_collection::SHIGA),
        "nara" => Some(area_id_collection::NARA),
        "wakayama" => Some(area_id_collection::WAKAYAMA),
        "tottori" => Some(area_id_collection::TOTTORI),
        "shimane" => Some(area_id_collection::SHIMANE),
        "okayama" => Some(area_id_collection::OKAYAMA),
        "hiroshima" => Some(area_id_collection::HIROSHIMA),
        "yamaguchi" => Some(area_id_collection::YAMAGUCHI),
        "tokushima" => Some(area_id_collection::TOKUSHIMA),
        "kagawa" => Some(area_id_collection::KAGAWA),
        "ehime" => Some(area_id_collection::EHIME),
        "kochi" => Some(area_id_collection::KOCHI),
        "fukuoka" => Some(area_id_collection::FUKUOKA),
        "saga" => Some(area_id_collection::SAGA),
        "nagasaki" => Some(area_id_collection::NAGASAKI),
        "kumamoto" => Some(area_id_collection::KUMAMOTO),
        "oita" => Some(area_id_collection::OITA),
        "miyazaki" => Some(area_id_collection::MIYAZAKI),
        "kagoshima" => Some(area_id_collection::KAGOSHIMA),
        "okinawa" => Some(area_id_collection::OKINAWA),
        "full_remote" => Some(area_id_collection::FULL_REMOTE),
        "overseas" => Some(area_id_collection::OVERSEAS),
        _ => None,
    }
}

pub fn get_job_id(job_name: &str) -> Option<u32> {
    use job_id_collection::*;
    match job_name.to_lowercase().as_str() {
        "engineer_sys_net" => Some(ENGINEER_SYS_NET),
        "creative_web" => Some(CREATIVE_WEB),
        "creative_game_media" => Some(CREATIVE_GAME_MEDIA),
        "plan_marketing" => Some(PLAN_MARKETING),
        "sales" => Some(SALES),
        "mgmt_cxo" => Some(MGMT_CXO),
        "account_admin_back" => Some(ACCOUNT_ADMIN_BACK),
        "assist_office" => Some(ASSIST_OFFICE),
        "service_personnel" => Some(SERVICE_PERSONNEL),
        "specialist_finance" => Some(SPECIALIST_FINANCE),
        "engineer_elec_mech" => Some(ENGINEER_ELEC_MECH),
        "arch_civil_plant" => Some(ARCH_CIVIL_PLANT),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            hp_link: None
        },
        GreenOfferInfo {
            company_name: "Company B".to_string(),
            job_title: "Product Manager".to_string(),
            offer_link: "/company/2/job/2".to_string(),
            hp_link: None
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
        assert_eq!(url, "https://www.green-jp.com/search/area/1/job/2");
    }

    #[test]
    fn test_get_area_id() {
        let area_id = get_area_id("tokyo");
        assert_eq!(area_id, Some(area_id_collection::TOKYO));
    }

    #[test]
    fn test_get_job_id() {
        let job_id = get_job_id("engineer_sys_net");
        assert_eq!(job_id, Some(job_id_collection::ENGINEER_SYS_NET));
    }

    #[test]
    fn test_create_serach_url() {
        let url = create_search_url(Some("1"), Some("2"));
        assert_eq!(url, "https://www.green-jp.com/search/area/1/job/2");

        let url = create_search_url(Some("1"), None);
        assert_eq!(url, "https://www.green-jp.com/area/1");

        let url = create_search_url(None, Some("2"));
        assert_eq!(url, "https://www.green-jp.com/job_type-h/2");
    }
}


