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
    add, a {RED}<package>{RESET}            Install a package.
    del, d {RED}<package>{RESET}            Delete a package.
    search, s {RED}<package>{RESET}         Search about a package.
    up, u                       Update the system.
        ");
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(it) => match it.as_str() {
            "add" | "a" => {
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
            "del" | "d" => {
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
            "search" | "s" => {
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
            "up" | "u" => {
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
            "help" | "h" => {
                help();
            } it => println!("Unknown action \"{RED}{it}{RESET}\". Use {RED}liy help{RESET} to see all commands."),
        },
        None => help(),
    }
    Ok(())
}
