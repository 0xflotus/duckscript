use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = indexof", "out");
}

#[test]
fn run_single_arg() {
    test::run_script_and_error(vec![create("")], "out = indexof", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = indexof "    some  text   " some "#,
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}

#[test]
fn run_found_multiple() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = indexof "    some some text   " some "#,
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = indexof "    some  text   " txt "#,
        CommandValidation::None,
    );
}
