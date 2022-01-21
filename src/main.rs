#[macro_use]
extern crate lazy_static;
use std::{
    io::{stdout, Write},
    time::Duration,
};

use chrono::prelude::Local;
use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand,
};
use figlet_rs::FIGfont;

lazy_static! {
    static ref FONT: FIGfont = FIGfont::standand().expect("Failed to load standard font");
}

const FORMAT_12_HOUR: &'static str = "%-I:%M:%S %p";
const FORMAT_24_HOUR: &'static str = "%H:%M:%S";

fn figlet(text: &str) -> Option<String> {
    Some(format!("{}", FONT.convert(text)?))
}

fn timer(time_format: &str, refresh_time: Duration) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    stdout.execute(cursor::Hide)?;

    loop {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 5))?;

        let time = Local::now().time().format(time_format).to_string();
        let figure = format!("{}", figlet(&time).unwrap());
        let figure_lines: Vec<&str> = figure.split('\n').collect();

        // should be (roughly) half of terminal size. Take figure size into account.
        let term_size = terminal::size()?;
        let x_offset = (term_size.0 / 2) - (figure_lines.first().unwrap().len() / 2) as u16;
        let y_offset = (term_size.1 / 2) - (figure_lines.len() / 2) as u16;

        for (n, line) in figure_lines.iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(x_offset, y_offset + n as u16))?
                .queue(style::PrintStyledContent(line.magenta()))?;
        }

        stdout.flush()?;

        std::thread::sleep(refresh_time);
    }
}

fn main() {
    ctrlc::set_handler(move || {
        stdout()
            .queue(cursor::Show)
            .expect("Error enabling cursor")
            .queue(cursor::MoveTo(0, 0))
            .expect("Error moving cursor")
            .queue(terminal::Clear(terminal::ClearType::All))
            .expect("Error clearing terminal");

        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // sleep time for redrawing screen
    let refresh_time = Duration::from_millis(100);

    let time_format = match "24h" {
        "12h" => FORMAT_12_HOUR,
        "24h" => FORMAT_24_HOUR,
        _ => FORMAT_24_HOUR,
    };

    timer(time_format, refresh_time).expect("Error running timer");
}
