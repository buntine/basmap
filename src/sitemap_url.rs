use hyper::status::StatusCode;

pub struct SitemapUrl {
    pub url: String,
    pub code: Result<StatusCode, StatusCode>,
}

impl SitemapUrl {
    pub fn new(url: String) -> SitemapUrl {
        SitemapUrl{
            url: url,
            code: Ok(StatusCode::Ok),
        }
    }

    pub fn is_success(&self) -> bool {
        self.code.is_ok()
    }

    pub fn status(&self) -> StatusCode {
        match self.code {
            Ok(s) => s,
            Err(s) => s,
        }
    }
}
