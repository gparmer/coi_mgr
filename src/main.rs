use anyhow;
//use csv::Reader;
//use serde::Deserialize;
use std::collections::HashSet;
use std::env;
use std::fs::File;
//use std::io::Read;

fn file_arg_to_reader(argn: usize) -> anyhow::Result<csv::Reader<File>> {
    let file_path = env::args_os()
        .nth(argn)
        .ok_or(anyhow::anyhow!("Argument {} not provided.", argn))?;
    let file = File::open(file_path)?;

    Ok(csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .has_headers(false)
        .from_reader(file))
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() -> anyhow::Result<()> {
    if env::args_os().len() != 2 {
        println!(
            "Usage: {} cois.tsv\nNote: the tsv must include tab-separated values.",
            env::args_os().nth(0).unwrap().to_str().unwrap()
        );
        return Err(anyhow::anyhow!("Incorrect number of arguments"));
    }

    let mut papers = Vec::new();
    for result in file_arg_to_reader(1)?.deserialize() {
        let name_str: String = result?;
        let formatted_names = name_str.trim().to_ascii_lowercase();
        let names: Vec<&str> = formatted_names.split(',').map(|n| n.trim()).collect();
        let mut set = HashSet::new();
        for name in names {
            set.insert(String::from(name));
        }
        papers.push(set);
    }

    let set_to_str = |set: &HashSet<String>| {
        let mut sorted: Vec<String> = set.iter().map(|s| s.clone()).collect();
        sorted.sort();
        let sorted: Vec<String> = sorted
            .iter()
            .map(|s| {
                let l: Vec<String> = s.split(' ').map(|ss| capitalize(ss)).collect();
                l.join(" ")
            })
            .collect();
        let mut s = sorted
            .iter()
            .fold("".to_string(), |acc, n| format!("{}, {}", acc, n));
        if s.len() > 2 {
            s.remove(0);
            s.remove(0);
        }

        s
    };

    let s = set_to_str(&papers[0]);
    println!("{}\t{}\t", s, s);

    for i in 1..papers.len() {
        let curr_set: &HashSet<String> = &papers[i];
        let prev_set: &HashSet<String> = &papers[i - 1];
        let i: HashSet<String> = curr_set
            .intersection(&prev_set)
            .map(|s| s.clone())
            .collect();

        let full = set_to_str(&curr_set);
        let evict = set_to_str(&curr_set.difference(&i).map(|s| s.clone()).collect());
        let admit = set_to_str(&prev_set.difference(&i).map(|s| s.clone()).collect());

        println!("{}\t{}\t{}", full, evict, admit);
    }

    Ok(())
}
