//! Wikipedia scraper for biological viruses.

use std::fmt::Debug;

extern crate hyper;
use hyper::Client;
use hyper::client::response::Response;

extern crate select;
use select::predicate::{Class, Name, Predicate};
use select::document::Document;

extern crate rayon;
use rayon::prelude::*;

type Link = String;
type VirusIndex = Vec<Link>;
type ScraperError = String;

/// Attributes of a single scraped virus.
#[derive(Debug)]
pub struct Virus {
    name: String,
    group: String,
    family: String,
}

fn main() {
    let client = Client::new();
    virus_index()
        .into_par_iter()
        .filter_map(|link| virus(&client, link).ok())
        .for_each(log);
}

fn log<D: Debug>(d: D) {
    extern crate time;

    println!("{} {:?}", time::now_utc().asctime(), d);
}

fn virus(client: &Client, link: Link) -> Result<Virus, ScraperError> {
    let response = try!(response(&client, link));
    let document = document(response);
    let name = try!(document.find(Class("firstHeading")).next().ok_or("Virus name not found"));
    let group = try!(document.find(Class("group")).next().ok_or("Virus group not found"));
    let family = try!(document.find(Class("family")).next().ok_or("Virus family not found"));
    Ok(Virus {
        name: name.text(),
        group: group.text(),
        family: family.text(),
    })
}

fn response(client: &Client, link: String) -> Result<Response, String> {
    client.get(&format!("https://en.wikipedia.org{}", link))
          .send()
          .map_err(|e| e.to_string())
}

fn virus_index() -> VirusIndex {
    let virus_index_response = read_virus_index();
    log("Extracting document");
    let document = document(virus_index_response);
    log("Parsing links");
    document.find(Name("li").descendant(Name("a")))
            .filter_map(|link| link.attr("href"))
            .filter(is_virus_link)
            .map(ToOwned::to_owned)
            .collect()
}

fn read_virus_index() -> Response {
    let client = Client::new();
    log("Reading virus index");
    client.get("https://en.wikipedia.org/w/index.php?title=Special:\
                WhatLinksHere/Virus_classification&limit=2000")
          .send()
          .unwrap()
}

fn document(mut response: Response) -> Document {
    use std::io::prelude::*;

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let body_str: &str = &body;
    Document::from(body_str)
}

fn is_virus_link(link: &&str) -> bool {
    link.ends_with("virus") && link.contains("/wiki/") && !link.contains(':')
}
