use clap::{self, Subcommand};

#[derive(clap::Parser,Debug)]
#[command(name = "weper_cli")]
#[command(version = "1.0")]
#[command(about = "求人サイトの情報をもってくるツールです。", long_about = None)]
#[command(author = "田中　樂空")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "run")]
    Run(RunArgs),
}

#[derive(clap::Args, Debug)]
pub struct RunArgs {
    /// 指定された職種の求人を検索します。
    #[arg(short, long, help = "大職種の指定 例: engineer", conflicts_with = "sub_job")]
    pub main_job: Option<String>,

    #[arg(short, long, help = "小職種の指定 例: backend", conflicts_with = "main_job")]
    pub sub_job: Option<String>,

    /// 指定された地域の求人を検索します。
    #[arg(short, long, help = "地域の指定 例: tokyo")]
    pub area: Option<String>,

    /// 取得する求人情報の数を指定します。
    #[arg(short, long, help = "取得する求人数の指定")]
    pub count: Option<u32>,
}

pub fn get_cli_args(cli: Cli) -> (Option<String>, Option<String>, Option<String>, Option<u32>) {
    match cli.commands {
        Commands::Run(args) => {
            (args.main_job, args.sub_job, args.area, args.count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cli_args_with_main_job() {
        let args = RunArgs { main_job: Some("test_job".to_string()), 
                                    sub_job: None,
                                      area: Some("test_area".to_string()),
                                      count: Some(1),
                                    };
        
        let cli = Cli { commands: Commands::Run(args) };

        let (main_job, sub_job, area, count) = get_cli_args(cli);

        assert_eq!(main_job.unwrap(), "test_job");
        assert_eq!(sub_job, None);
        assert_eq!(area.unwrap(), "test_area");
        assert_eq!(count.unwrap(), 1);
                                }

    #[test]
    fn test_get_cli_args_without_all_args() {
        let args = RunArgs { main_job: None, sub_job: None, area: None, count: None };
        let cli = Cli { commands: Commands::Run(args) };

        let (main_job, sub_job, area, count) = get_cli_args(cli);

        assert_eq!(main_job, None);
        assert_eq!(sub_job, None);
        assert_eq!(area, None);
        assert_eq!(count, None);
    }
}

