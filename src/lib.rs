pub use anyhow::Result;
use std::collections::HashMap;
use std::{env, string};

pub struct ArgMatches {
    values: HashMap<String, Option<String>>,
    subcommand: Option<(String, Box<ArgMatches>)>,
}

impl ArgMatches {
    fn new() -> Self {
        ArgMatches {
            values: HashMap::new(),
            subcommand: None,
        }
    }

    pub fn value_of(&self, name: &str) -> Option<&str> {
        self.values.get(name).and_then(|v| v.as_deref())
    }

    pub fn is_present(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn subcommand(&self) -> Option<(&str, &ArgMatches)> {
        self.subcommand
            .as_ref()
            .map(|(name, matches)| (name.as_str(), matches.as_ref()))
    }
}

pub struct Arg {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub takes_value: bool,
}

impl Arg {
    pub fn new(name: &str, description: &str, required: bool, takes_value: bool) -> Self {
        Arg {
            name: name.to_string(),
            description: description.to_string(),
            required,
            takes_value,
        }
    }
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub args: Vec<Arg>,
    pub subcommands: Vec<Command>,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            description: String::new(),
            args: vec![],
            subcommands: vec![],
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.subcommands.push(subcommand);
        self
    }

    pub fn parse(&self) -> Result<ArgMatches> {
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse_args(&args)
    }

    pub fn parse_args(&self, args: &[String]) -> Result<ArgMatches> {
        let mut matches = ArgMatches::new();

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg == "--help" || arg == "-h" {
                self.print_help();
                std::process::exit(0);
            } else if arg.starts_with("--") {
                let name = arg.trim_start_matches("--");
                if let Some(arg_def) = self.args.iter().find(|a| a.name == name) {
                    if arg_def.takes_value {
                        i += 1;
                        if i < args.len() {
                            matches
                                .values
                                .insert(name.to_string(), Some(args[i].clone()));
                        } else {
                            return Err(anyhow::anyhow!("Option --{} requires a value", name));
                        }
                    } else {
                        matches.values.insert(name.to_string(), None);
                    }
                } else {
                    return Err(anyhow::anyhow!("Unknown option: {}", arg));
                }
            } else {
                // Subcommand
                if let Some(subcmd) = self.subcommands.iter().find(|cmd| cmd.name == *arg) {
                    let sub_args = &args[i + 1..];
                    let sub_matches = subcmd.parse_args(sub_args)?;
                    matches.subcommand = Some((subcmd.name.clone(), Box::new(sub_matches)));
                    break;
                } else {
                    return Err(anyhow::anyhow!("Unknown argument: {}", arg));
                }
            }
            i += 1;
        }
        Ok(matches)
    }

    pub fn generate_help(&self) -> String {
        let mut help = format!("Usage: {} [OPTIONS] [SUBCOMMAND]\n\n", self.name);
        help.push_str(&format!("{}\n\n", self.description));

        if !self.args.is_empty() {
            help.push_str("Options:\n");
            for arg in &self.args {
                help.push_str(&format!("  --{}\t{}\n\n", arg.name, arg.description));
            }
        }

        if !self.subcommands.is_empty() {
            help.push_str("Subcommands:\n");
            for subcommand in &self.subcommands {
                help.push_str(&format!(
                    "  {}\t{}\n\n",
                    subcommand.name, subcommand.description
                ));
            }
        }

        help
    }

    pub fn print_help(&self) {
        println!("{}", self.generate_help());
    }
}
