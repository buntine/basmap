extern crate basmap;

use basmap::basmap::Basmap;
use std::fs::File;
use std::io::BufReader;

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
