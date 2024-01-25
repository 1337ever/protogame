use crate::body::organs::{liver::Liver, stomach::Stomach, Organs};
use crate::reagents::*;
use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use clap::Parser;

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
pub struct ExampleCommand {
    /// Some message
    msg: String,
}

pub fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        println!("{}", msg);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "addreagent")]
pub struct AddReagentCommand {
    reagent: Reagent,
}

pub fn add_reagent_command(mut log: ConsoleCommand<AddReagentCommand>) {
    if let Some(Ok(AddReagentCommand { reagent })) = log.take() {}
}
