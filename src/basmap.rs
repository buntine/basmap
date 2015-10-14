pub struct Basmap {
    pub url: String,
    pub concurrent: i32,
    pub sleep: i32,
    pub redirects: bool,
    pub verbose: bool,
}

impl Basmap {
    pub fn run(&self) {
        println!("URL: {}\nVERBOSE: {}\nREDIRECTS: {}\nCONCURRENT: {}\nSLEEP: {}\n", self.url, self.verbose, self.redirects, self.concurrent, self.sleep);
    }
}
