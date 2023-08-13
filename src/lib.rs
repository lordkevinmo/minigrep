use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // The args variable in main function is the owner of the argments values and is only letting
        // this function (parse_config) borrow them. Which means we'd violate Rust's borrowing rules
        // if #Config tried to take o<<nership of the values in args. That's why we use clone method.
        // There are a number of ways we could manage the String data; the easiest, though somewhat
        // inefficient, route is to call the clone method on the values. This will make a full copy
        // of the data for the Config instance to own, which takes more time and memory than storing
        // a reference to the string data. However, cloning the data also makes our code very
        // straightforward because we don’t have to manage the lifetimes of the references; in this
        // circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text: \n{contents}");

    Ok(())
}