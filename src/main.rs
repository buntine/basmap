extern crate getopts;

use getopts::Options;
use std::env;

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

    if (matches.opt_present("h")) {
        print_usage(&program, options);
        return;
    }
}
