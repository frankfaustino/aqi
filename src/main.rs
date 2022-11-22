use color_eyre::Result;
use serde::Deserialize;
use structopt::StructOpt;

// a struct that is converted from command line arguments
// and environment variablse
#[derive(StructOpt)]
struct Aqi {
    // sets cli flags and env var as fallback
    // https://docs.rs/structopt/latest/structopt/#environment-variable-fallback
    #[structopt(
        short = "t",
        long = "token",
        env = "AQI_TOKEN"
    )]
    api_token: String,

    #[structopt(subcommand)]
    command: Opt
}

#[derive(StructOpt)]
enum Opt {
    Info,
    Search
}

#[derive(Deserialize)]
struct SearchResponse {
    data: Vec<StationAqi>
}

#[derive(Deserialize)]
struct StationAqi {
    aqi: String,
    station: serde_json::Value
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let client = reqwest::Client::new();
    let args = Aqi::from_args();

    match args.command {
        Opt::Info => {
            let response = client
                .get("https://api.waqi.info/feed/beijing/")
                .query(&[("token", args.api_token)])
                .send()
                .await?
                .json::<SearchResponse>()
                .await?;

                for station in response.data {
                    print!("{}", station.aqi)
                }
            // println!("{}", serde_json::to_string(&response)?);
        }
        Opt::Search => {
            let response = client
                .get("https://api.waqi.info/search/")
                .query(&[
                    ("token", args.api_token),
                    ("keyword", String::from("San Francisco"))
                ])
                .send()
                .await?
                .json::<SearchResponse>()
                .await?;

                for station in response.data {
                    print!(
                        "{} {}",
                        station.aqi,
                        station.station["name"]
                            .as_str()
                            .unwrap()
                    )
                }
        }
    }

    Ok(())
}
