use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    fs::Metadata,
    io,
    path::Path,
    time::{Instant, SystemTime},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

struct Drive {
    letter: char,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

struct File {
    name: String,
    path: Box<Path>,
    meta: Option<Metadata>,
}

struct Dir {
    name: String,
    path: Box<Path>,
    dirs: Vec<Dir>,
    files: Vec<File>,
    meta: Option<Metadata>,
}

struct RW {
    drives: Vec<Drive>,
    curpath: Option<Box<Path>>,
    curdir: Option<Dir>,
    sel_file: Option<u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // opener::open(r"J:\Mujchiny.jenschiny.i.deti.2014.Blu-ray.1080p_Плейлист_250.mkv")?;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut rw = RW {
        drives: vec![],
        curdir: None,
        sel_file: None,
        curpath: None,
    };
    for letter in 'A'..='Z' {
        if std::fs::read_dir(format!("{letter}:")).is_ok() {
            rw.drives.push(Drive {
                letter,
                dirs: vec![],
                files: vec![],
            })
        }
    }
    loop {
        terminal.draw(|f| ui(f, &rw))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, rw: &RW) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Percentage(80),
                Constraint::Max(2),
            ]
            .as_ref(),
        )
        .split(f.size());

    let curpathblock = Block::default().borders(Borders::BOTTOM);
    let curpath = match &rw.curpath {
        Some(path) => path.to_string_lossy().to_string(),
        None => "Home".to_owned(),
    };
    let curpathparag = Paragraph::new(curpath).block(curpathblock);
    f.render_widget(curpathparag, chunks[0]);
    let dirs: Vec<Spans> = match &rw.curdir {
        Some(dir) => std::fs::read_dir(dir.path.clone())
            .unwrap()
            .map(|path| Spans::from(path.unwrap().file_name().to_string_lossy().to_string()))
            .collect(),
        None => rw
            .drives
            .iter()
            .map(|drive| Spans::from(format!("{}:", drive.letter)))
            .collect(),
    };
    let dirsparag = Paragraph::new(dirs);
    f.render_widget(dirsparag, chunks[1]);
    let block = Block::default().title("Block 2").borders(Borders::TOP);
    f.render_widget(block, chunks[2]);
}
