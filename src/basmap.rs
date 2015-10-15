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

    fn parse_sitemap(&self, sitemap: &String) -> Result<usize, &str> {
        // Parse out all <loc> values from sitemap.
        // Return Ok(self.urls.len())

        Ok(self.urls.len())
    }

    pub fn parse(&self) -> Result<usize, &str> {
        let sitemap = match self.fetch_sitemap() {
            Ok(s) => s,
            Err(s) => { return Err(s) }
        };

        self.parse_sitemap(&sitemap)
    }

    pub fn run(&self) {
        // Make iterator from self.urls of groups of self.concurrent
        // For each group, spawn N threads. Each returns bool of success.
        // Wait for each. Join back and record success.
        // Repeat until exhausted.
        // Report on total success vs fail.

        if self.urls.is_empty() {
            println!("No URLs to check!");
        }
    }
}
