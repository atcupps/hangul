use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use hangul::string::StringComposer;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| render(frame, &app))?;

        match event::read()? {
            Event::Key(key) if handle_key(&mut app, key) => break Ok(()),
            Event::Resize(_, _) => {}
            _ => {}
        }
    }
}

#[derive(Debug)]
struct App {
    composer: StringComposer,
    status: String,
}

impl App {
    fn new() -> Self {
        Self {
            composer: StringComposer::new(),
            status: "Mock Korean keyboard: type roman keys, Esc to quit".to_string(),
        }
    }

    fn composed_text(&self) -> String {
        self.composer
            .as_string()
            .unwrap_or_else(|err| format!("Error composing text: {err}"))
    }

    fn backspace(&mut self) {
        match self.composer.pop() {
            Ok(Some(letter)) => {
                self.status = format!("Removed '{:?}'", letter);
            }
            Ok(None) => {
                self.status = "Nothing to backspace".to_string();
            }
            Err(err) => {
                self.status = format!("Error during backspace: {err}");
            }
        }
    }

    fn handle_char(&mut self, key_char: char) {
        let input = match map_key_to_jamo(key_char) {
            Some(c) => c,
            None => key_char,
        };

        match self.composer.push_char(input) {
            Ok(()) => {
                self.status = format!("Added '{input}'");
            }
            Err(err) => {
                self.status = format!("Error adding '{input}': {err}");
            }
        }
    }
}

fn map_key_to_jamo(key_char: char) -> Option<char> {
    match key_char {
        // Consonants (2-beolsik)
        'r' => Some('ㄱ'),
        'R' => Some('ㄲ'),
        's' => Some('ㄴ'),
        'e' => Some('ㄷ'),
        'E' => Some('ㄸ'),
        'f' => Some('ㄹ'),
        'a' => Some('ㅁ'),
        'q' => Some('ㅂ'),
        'Q' => Some('ㅃ'),
        't' => Some('ㅅ'),
        'T' => Some('ㅆ'),
        'd' => Some('ㅇ'),
        'w' => Some('ㅈ'),
        'W' => Some('ㅉ'),
        'c' => Some('ㅊ'),
        'z' => Some('ㅋ'),
        'x' => Some('ㅌ'),
        'v' => Some('ㅍ'),
        'g' => Some('ㅎ'),
        // Vowels (2-beolsik)
        'k' => Some('ㅏ'),
        'o' => Some('ㅐ'),
        'O' => Some('ㅒ'),
        'i' => Some('ㅑ'),
        'j' => Some('ㅓ'),
        'p' => Some('ㅔ'),
        'P' => Some('ㅖ'),
        'u' => Some('ㅕ'),
        'h' => Some('ㅗ'),
        'y' => Some('ㅛ'),
        'n' => Some('ㅜ'),
        'b' => Some('ㅠ'),
        'm' => Some('ㅡ'),
        'l' => Some('ㅣ'),
        _ => None,
    }
}

fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }

    match key.code {
        KeyCode::Esc => return true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Backspace => app.backspace(),
        KeyCode::Enter => {
            app.handle_char('\n');
        }
        KeyCode::Char(c) => app.handle_char(c),
        _ => {}
    }

    false
}

fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(5),
        Constraint::Length(4),
    ])
    .split(frame.area());

    let header_text = "Hangul Composer (2-beolsik mock)\nType roman keys: r=ㄱ, s=ㄴ, e=ㄷ, f=ㄹ, a=ㅁ, q=ㅂ, t=ㅅ, d=ㅇ, w=ㅈ, c=ㅊ, z=ㅋ, x=ㅌ, v=ㅍ, g=ㅎ; k=ㅏ, o=ㅐ, i=ㅑ, j=ㅓ, p=ㅔ, u=ㅕ, h=ㅗ, y=ㅛ, n=ㅜ, b=ㅠ, m=ㅡ, l=ㅣ. Shifted q/w/e/r/t give double consonants; O/P give ㅒ/ㅖ.\nPress Esc (or Ctrl+C) to quit.";
    let header_block = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Instructions"))
        .style(Style::default().bold());

    let body_block = Paragraph::new(app.composed_text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1))
                .title("Composed Text"),
        )
        .style(Style::default().italic());

    let footer_block = Paragraph::new(app.status.clone())
        .block(Block::default().borders(Borders::ALL).title("Status"));

    frame.render_widget(header_block, layout[0]);
    frame.render_widget(body_block, layout[1]);
    frame.render_widget(footer_block, layout[2]);
}
