//use std::env;
use std::process;
use std::error::Error;

extern crate clap;
use clap::{Arg, App, SubCommand};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Payee")]
    payee: String,
    #[serde(rename = "Account number")]
    account_number: String,
    #[serde(rename = "Transaction type")]
    transaction_type: String,
    #[serde(rename = "Payment reference")]
    payment_reference: String,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Amount (EUR)")]
    amount: Option<f64>,
    #[serde(rename = "Amount (Foreign Currency)")]
    foreign_currency: Option<f64>,
    #[serde(rename = "Type Foreign Currency")]
    type_foreign_crrency: String,
    #[serde(rename = "Exchange Rate")]
    exchange_rate: String,
}

fn main() {
    let matches = App::new("bank2ledger")
                          .version("0.1")
                          .author("mkfd")
                          .about("Convert bank csv to ledger format")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("test")
                                      .about("controls testing features")
                                      .version("1.3")
                                      .author("Someone E. <someone_else@other.com>")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)


    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    let filename = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", filename);
    if let Err(err) = run(filename.to_string()) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    // more program logic goes here...
}

fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
