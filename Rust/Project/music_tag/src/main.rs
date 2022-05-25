use serde::Deserialize;
use taglib;

use std::{env, fs, process};

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: u32,
    pub track: u32,
    pub comment: String,
    pub genre: String,
}

fn main() {
    let editor: String = env::var("EDITOR").unwrap();
    let args: Vec<_> = std::env::args().collect();
    let music_path: &str = args[1].as_ref();
    let tag_path: &str = "/tmp/music_tag.toml";

    fs::write(tag_path, Tag::print_struct(Tag::read_struct(music_path)))
        .expect("Unable to write file");

    process::Command::new(editor)
        .arg(tag_path)
        .status()
        .expect("Something went wrong");

    let text: String = fs::read_to_string(tag_path).expect("#01: /tmp/music_tag.toml");

    Tag::edit_tag(music_path, text.as_ref());
}

impl Tag {
    pub fn edit_tag(music_path: &str, text: &str) {
        let file = taglib::File::new(music_path).expect("#09: file");
        let tag: Tag = toml::from_str(text).expect("#08: tags");
        let mut file_tag = file.tag().unwrap();

        file_tag.set_title(&tag.title);
        file_tag.set_artist(&tag.artist);
        file_tag.set_album(&tag.album);
        file_tag.set_year(tag.year);
        file_tag.set_track(tag.track);
        file_tag.set_comment(&tag.comment);
        file_tag.set_genre(&tag.genre);

        file.save();
    }

    pub fn read_struct(music_path: &str) -> Self {
        let file = taglib::File::new(music_path).expect("#09: file");
        let file_tag = file.tag().unwrap();

        Tag {
            title: file_tag.title().unwrap_or_default(),
            artist: file_tag.artist().unwrap_or_default(),
            album: file_tag.album().unwrap_or_default(),
            year: file_tag.year().unwrap_or_default(),
            track: file_tag.track().unwrap_or_default(),
            comment: file_tag.comment().unwrap_or_default(),
            genre: file_tag.genre().unwrap_or_default(),
        }
    }

    pub fn print_struct(tag: Tag) -> String {
        format!(
            "title   = \"{}\"
artist  = \"{}\"
album   = \"{}\"
year    = {}
track   = {}
comment = \"{}\"
genre   = \"{}\"",
            tag.title, tag.artist, tag.album, tag.year, tag.track, tag.comment, tag.genre,
        )
    }
}
