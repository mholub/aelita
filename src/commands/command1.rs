use clap::{App, Arg, ArgMatches, Clap, IntoApp, Subcommand};

use crate::Executable;
#[derive(Clap, Debug)]
#[clap(about = "Test command, does nothing useful, has argument")]
pub struct Command1 {
    name: String
}

impl Executable for Command1 {
    fn execute(&self) -> Result<(), std::io::Error> {
        println!("Hello!");
        Ok(())
    }
}