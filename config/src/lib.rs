use clap;

#[derive(clap::Parser, Debug)]
#[command(name = "weper_cli")]
#[command(version = "1.0")]
#[command(about = "求人サイトの情報をもってくるツールです。", long_about = None)]
#[command(author = "田中　樂空")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Run(RunArgs),
}

#[derive(clap::Subcommand, Debug)]
pub enum RunArgs {
    #[command(name = "green", about = "Greenの求人情報を取得します")]
    Green(GreenArgs),
    #[command(name = "indeed", about = "Indeedの求人情報を取得します")]
    Indeed(IndeedArgs),
}

// greenのコマンドの引数
#[derive(clap::Args, Debug)]
pub struct GreenArgs {
    /// 指定された職種の求人を検索します。
    #[arg(
        short,
        long,
        help = "大職種の指定 例: engineer",
        conflicts_with = "sub_job"
    )]
    pub main_job: Option<String>,

    #[arg(
        short,
        long,
        help = "小職種の指定 例: backend",
        conflicts_with = "main_job"
    )]
    pub sub_job: Option<String>,

    /// 指定された地域の求人を検索します。
    #[arg(short, long, help = "地域の指定 例: tokyo")]
    pub area: Option<String>,

    /// 取得する求人情報の数を指定します。
    #[arg(short, long, help = "取得する求人数の指定")]
    pub count: Option<u32>,
}

// indeedのコマンドの引数
#[derive(clap::Args, Debug)]
pub struct IndeedArgs {
    /// 指定された職種の求人を検索します。
    #[arg(short, long, help = "職種関連の検索キーワード")]
    pub job_word: Option<String>,
    /// 指定された地域の求人を検索します。
    #[arg(short, long, help = "地域関連の検索キーワード")]
    pub area_word: Option<String>,
    #[arg(short, long, help = "取得する求人数の指定")]
    pub count: Option<u32>,
}
