use std::collections::HashSet;

use glob::{glob, Pattern};

use crate::yml::YmlRule;

pub struct GlobParser {
    yml_rule: YmlRule,
}

impl GlobParser {
    pub fn new(yml_rule: YmlRule) -> GlobParser {
        GlobParser { yml_rule }
    }
    pub fn get_all_target_files(&self) -> Vec<String> {
        let mut result = HashSet::new();
        // glob result for all target and ignore pattern satisfied
        for target in self.yml_rule.target.iter() {
            glob(target)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|path| {
                    self.yml_rule.ignore.iter().all(|ignore_pattern| {
                        !Pattern::new(&ignore_pattern).unwrap().matches_path(path)
                    })
                })
                .for_each(|path| {
                    result.insert(path.display().to_string());
                });
        }
        Vec::from_iter(result)
    }
}

#[cfg(test)]
mod test {
    use crate::yml::YmlRule;

    use super::GlobParser;

    #[test]
    fn test_get_all_target_files() {
        let glob_parser = GlobParser::new(YmlRule {
            ds: std::collections::HashMap::new(),
            target: vec!["src/*".to_string(), "./".to_string()],
            ignore: vec!["node_modules".to_string(), ".env*".to_string()],
        });

        glob_parser.get_all_target_files();
        todo!("implement test for get_all_target_files")
    }
}
