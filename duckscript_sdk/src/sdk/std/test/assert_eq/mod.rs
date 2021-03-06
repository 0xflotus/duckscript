use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "AssertEquals")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["assert_eq".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Crash("Assert failed, two arguments are required.".to_string())
        } else {
            let passed = arguments[0] == arguments[1];

            if passed {
                CommandResult::Continue(Some("true".to_string()))
            } else {
                let error_message = if arguments.len() == 2 {
                    format!(
                        "Assert failed, value: {} does not match: {}",
                        &arguments[0], &arguments[1]
                    )
                    .to_string()
                } else {
                    arguments[2].clone()
                };

                CommandResult::Crash(error_message)
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
