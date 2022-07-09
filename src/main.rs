use std::{
    env,
    io::Result,
    process::{
        Command,
        Stdio,
        self,
        ExitStatus
    },
};
mod util;
use util::*;

fn exit(e: ExitStatus) {
    process::exit(e.code().unwrap_or(0));
}

fn help() {
    println!("Usage: liy {GREEN}[OPTIONS]{RESET}\n
OPTIONS
    add {RED}<package>{RESET}            Install a package.
    del {RED}<package>{RESET}            Delete a package.
    search {RED}<package>{RESET}         Search about a package.
    up                       Update the system.
        ");
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(it) => match it.as_str() {
            "add" => {
               exit(
                    Command::new("xbps-install")
                        .arg("-S")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "del" => {
                exit(
                    Command::new("xbps-remove")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "search" => {
               exit(
                   Command::new("xbps-query")
                        .arg("-Rs")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "up" => {
                exit(
                    Command::new("xbps-install")
                        .arg("-Su")
                        .args(args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()?
                        .wait()?,
                );
            }
            "help" => {
                help();
            } it => println!("Unknown action {RED}{it}{RESET}."),
        },
        None => help(),
    }
    Ok(())
}
