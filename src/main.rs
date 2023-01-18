use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::json;

use app_switcher::alfred::AlfredItem;
use app_switcher::switcher::Switcher;

#[derive(Parser)]
#[command(name = "switcher", author = "jxq", version = "0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "FILE")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    GetApp { key: String },
    ListProfiles,
    Detail { profile: String },
    ChangeProfile { profile: String },
}

fn get_app(config: &str, key: &str) -> Result<()> {
    let switcher = Switcher::from_file(config)?;
    let app = switcher.get_app(key)?;

    print!("{}", app);

    Ok(())
}

fn list_profiles(config: &str) -> Result<()> {
    let switcher = Switcher::from_file(config)?;

    let v: Vec<AlfredItem> = switcher
        .list_profiles()
        .into_iter()
        .map(|x| AlfredItem::from(x.clone()))
        .collect();

    println!("{:#}", json!({ "items": v }));

    Ok(())
}

fn detail(config: &str, profile: &str) -> Result<()> {
    let switcher = Switcher::from_file(config)?;
    let key_to_app = switcher.get_detail(profile)?;

    let mut alfred_result: Vec<AlfredItem> = vec![];

    for (key, app) in key_to_app {
        let x = AlfredItem::new_with_sub(key, app);

        alfred_result.push(x);
    }

    println!("{:#}", json!({ "items": alfred_result }));

    Ok(())
}

fn change_profile(config: &str, profile: &str) -> Result<()> {
    let mut switcher = Switcher::from_file(config)?;

    switcher.change_profile(profile)?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = cli.config;

    match &cli.command {
        Commands::GetApp { key } => get_app(&config, key)?,
        Commands::ListProfiles => list_profiles(&config)?,
        Commands::Detail { profile } => detail(&config, profile)?,
        Commands::ChangeProfile { profile } => {
            change_profile(&config, profile)?
        }
    };

    Ok(())
}
