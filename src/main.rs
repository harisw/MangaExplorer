extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    scrape_manga("https://mangajar.com/manga");
}

fn scrape_manga(url: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    // let body = resp.
    // assert!(resp.status().is_success());
    // println!("{:?}", resp);
    Document::from_read(resp)
            .unwrap()
            .find(Name("article"))
            .filter_map(|n| n.find(Name("a")).nth(0))
            .for_each(|x| println!("{:?}", x.attr("href").unwrap()));

            // .for_each(|x| println!("{}", x.unwrap()))
    //             .for_each(|n| n.find(Name("a")).attr("href") )
    
}