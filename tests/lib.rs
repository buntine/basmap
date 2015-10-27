extern crate basmap;
extern crate hyper;

use basmap::basmap::Basmap;
use basmap::sitemap_url::SitemapUrl;

use std::fs::File;
use std::io::BufReader;

use hyper::status::StatusCode;

#[test]
fn new_basmap() {
    let basmap = Basmap::new(10, 1000, true, true);

    assert_eq!(basmap.concurrent, 10);
    assert_eq!(basmap.sleep, 1000);
    assert_eq!(basmap.redirects, true);
    assert_eq!(basmap.verbose, true);
}

#[test]
fn valid_parse() {
    let mut basmap = Basmap::new(10, 1000, true, true);
    let file = File::open("./tests/valid.xml").unwrap();
    let reader = BufReader::new(file);
    let count = basmap.parse(reader);

    assert_eq!(count, 5);
}

#[test]
fn invalid_parse() {
    let mut basmap = Basmap::new(10, 1000, true, true);
    let file = File::open("./tests/invalid.xml").unwrap();
    let reader = BufReader::new(file);
    let count = basmap.parse(reader);

    assert_eq!(count, 0);
}

#[test]
fn new_sitemap_url() {
    let su = SitemapUrl::new("http://www.swagger.cool/home/".to_string());

    assert_eq!(su.url, "http://www.swagger.cool/home/".to_string());
    assert_eq!(su.code, Ok(StatusCode::Ok));
}

#[test]
fn sitemap_url_is_success() {
    let su = SitemapUrl::new("http://www.swagger.cool/home/".to_string());

    assert!(su.is_success());
}

#[test]
fn sitemap_url_is_not_success() {
    let mut su = SitemapUrl::new("http://www.swagger.cool/home/".to_string());

    su.code = Err(StatusCode::NotFound);

    assert_eq!(su.is_success(), false);
}

#[test]
fn sitemap_url_status_ok () {
    let su = SitemapUrl::new("http://www.swagger.cool/home/".to_string());

    assert_eq!(su.status(), StatusCode::Ok);
}

#[test]
fn sitemap_url_status_err () {
    let mut su = SitemapUrl::new("http://www.swagger.cool/home/".to_string());

    su.code = Err(StatusCode::NotFound);

    assert_eq!(su.status(), StatusCode::NotFound);
}
