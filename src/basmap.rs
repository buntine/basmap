extern crate hyper;
extern crate xml;

use hyper::Client;
use hyper::client::response::Response;
use hyper::header::Connection;

use xml::reader::{EventReader, XmlEvent};

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

    fn fetch_sitemap(&self) -> Result<Response, &str> {
        let client = Client::new();
        let resp = client.get(&self.url[..])
            .header(Connection::close())
            .send();
        
        match resp {
            Ok(r) => Ok(r),
            _ => Err("Invalid sitemap URL")
        }
    }

    fn parse_sitemap(&mut self, sitemap: Response) -> Result<usize, &str> {
        let parser = EventReader::new(sitemap);
        let mut in_loc = false;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement{name, ..}) => {
                    if &name.local_name[..] == "loc" && name.prefix.is_none() {
                        in_loc = true;
                    }
                }
                Ok(XmlEvent::Characters(b)) => {
                    if in_loc {
                        self.urls.push(b);
                    }
                }
                _ => { in_loc = false; }
            }
        }

        Ok(self.urls.len())
    }

    pub fn parse(&self) -> Result<usize, &str> {
        let sitemap = match self.fetch_sitemap() {
            Ok(s) => s,
            Err(s) => { return Err(s) }
        };

        self.parse_sitemap(sitemap)
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
