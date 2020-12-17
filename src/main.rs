mod commands;

use std::{io, vec};
use dialoguer::{Input, Select, console::Term, theme::ColorfulTheme};
use clap::{App, Arg, ArgMatches, Clap, IntoApp, Subcommand};
use commands::*;

#[macro_use]
extern crate trait_enum;

pub trait Executable {
    fn execute(&self) -> Result<(), io::Error>;
}

trait_enum! {
    #[derive(Clap, Debug)]
    enum Command: Executable {
        Command1,
        Command2
    }
}

#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    command: Option<Command>
}

fn main() -> Result<(), io::Error> { 
    let opts = Opts::parse();

    match opts.command {
        Some(cmd) => {
            cmd.execute()
        },
        None => {
            while interactive_shell()? {
                println!();
            }
            Ok(())
        }
    }
}

fn interactive_fill_command_arguments(app: &mut App) -> Result<Option<Command>, io::Error> {
   // println!("{:#?}", app);
   // println!("-----------------");

    let arguments = app.get_arguments().collect::<Vec<_>>();

    if arguments.len() > 0 {
        println!("\n{}\n", app.clone().generate_usage());
    }
    
    let mut filled_args: Vec<String> = vec![];

    for arg in arguments {
        let input : String = Input::new()
            .with_prompt(format!("Please provide {}", arg.get_name()))
            .interact_text()?;

        filled_args.push(format!("--{}", arg.get_name()));
        filled_args.push(input);
    }

    //println!("Args: {:?}", filled_args);

    let matches = app.try_get_matches_from_mut(filled_args).map_err(|clap_error| 
        io::Error::new(io::ErrorKind::Other, clap_error.to_string()))?;

    let command = Command::from_subcommand(Some((app.get_name(), &matches)));

    Ok(command)
}

fn interactive_shell() -> Result<bool, io::Error> {
    let app:App = Opts::into_app();

    let mut subcommands = app.get_subcommands().collect::<Vec<_>>();
    
    let subcommand_names = subcommands.iter().map(|c| c.get_name()).collect::<Vec<_>>();

    let subcommand_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select what to do (q to exit):")
        .items(&subcommand_names)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    if let Some(subcommand_idx) = subcommand_selection {
        let mut subcommand = subcommands[subcommand_idx].clone();

        if let Some(cmd) = interactive_fill_command_arguments(&mut subcommand)? {
            println!();
            cmd.execute()?
        }

        Ok(true)
    } else {
        Ok(false)
    }
}