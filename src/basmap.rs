#[derive(Default)]
pub struct Basmap {
    pub url: String,
    pub concurrent: i32,
    pub sleep: i32,
    pub redirects: bool,
    pub verbose: bool,
    urls: Vec<String>,
}

impl Basmap {
    pub fn new(url: String, concurrent: i32, sleep: i32, verbose: bool, redirects: bool) -> Basmap {
        Basmap{
            url: url,
            concurrent: concurrent,
            sleep: sleep,
            verbose: verbose,
            redirects: redirects,
            urls: vec![]}
    }

    pub fn parse(&self) -> Result<i32, &str> {
        Ok(32)
    }

    pub fn run(&self) {
        println!("URL: {}\nVERBOSE: {}\nREDIRECTS: {}\nCONCURRENT: {}\nSLEEP: {}\n", self.url, self.verbose, self.redirects, self.concurrent, self.sleep);
    }
}
