use std::collections::HashSet;

const SOURCE: &str = "./data/sources/englands_immigrants_search_results.csv";
const NAMES: &str = "./data/name.txt";
const CITIES: &str = "./data/city.txt";

fn main() {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(SOURCE).unwrap();

    let mut names = HashSet::<String>::new();
    let mut cities = HashSet::<String>::new();

    for result in reader.records() {
        let line = result.unwrap();

        if let Some(name) = line.get(4) {
            names.insert(name.to_string());
        }

        if let Some(city) = line.get(12) {
            cities.insert(city.to_string());
        }
    }
    dbg!(&cities);
    dbg!(cities.len());
}