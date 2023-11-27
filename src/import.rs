#![allow(dead_code)]

use std::{collections::HashSet, fs::File, io::Write};

fn main() {
    // names_towns();
    // ages();
    saints();
}

fn names_towns() {
    const SOURCE: &str = "./data/sources/englands_immigrants_search_results.csv";
    const NAMES: &str = "./data/name.txt";
    const TOWNS: &str = "./data/town.txt";

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(SOURCE).unwrap();

    let name_field = reader
        .headers().unwrap().iter()
        .position(|field| field == "forename").unwrap();
    let town_field = reader
        .headers().unwrap().iter()
        .position(|field| field == "origin_town").unwrap();

    let mut names = HashSet::<String>::new();
    let mut towns = HashSet::<String>::new();

    for result in reader.records() {
        let line = result.unwrap();

        if let Some(name) = line.get(name_field) {
            if name.chars().all(|c| c.is_alphabetic()) {
                names.insert(name.to_string());
            }
        }

        if let Some(town) = line.get(town_field) {
            if
                town.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) &&
                !town.trim().is_empty()
            {
                towns.insert(town.to_string());
            }
        }
    }

    let mut names_file = File::create(NAMES).unwrap();
    let mut towns_file = File::create(TOWNS).unwrap();

    for name in names {
        let name = format!("{name}\n");
        names_file.write_all(name.as_bytes()).unwrap();
    }

    for town in towns {
        let town = format!("{town}\n");
        towns_file.write_all(town.as_bytes()).unwrap();
    }
}

fn ages() {
    const NAMES: &str = "./data/age.txt";
    let mut ages_file = File::create(NAMES).unwrap();

    for age in 15..400 {
        let age = format!("{age}\n");
        ages_file.write_all(age.as_bytes()).unwrap();
    }
}

fn saints() {
    const SOURCE: &str = "./data/sources/wikipedia-saints.csv";
    const SAINTS: &str = "./data/saint.txt";

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(SOURCE).unwrap();

    let mut output = File::create(SAINTS).unwrap();

    for result in reader.records() {
        let name = result.unwrap();
        let name = name.get(0).unwrap();
        if name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
            let name = format!("{name}\n");
            output.write_all(name.as_bytes()).unwrap();
        }
    }
}
