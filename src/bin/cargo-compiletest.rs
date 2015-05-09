//! The compiletest utility as a cargo subcommand
//!
//! The binary 'cargo-compiletest' must exist on your PATH somewhere, then it
//! should be a matter of running
//!
//!     cargo compiletest
//!
//! to execute this program.
//!
#![feature(path_ext)]
#![feature(collections)]
extern crate compiletest_rs as compiletest;
extern crate getopts;

use std::env;
use std::path::{PathBuf, Path};
use std::fs::PathExt;
use getopts::Options;

use compiletest::common::{Config, Mode};

fn run_mode(mode: Mode, mut cfg: Config) {
    cfg.mode = mode;
    cfg.src_base = PathBuf::from(format!("tests/{}", mode));
    compiletest::run_tests(&cfg);
}

#[allow(non_upper_case_globals)]
const all_modes: &'static [Mode] = &[
    Mode::CompileFail,
    Mode::ParseFail,
    Mode::RunFail,
    Mode::RunPass,
    Mode::RunPassValgrind,
    Mode::Pretty,
    Mode::DebugInfoGdb,
    Mode::DebugInfoLldb,
    Mode::Codegen,
];

// Usage string
const USAGE: &'static str = "Execute the compiletest subcommand

Usage:
    cargo compiletest [<args>...]";

const HELPMSG: &'static str =
"This utility is useful for testing negative cases, ie cases that are expected
to not compile or run successfully.

It requires a 'tests' folder and a separate folder for each mode.";

fn usage(opts: &Options, exit_status: i32) -> ! {
    print!("{}\n", opts.usage(USAGE));
    print!("{}", HELPMSG);
    std::process::exit(exit_status);
}

fn options() -> Options {
    let mut opts = Options::new();
    opts.optflag("v", "verbose", "Use verbose output")
        .optflag("h", "help", "Print this help message and exit")
        .optopt("m", "modes", "Specify a comma-separated list of modes  to run (default all)", "MODES");
    opts
}

// Command-line arguments handling
fn handle_args() -> Option<(Config, Vec<Mode>)> {
    let mut config = compiletest::default_config();

    let args: Vec<_> = env::args().skip(2).collect();
    let opts = options();

    let matches = match opts.parse(&args[..]) {
        Ok(matches) => matches,
        Err(e) => panic!("{}", e),
    };

    if matches.opt_present("h") {
        usage(&opts, 0);
    }

    let modes: Vec<Mode> = match matches.opt_str("m") {
        Some(s) => {
            let mut err = false;
            let modes = s.split(',').filter_map(|s| match s.parse() {
                Ok(mode) => Some(mode),
                Err(_)   => {
                    println!("'{}' is not a valid mode", s);
                    err = true;
                    None
                }
            }).collect();
            if err {
                return None
            }
            modes
        }
        None => vec![]
    };

    config.verbose = matches.opt_present("v");

    Some((config, modes))
}

fn main() {
    let (config, mut modes) = match handle_args() {
        Some(opts) => opts,
        None => return
    };

    let tests_folder = Path::new("tests");
    if !tests_folder.is_dir() {
        println!("error: No 'tests' folder were found");
        usage(&options(), 1);
    }


    let modes_specified = !modes.is_empty();
    if !modes_specified {
        modes.push_all(all_modes);
    }

    let mut any_tests_run = false;
    for mode in modes {
        let dir = format!("tests/{}", mode);
        let path = Path::new(&dir[..]);
        if path.is_dir() {
            run_mode(mode, config.clone());
            any_tests_run = true;
        } else if modes_specified {
            println!("Could not find '{}' folder in 'tests'. Please ensure it exists.", mode);
            std::process::exit(1);
        }
    }

    if !any_tests_run {
        println!("No tests were found.");
    }
}
