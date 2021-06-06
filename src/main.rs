use anyhow::Result;
use crossterm::{
    cursor::{
        DisableBlinking, EnableBlinking, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show,
    },
    event::{poll, read, Event, KeyCode},
    style::Print,
    terminal::{self, enable_raw_mode, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{io::stdout, io::Write, time::Duration};
use terminal::Clear;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    Editor::default().run()
}

struct Editor {
    running: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self { running: true }
    }
}

impl Editor {
    pub fn run(&mut self) -> Result<()> {
        let mut stdout = stdout();

        enable_raw_mode()?;
        stdout
            .execute(EnterAlternateScreen)?
            .execute(EnableBlinking)?
            .execute(Hide)?;

        self.draw_welcome_screen()?;

        while self.running {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => match event.code {
                        KeyCode::Esc => break,
                        KeyCode::Left => {
                            stdout.execute(MoveLeft(1))?;
                        }
                        KeyCode::Right => {
                            stdout.execute(MoveRight(1))?;
                        }
                        KeyCode::Up => {
                            stdout.execute(MoveUp(1))?;
                        }
                        KeyCode::Down => {
                            stdout.execute(MoveDown(1))?;
                        }
                        _ => {}
                    },
                    Event::Mouse(_event) => {}
                    Event::Resize(_width, _height) => self.draw_welcome_screen()?,
                }
            }

            stdout.execute(Show)?.flush()?;
        }

        stdout
            .execute(Show)?
            .execute(DisableBlinking)?
            .execute(Clear(ClearType::All))?
            .execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn draw_welcome_screen(&self) -> Result<()> {
        let mut stdout = stdout();
        let (width, height) = terminal::size()?;
        for row in 0..height {
            stdout
                .execute(MoveTo(0, row))?
                .execute(Clear(ClearType::CurrentLine))?;
            let message = if row == height / 3 {
                let mut message = format!("Wave editor -- version {}", VERSION);
                let padding = width.saturating_sub(message.len() as _) / 2;
                let spaces = " ".repeat(padding.saturating_sub(1) as _);
                message = format!("~{}{}", spaces, message);
                message.truncate(width as _);
                message
            } else {
                "~\r".to_string()
            };
            stdout.execute(Print(message))?;
        }
        Ok(())
    }
}
