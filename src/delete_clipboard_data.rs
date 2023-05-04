use x11_clipboard::Clipboard;
use std::time::Duration;

pub fn delete_data() {
    let clip = Clipboard::new().unwrap();
    let val = clip.load(clip.setter.atoms.clipboard,
        clip.setter.atoms.utf8_string,
        clip.setter.atoms.property, Duration::from_secs(3)).unwrap();
    let val = String::from_utf8(val).unwrap();
    println!("{}", val);
}
