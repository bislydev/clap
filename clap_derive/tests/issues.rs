// https://github.com/TeXitoi/structopt/issues/{NUMBER}

mod utils;
use utils::*;

use clap::{AppSettings, ArgGroup, Args, Parser, Subcommand};

#[test]
fn issue_151() {
    #[derive(Args, Debug)]
    #[clap(group = ArgGroup::new("verb").required(true).multiple(true))]
    struct Opt {
        #[clap(long, group = "verb")]
        foo: bool,
        #[clap(long, group = "verb")]
        bar: bool,
    }

    #[derive(Debug, Parser)]
    struct Cli {
        #[clap(flatten)]
        a: Opt,
    }

    assert!(Cli::try_parse_from(&["test"]).is_err());
    assert!(Cli::try_parse_from(&["test", "--foo"]).is_ok());
    assert!(Cli::try_parse_from(&["test", "--bar"]).is_ok());
    assert!(Cli::try_parse_from(&["test", "--zebra"]).is_err());
    assert!(Cli::try_parse_from(&["test", "--foo", "--bar"]).is_ok());
}

#[test]
fn issue_289() {
    #[derive(Parser)]
    #[clap(setting = AppSettings::InferSubcommands)]
    enum Args {
        SomeCommand {
            #[clap(subcommand)]
            sub: SubSubCommand,
        },
        AnotherCommand,
    }

    #[derive(Subcommand)]
    #[clap(setting = AppSettings::InferSubcommands)]
    enum SubSubCommand {
        TestCommand,
    }

    assert!(Args::try_parse_from(&["test", "some-command", "test-command"]).is_ok());
    assert!(Args::try_parse_from(&["test", "some", "test-command"]).is_ok());
    assert!(Args::try_parse_from(&["test", "some-command", "test"]).is_ok());
    assert!(Args::try_parse_from(&["test", "some", "test"]).is_ok());
}

#[test]
fn issue_324() {
    fn my_version() -> &'static str {
        "MY_VERSION"
    }

    #[derive(Parser)]
    #[clap(version = my_version())]
    struct Opt {
        #[clap(subcommand)]
        _cmd: Option<SubCommand>,
    }

    #[derive(Subcommand)]
    enum SubCommand {
        Start,
    }

    let help = get_long_help::<Opt>();
    assert!(help.contains("MY_VERSION"));
}

#[test]
fn issue_418() {
    #[derive(Debug, Parser)]
    struct Opts {
        #[clap(subcommand)]
        /// The command to run
        command: Command,
    }

    #[derive(Debug, Subcommand)]
    enum Command {
        /// Reticulate the splines
        #[clap(visible_alias = "ret")]
        Reticulate {
            /// How many splines
            num_splines: u8,
        },
        /// Frobnicate the rest
        #[clap(visible_alias = "frob")]
        Frobnicate,
    }

    let help = get_long_help::<Opts>();
    assert!(help.contains("Reticulate the splines [aliases: ret]"));
}

#[test]
fn issue_490() {
    use clap::Parser;
    use std::iter::FromIterator;
    use std::str::FromStr;

    struct U16ish;
    impl FromStr for U16ish {
        type Err = ();
        fn from_str(_: &str) -> Result<Self, Self::Err> {
            unimplemented!()
        }
    }
    impl<'a> FromIterator<&'a U16ish> for Vec<u16> {
        fn from_iter<T: IntoIterator<Item = &'a U16ish>>(_: T) -> Self {
            unimplemented!()
        }
    }

    #[derive(Parser, Debug)]
    struct Opt {
        opt_vec: Vec<u16>,
        #[clap(long)]
        opt_opt_vec: Option<Vec<u16>>,
    }

    // Assert that it compiles
}
