use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let history = read_history().expect("Failed to read history");
    let formatted_history: Vec<String> = history
        .iter()
        .enumerate()
        .rev()
        .map(|(i, cmd)| format!("{}     {}", i, cmd))
        .collect();

    let selected_command = fuzzy_search_and_select(&formatted_history)
        .expect("Fuzzy search failed")
        .map(|cmd| cmd.splitn(2, "     ").nth(1).unwrap_or("").to_string());

    if let Some(command) = selected_command {
        copy_to_clipboard(&command).expect("Failed to copy command to clipboard");
        println!("Copied to clipboard: {}", command);
    } else {
        println!("No command selected.");
    }
}

pub fn read_history() -> io::Result<Vec<String>> {
    let shell = env::var("SHELL").unwrap_or_default();
    let history_file = if shell.contains("zsh") {
        env::var("HISTFILE")
            .unwrap_or_else(|_| format!("{}/.zsh_history", env::var("HOME").unwrap()))
    } else {
        env::var("HISTFILE")
            .unwrap_or_else(|_| format!("{}/.bash_history", env::var("HOME").unwrap()))
    };

    let file = fs::File::open(history_file)?;
    let reader = io::BufReader::new(file);
    let history: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            if let Some(pos) = line.find(";") {
                line[(pos + 1)..].to_string()
            } else {
                line
            }
        })
        .collect();
    Ok(history)
}

pub fn fuzzy_search_and_select(history: &[String]) -> io::Result<Option<String>> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut selected_index = 0;
    let mut search_term = String::new();
    let mut matches = get_matches(history, &search_term);

    display_matches(&matches, selected_index, &mut stdout, &search_term);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') | Key::Esc => break,
            Key::Char(c) => {
                search_term.push(c);
                matches = get_matches(history, &search_term);
                display_matches(&matches, selected_index, &mut stdout, &search_term);
            }
            Key::Backspace => {
                search_term.pop();
                matches = get_matches(history, &search_term);
                display_matches(&matches, selected_index, &mut stdout, &search_term);
            }
            Key::Up => {
                if selected_index > 0 {
                    selected_index -= 1;
                }
                display_matches(&matches, selected_index, &mut stdout, &search_term);
            }
            Key::Down => {
                if selected_index < matches.len().saturating_sub(1) {
                    selected_index += 1;
                }
                display_matches(&matches, selected_index, &mut stdout, &search_term);
            }
            Key::Ctrl('c') => {
                println!("\nExiting...");
                return Ok(None);
            }
            _ => {}
        }
    }

    Ok(matches.get(selected_index).cloned().cloned())
}

fn get_matches<'a>(history: &'a [String], search_term: &str) -> Vec<&'a String> {
    let terms: Vec<&str> = search_term.split_whitespace().collect();
    history
        .iter()
        .filter(|cmd| terms.iter().all(|term| cmd.contains(term)))
        .take(10)
        .collect()
}

fn display_matches(
    matches: &[&String],
    selected_index: usize,
    stdout: &mut impl Write,
    search_term: &str,
) {
    write!(stdout, "\x1B[2J\x1B[1;1H").unwrap(); // Clear screen
    write!(stdout, "Search term: {}\r\n", search_term).unwrap(); // Display search term
    write!(stdout, "---------------------------------------------\r\n").unwrap();
    for (i, cmd) in matches.iter().enumerate() {
        if i == selected_index {
            write!(stdout, "> {}\r\n", cmd).unwrap();
        } else {
            write!(stdout, "  {}\r\n", cmd).unwrap();
        }
    }
    write!(stdout, "---------------------------------------------\r\n").unwrap();
    stdout.flush().unwrap();
}

pub fn copy_to_clipboard(command: &str) -> io::Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(command.to_owned()).unwrap();
    Ok(())
}
