extern crate hyper;
extern crate getopts;
extern crate basmap;

use getopts::Options;
use std::env;

use hyper::Client;
use hyper::header::Connection;

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
    options.optflag("r", "redirects", "Consider HTTP redirects (30x) successfuly");
    options.optflag("v", "verbose", "Print progress verbosely");

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
    let concurrent: usize = match matches.opt_str("c") {
        Some(c) => { c.parse::<usize>().unwrap() }
        None => { 3 }
    };
    let sleep: u32 = match matches.opt_str("s") {
        Some(m) => { m.parse::<u32>().unwrap() }
        None => { 1000 }
    };

    let client = Client::new();
    let resp = client.get(&url[..])
        .header(Connection::close())
        .send()
        .ok()
        .expect("Invalid sitemap URL");
        
    let mut basmap = Basmap::new(concurrent,
                                 sleep,
                                 verbose,
                                 redirects);

    match basmap.parse(resp) {
        Ok(n) => { println!("Fetched {} URLs from {}\n", n, url) }
        Err(e) => { panic!(e.to_string()) }
    }

    basmap.run();
    basmap.summarize();
}
