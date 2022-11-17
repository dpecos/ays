use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

const TTY_NAME: &str = "/dev/tty";

fn clear_tty() -> io::Result<usize> {
    let tty = File::open(TTY_NAME)?;
    let mut reader = BufReader::new(tty);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)
}

fn prompt_user(prompt: Option<String>) -> io::Result<bool> {
    let mut tty_wo = File::create("/dev/tty")?;
    write!(
        tty_wo,
        "{} [y/N] ",
        prompt.unwrap_or("Are you sure?".to_string())
    )?;

    let tty_ro = File::open("/dev/tty")?;
    let input = tty_ro
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char);

    writeln!(tty_wo, "")?;

    let contents = input.unwrap();
    let confirmed = contents.eq_ignore_ascii_case(&'y');

    if !contents.eq(&'\n') {
        // if amount of chars left to consume is different than 1 (/n) it wasn't an affirmative
        // response, but something else
        if clear_tty()? != 1 {
            return Ok(false);
        } else {
            return Ok(confirmed);
        }
    }

    Ok(confirmed)
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
    let args: Vec<String> = env::args().collect();
    let prompt = args.get(1);

    match prompt_user(prompt.cloned()) {
        Ok(answer) => {
            if answer {
                pipe_stdin_into_stdout()?
            }
        }
        Err(err) => println!("Error: {}", err),
    }
    Ok(())
}
