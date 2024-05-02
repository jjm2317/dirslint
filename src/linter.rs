// input: ds rules, target files
// output: list of files that are not satisfied by ds rules

use crate::{glob::GlobParser, yml::YmlRule};

pub struct LintMessage {
    file: String,
    pub message: String,
}

pub struct Linter {
    rule: YmlRule,
    target_files: Vec<String>,
    messages: Vec<LintMessage>,
}

impl Linter {
    pub fn new(yml_rule: YmlRule) -> Linter {
        Linter {
            rule: yml_rule.clone(),
            target_files: GlobParser::get_all_target_files(&yml_rule),
            messages: vec![],
        }
    }

    pub fn verify(&self) -> Vec<LintMessage> {
        let mut messages = vec![];

        for target_file in self.target_files.iter() {
            for key in self.rule.ds.keys() {
                if GlobParser::is_file_match_pattern(target_file, key, Some(true)) {
                    let mut is_passed = false;
                    for pattern in self.rule.ds.get(key).unwrap().iter() {
                        if GlobParser::is_file_match_pattern(target_file, pattern, None) {
                            is_passed = true;
                            break;
                        }
                    }
                    if !is_passed {
                        messages.push(LintMessage {
                            file: target_file.clone(),
                            message: format!(
                                "file {:} in {:} doesn't satisfy rule {:?}",
                                target_file,
                                key,
                                self.rule.ds.get(key).unwrap()
                            ),
                        });
                    }
                }
            }
        }
        messages
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::yml::YmlRule;

    use super::{LintMessage, Linter};

    #[test]
    fn test_verify() {
        let mut ds = HashMap::new();
        ds.insert(
            "src/mock/**/*".to_string(),
            vec!["*.txt".to_string(), "*.yml".to_string()],
        );
        let rule = YmlRule {
            ds,
            target: vec!["src/mock/**/*".to_string()],
            ignore: vec![
                "src/mock/ignore/**".to_string(),
                "src/mock/ignore".to_string(),
            ],
        };
        let linter = Linter::new(rule);
        let expected = [LintMessage {
            file: "src/mock/fail".to_string(),
            message: "".to_string(),
        }];

        let result = linter.verify();

        assert!(result.len() > 0, "expected vec len {:?}", result.len());

        for (idx, message) in result.into_iter().enumerate() {
            assert_eq!(
                expected[idx].file, message.file,
                "file wrong, expected {:?} got {:?}",
                expected[idx].file, message.file
            );
        }
    }
}
