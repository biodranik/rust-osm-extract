use osmpbf::{Element, ElementReader};
use std::io::{Write};
use std::{env, fmt, io};
use std::ops::Add;

const KEYS_PREFIXES: [&str; 1] = [
    "socket:",
];
//const KEYS: [&str; 1] = ["local_ref"];
//const KEYS: [&str; 1] = ["addr:street"];
//const KEYS: [&str; 3] = ["name", "addr:street", "addr:housename"];

struct Stat {
    count: u64,
}

impl Stat {
    fn zero() -> Stat {
        Stat {
            count: 0,
        }
    }
}

impl Add for Stat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            count: self.count + other.count,
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Total found tags: {}", self.count)
    }
}

fn process_tags<'a, Iter: Iterator<Item = (&'a str, &'a str)>>(t: &str, id: i64, iter: Iter) -> Stat {
    let mut found = false;
    for (key, value) in iter {
        //if KEYS.contains(&key) { // || KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
        //if key.starts_with("alt_name") {
            //if value.contains(";") {
                //writeln!(io::stdout(), "{value}").expect("Write to stdout failed");
                //return Stat{count: 1};
            //}
         //for (key1, value1) in iter {
        if KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
            found = true;
            break;
          //}
        }
    }
    if found {
        for (key, value) in iter {
            write!(io::stdout(), "{key}={value}\n").expect("Write to stdout failed");
        }
        // write!(io::stdout(), "https://www.openstreetmap.org/{}/{}\n\n", t, id).expect("Write to stdout failed");
        Stat { count: 1 }
    } else {
        Stat::zero()
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to osm.pbf file>", args[0]);
        return Ok(());
    }

    let osm_pbf_path = &args[1];
    let reader = ElementReader::from_path(osm_pbf_path)?;

    //let stdout = io::stdout(); // get the global stdout entity
    //let lock = stdout.lock(); // avoid locking/unlocking
    //let mut handle = BufWriter::new(lock); // optional: wrap that handle in a buffer

    let stat = reader
        .par_map_reduce(
            |element| match element {
                Element::Node(e) => process_tags("node", e.id(), e.tags()),
                Element::DenseNode(e) => process_tags("node", e.id(), e.tags()),
                Element::Way(e) => process_tags("way", e.id(), e.tags()),
                Element::Relation(e) => process_tags("relation", e.id(), e.tags()),
            },
            || Stat::zero(),
            |stat1, stat2| stat1 + stat2
        )?;

    writeln!(io::stdout(), "{stat}").expect("Write to stdout failed");

    Ok(())
}
