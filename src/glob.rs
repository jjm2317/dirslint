use std::collections::HashSet;

use glob::{glob, Pattern};

use crate::yml::YmlRule;

pub struct GlobParser {
    get_all_target_files: fn(YmlRule) -> Vec<String>,
}

impl GlobParser {
    pub fn get_all_target_files(yml_rule: &YmlRule) -> Vec<String> {
        let mut result = HashSet::new();
        // glob result for all target and ignore pattern satisfied
        for target in yml_rule.target.iter() {
            glob(target)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|path| {
                    yml_rule.ignore.iter().all(|ignore_pattern| {
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
    use crate::{glob::GlobParser, yml::YmlRule};

    #[test]
    fn test_get_all_target_files() {
        let expected_result = vec![
            "src/mock/test.yml".to_string(),
            "src/mock/a.txt".to_string(),
        ]
        .sort();
        let rule = YmlRule {
            ds: std::collections::HashMap::new(),
            target: vec!["src/mock/**/*".to_string()],
            ignore: vec![
                "src/mock/ignore/**".to_string(),
                "src/mock/ignore".to_string(),
            ],
        };
        let result = GlobParser::get_all_target_files(&rule).sort();

        assert_eq!(
            expected_result, result,
            "target files are not valid expected {:?}, got {:?}",
            expected_result, result,
        );
    }
}
