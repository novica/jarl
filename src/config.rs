use crate::utils::parse_rules_cli;

#[derive(Clone)]
pub struct Config<'a> {
    pub rules: Vec<&'a str>,
}

pub fn build_config(rules_cli: &str) -> Config {
    let rules = parse_rules_cli(rules_cli);

    Config { rules }
}
