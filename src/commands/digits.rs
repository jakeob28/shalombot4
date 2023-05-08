use crate::digits::DigitsSolver;
use serenity::builder;
use serenity::model::application::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::CommandDataOptionValue::Integer;
use serenity::model::application::interaction::application_command::CommandDataOptionValue::String;
use serenity::model::prelude::command::CommandOptionType;

pub fn run(options: &[CommandDataOption]) -> std::string::String {
    let target = options
        .get(0)
        .expect("expected target")
        .resolved
        .as_ref()
        .unwrap()
        .clone();
    let nums = options
        .get(1)
        .expect("expected numbers")
        .resolved
        .as_ref()
        .unwrap()
        .clone();

    if let (Integer(target), String(mut nums)) = (target, nums) {
        nums = nums.replace(' ', "");
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
    "Couldn't read args".to_string()
}
pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("digits")
        .description("Solve a digits puzzle")
        .create_option(|option| {
            option
                .name("target")
                .description("target numer")
                .kind(CommandOptionType::Integer)
                .min_int_value(0)
                .max_int_value(10000)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("given_numbers")
                .description("the numbers you are given to work with, comma separated")
                .kind(CommandOptionType::String)
                .max_length(20)
                .required(true)
        })
}
