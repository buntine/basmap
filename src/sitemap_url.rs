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
}


