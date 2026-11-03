use clap::Parser;
use std::time::Duration;
use ureq::Agent;

#[derive(Parser)]
struct Args {
    /// City name to get the weather for
    #[arg(short, long)]
    city: Option<String>,

    /// Output format (1 classic format , 2 for Data-rich format)
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    format: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let weather = get_weather(args.city, args.format)?;
    println!("{}", weather);
    Ok(())
}

fn get_weather(city: Option<String>, format: u8) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = match format {
        1 => "https://wttr.is".to_string(),
        2 => "https://v2n.wttr.is".to_string(),
        _ => unreachable!(),
    };
    let url = match city {
        Some(city) => format!("{}/{}", base_url, urlencoding::encode(&city)),
        None => base_url,
    };
    println!("Fetching weather from: {}", url);
    let agent: Agent = Agent::config_builder()
        .https_only(true)
        .timeout_per_call(Some(Duration::from_secs(5)))
        .user_agent("curl")
        .build()
        .into();
    let mut responce = agent.get(&url).call()?;
    let text = responce.body_mut().read_to_string()?;
    Ok(text)
}
