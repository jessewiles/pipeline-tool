extern crate argparse;

pub mod pipelines;

use argparse::{ArgumentParser, Store, StoreTrue};
use pipelines::Pipelines;
use serde_yaml::{self};

include!("yaml.rs");

fn main() {
    let mut pipelines_path = "pipelines.yml".to_string();
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
        ap.refer(&mut pipelines_path).add_argument(
            "Config path",
            Store,
            "Path to the pipeline tool configuration file.",
        );
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("name is {}", name);
    }

    let ppipelines: pipelines::Pipelines = load_pipelines(&pipelines_path);
    println!("{:?}", ppipelines);
}
