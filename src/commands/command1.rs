use clap::{App, Arg, ArgMatches, Clap, IntoApp, Subcommand};

use crate::Executable;
#[derive(Clap, Debug)]
pub struct Command1 {
    name: String
}

impl Executable for Command1 {
    fn execute(&self) -> Result<(), std::io::Error> {
        println!("Hello!");
        Ok(())
    }
}