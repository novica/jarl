use crate::rule_table::RuleTable;

pub(crate) mod any_duplicated;
pub(crate) mod any_is_na;
pub(crate) mod class_equals;
pub(crate) mod duplicated_arguments;
pub(crate) mod empty_assignment;
pub(crate) mod equal_assignment;
pub(crate) mod equals_na;
pub(crate) mod grepv;
pub(crate) mod length_levels;
pub(crate) mod length_test;
pub(crate) mod lengths;
pub(crate) mod redundant_equals;
pub(crate) mod true_false_symbol;
pub(crate) mod which_grepl;

/// List of supported rules and whether they have a safe fix.
///
/// Possible categories:
/// * CORR: correctness, code that is outright wrong or useless
/// * SUSP: suspicious, code that is most likely wrong or useless
/// * PERF: performance, code that can be written to run faster
/// * READ: readibility, code is correct but can be written in a way that is
///         easier to read.
pub fn all_rules_and_safety() -> RuleTable {
    let mut rule_table = RuleTable::empty();
    rule_table.enable("any_duplicated", "PERF", true, None);
    rule_table.enable("any_is_na", "PERF", true, None);
    rule_table.enable("class_equals", "SUSP", true, None);
    rule_table.enable("duplicated_arguments", "SUSP", true, None);
    rule_table.enable("empty_assignment", "READ", true, None);
    rule_table.enable("equal_assignment", "READ", true, None);
    rule_table.enable("equals_na", "CORR", true, None);
    rule_table.enable("grepv", "READ", true, Some((4, 5)));
    rule_table.enable("length_levels", "PERF,READ", true, None);
    rule_table.enable("length_test", "CORR", true, None);
    rule_table.enable("lengths", "PERF,READ", true, None);
    rule_table.enable("redundant_equals", "READ", true, None);
    rule_table.enable("true_false_symbol", "READ", false, None);
    rule_table.enable("which_grepl", "PERF,READ", true, None);
    rule_table
}
