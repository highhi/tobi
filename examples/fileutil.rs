use std::fs;
use std::io::Read;
use tonbi::{Arg, ArgMatches, Command};

fn main() {
    let app = Command::new("fileutil")
        .version("1.0")
        .description("A simple file utility")
        .subcommand(
            Command::new("cat")
                .description("Display file contents")
                .arg(
                    Arg::new("file", "File to display")
                        .required(true)
                        .short('f')
                        .takes_value(),
                ),
        )
        .subcommand(
            Command::new("copy")
                .description("Copy a file")
                .arg(Arg::new("source", "Source file").required(true).is_value())
                .arg(
                    Arg::new("destination", "Destination file")
                        .required(true)
                        .is_value(),
                ),
        )
        .subcommand(
            Command::new("rename")
                .description("Rename a file")
                .arg(Arg::new("old", "Old file name").required(true))
                .arg(Arg::new("new", "New file name").required(true)),
        );

    let matches = app.parse().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    match matches.subcommand() {
        Some(("cat", sub_matches)) => cat(sub_matches),
        Some(("copy", sub_matches)) => copy(sub_matches),
        Some(("rename", sub_matches)) => rename(sub_matches),
        _ => {
            eprintln!("No subcommand was used");
            std::process::exit(1);
        }
    }
}

fn cat(matches: &ArgMatches) {
    let file = matches.value_of("file").unwrap();
    println!("Displaying contents of file: {}", file);
    let mut content = String::new();
    fs::File::open(file)
        .and_then(|mut f| f.read_to_string(&mut content))
        .unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        });
    println!("{}", content);
}

fn copy(matches: &ArgMatches) {
    let source = matches.value_of("source").unwrap();
    let destination = matches.value_of("destination").unwrap();
    fs::copy(source, destination).unwrap_or_else(|e| {
        eprintln!("Error copying file: {}", e);
        std::process::exit(1);
    });
    println!("File copied successfully");
}

fn rename(matches: &ArgMatches) {
    let old = matches.value_of("old").unwrap();
    let new = matches.value_of("new").unwrap();
    fs::rename(old, new).unwrap_or_else(|e| {
        eprintln!("Error renaming file: {}", e);
        std::process::exit(1);
    });
    println!("File renamed successfully");
}
