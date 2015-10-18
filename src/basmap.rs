extern crate xml;
extern crate hyper;

use xml::reader::{EventReader, XmlEvent};

use std::io::Read;

use std::sync::Arc;
use std::thread;

use hyper::Client;
use hyper::header::Connection;
use hyper::status::StatusCode;

pub struct SitemapUrl {
    url: String,
    code: Result<StatusCode, StatusCode>,
}

impl SitemapUrl {
    pub fn new(url: String) -> SitemapUrl {
        SitemapUrl{
            url: url,
            code: Ok(StatusCode::Ok),
        }
    }
}

pub struct Basmap {
    pub concurrent: usize,
    pub sleep: u32,
    pub redirects: bool,
    pub verbose: bool,
    urls: Vec<SitemapUrl>,
}

impl Basmap {
    pub fn new(concurrent: usize, sleep: u32, verbose: bool, redirects: bool) -> Basmap {
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

            for chunk in urls.chunks_mut(self.concurrent) {
                let threads: Vec<std::thread::JoinHandle<Result<StatusCode, StatusCode>>> = chunk.iter().map(|url| {
                    let sync_client = client.clone();
                    let full_url = String::from(&url.url[..]);

                    thread::spawn(move || { 
                        let resp = sync_client.head(&full_url[..])
                            .header(Connection::keep_alive())
                            .send();

                        match resp {
                            Ok(r) => {
                                let status = r.status;
                                if status.is_success() { Ok(status) } else { Err(status) }
                            }
                            _ => Err(StatusCode::Unregistered(0)),
                        }
                    })
                }).collect();

                let mut it = chunk.iter_mut().zip(threads);

                while let Some((url, thread)) = it.next() {
                    let result = thread.join().unwrap();
                    url.code = result;

                    match self.verbose {
                        true => println!("{} is {}", url.url, result.unwrap()),
                        false => {
                            print!("{}", if result.is_ok() {"."} else {"x"});
                            io::stdout().flush();
                        }
                    }
                }

                match self.verbose {
                    true => println!("--- Sleeping for {} milliseconds ---", self.sleep),
                    false => { 
                        print!(", ");
                        io::stdout().flush();
                    }
                }

                thread::sleep_ms(self.sleep);
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
