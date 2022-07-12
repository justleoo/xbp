use std::{
    env,
    io::Result,
    process::{Command, Stdio},
};
mod util;
use util::*;

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
            } it => println!("Unknown action \"{RED}{it}{RESET}\". Use {RED}xbp help{RESET} to see all commands."), 
        },
        None => help(),
    }
    Ok(())
}
