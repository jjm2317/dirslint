use std::env;

pub mod cli;
pub mod glob;
pub mod linter;
pub mod yml;
fn main() {
    let args: Vec<String> = env::args().collect();

    let cli = cli::Cli::new((&args[1..]).to_vec());

    let config_file_path = cli.option.get("config");

    let yml_parser = yml::YmlParser::new(config_file_path.unwrap());
}
