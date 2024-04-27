mod commands;
mod formatting;

use clap::{Parser, Subcommand};
use url::Url;

#[derive(Debug, Parser)]
#[clap(version, about)]
struct Cli {
    /// URL of the schedule API to consume
    #[clap(
        long,
        env,
        value_name = "URL",
        default_value = "https://www.emfcamp.org/schedule/2022.json"
    )]
    api_url: Url,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the complete schedule
    Full(commands::full::FullOptions),

    /// Show the complete scheule minus events from the past
    Upcoming(commands::upcoming::UpcomingOptions),

    /// Show the EPG style now and next for venue(s)
    NowNext(commands::now_next::NowNextOptions),

    /// Show details for a specific event
    Details(commands::details::EventDetailsOptions),

    /// List all venues
    Venues,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let client = emfcamp_schedule_api::Client::new(args.api_url);

    let schedule = client.get_schedule().await;

    match args.command {
        Command::Full(args) => commands::full::run(args, schedule),
        Command::Upcoming(args) => commands::upcoming::run(args, schedule),
        Command::NowNext(args) => commands::now_next::run(args, schedule),
        Command::Details(args) => commands::details::run(args, schedule),
        Command::Venues => commands::venues::run(schedule),
    }
}
