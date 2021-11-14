// analysiations
// for all children of dir
//      if child is dir:
//          analysations (child)
//      else:
//          c = checksum(child)
//          if c not in fileMap:
//              fileMap[c] = vec[];
//          fileMap[c].push(child)

use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::{collections::HashMap, fs::File};

use clap::{App, Arg};
use sha2::{Digest, Sha256};

/*
lol wtf
*/
fn print_progress(path: &Path){
    let mut x = path.to_str().unwrap();
    let mut xx = format!("");
    if x.chars().count() > 70 {
        for i in x.chars() {
            if xx.len() > 70 { break; }
            xx += &i.to_string();
        }
        x = xx.as_str();
    }
    let x = format!("{}...", x);
    print!("scanning {:74}\r", x);
}

/*
find doubles
*/
fn find_doubles(map: &mut HashMap<String, Vec<String>>, path: &Path) {
    print_progress(path);
    if let Ok(md) = path.metadata() {
        if md.is_dir() {
            if let Ok(children) = read_dir(path) {
                for child in children.flatten() {
                    find_doubles(map, &child.path());
                }
            }
        } else if md.is_file() {
            if let Ok(mut file) = File::open(path) {
                let mut sha256 = Sha256::default();
                if let Err(err) = io::copy(&mut file, &mut sha256) {
                    eprintln!("error reading {:?}: {}", path, err);
                }
                let hash = sha256.finalize();
                let c = format!("{:x}", hash);
                map.entry(c.clone()).or_insert_with(Vec::new);
                if let Some(p) = path.to_str() {
                    map.entry(c).and_modify(|entry| entry.push(p.to_string()));
                }
            }
        }
    } else {
        eprintln!("could not open {:?}", path);
    }
}

fn main() -> io::Result<()> {
    let arg_matches = App::new("duplicate finder")
        .version("0.1.0")
        .author("hyde <hiddy.tiddey@gmail.com>")
        .about("find duplicate files")
        .arg(
            Arg::with_name("dirname")
                .takes_value(true)
                .value_name("DIR")
                .help("root directory to scan")
                .index(1),
        )
        .get_matches();

    let root_dir = if let Some(root) = arg_matches.value_of("dirname") {
        Path::new(root)
    } else {
        Path::new(".")
    };

    // if let Some(path) = root_dir.to_str() {
    //     println!("scanning {}...", path);
    //     let mut file = File::open(path)?;
    //     let mut sha256 = Sha256::default();
    //     io::copy(&mut file, &mut sha256)?;
    //     let hash = sha256.finalize();
    // }

    let mut doubles = HashMap::new();
    find_doubles(&mut doubles, root_dir);

    let mut num_doubles = 0;
    let mut scanned_files = 0;
    for (_, double) in doubles {
        assert_ne!(double.len(), 0);
        if double.len() == 1 { 
            scanned_files += 1;
            continue;
        }
        num_doubles += double.len();
        scanned_files += double.len();
        let first = &double[0];
        println!("found {:3} doubles: {}", double.len(), first);
        double[1..].iter().for_each(|i| {
            println!("{:18} {}","", i);
        });
        println!();
    }

    println!("found {}/{} doubles ", num_doubles, scanned_files);
        
    Ok(())
}
