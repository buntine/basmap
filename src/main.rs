extern crate hyper;
extern crate flate2;
extern crate basmap;

use std::env;

use hyper::Client;
use hyper::header::Connection;

use std::io::stdout;
use std::io::prelude::*;
use flate2::read::GzDecoder;

use basmap::basmap::Basmap;
use basmap::options::OptionBuilder;

fn send_ping(client: &Client, engine: &str, url: String) {
    print!("Pinging {}... ", engine);
    stdout().flush().ok();

    match client.get(&url[..]).send() {
        Ok(_) => println!("Success"),
        Err(_) => println!("Failed"),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let options = OptionBuilder::new();
    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.wants_help() {
        options.print_usage(&program);
        return;
    }

    let url = matches.url();
    let verbose = matches.opt_present("v");
    let redirects = matches.opt_present("r");
    let gzip = matches.opt_present("z");
    let ping_google = matches.opt_present("google");
    let ping_bing = matches.opt_present("bing");
    let concurrent: usize = match matches.opt_str("c") {
        Some(c) => { c.parse::<usize>().ok().expect("Invalid concurrency value") }
        None => { 5 }
    };
    let sleep: u32 = match matches.opt_str("s") {
        Some(m) => { m.parse::<u32>().ok().expect("Invalid sleep value") }
        None => { 1000 }
    };
    let min_ping: f32 = match matches.opt_str("min-ping") {
        Some(m) => { 
            let min = m.parse::<f32>().ok().expect("Invalid minimum ping success rate");
            if min >= 0.0 && min <= 100.0 { min } else { 100.0 }
        }
        None => { 100.0 }
    };

    let client = Client::new();
    let resp = client.get(&url[..])
        .header(Connection::close())
        .send()
        .ok()
        .expect("Invalid sitemap URL");
        
    let mut basmap = Basmap::new(concurrent, sleep, verbose, redirects);

    {
        let parsed = if gzip {
            let bytes: Vec<u8> = resp.bytes().map(|b| b.unwrap()).collect();
            let decoded = GzDecoder::new(&bytes[..]).ok().expect("Unable to decode Gzipped response");

            basmap.parse(decoded)
        } else {
            basmap.parse(resp)
        };

        println!("Fetched {} URLs from {}\n", parsed, url);
    }

    basmap.run();
    let success_rate = basmap.summarize();

    if success_rate >= min_ping {
        if ping_google {
            send_ping(&client, "Google",
                      format!("http://www.google.com/webmasters/sitemaps/ping?sitemap={}", url));
        }

        if ping_bing {
            send_ping(&client, "Bing",
                      format!("http://www.bing.com/webmaster/ping.aspx?siteMap={}", url));
        }
    }
}
