use std::{collections::HashMap, env};

pub struct Cli {
    pub option: HashMap<String, String>,
}

impl Cli {
    pub fn new(args: Vec<String>) -> Cli {
        let mut option = HashMap::new();
        // if args has config flag insert value to config key
        let config_position = args.iter().position(|x| x == "--config");
        if config_position != None {
            option.insert(
                "config".to_string(),
                args[config_position.unwrap() + 1].clone(),
            );
        } else {
            option.insert("config".to_string(), "dslint.yml".to_string());
        }
        Cli { option }
    }
}
