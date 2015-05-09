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

fn main() {
    let (config, mut modes) = match handle_args() {
        Some(opts) => opts,
        None => return
    };

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

const USAGE: &'static str = "Execute the compiletest subcommand

Usage:
    cargo compiletest [<args>...]";

fn usage(opts: &Options) -> ! {
    print!("{}", opts.usage(USAGE));
    std::process::exit(0);
}

fn handle_args() -> Option<(Config, Vec<Mode>)> {
    let mut config = compiletest::default_config();

    let args: Vec<_> = env::args().skip(2).collect();

    let mut opts = Options::new();
    opts.optflag("v", "verbose", "Use verbose output")
        .optflag("h", "help", "Print this help message and exit")
        .optopt("m", "modes", "Specify a comma-separated list of modes  to run (default all)", "MODES");

    let matches = match opts.parse(&args[..]) {
        Ok(matches) => matches,
        Err(e) => panic!("{}", e),
    };

    if matches.opt_present("h") {
        usage(&opts);
    }



    //cfg.mode = mode.parse().ok().expect("Invalid mode");

    config.verbose = matches.opt_present("v");

    Some((config, vec!()))
}
