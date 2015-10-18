extern crate xml;
extern crate hyper;

use xml::reader::{EventReader, XmlEvent};

use std::io::Read;

use std::sync::Arc;
use std::thread;

use hyper::Client;
use hyper::header::Connection;

pub struct SitemapUrl {
    url: String,
    code: Result<i32, i32>,
}

impl SitemapUrl {
    pub fn new(url: String) -> SitemapUrl {
        SitemapUrl{
            url: url,
            code: Ok(200),
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

    pub fn run(&mut self) {
        if self.urls.is_empty() {
            println!("No URLs to check!");
        } else {
            let client = Arc::new(Client::new());
            let mut urls = &mut self.urls[..];

            for chunk in urls.chunks_mut(self.concurrent as usize) {
                let threads: Vec<std::thread::JoinHandle<Result<i32, i32>>> = chunk.iter().map(|url| {
                    let sync_client = client.clone();
                    let full_url = String::from(&url.url[..]);

                    thread::spawn(move || { 
                        let resp = sync_client.head(&full_url[..])
                            .header(Connection::keep_alive())
                            .send();

                        match resp {
                            Ok(r) => Ok(204),
                            _ => Err(0),
                        }
                    })
                }).collect();

                let mut it = chunk.iter_mut().zip(threads);

                while let Some((url, thread)) = it.next() {
                    let result = thread.join().unwrap();
                    url.code = result;

                    println!("{} is {}", url.url, url.code.unwrap());
                }

                println!("---");
            }
        }
    }

    pub fn summarize(&self) {
        // Try fold using a tuple.
        let (success, fail): (Vec<_>, Vec<_>) = self.urls.iter().partition(|&u| u.code.is_ok());

        println!("TOTAL SUCCESS: {}", success.len());
        println!("TOTAL FAIL: {}", fail.len());
    }
}
