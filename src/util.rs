use std::{
    process::{ExitStatus, self}
};

pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const RESET: &str = "\x1B[0m";

pub fn help() {
    println!("Usage: xbp {GREEN}[OPTIONS]{RESET}\n
OPTIONS
    add, a {RED}<package>{RESET}            Install a package.
    del, d {RED}<package>{RESET}            Delete a package.
    search, s {RED}<package>{RESET}         Search about a package.
    up, u                       Update the system.
        ");
}

pub fn exit(e: ExitStatus) {
    process::exit(e.code().unwrap_or(0));
}
