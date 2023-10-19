use osmpbf::{Element, ElementReader};
use std::io::{BufWriter, Write};
use std::{env, fmt, io};

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

struct Stat {
    total_operator: u64,
    operator_equal_to_name: u64,
    operator_equal_to_brand: u64,
}

impl Stat {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn zero() -> Stat {
        Stat {
            total_operator: 0,
            operator_equal_to_name: 0,
            operator_equal_to_brand: 0,
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "Total # of features with operator tag: {}\n\
        # of features where operator tag value is equal to one of the name tags: {}\n\
        # of features where operator tag value is equal to the brand tag value: {}",
            self.total_operator, self.operator_equal_to_name, self.operator_equal_to_brand
        )
    }
}

fn get_tags<'a, Iter: Iterator<Item = (&'a str, &'a str)>>(iter: Iter) -> Stat {
    let mut names: Vec<String> = Vec::new();
    let mut brand: &str = "";
    let mut operator: &str = "";
    for (key, value) in iter {
        if KEYS.contains(&key) || KEYS_PREFIXES.iter().any(|&s| key.starts_with(s)) {
            names.push(value.to_string());
        } else if key == "brand" {
            brand = value;
        } else if key == "operator" {
            operator = value;
        }
    }

    let mut stat = Stat::zero();
    if !operator.is_empty() {
        stat.total_operator = 1;
        for name in names {
            if name == operator {
                stat.operator_equal_to_name = 1;
            }
        }
        if operator == brand {
            stat.operator_equal_to_brand = 1;
        }
    }

    return stat;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to osm.pbf file>", args[0]);
        return Ok(());
    }

    let osm_pbf_path = &args[1];
    let reader = ElementReader::from_path(osm_pbf_path).unwrap();

    let stdout = io::stdout(); // get the global stdout entity
    let lock = stdout.lock(); // avoid locking/unlocking
    let mut handle = BufWriter::new(lock); // optional: wrap that handle in a buffer

    let stat = reader
        .par_map_reduce(
            |element| match element {
                Element::Node(e) => get_tags(e.tags()),
                Element::DenseNode(e) => get_tags(e.tags()),
                Element::Way(e) => get_tags(e.tags()),
                Element::Relation(e) => get_tags(e.tags()),
            },
            || Stat::zero(),
            |stat1, stat2| Stat {
                total_operator: stat1.total_operator + stat2.total_operator,
                operator_equal_to_name: stat1.operator_equal_to_name + stat2.operator_equal_to_name,
                operator_equal_to_brand: stat1.operator_equal_to_brand
                    + stat2.operator_equal_to_brand,
            },
        )
        .unwrap();

    writeln!(handle, "{stat}").expect("Write to stdout failed");

    Ok(())
}
