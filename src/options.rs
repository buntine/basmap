use getopts::Options;
use getopts::Matches;

pub struct OptionBuilder {
    options: Options,
}

pub struct OptionManager {
    matches: Matches,
}

impl OptionBuilder {
    pub fn new() -> OptionBuilder {
        let mut options = Options::new();

        options.optopt("c", "", "Amount of concurrent HTTP requests (default 3)", "NUMBER");
        options.optopt("s", "", "Milliseconds to sleep between requests (default 1000)", "NUMBER");
        options.optflag("h", "help", "Print this help menu");
        options.optflag("r", "redirects", "Consider HTTP redirects (30x) successful");
        options.optflag("z", "gzip", "Decode gzipped response");
        options.optflag("v", "verbose", "Print verbose summary");
        options.optflag("", "google", "Ping Sitemap to Google");
        options.optflag("", "bing", "Ping Sitemap to Bing");
        options.optopt("", "min-ping", "Minimum success rate % required to ping search engines (default 100)", "NUMBER");

        OptionBuilder{options: options}
    }

    pub fn print_usage(&self, program: &str) {
        let brief = format!("Usage: {} URL [options]", program);
        println!("{}", self.options.usage(&brief));
    }

    pub fn parse(&self, args: &[String]) -> Result<OptionManager, String> {
        match self.options.parse(args) {
            Ok(m) => Ok(OptionManager::new(m)),
            Err(f) => Err(f.to_string()),
        }
    }
}

impl OptionManager {
    pub fn new(matches: Matches) -> OptionManager {
        OptionManager{matches: matches}
    }

    pub fn wants_help(&self) -> bool {
        self.matches.opt_present("h") || self.matches.free.is_empty()
    }

    pub fn url(&self) -> String {
        self.matches.free[0].clone()
    }

    pub fn verbose(&self) -> bool {
        self.matches.opt_present("v")
    }

    pub fn redirects(&self) -> bool {
        self.matches.opt_present("r")
    }

    pub fn gzip(&self) -> bool {
        self.matches.opt_present("z")
    }

    pub fn ping_google(&self) -> bool {
        self.matches.opt_present("google")
    }

    pub fn ping_bing(&self) -> bool {
        self.matches.opt_present("bing")
    }
}
