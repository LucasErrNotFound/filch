use x11_clipboard::Clipboard;
use spinners::{Spinner, Spinners};
use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn listen_data(filename: String) -> std::io::Result<()> {
    let _loading: Spinner = Spinner::new(Spinners::Dots, "I'm now listening for data...".to_string());
    let clipboard: Clipboard = Clipboard::new().unwrap();
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    loop {
        let val: Vec<u8> = clipboard
            .load_wait(
                clipboard.setter.atoms.clipboard,
                clipboard.setter.atoms.string,
                clipboard.setter.atoms.property,
            ).unwrap();

        let val: String = String::from_utf8(val).unwrap();
        store_file(&val, &filename, &utc.to_string(), &local.to_string())?;

        println!("\n\n{}", utc);
        println!("{}", local);
        println!("\n=====> {}\n", val);
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
