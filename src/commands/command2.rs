use clap::{App, Arg, ArgMatches, Clap, IntoApp, Subcommand};

use crate::Executable;
#[derive(Clap, Debug)]
#[clap(about = "Another useless command")]
pub struct Command2 {}

impl Executable for Command2 {
    fn execute(&self) -> Result<(), std::io::Error> {
        println!("Hello world!");
        Ok(())
    }
}