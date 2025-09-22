use clap::{Parser, Subcommand};

mod config;
mod http_client;
mod job_processor;
mod scraper;
mod show_jobs;
mod test;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Parser)]
#[command(name = "oxidation")]
#[command(about = "A CLI application built with Rust")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize configuration file")]
    Init,
    #[command(about = "Show current configuration")]
    Config,
    #[command(about = "Test concurrent job processing")]
    Test,
    #[command(about = "Default command that shows help")]
    Default,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            if let Err(e) = config::Config::init() {
                eprintln!("Error initializing config: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Config) => async_config().await,

        Some(Commands::Test) => test::test_concurrent_jobs().await,

        Some(Commands::Default) | None => scrape_jobs_postings().await,
    }
}

async fn scrape_jobs_postings() {
    match config::Config::load() {
        Ok(config) => {
            println!("Starting scraper...\n");
            let postings = scraper::scrape_jobs_postings(&config).await;
            show_jobs::render_result(postings);
        }
        Err(e) => {
            eprintln!("Error loading config: {e}");
            std::process::exit(1);
        }
    };
}

async fn async_config() {
    match config::Config::load() {
        Ok(config) => {
            println!("Current configuration:");
            if let Ok(toml_str) = toml::to_string_pretty(&config) {
                println!("{toml_str}");
            }
            if let Some(path) = config::Config::config_file_path() {
                println!("Config file location: {}", path.display());
            }
        }
        Err(e) => {
            eprintln!("Error loading config: {e}");
            std::process::exit(1);
        }
    };
}
