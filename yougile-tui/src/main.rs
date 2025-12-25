use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
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
    // Initialize logging to file to avoid interfering with TUI
    let log_file = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("yougile-tui")
        .join("yougile-tui.log");
    
    // Create log directory if it doesn't exist
    if let Some(parent) = log_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?)))
        .try_init()?;

    log::info!("Starting YouGile TUI application");
    log::info!("Log file: {:?}", log_file);

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
                // Pass ALL key events to app handler
                // App will decide whether to quit or not
                app.handle_key_event(key).await?;
            }
        } else {
            // Timeout - process any pending async tasks
            app.process_pending().await?;
        }

        // Check if app should quit
        if app.should_quit() {
            log::info!("Application quit requested");
            return Ok(());
        }
    }
}
