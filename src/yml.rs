use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct YmlRule {
    pub ds: HashMap<String, Vec<String>>,
    target: Vec<String>,
    ignore: Vec<String>,
}

pub struct YmlParser {
    filename: String,
    pub content: String,
    pub yml_rule: YmlRule,
}

impl YmlParser {
    //https://parsiya.net/blog/2022-10-16-yaml-wrangling-with-rust/
    pub fn new(filename: &str) -> YmlParser {
        let content = std::fs::read_to_string(filename).unwrap();
        let yml_rule = serde_yaml::from_str::<YmlRule>(&content).unwrap();
        let parser = YmlParser {
            filename: filename.to_string(),
            content,
            yml_rule: yml_rule,
        };

        // todo!("validate the yml file");

        parser
    }

    fn validate() {
        todo!("implement logic for validating the yml file");
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::yml::YmlRule;

    use super::YmlParser;

    #[test]
    fn test_constructor() {
        let yml_parser = YmlParser::new("./src/test.yml");
        let mut ds: HashMap<String, Vec<String>> = HashMap::new();
        ds.insert(
            String::from("src/[hook|components]/*"),
            vec![String::from("*.(ts|tsx|js|jsx)"), String::from("")],
        );

        ds.insert(
            String::from("**/src/"),
            vec![String::from("use*.(ts|tsx|js|jsx)"), String::from("")],
        );
        let expected_yml_rule: YmlRule = YmlRule {
            ds,
            target: vec!["src/*".to_string(), "./".to_string()],
            ignore: vec!["node_modules".to_string(), ".env*".to_string()],
        };
        assert_eq!(
            expected_yml_rule, yml_parser.yml_rule,
            "parsed yml rule is not valid expected {:?}, got {:?}",
            expected_yml_rule, yml_parser.yml_rule,
        );
    }
}
