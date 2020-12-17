use std::{thread, time::Duration};

use clap::{App, Arg, ArgMatches, Clap, IntoApp, Subcommand};
use indicatif::ProgressBar;

use crate::Executable;
#[derive(Clap, Debug)]
#[clap(about = "Runs fake unit tests")]
pub struct RunUnitTests {
    #[clap(short, long, about = "Run all tests (takes longer time)")]
    all_tests: bool
}

impl Executable for RunUnitTests {
    fn execute(&self) -> Result<(), std::io::Error> {
        if self.all_tests {
            println!("All tests:");
            run_fake_tests(117);
        } else {
            println!("Essential tests:");
            run_fake_tests(23);
        }
        println!("Done!");
        Ok(())
    }
}

fn run_fake_tests(n: u32) {
    let bar = ProgressBar::new(n as u64);
    for _ in 0..n {
        bar.inc(1);
        thread::sleep(Duration::from_millis(10));
    }
    bar.finish();
}