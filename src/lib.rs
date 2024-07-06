pub use anyhow::Result;
use std::collections::HashMap;
use std::env;

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
    pub short: Option<char>,
    pub is_value: bool,
}

impl Arg {
    pub fn new(name: &str, description: &str) -> Self {
        Arg {
            name: name.to_string(),
            description: description.to_string(),
            required: false,
            takes_value: false,
            short: None,
            is_value: false,
        }
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn takes_value(mut self) -> Self {
        self.takes_value = true;
        self
    }

    pub fn is_value(mut self) -> Self {
        self.is_value = true;
        self
    }
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub version: Option<String>,
    pub args: Vec<Arg>,
    pub subcommands: Vec<Command>,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            description: String::new(),
            version: None,
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

    pub fn version(mut self, vertion: &str) -> Self {
        self.version = Some(vertion.to_string());
        self
    }

    pub fn parse(&self) -> Result<ArgMatches> {
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse_args(&args)
    }

    pub fn parse_args(&self, args: &[String]) -> Result<ArgMatches> {
        let mut matches = ArgMatches::new();

        let mut arg_index = 0;

        while arg_index < args.len() {
            let arg = &args[arg_index];
            if arg == "--help" || arg == "-h" {
                self.print_help();
                std::process::exit(0);
            } else if arg == "--version" || arg == "-V" {
                if let Some(version) = &self.version {
                    println!("{}", version);
                    std::process::exit(0);
                }
                std::process::exit(0);
            // When long option
            } else if arg.starts_with("--") {
                let name = arg.trim_start_matches("--");
                self.process_long_option(&mut matches, name, args, &mut arg_index)?;
            // When short option
            } else if arg.starts_with('-') {
                let shorts = arg.trim_start_matches('-').chars();
                for short in shorts {
                    self.process_short_option(&mut matches, short, args, &mut arg_index)?;
                }
            // When subcommand
            } else if let Some(subcmd) = self.subcommands.iter().find(|cmd| cmd.name == *arg) {
                let sub_args = &args[arg_index + 1..];
                let sub_matches = subcmd.parse_args(sub_args)?;
                matches.subcommand = Some((subcmd.name.clone(), Box::new(sub_matches)));
                break;
            // When value
            } else if self.args[arg_index].is_value {
                matches
                    .values
                    .insert(self.args[arg_index].name.clone(), Some(arg.clone()));
            } else {
                return Err(anyhow::anyhow!("Unknown argument: {}", arg));
            }
            arg_index += 1;
        }
        Ok(matches)
    }

    fn process_long_option(
        &self,
        matches: &mut ArgMatches,
        name: &str,
        args: &[String],
        arg_index: &mut usize,
    ) -> Result<()> {
        if let Some(arg_def) = self.args.iter().find(|a| a.name == name) {
            if arg_def.takes_value {
                *arg_index += 1;
                if *arg_index < args.len() {
                    matches
                        .values
                        .insert(name.to_string(), Some(args[*arg_index].clone()));
                } else {
                    return Err(anyhow::anyhow!("Unknown option: {}", name));
                }
            } else {
                matches.values.insert(name.to_string(), None);
            }
        } else {
            return Err(anyhow::anyhow!("Unknown option: {}", name));
        }
        Ok(())
    }

    fn process_short_option(
        &self,
        matches: &mut ArgMatches,
        short: char,
        args: &[String],
        arg_index: &mut usize,
    ) -> Result<()> {
        if let Some(arg_def) = self.args.iter().find(|a| a.short == Some(short)) {
            if arg_def.takes_value {
                *arg_index += 1;
                if *arg_index < args.len() {
                    matches
                        .values
                        .insert(arg_def.name.clone(), Some(args[*arg_index].clone()));
                } else {
                    return Err(anyhow::anyhow!("Option -{} requires a value", short));
                }
            } else {
                matches.values.insert(arg_def.name.clone(), None);
            }
        } else {
            return Err(anyhow::anyhow!("Unknown option: -{}", short));
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test")
            .description("A test command")
            .version("1.0");

        assert_eq!(cmd.name, "test");
        assert_eq!(cmd.description, "A test command");
        assert_eq!(cmd.version, Some("1.0".to_string()));
    }

    #[test]
    fn test_arg_creation() {
        let arg = Arg::new("name", "Name of the person")
            .short('n')
            .required(true)
            .takes_value();

        assert_eq!(arg.name, "name");
        assert_eq!(arg.description, "Name of the person");
        assert_eq!(arg.required, true);
        assert_eq!(arg.takes_value, true);
        assert_eq!(arg.short, Some('n'));
    }

    #[test]
    fn test_parse_long_option() {
        let cmd = Command::new("test").arg(Arg::new("option", "A test option").takes_value());

        let args = vec!["--option".to_string(), "value".to_string()];
        let matches = cmd.parse_args(&args).unwrap();
        assert_eq!(matches.value_of("option"), Some("value"))
    }

    #[test]
    fn test_parse_short_option() {
        let cmd =
            Command::new("test").arg(Arg::new("option", "A test option").short('o').takes_value());

        let args = vec!["-o".to_string(), "value".to_string()];
        let matches = cmd.parse_args(&args).unwrap();

        assert_eq!(matches.value_of("option"), Some("value"));
    }

    #[test]
    fn test_subcommand() {
        let cmd = Command::new("test").subcommand(
            Command::new("sub").arg(Arg::new("suboption", "A sub option").takes_value()),
        );

        let args = vec![
            "sub".to_string(),
            "--suboption".to_string(),
            "value".to_string(),
        ];
        let matches = cmd.parse_args(&args).unwrap();

        assert!(matches.subcommand().is_some());
        if let Some((subcmd, submatches)) = matches.subcommand() {
            assert_eq!(subcmd, "sub");
            assert_eq!(submatches.value_of("suboption"), Some("value"));
        } else {
            panic!("Subcommand was not parsed correctly");
        }
    }
}
