use tobi::{Arg, Command};

fn main() {
    let mut cmd = Command::new("example", "An example CLI application");
    cmd.add_arg(Arg {
        name: "verbose".to_string(),
        description: "Enable verbose output".to_string(),
        required: false,
        takes_value: false,
    });
    cmd.add_arg(Arg {
        name: "name".to_string(),
        description: "Name to greet".to_string(),
        required: false,
        takes_value: true,
    });

    match cmd.parse() {
        Ok(matches) => {
            if matches.is_present("verbose") {
                println!("Verbose mode enabled");
            }
            if let Some(name) = matches.value_of("name") {
                println!("Hello, {}!", name);
            } else {
                println!("Hello, World!");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
