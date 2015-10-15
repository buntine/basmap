extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

pub struct Basmap {
    pub url: String,
    pub concurrent: i32,
    pub sleep: i32,
    pub redirects: bool,
    pub verbose: bool,
    urls: Vec<String>,
}

impl Basmap {
    pub fn new(url: String, concurrent: i32, sleep: i32, verbose: bool, redirects: bool) -> Basmap {
        Basmap{
            url: url,
            concurrent: concurrent,
            sleep: sleep,
            verbose: verbose,
            redirects: redirects,
            urls: vec![]}
    }

    fn fetch_sitemap(&self) -> Result<String, &str> {
        let client = Client::new();
        let resp = client.get(&self.url[..])
            .header(Connection::close())
            .send();
        
        if let Ok(mut r) = resp {
            let mut body = "".to_string();

            match r.read_to_string(&mut body) {
                Ok(_) => Ok(body),
                _ => Err("Could not read HTTP response")
            }
        } else {
            Err("Invalid sitemap URL")
        }
    }

    pub fn parse(&self) -> Result<usize, &str> {
        let sitemap = match self.fetch_sitemap() {
            Ok(s) => s,
            Err(s) => { return Err(s) }
        };

        Ok(self.urls.len())
    }

    pub fn run(&self) {
        println!("URL: {}\nVERBOSE: {}\nREDIRECTS: {}\nCONCURRENT: {}\nSLEEP: {}\n", self.url, self.verbose, self.redirects, self.concurrent, self.sleep);
    }
}
