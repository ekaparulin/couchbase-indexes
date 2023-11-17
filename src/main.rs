use clap::{App, Arg};
use std::fs;

mod indexes;

fn main() {

  let matches = App::new("Couchbase index definition generator")
    .about("Parses JSON output of `SELECT * from system:indexes;` \nto N1QL statements")
    .arg(
        Arg::new("input")
            .value_name("FILE")
            .index(1)
            .required(true),
    )
    .arg(
        Arg::new("verbose")
            .help("Enabled verbose output")
            .short('v')
            .long("verbose")
    )
    .arg(
        Arg::new("if_not_exists")
            .help("Add IF NOT EXISTS option")
            .short('n')
            .long("if-not-exists")
    )
    .arg(
        Arg::new("defer_build")
            .help("Defer build")
            .short('d')
            .long("defer-build")
    )
    .arg(
      Arg::new("bucket")
          .value_name("BUCKET")
          .help("Filter by bucket")
          .short('b')
          .long("bucket")
  )
    .get_matches();

  let input_file = matches.value_of("input").unwrap();
  let is_verbose = matches.is_present("verbose");
  let if_not_exists = matches.is_present("if_not_exists");
  let defer_build = matches.is_present("defer_build");
  let bucket= matches.value_of("bucket");

  if is_verbose {
    eprintln!("Using input file: {}", input_file);
    if bucket.is_some() {
      eprintln!("Filtering by bucket: {}", bucket.unwrap());
    }

  }

  let contents = fs::read_to_string(input_file)
  .expect("Should have been able to read the file");

  for indexes in serde_json::from_str::<indexes::Indexes>(&contents).unwrap().iter() {
      let idx = indexes.get("indexes").unwrap();
      let n1ql = idx.to_n1ql(bucket, Some(if_not_exists), Some(defer_build));
      if n1ql.is_some() {
        println!("{}",n1ql.unwrap());
      }
  }
}
