use std::process::Stdio;

use super::raw_info::YtDlpRawInfo;

use {
    anyhow::{anyhow, Result},
    tauri::api::process::Command,
};

#[derive(PartialEq)]
pub enum Protocol {
    HTTP,
    HLS,
    DASH,
    Other,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Resolution {
    Audio,
    SD240P,
    SD360P,
    SD480P,
    HD720P,
    HD1080P,
    HD2160P,
    SHD4K,
    SHD8K,
}

pub struct Format {
    pub url: String,
    protocol: Protocol,
    pub resolution: Resolution,
}

pub struct Info {
    pub format: Vec<Format>,
}

pub fn fetch_info_from_url(url: String) -> Result<YtDlpRawInfo> {
    // TODO: if .arg(...) gets implemented in tauri v2, use that
    let proc_output = Command::new_sidecar("yt-dlp")?
        .args([url.as_str(), "--skip-download", "--dump-json"])
        .output()?;

    // TODO: check proc_output.status.success()
    let output = proc_output.stdout;
    let raw_info: YtDlpRawInfo = serde_json::from_str(&output)?;
    Ok(raw_info)
}

pub fn process_raw_info(raw: YtDlpRawInfo) -> Result<Info> {
    let mut info = Info {
        format: Vec::with_capacity(raw.formats.len()),
    };
    for format in raw.formats {
        let url = format.url;

        let protocol = match format.protocol.as_str() {
            "http" | "https" | "ftp" | "ftps" => Protocol::HTTP,
            "m3u8" | "m3u8_native" => Protocol::HLS,
            "http_dash_segments" => Protocol::DASH,
            _ => Protocol::Other,
        };

        let resolution = if format.resolution.contains("x") {
            let (w, h) = {
                // NOTE: always safe as long as the pattern exists in the string
                let (a, b) = unsafe { format.resolution.split_once("x").unwrap_unchecked() };
                (a.parse::<u32>()?, b.parse::<u32>()?)
            };
            match (w * h) as u32 {
                8294401..=u32::MAX => Resolution::SHD8K,
                3686401..=8294400 => Resolution::SHD4K,
                2073601..=3686400 => Resolution::HD2160P,
                921601..=2073600 => Resolution::HD1080P,
                409921..=921600 => Resolution::HD720P,
                230401..=409920 => Resolution::SD480P,
                130561..=230400 => Resolution::SD360P,
                ..=1305060 => Resolution::SD240P,
            }
        } else if format.resolution.contains("audio only") {
            Resolution::Audio
        } else if format.resolution.contains("p") {
            let resolution_class = format.resolution;
            match resolution_class.as_str() {
                "240p" => Resolution::SD240P,
                "360p" => Resolution::SD360P,
                "480p" => Resolution::SD480P,
                "720p" => Resolution::HD720P,
                "936p" | "1080p" => Resolution::HD1080P,
                _ => Resolution::SHD4K,
            }
        } else {
            Err(anyhow!("unknown resolution format"))?
        };

        info.format.push(Format {
            url,
            protocol,
            resolution,
        });
    }

    // TODO: keep other resolutions
    let mut highest_res = Resolution::Audio;
    for _ in 1..10 {
        info.format.retain(|fmt| {
            if fmt.resolution < highest_res {
                return false;
            }
            highest_res = fmt.resolution.clone();
            true
        });
    }

    // TODO: keep other protocols than HTTP
    info.format.retain(|fmt| fmt.protocol == Protocol::HTTP);

    Ok(info)
}
