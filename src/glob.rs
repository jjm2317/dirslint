use std::{collections::HashSet, path::Path};

use glob::{glob, Pattern};

use crate::yml::YmlRule;

pub struct GlobParser {
    get_all_target_files: fn(YmlRule) -> Vec<String>,
    is_file_match_pattern: fn(&str, &str) -> bool,
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

    pub fn is_file_match_pattern(filename: &str, pattern: &str, is_strict: Option<bool>) -> bool {
        println!("{:?} {:?}", filename, pattern);
        /*
         * return true if filename's last path match pattern.
         * e.g.
         * - "src/mock/test.yml" match pattern "test.yml"
         * - "src/mock/success" match pattern "success"
         */
        if filename.contains("/") && !is_strict.unwrap_or(false) {
            let last_path = filename.split("/").last().unwrap();
            if Pattern::new(pattern)
                .unwrap()
                .matches_path(Path::new(last_path))
            {
                return true;
            }
        }
        Pattern::new(pattern)
            .unwrap()
            .matches_path(Path::new(filename))
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
    #[test]
    fn test_is_file_match_pattern() {
        let filename = "src/mock/test.yml";
        let pattern = "*.yml";
        let result = GlobParser::is_file_match_pattern(filename, pattern, Some(true));

        assert_eq!(
            true, result,
            "file is not match pattern expected {:?}, got {:?}",
            true, result,
        );

        let dirname = "src/mock/success";
        let dirs = ["src/mock/success", "success"];
        for dir in dirs.iter() {
            let dir_result = GlobParser::is_file_match_pattern(dirname, dir, None);
            assert_eq!(
                true, dir_result,
                "dir {:?} is not match pattern {:?} expected {:?}, got {:?}",
                dirname, dir, true, dir_result,
            )
        }
    }
}
