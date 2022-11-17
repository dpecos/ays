use std::fs::File;
use std::io::{self, prelude::*, Write};

fn prompt_user() -> io::Result<bool> {
    let mut tty_wo = File::create("/dev/tty")?;
    write!(tty_wo, "Are you sure? y/[n] ")?;

    let tty_ro = File::open("/dev/tty")?;
    let input = tty_ro
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char);

    writeln!(tty_wo, "")?;

    Ok(input.unwrap().eq_ignore_ascii_case(&'y'))
}

fn pipe_stdin_into_stdout() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();

    for (_index, line) in stdin.lines().enumerate() {
        stdout.write_all(line?.as_bytes())?;
        stdout.write(b"\n")?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    match prompt_user() {
        Ok(answer) => {
            if answer {
                pipe_stdin_into_stdout()?
            }
        }
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}
