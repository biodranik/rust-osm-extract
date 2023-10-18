use osmpbf::{ElementReader, Element};
use std::collections::HashMap;
use std::collections::HashSet;
use std::{env, io};
use std::fs;
use std::io::{BufWriter, Stdout, StdoutLock, Write};
// use std::path::Path;
// use fs::File;

const KEYS_PREFIXES: [&str; 9] = ["name:", "int_name", "loc_name", "nat_name", "official_name", "old_name", "reg_name", "short_name", "alt_name"];
const KEYS: [&str; 3] = ["name", "addr:street", "addr:housename"];

type ValuesSet = HashSet<String> ;
type KeysMap = HashMap<String, ValuesSet>;

fn get_tags<'a, Iter: Iterator<Item = (&'a str, &'a str)>>(iter: Iter) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for (key, value) in iter {
        if KEYS.contains(&key) || KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
            vec.push(value.to_string());
        }
    }
    return vec
}


// fn save_tags<'a, Iter: Iterator<Item = (&'a str, &'a str)>>(iter: Iter, handle: &mut BufWriter<StdoutLock>) {
//     for (key, value) in iter {
//         //if KEYS.contains(&key) || KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
//         if key == "addr:housenumber" {
//             writeln!(handle, "{value}").expect("Write to stdout failed");
//             // match keys_map.get_mut(key) {
//             //     Some(set) => {
//             //         set.insert(value.to_string());
//             //         ()
//             //     },
//             //     None => {
//             //         keys_map.insert(key.to_string(), HashSet::from([value.to_string()]));
//             //         ()
//             //     }
//             // }
//         }
//     }
// }

// fn reduce<'a, Iter: Iterator<Item = (&'a str, &'a str)>>(a: Iter, b: Iter) -> Iter {
//
// }

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to osm.pbf file> [optional output file]", args[0]);
        return Ok(());
    }

    let use_stdout = args.len() < 3;
    let out_file = if !use_stdout { &args[2] } else { "" };

    let osm_pbf_path = &args[1];
    let reader = ElementReader::from_path(osm_pbf_path).unwrap();

    let mut keys: KeysMap = HashMap::new();

    let stdout = io::stdout(); // get the global stdout entity
    let lock = stdout.lock();  // avoid locking/unlocking
    let mut handle = BufWriter::new(lock); // optional: wrap that handle in a buffer


    let result = reader.par_map_reduce(|element| {
        match element {
            Element::Node(e) => get_tags(e.tags()),
            Element::DenseNode(e) => get_tags(e.tags()),
            Element::Way(e) => get_tags(e.tags()),
            Element::Relation(e) => get_tags(e.tags()),
        }
    },
                          || Vec::new(),
                          |mut a, mut b| {a.append(&mut b); a}
    ).unwrap();

    for str in result {
        writeln!(handle, "{str}").expect("Write to stdout failed");
    }

    //let len = result.len();
    //println!("{:?}", result);

    // reader.for_each(|element| {
    //     match element {
    //         Element::Node(e) => save_tags(e.tags(), &mut handle),
    //         Element::DenseNode(e) => save_tags(e.tags(), &mut handle),
    //         Element::Way(e) => save_tags(e.tags(), &mut handle),
    //         Element::Relation(e) => save_tags(e.tags(), &mut handle),
    //     }
    // }).unwrap();

    // Write collected data to files.
    // let dir = Path::new("./out");
    // fs::create_dir_all(dir)?;
    //
    // for (key, set) in keys.iter() {
    //     let file = dir.join(key);
    //     let writer = File::options().create(true).write(true).open(&file);
    //     if writer.is_err() {
    //         println!("Error creating {} : {}", file.display(), writer.unwrap_err());
    //         continue;
    //     }
    //     let mut writer = writer.unwrap();
    //     println!("Writing {} values to {}", set.len(), file.display());
    //     for val in set.iter() {
    //         writeln!(writer, "{val}").expect("Write failed");
    //     }
    // }

    Ok(())
}
