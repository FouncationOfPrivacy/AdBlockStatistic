use adblock::engine::Engine;
use adblock::lists::{FilterFormat, FilterSet};

use std::env;
use std::collections::HashMap;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn check_urls(
        blocker: &adblock::engine::Engine,
        urls: &Vec<String>,
        histogram: &mut HashMap<String, i32>) {

    for url in urls {
        // Scan the URL and check the matched rules.
        let blocker_result = blocker.check_network_urls(url, "", "");

        if let Some(filters) = blocker_result.filter {
            let items: Vec<&str> = filters.split("<+>").collect();

            for item in &items {
                let filter = item.trim();
                *histogram.entry(filter.to_string()).or_insert(0) += 1;
            }
        }
    }
}

fn map_to_file(filename: impl AsRef<Path>, histogram: &HashMap<String, i32>)
    -> std::io::Result<()> {

    let mut writer = BufWriter::new(File::create(filename)?);
    for (key, value) in histogram {
        write!(writer, "{}\t{}\n", key, value);
    }
    writer.flush()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run PATH_RULE PATH_URLS PATH_OUT");
        return;
    }

    // Load EasyList rule file.
    let rule_file = &args[1];
    let rules = lines_from_file(rule_file)
        .expect("Could not load EasyList rules.");

    // Load the to be examined URLs.
    let url_file = &args[2];
    let urls = lines_from_file(url_file)
        .expect("Could not load the URLs.");

    let out_file = &args[3];

    // Create the filter engine.
    let mut filter_set = FilterSet::new(true);
    filter_set.add_filters(&rules, FilterFormat::Standard);
    let blocker = Engine::from_filter_set(filter_set, true);

    // Check URLs by the loaded rules and collect the statistics.
    let mut histogram: HashMap<String, i32> = HashMap::new();
    check_urls(&blocker, &urls, &mut histogram);

    // Dump the histogram.
    map_to_file(out_file, &histogram);
}