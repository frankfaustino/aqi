use color_eyre::Result;
// use inflector::Inflector;
use serde::Deserialize;
use clap::{Parser, Subcommand};

// a struct that is converted from command line args and env vars
#[derive(Parser)]
struct Aqi {
    // sets cli flags and env var as fallback
    // https://docs.rs/structopt/latest/structopt/#environment-variable-fallback
    #[arg(
        short,
        long,
        env = "AQI_TOKEN"
    )]
    api_token: String,

    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    Info { url: String },
    Search { keyword: String }
}

#[derive(Deserialize)]
struct InfoResponse {
    data: StationInfo
}

#[derive(Deserialize)]
struct StationInfo {
    aqi: u32,
    // attributions: serde_json::Value,
    city: City,
    // forecast: serde_json::Value,
    // iaqi: serde_json::Value
}

#[derive(Deserialize)]
struct City {
    name: String,
    // url: String
}

#[derive(Deserialize)]
struct SearchResponse {
    data: Vec<StationAqi>
}

#[derive(Deserialize)]
struct StationAqi {
    aqi: String,
    station: Station
}

#[derive(Deserialize)]
struct Station {
    name: String,
    url: String
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let client = reqwest::Client::new();
    let Aqi { api_token, command } = Aqi::parse();

    match command {
        Command::Info { url } => {
            let response = client
                .get(format!("https://api.waqi.info/feed/{}/", url))
                .query(&[("token", api_token)])
                .send()
                .await?
                .json::<InfoResponse>()
                .await?;

            let StationInfo { city, aqi, .. } = response.data;
            println!("{} aqi: {}", city.name, aqi);
        }
        Command::Search { keyword } => {
            let response = client
                .get("https://api.waqi.info/search/")
                .query(&[
                    ("token", api_token),
                    ("keyword", keyword)
                ])
                .send()
                .await?
                .json::<SearchResponse>()
                .await?;

            for station in response.data {
                print!(
                    "{:2} {} {} \n",
                    station.aqi,
                    station.station.name,
                    station.station.url,
                )
            }
        }
    }

    Ok(())
}
