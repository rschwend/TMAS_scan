// Kleine digitale Uhr – reine Standardbibliothek, keine externen Crates.
//
// Verwendete std-Module:
//   std::time    -> aktuelle Zeit ermitteln
//   std::thread  -> Sekundentakt (sleep)
//   std::io      -> Ausgabe ins Terminal ohne Zeilenumbruch (flush)
//   std::env     -> optionale Kommandozeilenargumente auswerten
//   std::process -> sauberes Beenden

use std::env;
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Wandelt Sekunden seit Unix-Epoch in eine HH:MM:SS Anzeige (UTC) um.
fn format_time(epoch_secs: u64) -> String {
    let seconds_in_day = epoch_secs % 86_400;
    let hours = seconds_in_day / 3600;
    let minutes = (seconds_in_day % 3600) / 60;
    let seconds = seconds_in_day % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn current_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| {
            eprintln!("Systemzeit liegt vor 1970-01-01, breche ab.");
            process::exit(1);
        })
        .as_secs()
}

fn print_usage() {
    println!("digitaluhr – einfache Konsolen-Digitaluhr (UTC)");
    println!();
    println!("Verwendung:");
    println!("  digitaluhr            läuft dauerhaft, aktualisiert jede Sekunde");
    println!("  digitaluhr --once     gibt die aktuelle Zeit einmal aus und beendet");
    println!("  digitaluhr --help     zeigt diese Hilfe an");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    let once = args.iter().any(|a| a == "--once");

    if once {
        println!("{}", format_time(current_epoch_secs()));
        return;
    }

    println!("Digitale Uhr (UTC) – Strg+C zum Beenden\n");

    let stdout = io::stdout();
    loop {
        let mut handle = stdout.lock();
        // \r setzt den Cursor an den Zeilenanfang zurück, sodass die
        // Anzeige "live" aktualisiert wird statt neue Zeilen zu erzeugen.
        write!(handle, "\r{}", format_time(current_epoch_secs())).unwrap();
        handle.flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}
