use clap::{self, Subcommand, Parser};

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
    #[arg(short, long, help = "職種の指定 例: engineer_sys_net")]
    job: Option<String>,

    /// 指定された地域の求人を検索します。
    #[arg(short, long, help = "地域の指定 例: tokyo")]
    area: Option<String>,

    /// 取得する求人情報の数を指定します。
    #[arg(short, long, help = "取得する求人数の指定")]
    count: Option<u32>,
}

pub fn get_cli_args(cli: Cli) -> (Option<String>, Option<String>, Option<u32>) {
    match cli.commands {
        Commands::Run(args) => {
            (args.job, args.area, args.count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cli_args() {
        let args = RunArgs { job: Some("test_job".to_string()), 
                                      area: Some("test_area".to_string()),
                                      count: Some(1),
                                    };
        
        let cli = Cli { commands: Commands::Run(args) };

        let (job, area, count) = get_cli_args(cli);

        assert_eq!(job.unwrap(), "test_job");
        assert_eq!(area.unwrap(), "test_area");
        assert_eq!(count.unwrap(), 1);

        let args = RunArgs { job: None, area: None, count: None };
        let cli = Cli { commands: Commands::Run(args) };

        let (job, area, count) = get_cli_args(cli);

        assert_eq!(job, None);
        assert_eq!(area, None);
        assert_eq!(count, None);
    }
}

