mod server;

use server::start_server;
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    time::Instant,
};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{block::Title, Block, Padding, Paragraph},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    let db_ctx = Arc::new(Mutex::new(
        rusqlite::Connection::open("passport.db3").unwrap(),
    ));

    start_server(db_ctx.clone());

    color_eyre::install()?;
    let result = AppUI::new().run(ratatui::init());
    ratatui::restore();
    result
}

pub struct AppUI {
    input: String,
}

impl AppUI {
    fn new() -> Self {
        Self {
            input: "".to_string(),
        }
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut last_update = Instant::now();
        loop {
            if last_update.elapsed().as_millis() >= 1000 / 60 {
                last_update = Instant::now();

                terminal.draw(|frame| self.render(frame))?;

                match event::read()? {
                    Event::Key(k) => match k.code {
                        KeyCode::Char(c) => match c {
                            'c' if k.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                            _ => self.input.push(c),
                        },
                        KeyCode::Backspace => {
                            if k.modifiers.contains(KeyModifiers::ALT) {
                                // TODO alt backspace key combo
                            } else {
                                self.input.pop();
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = layout(frame);

        frame.render_widget(
            paragraph(
                "Console",
                vec![
                    "KTT".italic(),
                    "> ".blue(),
                    self.input.clone().into(),
                    "_".into(),
                ],
            ),
            layout[0],
        );

        frame.render_widget(
            paragraph("Status", vec!["EVERYTHING BROKE".red()]),
            layout[1],
        );
    }
}

fn paragraph<'a>(title: &'a str, content: Vec<Span<'a>>) -> Paragraph<'a> {
    Paragraph::new(Line::from(content)).block(block(title))
}

fn layout(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area())
}

fn block<'a>(text: &'a str) -> Block<'a> {
    Block::bordered()
        .padding(Padding::symmetric(2, 1))
        .gray()
        .title(Line::from(format!(" {} ", text)).alignment(Alignment::Center))
}
