# aelita
Interactive shell wrapper around *clap* (https://docs.rs/clap/3.0.0-beta.2/clap/index.html). Intended to be used by less techy people who are not very comfortable with console.
It takes clap Opts struct and adds interactive shell support to it.

So instead of 

```
my-app subcommand -arg1 -arg2 somevalue
```
You just run 
```
my-app
```
And it interactively asks you which subcommand to run, requests arguments from you and so on.

![Screenshot](doc/prototype.gif)

## Example definition of subcommand
Subcommand is any struct deriving *Clap* trait and implementing Executable trait
```rust
pub trait Executable {
    fn execute(&self) -> Result<(), io::Error>;
}
```
Like this fake unit tests command
```rust
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
```


Single evening prototype implementation. Barely works. Doesn't support much of clap features (like default values, different argument types, etc) yet.
