extern crate xml;

use xml::reader::{EventReader, XmlEvent};

use std::io::Read;

use std::thread;

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
            let mut urls = &mut self.urls[..];
            let chunks = urls.chunks_mut(self.concurrent as usize);

            for chunk in chunks {
                let threads: Vec<std::thread::JoinHandle<Result<i32, i32>>> = chunk.iter().map(|url| {
                    thread::spawn(move || { 
                        Ok(204)
                    })
                }).collect();

                for t in threads {
                    let result = t.join().unwrap();
                    chunk[0].code = result;

                    println!("{} is {}", chunk[0].url, result.unwrap());
                }

                let mut it = chunk.iter_mut().zip(&threads);
                while let Some((url, thread)) = it.next() {
                    let result = thread.join().unwrap();
                    println!("{} is {}", url.url, result.unwrap());
                   url.code = Ok(204);
                }

                println!("---");
            }
        }
    }
}
