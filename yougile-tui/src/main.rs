use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::error::Error;
use std::io;
use std::time::Duration;

mod api;
mod app;
mod config;
mod handlers;
mod ui;

use app::App;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()?;

    log::info!("Starting YouGile TUI application");

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => {
            log::info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("\nPlease set environment variables:");
            eprintln!("  export YOUGILE_API_URL=\"https://api.yougile.com\"");
            eprintln!("  export YOUGILE_API_TOKEN=\"your_token\"");
            eprintln!("\nOr create ~/.config/yougile-tui/config.toml with:");
            eprintln!("  api_url = \"https://api.yougile.com\"");
            eprintln!("  api_token = \"your_token\"");
            std::process::exit(1);
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = match App::new(config).await {
        Ok(app) => app,
        Err(e) => {
            // Restore terminal before exiting
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            
            eprintln!("Failed to initialize app: {}", e);
            std::process::exit(1);
        }
    };

    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Fatal error: {}", err);
        std::process::exit(1);
    }

    log::info!("Application closed successfully");
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, &app))?;

        // Handle events with timeout for async operations
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Handle quit
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    log::info!("User initiated quit");
                    return Ok(());
                }

                // Pass to app handler
                app.handle_key_event(key).await?;
            }
        } else {
            // Timeout - process any pending async tasks
            app.process_pending().await?;
        }

        // Check if app should quit
        if app.should_quit() {
            return Ok(());
        }
    }
}
