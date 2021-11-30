extern crate clap;
use clap::{App, Arg};
use std::{error, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("MyApp")
        .version("0.1")
        .author("Yixian Du")
        .about(".gitignore file generator")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity level"),
        )
        .arg(
            Arg::with_name("features")
                .required(true)
                .takes_value(true)
                .min_values(1)
                .help("features"),
        )
        .get_matches();

    let features: Vec<&str> = matches.values_of("features").unwrap().clone().collect();
    let url = format!(
        "https://www.toptal.com/developers/gitignore/api/{}",
        features.join(",")
    );
    println!("Generate .gitignore from {}", url);
    let content = reqwest::get(url).await?.text().await?;
    let mut file = std::fs::File::create(".gitignore").expect("Failed to create .gitignore");
    file.write_all(content.as_bytes())
        .expect("Failed to write to .gitignore");
    Ok(())
}
