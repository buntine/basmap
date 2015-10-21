extern crate xml;
extern crate hyper;
extern crate ansi_term;

use std::collections::HashMap;

use std::io::Read;
use std::io::Write;
use std::io::stdout;

use std::sync::Arc;
use std::thread;

use xml::reader::{EventReader, XmlEvent};

use hyper::Client;
use hyper::header::Connection;
use hyper::status::StatusCode;

use ansi_term::Colour;
use ansi_term::Colour::{Red, Green};
//use ansi_term::Style;

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

    pub fn parse<T: Read>(&mut self, reader: T) -> usize {
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

        self.urls.len()
    }

    pub fn run(&mut self) {
        if self.urls.is_empty() {
            println!("No URLs to check!");
        } else {
            let client = Arc::new(Client::new());
            let mut urls = &mut self.urls[..];
            let redirects = self.redirects;

            for chunk in urls.chunks_mut(self.concurrent) {
                let threads: Vec<std::thread::JoinHandle<Result<StatusCode, StatusCode>>> = chunk.iter().map(|url| {
                    let sync_client = client.clone();
                    let full_url = url.url.to_string();

                    thread::spawn(move || { 
                        let resp = sync_client.head(&full_url[..])
                            .header(Connection::close())
                            .send();

                        match resp {
                            Ok(r) => {
                                let success = r.status.is_success() || (redirects && r.status.is_redirection());
                                if success {Ok(r.status)} else {Err(r.status)}
                            }
                            _ => Err(StatusCode::Unregistered(0)),
                        }
                    })
                }).collect();

                let mut it = chunk.iter_mut().zip(threads);

                while let Some((url, thread)) = it.next() {
                    let result = thread.join().unwrap();
                    url.code = result;

                    print!("{}", if result.is_ok() {Green.paint(".")} else {Red.paint("x")});
                    stdout().flush().ok();
                }

                print!(", ");
                stdout().flush().ok();

                thread::sleep_ms(self.sleep);
            }
        }
    }

    fn status_code_hash<'a>(&'a self, urls: &Vec<&'a SitemapUrl>) -> HashMap<StatusCode, Vec<&'a str>> {
        let mut codes = HashMap::new();

        for u in urls {
            let status = match u.code {
                Ok(s) => s,
                Err(s) => s,
            };

            codes.entry(status).or_insert(Vec::new());

            if let Some(sc) = codes.get_mut(&status) {
                sc.push(&u.url[..]);
            }
        }

        codes
    }

    fn summarize_results(&self, results: &HashMap<StatusCode, Vec<&str>>, verbose: &bool, colour: Colour) {
        if !results.is_empty() {
            for (code, urls) in results {
                let code_str = code.to_string();
                let title = colour.underline().bold().paint(&code_str[..]);

                println!("{}: {}\n", title, urls.len());

                if *verbose {
                    for u in urls {
                        println!("  - {}", u);
                    }
                    print!("\n")
                }
            }
        }
    }

    pub fn summarize(&self) {
        let (total_success, total_fail): (Vec<_>, Vec<_>) = self.urls.iter().partition(|&u| u.code.is_ok());
        let success_hash = self.status_code_hash(&total_success);
        let fail_hash = self.status_code_hash(&total_fail);

        println!("\n");

        self.summarize_results(&success_hash, &self.verbose, Green);
        self.summarize_results(&fail_hash, &true, Red);
    }
}
