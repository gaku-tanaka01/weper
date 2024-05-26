use weper_lib::{self, google};
use green;
use config::Cli;
use config;
use clap::Parser;
use tokio;
use csv;
use std::fs::File;
use std::io::Write;
use std::io;
use chrono::Local;
use csv_config;
#[tokio::main]
async fn main() {

    //コマンドのクライアントを作成して引数を解析する。
    let cli = Cli::parse();
    let (job, area, count) = config::get_cli_args(cli);

    //日付を取得
    let now = Local::now();
    let now_str = now.format("%Y年%m月%d日").to_string();

    //CSVファイル名
    let file_name = format!(
        "{}_{}_{}件_{}.csv",
        &area.as_deref().unwrap_or("指定なし"),
        &job.as_deref().unwrap_or("指定なし"),
        &count.unwrap_or(1*green::GREEN_OFFER_PER_PAGE),
        now_str,
    );
    
    //areaがあったらarea_idを得る。 
    let area_id: Option<String> = if let Some(area_name) = area {
    green::get_area_id(area_name.as_str()).map(|id| id.to_string())
    } else {
        None
    };

    //jobがあったらjob_idを得る。
    let job_id = if let Some(job_name) = job {
        green::get_job_id(job_name.as_str()).map(|id| id.to_string())
    } else {
        None
    };

   // URLを作成する
    let url = green::create_search_url(
        area_id.as_deref(),
        job_id.as_deref()
    );

    println!("URL: {}", url);

    //CSVファイルの作成
    let file = match File::create(&file_name) {
        Ok(file) => {println!("{}を作成しています。", &file_name); file},
        Err(e) => panic!("CSVファイルを作成できませんでした。\nerror:{}", e),
    };

    //CSVにBOMを書き込みUTF-8 BOMにする excelのため
    let mut buf_writer = io::BufWriter::new(file);

    // UTF-8 BOMを書き込む
    match buf_writer.write_all(b"\xEF\xBB\xBF") {
        Ok(_) => {},
        Err(e) => panic!("BOM書き込み失敗\nerror:{}", e),
    };

    //BufWriterをCSVWriterに変換
    let mut writer = csv::Writer::from_writer(buf_writer);

    /* CSVファイルに書き込みをする。 */
    for i in 0..count.map(|c| c / green::GREEN_OFFER_PER_PAGE).unwrap_or(1) {

        //page指定をURLの末尾に追加
        let url_with_page_num = url.to_string() + "?page=" + &(i+1).to_string();

        //HTMLをURLから取得
        let html = match weper_lib::get_html(&url_with_page_num).await{
            Ok(html) => html,
            Err(e) => panic!("HTML取得エラー: {}", e),
        };

        let mut offers_info = match green::get_offer_info(&html) {
            Ok(offers_info) => offers_info,
            Err(e) => panic!("オファーオブジェクトの取得に失敗しました。\nerror:{}", e),
        };

        for object in &mut offers_info {
            let html_for_hp =match google::search_google_for_html_with(&format!("{} HP", &object.company_name)).await {
                Ok(html) => html,
                Err(e) => {
                    eprintln!("Google検索結果取得エラー：{}", e);
                    continue;
                }
            };

            let first_element_for_hp = match google::get_first_google_result_html_elemnt(&html_for_hp) {
                Ok(element) => element,
                Err(e) => {
                    eprintln!("{}　HP要素取得エラー：{}", &object.company_name, e);
                    continue;
                }
            };

            //offer_infoにHPリンクを追加
             match weper_lib::extract_link_from_href(&first_element_for_hp) {
                Ok(link) => object.hp_link = Some(link),
                Err(_) => object.hp_link =  Some("URL取得失敗".to_string())
            };            
        }
        
        match csv_config::write_to_csv(&mut writer, &offers_info) {
            Ok(_) => println!("検索結果{}ページ目の書き込み完了", i+1),
            Err(e) => {
                eprintln!("検索結果{}ページ目の書き込み失敗\nerror:{}", i+1, e);
                continue;
            },
        };
    }
    println!("正常に完了しました。");

}

 
