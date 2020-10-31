use adblock::engine::Engine;
use adblock::lists::{FilterFormat, FilterSet};

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn main() {

    let rules = lines_from_file("./data/easylist.to/easylist/easylist.txt")
        .expect("Could not load lines");

    let mut filter_set = FilterSet::new(true);
    filter_set.add_filters(&rules, FilterFormat::Standard);

    let blocker = Engine::from_filter_set(filter_set, true);
    let blocker_result = blocker.check_network_urls("http://example.com/-advertisement-icon.", "http://example.com/helloworld", "image");

    println!("Blocker result: {:?}", blocker_result);
}