#[macro_use]
extern crate clap;
use clap::App;

extern crate console;

mod check;
mod paths;
mod opentest;

fn main(){
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    match matches.subcommand(){
        ("check", Some(args))=>{
            check::launch(args);
        },
        ("opentest", Some(args)) => {
            opentest::launch(args);
        },
        _ => {
            println!("Command not valid")
        }
    }
}