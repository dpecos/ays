use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

const TTY_NAME: &str = "/dev/tty";

const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RESET: &str = "\x1b[0m";

fn clear_tty() -> io::Result<usize> {
    let tty = File::open(TTY_NAME)?;
    let mut reader = BufReader::new(tty);
    let mut buffer = String::new();
    reader.read_line(&mut buffer)
}

fn prompt_user(prompt: Option<String>) -> io::Result<bool> {
    let mut tty_wo = File::create(TTY_NAME)?;
    write!(
        tty_wo,
        "{} [y/N] ",
        prompt.unwrap_or("Are you sure?".to_string())
    )?;

    let tty_ro = File::open(TTY_NAME)?;
    let input = tty_ro
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as char);

    let contents = input.unwrap();
    let mut confirmed = contents.eq_ignore_ascii_case(&'y');

    if !contents.eq(&'\n') {
        // if amount of chars left to consume is different than 1 (/n) it wasn't an affirmative
        // response, but something else
        if clear_tty()? != 1 {
            confirmed = false
        }
    }

    if confirmed {
        write!(tty_wo, "{}Ok!", COLOR_GREEN)?;
    } else {
        write!(tty_wo, "{}Cancelled", COLOR_RED)?;
    }
    writeln!(tty_wo, "{}", COLOR_RESET)?;

    // tty_ro.close();
    // tty_wo.close();

    Ok(confirmed)
}

fn read_line_from_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn pipe_stdin_into_stdout(first_line_from_stdin: String) -> io::Result<()> {
    let stdin = io::stdin();
    let line_reader = io::Cursor::new(first_line_from_stdin);
    let input_reader = line_reader.chain(BufReader::new(stdin));

    let mut stdout = io::stdout().lock();

    for (_index, line) in input_reader.lines().enumerate() {
        stdout.write_all(line?.as_bytes())?;
        stdout.write(b"\n")?;
    }

    stdout.flush()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let prompt = args.get(1);

    let line_from_stdin = read_line_from_stdin()?;

    if !line_from_stdin.is_empty() {
        match prompt_user(prompt.cloned()) {
            Ok(answer) => {
                if answer {
                    pipe_stdin_into_stdout(line_from_stdin)?
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }
    Ok(())
}
