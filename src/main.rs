use osmpbf::{Element, ElementReader};
use std::io::{BufWriter, Write};
use std::{env, io};

const KEYS_PREFIXES: [&str; 9] = [
    "name:",
    "int_name",
    "loc_name",
    "nat_name",
    "official_name",
    "old_name",
    "reg_name",
    "short_name",
    "alt_name",
];
const KEYS: [&str; 1] = ["name"];
//const KEYS: [&str; 3] = ["name", "addr:street", "addr:housename"];

fn get_tags<'a, Iter: Iterator<Item = (&'a str, &'a str)>>(iter: Iter) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for (key, value) in iter {
        if KEYS.contains(&key) || KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
            vec.push(value.to_string());
        }
    }
    return vec;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: {} <path to osm.pbf file> [optional output file]",
            args[0]
        );
        return Ok(());
    }

    let osm_pbf_path = &args[1];
    let reader = ElementReader::from_path(osm_pbf_path).unwrap();

    let stdout = io::stdout(); // get the global stdout entity
    let lock = stdout.lock(); // avoid locking/unlocking
    let mut handle = BufWriter::new(lock); // optional: wrap that handle in a buffer

    let result = reader
        .par_map_reduce(
            |element| match element {
                Element::Node(e) => get_tags(e.tags()),
                Element::DenseNode(e) => get_tags(e.tags()),
                Element::Way(e) => get_tags(e.tags()),
                Element::Relation(e) => get_tags(e.tags()),
            },
            || Vec::new(),
            |mut a, mut b| {
                a.append(&mut b);
                a
            },
        )
        .unwrap();

    for str in result {
        writeln!(handle, "{str}").expect("Write to stdout failed");
    }

    Ok(())
}
