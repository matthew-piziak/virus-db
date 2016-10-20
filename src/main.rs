use std::fs::File;
use std::io::prelude::*;

extern crate rustc_serialize;
use rustc_serialize::json;

extern crate hyper;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct VirusDatabase  {
    viruses: Vec<Virus>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Virus {
    name: String,
    link: String,
}

fn main() {
    let mut viruses_json_file = File::open("viruses.json").unwrap();
    let mut viruses_json_string = String::new();
    viruses_json_file.read_to_string(&mut viruses_json_string).unwrap();
    let virus_db: VirusDatabase = json::decode(&viruses_json_string).unwrap();
    let client = hyper::Client::new();
    for virus in virus_db.viruses {
        println!("virus: {:?}", virus.name);
        let link: &str = &format!("https://en.wikipedia.org/wiki/{}", virus.link);
        let res = client.get(link).send().unwrap();
        match res.status {
            hyper::Ok => {println!("OK");}
            other => {println!("failure: {:?}", other); panic!("Quitting.")}
        }
    }
}
