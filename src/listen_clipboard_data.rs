use x11_clipboard::Clipboard;
use spinners::{Spinner, Spinners};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn listen_data(filename: String) -> std::io::Result<()> {
    let _loading = Spinner::new(Spinners::Dots, "I'm now listening for data...".to_string());
    let clipboard = Clipboard::new().unwrap();

    loop {
        let val = clipboard
            .load_wait(
                clipboard.setter.atoms.clipboard,
                clipboard.setter.atoms.string,
                clipboard.setter.atoms.property,
            ).unwrap();

        let val = String::from_utf8(val).unwrap();
        store_file(val, filename.clone())?;
    }
}

fn store_file(data: String, FILENAME: String) -> std::io::Result<()> {
    let file_path = FILENAME;

    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true) // create the file if it doesn't exist
        .open(file_path)?;

    let mut writer = BufWriter::new(&file);
    writeln!(writer, "\n{}", data.to_string())?;
    writer.flush()?;

    Ok(())
}
