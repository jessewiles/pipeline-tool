extern crate argparse;

pub mod things;

use argparse::{ArgumentParser, Store, StoreTrue};
use serde_yaml::{self};
use things::Pipelines;

include!("yaml.rs");

fn main() {
    let mut config_path = "config.yml".to_string();
    let mut verbose = false;
    let mut name = "World".to_string();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Pipeline runner tool.");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut name)
            .add_option(&["--name"], Store, "Name for the greeting");
        ap.refer(&mut config_path).add_argument(
            "Config path",
            Store,
            "Path to the pipeline tool configuration file.",
        );
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", name);
    }
    println!("Hello {}!", name);
    let pconfig: things::Pipelines = load_config(&config_path);
    println!("{:?}", pconfig);
}
