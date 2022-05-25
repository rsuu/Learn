use std::path::{Path, PathBuf};

fn main() {
    let file = "./test.cue";
    let file_path = PathBuf::from(file);
    let metadata = parse(&file_path);

    for v in metadata.iter() {
        println!("{}", to_ffmpeg(v, &file_path).unwrap());
    }
}

fn to_ffmpeg(meta: &Metadata, file_path: &Path) -> Option<String> {
    let ffmpeg = String::from("ffmpeg -i ");
    let b = format!(
        "\"{path}\" -c copy -ss {start} -t {len} -metadata artist=\"{artist}\"
        ",
        path = file_path.display(),
        start = meta.start,
        len = meta.len,
        artist = meta.artist.as_ref().unwrap(),
    );

    Some(ffmpeg + &b)
    /*
     ffmpeg -i "cdimage.wav" \
     -c copy \
        -ss 01:12:00.733333 -t 00:01:30.213333 \
        -metadata artist="汐宮栞 starring 花澤香菜" \
        -metadata title="コイノシルシ feat.汐宮栞 (TV-EDIT)" \
        -metadata album="神のみぞ知るセカイ Original Soundtrack" \
        -metadata track="44/46" \
        -metadata genre="Soundtrack" \
        -metadata date="2011" \
        '44 - 汐宮栞 starring 花澤香菜 - コイノシルシ feat.汐宮栞 (TV-EDIT).opus'
    */
}
fn parse(file_path: &Path) -> Vec<Metadata> {
    use cue::{cd::CD, cd_text::PTI};
    use cue_split::utils::file;

    let cue_sheet = match file::read_from_file(&file_path) {
        Ok(t) => t,
        Err(_) => {
            panic!()
        }
    };

    let cd = CD::parse(cue_sheet).unwrap();

    let mut v: Vec<Metadata> = Vec::default();

    for (index, track) in cd.tracks().iter().enumerate() {
        v.push(Metadata {
            title: track.get_cdtext().read(PTI::Title),
            artist: track.get_cdtext().read(PTI::Performer),
            start: track.get_start(),
            len: track.get_length(),
            track: index + 1,
        });
    }

    v
}

#[derive(Debug)]
pub struct Metadata {
    title: Option<String>,
    artist: Option<String>,
    start: i64,
    len: i64,
    track: usize,
}
