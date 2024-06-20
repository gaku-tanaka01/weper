use clap::Parser;
use config::{Cli, Commands, RunArgs};
use green;
use indeed;

#[tokio::main]
async fn main() {
    //コマンドのクライアントを作成して引数を解析する。
    let cli = Cli::parse();
    match &cli.commands {
        Commands::Run(run_args)=> match run_args {
            RunArgs::Green(green_args) => {
                println!("Greenの求人情報を取得します: {:?}", green_args);
                match green::run_green_scraper(green_args).await {
                    Ok(()) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            RunArgs::Indeed(indeed_args) => {
                println!("Indeedの求人情報を取得します: {:?}", indeed_args);
                match indeed::run_indeed_scraper(indeed_args).await {
                    Ok(()) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        },
    }

    println!("正常に完了しました。");
}
