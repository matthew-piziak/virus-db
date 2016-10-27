use std::io::prelude::*;

extern crate hyper;
use hyper::client::response::Response;

extern crate select;
use select::predicate::Name;
use select::predicate::Class;

extern crate rayon;
use rayon::prelude::*;

type Link = String;
type VirusIndex = Vec<Link>;

#[derive(Debug)]
pub struct Virus {
    name: String,
    group: String,
    family: String,
}

fn main() {
    let mut virus_db = virus_db().expect("Could not load virus database");
    virus_db.par_iter_mut().for_each(|link| {
        let client = hyper::Client::new();
        if let Ok(virus) = virus(&client, link.clone()) {
            println!("{:?}", virus);
        }
    });
}

fn virus(client: &hyper::Client, link: String) -> Result<Virus, String> {
    let response = try!(response(&client, link));
    let document = document(response);
    let name = try!(document.find(Class("firstHeading")).first().ok_or("Virus name not found"));
    let group = try!(document.find(Class("group")).first().ok_or("Virus group not found"));
    let family = try!(document.find(Class("family")).first().ok_or("Virus family not found"));
    Ok(Virus {
        name: name.text(),
        group: group.text(),
        family: family.text(),
    })
}

fn response(client: &hyper::Client, link: String) -> Result<Response, String> {
    client.get(&format!("https://en.wikipedia.org{}", link))
          .send()
          .map_err(|e| e.to_string())
}

fn virus_db() -> Result<VirusIndex, &'static str> {
    let virus_index_response = read_virus_index();
    println!("Extracting document");
    let document = document(virus_index_response);
    println!("Parsing links");
    let links = document.find(Name("li"))
                        .find(Name("a"))
                        .iter()
                        .filter_map(|link| link.attr("href").map(ToOwned::to_owned))
                        .filter(is_virus_link)
                        .collect();
    return Ok(links);
}

fn read_virus_index() -> Response {
    let client = hyper::Client::new();
    println!("Reading list of viruses");
    client.get("https://en.wikipedia.org/w/index.php?title=Special:\
                WhatLinksHere/Virus_classification&limit=2000")
          .send()
          .unwrap()
}

fn document(mut response: Response) -> select::document::Document {
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let body_str: &str = &body;
    select::document::Document::from(body_str)
}

fn is_virus_link(link: &String) -> bool {
    link.ends_with("virus") && link.contains("/wiki/") && !link.contains(":")
}
