use serenity::all::CommandOptionType::{Integer, String};
use serenity::all::{CommandDataOption, CreateCommand, CreateCommandOption};

use crate::digits::DigitsSolver;

pub fn run(options: &Vec<CommandDataOption>) -> std::string::String {
    let target = options
        .get(0)
        .expect("expected target")
        .value
        .as_i64()
        .expect("expected integers");
    let mut nums = options
        .get(1)
        .expect("expected numbers")
        .value
        .as_str()
        .expect("expected string");

    let binding = nums.replace(' ', "");
    nums = &*binding;
    let nums: Vec<usize> = nums
        .split(',')
        .map(|s| s.parse().expect("error parsing"))
        .collect();
    let solver = DigitsSolver::<6>::solve(target as usize, nums);
    let solutions = solver.get_solutions();
    let mut response = "(Some) Solutions:".to_string();
    for solution in solutions {
        response += "\n||";
        for operation in solution {
            response += &*(operation.to_string() + ", ");
        }
        response += "||";
        response = response.replace(", ||", "||");
    }

    return response;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("digits")
        .description("Solve a digits puzzle")
        .add_option(
            CreateCommandOption::new(Integer, "target", "target number")
                .min_int_value(0)
                .max_int_value(10000)
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                String,
                "given_numbers",
                "the numbers you are given to work with, comma separated",
            )
            .max_length(20)
            .required(true),
        )
}
