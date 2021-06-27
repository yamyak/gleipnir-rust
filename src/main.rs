use std::env;

mod node;
mod chain;
mod block;

fn main() {
    // default config file path used if no path provided
    let default_path = String::from("config.ini");
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    let path: String;
    // if no command line arguments provided
    if args.len() < 2 {
        // use the default path
        path = default_path;
    }
    else
    {
        // get the command line argument
        path = args.get(1).unwrap().to_string();
    }
    let _node = node::Node::new(path);
}
