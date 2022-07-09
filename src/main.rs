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

fn exit(e: ExitStatus) {
    process::exit(e.code().unwrap_or(0));
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
            } it => println!("Unknown action {it}."),
        },
        None => println!("Usage: liy [OPTIONS]\n
OPTIONS
    add <package>            Install a package.
    del <package>            Delete a package.
    search <package>         Search about a package.
    up                       Update the system.
        "),
    }
    Ok(())
}
