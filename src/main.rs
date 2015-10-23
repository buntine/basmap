extern crate hyper;
extern crate getopts;
extern crate basmap;
extern crate flate2;

use getopts::Options;
use std::env;

use hyper::Client;
use hyper::header::Connection;

use std::io::prelude::*;
use flate2::read::GzDecoder;

use basmap::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} URL [options]", program);

    println!("{}", opts.usage(&brief));
}

fn build_options() -> Options {
    let mut options = Options::new();

    options.optopt("c", "", "Amount of concurrent HTTP requests (default 3)", "NUMBER");
    options.optopt("s", "", "Milliseconds to sleep between requests (default 1000)", "NUMBER");
    options.optflag("h", "help", "Print this help menu");
    options.optflag("r", "redirects", "Consider HTTP redirects (30x) successful");
    options.optflag("z", "gzip", "Decode gzip response");
    options.optflag("v", "verbose", "Print verbose summary");
    options.optflag("", "google", "Ping Sitemap to Google");
    options.optflag("", "bing", "Ping Sitemap to Bing");
    options.optopt("", "min-ping", "Minimum success rate % required to ping search engines (default 100)", "NUMBER");

    options
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let options = build_options();
    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&program, options);
        return;
    }

    let url = matches.free[0].clone();
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
            let decoded = match GzDecoder::new(&bytes[..]) {
                Ok(d) => d,
                Err(_) => panic!("Unable to decode Gzipped response"),
            };

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
            println!("Successfully pinged Google");
        }

        if ping_bing {
            println!("Successfully pinged Bing");
        }
    }
}
