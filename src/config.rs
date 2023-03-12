use std::env;

pub struct Config {
    html_path: String,
}

impl Config {
    pub fn setup() -> Self {
        let mut args = env::args().skip(1);

        let html_path = match args.next() {
            Some(html_path) => html_path,
            None => panic!("no html path has been given"),
        };

        Self { html_path, }
    }

    pub fn get_html_path(&self) -> &str {
        &self.html_path
    }
}