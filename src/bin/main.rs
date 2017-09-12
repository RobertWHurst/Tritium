extern crate tritium;
#[macro_use]
extern crate clap;
use std::process;
use std::path::PathBuf;
use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = get_cli_matches();
    let config_path = get_config_path(matches);

    let mut proxy = tritium::Proxy::new();
    proxy.load_config(config_path);
    proxy.run();
}

fn get_cli_matches<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Sets the config location")
                .takes_value(true),
        )
        .get_matches()
}

fn get_config_path(matches: ArgMatches) -> PathBuf {
    match matches.value_of("config") {
        Some(p) => PathBuf::from(p),
        None => {
            println!("CONFIG_FILE is required. See -h for help.");
            process::exit(1);
        }
    }
}
