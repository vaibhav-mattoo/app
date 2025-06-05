use notify::{Watcher, RecursiveMode, RecommendedWatcher, Event};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs;

pub fn monitor_history_file() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        notify::Config::default(),
    )?;
    
    let history_files = vec![
        dirs::home_dir().unwrap().join(".zsh_history"),
        dirs::home_dir().unwrap().join(".bash_history"),
    ];
    
    for file in &history_files {
        if file.exists() {
            watcher.watch(file, RecursiveMode::NonRecursive)?;
        }
    }
    
    loop {
        match rx.recv() {
            Ok(event) => {
                println!("History file changed: {:?}", event);
                if let Some(latest_command) = get_latest_command()? {
                    println!("New command: {}", latest_command);
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

pub fn get_latest_command() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().unwrap();
    let zsh_history = home.join(".zsh_history");
    
    if zsh_history.exists() {
        let content = fs::read_to_string(&zsh_history)?;
        let lines: Vec<&str> = content.lines().collect();
        
        if let Some(last_line) = lines.last() {
            if let Some(command_part) = last_line.split(';').nth(1) {
                return Ok(Some(command_part.to_string()));
            }
        }
    }
    
    Ok(None)
}