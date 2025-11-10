use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VideoInfo {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Deserialize, Debug)]
pub struct Stream {
    pub codec_name: Option<String>,
    pub codec_type: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Format {
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
}