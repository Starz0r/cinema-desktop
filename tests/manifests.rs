use crate::remote_media_info::process_raw_info;
use crate::remote_media_info::raw_info::YtDlpRawInfo;

#[test]
fn youtube_manifest() {
    let manifest = std::fs::read_to_string("tests/youtube.json").unwrap();
    let raw: YtDlpRawInfo = serde_json::from_str(&manifest).unwrap();

    let info = process_raw_info(raw.clone()).unwrap();
    for fmt in info.format {
        dbg!(fmt.resolution);
    }
}

#[test]
fn twitter_manifest() {
    let manifest = std::fs::read_to_string("tests/twitter.json").unwrap();
    let raw: YtDlpRawInfo = serde_json::from_str(&manifest).unwrap();

    let info = process_raw_info(raw.clone()).unwrap();
    for fmt in info.format {
        dbg!(fmt.resolution);
    }
}

#[test]
fn twitch_manifest() {
    let manifest = std::fs::read_to_string("tests/twitch.json").unwrap();
    let raw: YtDlpRawInfo = serde_json::from_str(&manifest).unwrap();

    let info = process_raw_info(raw.clone()).unwrap();
    for fmt in info.format {
        dbg!(fmt.resolution);
    }
}

#[test]
fn youtube_shorts_manifest() {
    let manifest = std::fs::read_to_string("tests/youtube-shorts.json").unwrap();
    let raw: YtDlpRawInfo = serde_json::from_str(&manifest).unwrap();

    let info = process_raw_info(raw.clone()).unwrap();
    for fmt in info.format {
        dbg!(fmt.resolution);
    }
}