// input: ds rules, target files
// output: list of files that are not satisfied by ds rules

use crate::{glob::GlobParser, yml::YmlRule};

struct LintMessage {
    file: String,
    message: String,
}

struct Linter {
    rule: YmlRule,
    target_files: Vec<String>,
    messages: Vec<LintMessage>,
}

impl Linter {
    fn new(yml_rule: &YmlRule) -> Linter {
        Linter {
            rule: yml_rule.clone(),
            target_files: GlobParser::get_all_target_files(yml_rule),
            messages: vec![],
        }
    }
}
