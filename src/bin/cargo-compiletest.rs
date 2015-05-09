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
extern crate compiletest_rs as compiletest;
extern crate getopts;

use std::env;
use std::path::{PathBuf, Path};
use std::fs::PathExt;
use getopts::Options;

use compiletest::common::Config;

fn run_mode(mode: &'static str, mut cfg: Config) {
    cfg.mode = mode.parse().ok().expect("Invalid mode");
    cfg.src_base = PathBuf::from(format!("tests/{}", mode));

    compiletest::run_tests(&cfg);
}

const MODES: &'static [&'static str] = &[
    "run-pass",
    "run-pass-valgrind",
    "run-fail",
    "parse-fail",
    "compile-fail",
    "pretty",
    "debuginfo-lldb",
    "debuginfo-gdb",
    "codegen",
    ];

fn main() {
    let config = handle_args();

    for mode in MODES {
        let dir_path = &format!("tests/{}", mode)[..];
        let path = Path::new(dir_path);
        if path.is_dir() {
            run_mode(mode, config.clone());
        }
    }
}

fn handle_args() -> Config {
    let mut config = compiletest::default_config();

    let args: Vec<_> = env::args().skip(1).collect();
    println!("{:?}", args);

    let mut opts = Options::new();
    opts.optflag("v", "verbose", "Use verbose output");
    opts.optflag("h", "help", "Print this help message and exit");

    let matches = match opts.parse(&args[..]) {
        Ok(matches) => matches,
        Err(e) => panic!("{}", e),
    };

    if matches.opt_present("h") {
        let brief = format!("Execute the compiletest subcommand");
        print!("{}", opts.usage(&brief));
        std::process::exit(0);
    }

    config.verbose = matches.opt_present("v");

    config
}
