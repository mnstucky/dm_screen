use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\t') => break,
            Key::Char(c) => write!(stdout, "\nChar").unwrap(),
            _ => write!(stdout, "\nOther").unwrap(),
        }
        write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();
        stdout.flush().unwrap();
    }
    print!("> ");
}
