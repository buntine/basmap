extern crate xml;

use xml::reader::{EventReader, XmlEvent};

use std::io::Read;

pub struct SitemapUrl {
    url: String,
    success: bool,
}

impl SitemapUrl {
    pub fn new(url: String) -> SitemapUrl {
        SitemapUrl{
            url: url,
            success: false
        }
    }
}

pub struct Basmap {
    pub concurrent: i32,
    pub sleep: i32,
    pub redirects: bool,
    pub verbose: bool,
    urls: Vec<SitemapUrl>,
}

impl Basmap {
    pub fn new(concurrent: i32, sleep: i32, verbose: bool, redirects: bool) -> Basmap {
        Basmap{
            concurrent: concurrent,
            sleep: sleep,
            verbose: verbose,
            redirects: redirects,
            urls: vec![]
        }
    }

    pub fn parse<T: Read>(&mut self, reader: T) -> Result<usize, &str> {
        let parser = EventReader::new(reader);
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
                        self.urls.push(SitemapUrl::new(b));
                    }
                }
                _ => { in_loc = false; }
            }
        }

        Ok(self.urls.len())
    }

    pub fn run(&self) {
        // Make iterator from self.urls of groups of self.concurrent
        // For each group, spawn N threads. Each returns bool of success.
        // Wait for each. Join back and record success.
        // Repeat until exhausted.
        // Report on total success vs fail.

        if self.urls.is_empty() {
            println!("No URLs to check!");
        } else {
            let urls = &self.urls[..];

            for chunk in urls.chunks(self.concurrent as usize) {
                for url in chunk {
                    println!("Spawning thread");
                }
                println!("---");
            }
        }
    }
}
