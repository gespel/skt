use std::io;
use sktlib::core::manifest_creator::ManifestCreator;

use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::style::*;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Result, Write};

fn main() -> Result<()> {
    let m = ManifestCreator::new();

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let items = vec!["Deployment", "Service"];
    let mut selected = 0usize;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // Header
                    Constraint::Length(5),  // Info
                    Constraint::Min(3),     // Menü
                ])
                .split(frame.size());

            let header = Paragraph::new("Stens Kubernetes Templator - qPress 'q' to quit")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

            frame.render_widget(header, chunks[0]);

            let info_text = match items[selected] {
                "Deployment" => "Ein Deployment erzeugt Pods basierend auf einem Template.",
                "Service"    => "Ein Service stellt stable Netzwerkkonnektivität zu Pods bereit.",
                _ => "",
            };

            let info = Paragraph::new(info_text)
                .block(Block::default().title("Info").fg(Color::LightBlue).borders(Borders::ALL));

            frame.render_widget(info, chunks[1]);

            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    ListItem::new(Line::from(*item).style(style))
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().title("Select").borders(Borders::ALL));

            frame.render_widget(list, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < items.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => break,
                    KeyCode::Char('q') => {
                        selected = usize::MAX;
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    if selected != usize::MAX {
        match items[selected] {
            "Deployment" => {
                let mut name = String::new();
                let mut image = String::new();

                print!("Enter deployment name: ");
                stdout().flush()?;
                io::stdin().read_line(&mut name)?;
                name = name.trim().to_string();

                print!("Enter image name: ");
                stdout().flush()?;
                io::stdin().read_line(&mut image)?;
                image = image.trim().to_string();

                println!("{}", m.create_deployment(name.as_str(), image.as_str()).unwrap())
            },
            _ => {

            }
        }
    } else {
        println!("Aborted.");
    }

    Ok(())
}
