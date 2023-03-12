use std::{sync::RwLock, fs};

use config::Config;
use dom::Dom;

mod utils;
mod dom;
mod config;

fn main() {
    
    let config = RwLock::new(Config::setup());

    let html = fs::read_to_string(config.read().unwrap().get_html_path())
        .expect("Was not able to read the file");
    
    let mut dom = Dom::new();
    dom.parse_html(html);

    println!("{}", dom);

}