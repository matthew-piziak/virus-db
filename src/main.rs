use std::io::prelude::*;

extern crate hyper;

extern crate select;
use select::predicate::Name;

pub struct VirusIndex {
    links: Vec<String>,
}

fn main() {
    let virus_db = virus_db().expect("Could not load virus database");
    let client = hyper::Client::new();
    for link in virus_db.links {
        println!("Reading {:?}", link);
        let response = client.get(&format!("https://en.wikipedia.org{}", link))
                             .send()
                             .unwrap();
        let document = document(response);
        for node in document.find(select::predicate::Class("group")).iter() {
            println!("Group: {:?}", node.text());
        }
        for node in document.find(select::predicate::Class("family")).iter() {
            println!("Family: {:?}", node.text());
        }
    }
}

fn virus_db() -> Result<VirusIndex, &'static str> {
    let client = hyper::Client::new();
    println!("Reading list of viruses");
    let response = client.get("https://en.wikipedia.org/w/index.php?title=Special:\
                               WhatLinksHere/Virus_classification&limit=2000")
                         .send()
                         .unwrap();
    println!("Extracting document");
    let document = document(response);
    println!("Parsing links");
    let links = document.find(Name("li"))
                        .find(Name("a"))
                        .iter()
        .filter_map(|link| link.attr("href").map(ToOwned::to_owned))
        .filter(is_virus_link)
        .collect();
    return Ok(VirusIndex { links: links });
}

fn document(mut response: hyper::client::response::Response) -> select::document::Document {
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let body_str: &str = &body;
    select::document::Document::from(body_str)
}

fn is_virus_link(link: &String) -> bool {
    link.contains("/wiki/") && !link.contains(":")
}
