use std::io::prelude::*;

extern crate hyper;

extern crate select;

pub struct VirusIndex {
    links: Vec<String>,
}

fn main() {
    let virus_db = virus_db().expect("Could not load virus database");
    let client = hyper::Client::new();
    for link in virus_db.links {
        let response = client.get(&format!("https://en.wikipedia.org/wiki/{}", link)).send().unwrap();
        let document = document(response);
        for node in document.find(select::predicate::Class("group")).iter() {
            println!("Group: {}", node.text());
        }
        for node in document.find(select::predicate::Class("family")).iter() {
            println!("Family: {}", node.text());
        }
    }
}

fn document(mut response: hyper::client::response::Response) -> select::document::Document {
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let body_str: &str = &body;
    select::document::Document::from(body_str)
}

fn virus_db() -> Result<VirusIndex, &'static str> {
    Ok(VirusIndex{links: vec![]})
}
