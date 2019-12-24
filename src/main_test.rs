use std::fs; // for reading and writing files
use serde_json::{Result, Value}; // for reading JSON files
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Obj {
    pages: Vec<Issue>,
}

#[derive(Deserialize, Debug)]
struct Issue {
    title: String,
}

fn main() {
    let json_str = fs::read_to_string("C:\\Users\\joekr\\Programs\\soswg_site\\test.json")
    .expect("Error loading file.");

    let issues = serde_json::from_str::<Obj>(&json_str).unwrap();
    let pages = issues.pages;
    let bla = &pages;
    println!("{:?}", bla[0].title);
    for i in pages {
        println!("{:#?}", i);
    }
}