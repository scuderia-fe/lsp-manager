use std::{error::Error, io};

use crate::registry::Package;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const ITEM_HEIGHT: usize = 4;
const INFO_TEXT: &str = "(Q) quit | (K) move up | (J) move down | (Enter) select";

struct App {
    items: Vec<Package>,
    state: TableState,
    scroll_state: ScrollbarState,
}

impl App {
    fn new(packages: Vec<Package>) -> Self {
        Self {
            items: packages,
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::default(),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    Char('j') | Down => app.next(),
                    Char('k') | Up => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(f.size());

    render_table(f, app, rects[0]);
    render_scrollbar(f, app, rects[0]);
    render_footer(f, app, rects[1]);
}

fn render_footer(f: &mut Frame, app: &mut App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT)).centered().block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double),
    );
    f.render_widget(info_footer, area);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header = ["Name", "Description"]
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let description = match data.description.clone().char_indices().nth(50) {
            None => data.description.clone(),
            Some(..) => String::from(&data.description.clone()[..50]),
        };

        let item = vec![
            Cell::from(Text::from(data.name.clone())),
            Cell::from(Text::from(data.description.clone())),
        ];

        item.iter().cloned().collect::<Row>().height(4)
    });

    let t = Table::new(rows, [50, 100]).header(header);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

pub fn print_table(packages: Vec<Package>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(packages);
    let res = run_app(&mut terminal, app);
    Ok(())
}
