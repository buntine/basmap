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
    let client = Client::new();
    let resp = client.get(&url[..])
        .header(Connection::close())
        .send()
        .ok()
        .expect("Invalid sitemap URL");
        
    let mut basmap = Basmap::new(matches.concurrent(), matches.sleep(),
                                 matches.verbose(), matches.redirects());

    {
        let parsed = if matches.gzip() {
            let bytes: Vec<u8> = resp.bytes().map(|b| b.ok().expect("IO error")).collect();
            let decoded = GzDecoder::new(&bytes[..]).ok().expect("Unable to decode Gzipped response");

            basmap.parse(decoded)
        } else {
            basmap.parse(resp)
        };

        println!("Fetched {} URLs from {}\n", parsed, url);
    }

    basmap.run();
    let success_rate = basmap.summarize();

    if success_rate >= matches.min_ping() {
        if matches.ping("google") {
            send_ping(&client, "Google",
                      format!("http://www.google.com/webmasters/sitemaps/ping?sitemap={}", url));
        }

        if matches.ping("bing") {
            send_ping(&client, "Bing",
                      format!("http://www.bing.com/webmaster/ping.aspx?siteMap={}", url));
        }
    }
}
