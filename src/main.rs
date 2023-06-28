use osmpbf::{ElementReader, Element, TagIter};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use fs::File;

const KEYS_PREFIXES: [&str; 9] = ["name:", "int_name", "loc_name", "nat_name", "official_name", "old_name", "reg_name", "short_name", "alt_name"];
const KEYS: [&str; 3] = ["name", "addr:street", "addr:housename"];

type ValuesSet = HashSet<String> ;
type KeysMap = HashMap<String, ValuesSet>;

fn save_tags(iter: TagIter, keys_map: &mut KeysMap) {
    for (key, value) in iter {
        if KEYS.contains(&key) || KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
            match keys_map.get_mut(key) {
                Some(set) => {
                    set.insert(value.to_string());
                    ()
                },
                None => {
                    keys_map.insert(key.to_string(), HashSet::from([value.to_string()]));
                    ()
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to osm.pbf file>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    let reader = ElementReader::from_path(file_path).unwrap();

    let mut keys: KeysMap = HashMap::new();

    // Increment the counter by one for each way.
    reader.for_each(|element| {
        if let Element::Node(node) = element {
            save_tags(node.tags(), &mut keys)
        } else if let Element::Way(way) = element {
            save_tags(way.tags(), &mut keys)
        } else if let Element::Relation(relation) = element {
            save_tags(relation.tags(), &mut keys)
        }
    }).unwrap();

    // Write collected data to files.
    let dir = Path::new("./out");
    fs::create_dir_all(dir)?;

    for (key, set) in keys.iter() {
        let file = dir.join(key);
        let mut writer = File::options().create(true).write(true).open(&file)?;
        println!("Writing {} values to {}", set.len(), file.display());
        for val in set.iter() {
            writeln!(writer, "{val}").expect("Write failed");
        }
    }

    Ok(())
}
