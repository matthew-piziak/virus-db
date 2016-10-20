use std::io::prelude::*;
use std::fs::File;
use std::io;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate hyper;

extern crate select;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct VirusDatabase {
    viruses: Vec<Virus>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Virus {
    name: String,
    link: String,
}

fn main() {
    let virus_db = virus_db().expect("Could not load virus database");
    let client = hyper::Client::new();
    for virus in virus_db.viruses {
        println!("virus: {:?}", virus.name);
        let response = client.get(&format!("https://en.wikipedia.org/wiki/{}", virus.link)).send().unwrap();
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

fn virus_db() -> io::Result<VirusDatabase> {
    let mut file = try!(File::open("viruses.json"));
    let mut file_contents = String::new();
    try!(file.read_to_string(&mut file_contents));
    Ok(json::decode(&file_contents).unwrap())
}
