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
    fn new(yml_rule: YmlRule) -> Linter {
        let glob_parser = GlobParser::new(yml_rule);
        Linter {
            rule: yml_rule,
            target_files: glob_parser.get_all_target_files(),
            messages: vec![],
        }
    }
}
