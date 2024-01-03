use clap::{Arg, ArgMatches, Command};

pub(crate) const CONFIG: &str = "config";

pub(crate) fn parse() -> ArgMatches {
    Command::new("AutoMod")
        .arg(
            Arg::new(CONFIG)
                .short('c')
                .default_value("automod.toml")
                .help("Path to configuration file"),
        )
        .get_matches()
}
