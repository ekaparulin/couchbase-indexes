use clap::{Command, Arg, ArgAction};
use std::fs;
use std::collections::BTreeMap;

mod indexes;

fn main() {

  let mut cmd = Command::new("Couchbase index definition generator")
      .bin_name("gen-indexes")
      .arg(Arg::new("input")
            .help("Input file")
            .short('i')
            .long("input")
            .value_name("INPUT_FILE")
            .action(ArgAction::Set))
      .arg(Arg::new("verbose")
            .help("Enable verbose output")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue)
      )
      .arg(Arg::new("if_not_exists")
            .help("Add IF NOT EXISTS option")
            .short('n')
            .long("if-not-exists")
            .action(ArgAction::SetTrue)
      )
      .arg(Arg::new("defer_build")
            .help("Defer build")
            .short('d')
            .long("defer-build")
            .action(ArgAction::SetTrue)
      )
      .arg(Arg::new("num_replica")
            .help("Set replica number")
            .short('r')
            .long("num-replica")
            .action(ArgAction::Set)
      )
      .arg(Arg::new("bucket")
            .value_name("BUCKET")
            .help("Filter by bucket")
            .short('b')
            .long("bucket")
            .action(ArgAction::Set)
      );
    let matches = cmd.clone().get_matches();


  
  if matches.get_one::<std::string::String>("input").is_none() {
    //panic!("Missing input file");
    let _ = cmd.print_long_help();
    std::process::exit(1);
  }

  let input_file = matches.get_one::<std::string::String>("input").unwrap();

  let is_verbose = matches.get_flag("verbose");

  let if_not_exists = matches.get_flag("if_not_exists");
  let defer_build = matches.get_flag("defer_build");

  let mut num_replica: Option<u8> = None;
  if matches.get_one::<String>("num_replica").map(String::as_str).is_some() {
    //   num_replica = Some(matches.value_of("num_replica").unwrap().parse::<u8>().unwrap());
    num_replica = Some(matches.get_one::<String>("num_replica").map(String::as_str).unwrap().parse::<u8>().unwrap());
  }

  let bucket= matches.get_one::<std::string::String>("bucket");

  if is_verbose {
    eprintln!("Using input file: {}", input_file);
    if bucket.is_some() {
      eprintln!("Filtering by bucket: {}", bucket.unwrap());
    }
  }

  let contents = fs::read_to_string(input_file)
  .expect("Should have been able to read the file");

  let mut sorted: BTreeMap<String, String>   = BTreeMap::new();

  for indexes in serde_json::from_str::<indexes::Indexes>(&contents).unwrap().iter() {
      let idx = indexes.get("indexes").unwrap();
      let n1ql = idx.to_n1ql(bucket, Some(if_not_exists), Some(defer_build), num_replica);
      if n1ql.is_some() {
        sorted.insert( idx.name(), n1ql.unwrap());
      }
  }

  for (_, n1ql) in sorted {
    println!("{}\n", n1ql);
  }


}
