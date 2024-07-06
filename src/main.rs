use tonbi::{Arg, Command, Result};

fn main() -> Result<()> {
    let app = Command::new("greeter")
        .version("1.0")
        .description("A simple greeting CLI application")
        .arg(Arg {
            name: "name".to_string(),
            description: "Name of the person to greet".to_string(),
            required: false,
            takes_value: true,
            short: Some('n'),
        })
        .arg(Arg {
            name: "enthusiastic".to_string(),
            description: "Add excitement to the greeting".to_string(),
            required: false,
            takes_value: false,
            short: Some('e'),
        })
        .subcommand(
            Command::new("farewell")
                .description("Say goodbye instead of hello")
                .arg(Arg {
                    name: "name".to_string(),
                    description: "Name of the person to bid farewell".to_string(),
                    required: false,
                    takes_value: true,
                    short: Some('n'),
                }),
        );

    let matches = app.parse()?;

    if let Some(("farewell", sub_matches)) = matches.subcommand() {
        let name = sub_matches.value_of("name").unwrap_or("friend");
        println!("Goodbye, {}!", name);
    } else {
        let name = matches.value_of("name").unwrap_or("world");
        let greeting = if matches.is_present("enthusiastic") {
            format!("Hello, {}!!!", name)
        } else {
            format!("Hello, {}.", name)
        };
        println!("{}", greeting);
    }

    Ok(())
}
