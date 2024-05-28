use clap::Parser;
use config::{Cli, Commands};
use green;

#[tokio::main]
async fn main() {
    //コマンドのクライアントを作成して引数を解析する。
    let cli = Cli::parse();

    match cli.commands {
        Commands::Run(args) => {
            let result = green::run_green_scraper(&args).await;
            match result {
                Ok(_) => {}
                Err(e) => eprintln!("エラーが発生しました: {}", e),
            }
        }
    }

    println!("正常に完了しました。");
}
