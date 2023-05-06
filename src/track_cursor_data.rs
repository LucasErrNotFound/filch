use x11_clipboard::Clipboard;
use spinners::{Spinner, Spinners};
use chrono::prelude::*;
use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn track_data(filename: String) -> std::io::Result<()> {
    let _loading: Spinner = Spinner::new(Spinners::Dots, "I'm now listening for data...".to_string());
    let clipboard: Clipboard = Clipboard::new().unwrap();
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    let last: String = String::new();

    loop {
        if let Ok(clip) = clipboard.load_wait(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property
        ) {
            let clip: Cow<str> = String::from_utf8_lossy(&clip);
            let clip: &str = clip
                .trim_matches('\u{0}')
                .trim();
            if !clip.is_empty() && last != clip {
                let last: String = clip.to_owned();

                store_file(&last, &filename, &utc.to_string(), &local.to_string())?;
                println!("\n{}", utc);
                println!("{}", local);
                println!("\n=====> {}\n\n", last);
            }
        }
    }
}

fn store_file(data: &String, FILENAME: &String, utc: &String, local: &String) -> std::io::Result<()> {
    let file_path: &String = FILENAME;
    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true) // create the file if it doesn't exist
        .open(file_path)?;

    let mut writer = BufWriter::new(&file);
    writeln!(writer, "\n{}", utc)?;
    writeln!(writer, "{}", local)?;
    writeln!(writer, "\n======> {}", data.to_string())?;
    writer.flush()?;

    Ok(())
}
