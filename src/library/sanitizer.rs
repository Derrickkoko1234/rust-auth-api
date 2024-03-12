// sanitizes client supplied data

use html_purifier::{purifier, Settings};

pub trait Sanitizer {
    fn prepare(&mut self);
}

pub fn purify(content: &str)->String{
    let settings = Settings {
        ..Settings::default()
    };
    purifier(content, settings)
}