use gst::prelude::*;
use std::{env, fs, path::PathBuf};

fn example_main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let path: String;

    if args.len() == 2 {
        let root = "file://";
        let p = fs::canonicalize(PathBuf::from(&args[1])).unwrap();
        path = format!("{}{}", root, p.to_str().unwrap());
    } else {
        println!("Usage: playbin uri");
        std::process::exit(-1)
    };

    let uri: &str = path.as_ref();

    let playbin = gst::ElementFactory::make("playbin", None).unwrap();
    playbin.set_property("uri", uri).unwrap();

    playbin
        .connect("audio-tags-changed", false, |values| {
            let playbin = values[0]
                .get::<glib::Object>()
                .expect("playbin \"audio-tags-changed\" signal values[1]");
            let idx = values[1]
                .get::<i32>()
                .expect("playbin \"audio-tags-changed\" signal values[1]");

            println!("audio tags of audio stream {} changed:", idx);

            let tags = playbin
                .emit_by_name("get-audio-tags", &[&idx])
                .unwrap()
                .unwrap();

            let tags = tags.get::<gst::TagList>().expect("tags");

            if let Some(lyrics) = tags.get::<gst::tags::Lyrics>() {
                println!("  Lyrics: {}", lyrics.get());
            }

            if let Some(artist) = tags.get::<gst::tags::Artist>() {
                println!("  Artist: {}", artist.get());
            }

            if let Some(title) = tags.get::<gst::tags::Title>() {
                println!("  Title: {}", title.get());
            }

            if let Some(album) = tags.get::<gst::tags::Album>() {
                println!("  Album: {}", album.get());
            }

            None
        })
        .unwrap();

    let bus = playbin.bus().unwrap();

    playbin
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed.src().map(|s| s == playbin).unwrap_or(false)
                    && state_changed.current() == gst::State::Playing
                {
                    let bin_ref = playbin.downcast_ref::<gst::Bin>().unwrap();
                    bin_ref.debug_to_dot_file(gst::DebugGraphDetails::all(), "PLAYING");
                }
            }

            _ => (),
        }
    }

    playbin
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    example_main();
}
