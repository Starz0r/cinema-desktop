use either::Either;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YtDlpRawInfo {
    pub formats: Vec<Format>,
    #[serde(rename = "_has_drm")]
    pub has_drm: Value,
    #[serde(rename = "_version")]
    pub version: Version,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(rename = "format_note")]
    pub format_note: Option<String>,
    #[serde(rename = "format_index")]
    pub format_index: Option<Value>,
    pub url: String,
    #[serde(rename = "manifest_url")]
    pub manifest_url: Option<String>,
    pub language: Option<Value>,
    pub ext: String,
    pub protocol: String,
    pub preference: Option<Value>,
    pub quality: Option<Value>,
    #[serde(rename = "has_drm")]
    pub has_drm: Option<bool>,
    pub vcodec: Option<String>,
    pub tbr: Option<f64>,
    pub resolution: String,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    pub vbr: Option<f64>,
    //#[serde(with = "either::serde_untagged")]
    //pub abr: Either<i64, f64>,
    pub format: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Option<String>,
    #[serde(rename = "filesize_approx")]
    pub filesize_approx: Option<i64>,
    pub fps: Option<Value>,
    pub acodec: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitles {
    pub en: Vec<En>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En {
    pub url: String,
    pub ext: String,
    pub protocol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub id: String,
    pub url: String,
    pub width: i64,
    pub height: i64,
    pub resolution: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestedFormat {
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(rename = "format_index")]
    pub format_index: Value,
    pub url: String,
    #[serde(rename = "manifest_url")]
    pub manifest_url: Option<String>,
    pub tbr: Option<f64>,
    pub ext: String,
    pub fps: Option<f64>,
    pub protocol: String,
    pub preference: Value,
    pub quality: Option<f64>,
    #[serde(rename = "has_drm")]
    pub has_drm: Option<bool>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    #[serde(rename = "dynamic_range")]
    pub dynamic_range: Option<String>,
    pub resolution: String,
    #[serde(rename = "aspect_ratio")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders2,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    pub abr: Option<f64>,
    pub vbr: Option<f64>,
    pub format: String,
    #[serde(rename = "format_note")]
    pub format_note: Option<String>,
    pub language: Option<String>,
    #[serde(rename = "filesize_approx")]
    pub filesize_approx: Option<i64>,
    pub rows: Option<i64>,
    pub columns: Option<i64>,
    #[serde(default)]
    pub fragments: Vec<Fragment>,
    #[serde(rename = "source_preference")]
    pub source_preference: Option<i64>,
    pub asr: Option<i64>,
    pub filesize: Option<i64>,
    #[serde(rename = "audio_channels")]
    pub audio_channels: Option<i64>,
    #[serde(rename = "language_preference")]
    pub language_preference: Option<i64>,
    pub container: Option<String>,
    #[serde(rename = "downloader_options")]
    pub downloader_options: Option<DownloaderOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpHeaders2 {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Accept")]
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fragment {
    pub url: String,
    pub duration: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloaderOptions {
    #[serde(rename = "http_chunk_size")]
    pub http_chunk_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
    #[serde(rename = "current_git_head")]
    pub current_git_head: Value,
    #[serde(rename = "release_git_head")]
    pub release_git_head: String,
    pub repository: String,
}
