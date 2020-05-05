mod read;
mod setting;
mod target;
mod templates;

use clap::{load_yaml, App};
use std::fs::File;
use std::io::Read;

use crate::templates::ConksatReplacer;

fn main() -> Result<(), String> {
    let yaml = load_yaml!("../cli.yaml");
    let app = App::from_yaml(yaml);
    let args = app.clone().get_matches();
    let config_file = args.value_of("config").unwrap();

    let c = crate::setting::Config::new(config_file)?;

    let mut source: Box<dyn Read> = if let Some(file_name) = args.value_of("source") {
        Box::new(File::open(file_name).map_err(|_| "failed to open source file".to_owned())?)
    } else {
        Box::new(std::io::stdin())
    };

    let v = crate::read::read_type(c.target().clone(), &mut source)?;

    let replacer: ConksatReplacer = c.into();

    let output = replacer.replace(&v)?;

    print!("{}", output);

    Ok(())
}
