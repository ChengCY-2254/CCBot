//! yt-dlp调用

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub struct YtDlp {
    search: Search,
}

pub enum Search {
    Bilibili,
    Youtube,
    TikTok,
}

impl Search {
    pub fn into_str(self) -> &'static str {
        match self {
            Search::Bilibili => "bilisearch",
            Search::Youtube => "ytsearch",
            Search::TikTok => "tiktok",
        }
    }
}

impl YtDlp {
    pub fn new(search: Search) -> YtDlp {
        YtDlp { search }
    }
    pub async fn search(self, keyword: &str, len: u8) -> crate::Result<Vec<VideoInfo>> {
        if len == 0 {
            return Err(anyhow!("搜索长度不能为0"));
        }
        let search_mode = self.search.into_str();
        let search = format!("{search_mode}:{len}:{keyword}");
        let mut yt_dlp_result = Command::new("yt-dlp")
            .args([
                "-j",
                &search,
                "-f",
                "ba[abr>0][vcodec=none]/best",
                "--no-playlist",
            ])
            .spawn()?
            .stdout
            .context("yt-dlp执行错误")?;
        let mut json = String::new();
        yt_dlp_result.read_to_string(&mut json).await?;
        serde_json::from_str(&json).map_err(|why| anyhow!("yt-dlp解析错误:{}", why))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHeaders {
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    pub accept: String,
    #[serde(rename = "Accept-Language")]
    pub accept_language: String,
    #[serde(rename = "Sec-Fetch-Mode")]
    pub sec_fetch_mode: String,
    pub referer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub url: String,
    pub ext: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcodec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acodec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range: Option<String>,
    pub tbr: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    #[serde(rename = "format_id")]
    pub format_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub protocol: String,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    pub vbr: f64,
    pub abr: f64,
    pub resolution: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "filesize_approx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize_approx: Option<i64>,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    #[serde(rename = "current_git_head")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_git_head: Option<String>,
    #[serde(rename = "release_git_head")]
    pub release_git_head: String,
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub uploader: String,
    #[serde(rename = "uploader_id")]
    pub uploader_id: String,
    #[serde(rename = "like_count")]
    pub like_count: i32,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub description: String,
    pub timestamp: i64,
    #[serde(rename = "view_count")]
    pub view_count: i32,
    #[serde(rename = "comment_count")]
    pub comment_count: i32,
    pub id: String,
    #[serde(rename = "_old_archive_ids")]
    pub old_archive_ids: Vec<String>,
    pub title: String,
    #[serde(rename = "http_headers")]
    pub http_headers: HttpHeaders,
    pub formats: Vec<Format>,
    pub duration: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<String>>,
    pub subtitles: HashMap<String, String>,
    #[serde(rename = "webpage_url")]
    pub webpage_url: String,
    #[serde(rename = "original_url")]
    pub original_url: String,
    #[serde(rename = "webpage_url_basename")]
    pub webpage_url_basename: String,
    #[serde(rename = "webpage_url_domain")]
    pub webpage_url_domain: String,
    pub extractor: String,
    #[serde(rename = "extractor_key")]
    pub extractor_key: String,
    #[serde(rename = "playlist_count")]
    pub playlist_count: i32,
    pub playlist: String,
    #[serde(rename = "playlist_id")]
    pub playlist_id: String,
    #[serde(rename = "playlist_title")]
    pub playlist_title: String,
    #[serde(rename = "playlist_uploader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_uploader: Option<String>,
    #[serde(rename = "playlist_uploader_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_uploader_id: Option<String>,
    #[serde(rename = "playlist_channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_channel: Option<String>,
    #[serde(rename = "playlist_channel_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_channel_id: Option<String>,
    #[serde(rename = "playlist_webpage_url")]
    pub playlist_webpage_url: String,
    #[serde(rename = "n_entries")]
    pub n_entries: i32,
    #[serde(rename = "playlist_index")]
    pub playlist_index: i32,
    #[serde(rename = "__last_playlist_index")]
    pub last_playlist_index: i32,
    #[serde(rename = "playlist_autonumber")]
    pub playlist_autonumber: i32,
    pub thumbnails: Vec<Thumbnail>,
    #[serde(rename = "display_id")]
    pub display_id: String,
    pub fulltitle: String,
    #[serde(rename = "duration_string")]
    pub duration_string: String,
    #[serde(rename = "upload_date")]
    pub upload_date: String,
    #[serde(rename = "release_year")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(rename = "requested_subtitles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_subtitles: Option<HashMap<String, String>>,
    #[serde(rename = "_has_drm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_drm: Option<bool>,
    pub epoch: i64,
    pub url: String,
    pub ext: String,
    pub acodec: String,
    pub vcodec: String,
    pub tbr: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,
    #[serde(rename = "format_id")]
    pub format_id: String,
    pub protocol: String,
    #[serde(rename = "audio_ext")]
    pub audio_ext: String,
    #[serde(rename = "video_ext")]
    pub video_ext: String,
    pub vbr: f64,
    pub abr: f64,
    pub resolution: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
    #[serde(rename = "filesize_approx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesize_approx: Option<i64>,
    pub format: String,
    #[serde(rename = "_filename")]
    pub filename: String,
    #[serde(rename = "_type")]
    pub type_: String,
    #[serde(rename = "_version")]
    pub version: Version,
}

#[cfg(test)]
mod tests {
    use crate::yt_dlp::{Search, YtDlp};

    #[tokio::test]
    async fn test_bilibili_search() {
        let bilibili = YtDlp::new(Search::Bilibili);
        let videos = bilibili.search("原神", 2).await.expect("无法检索内容");
        let video = &videos[0];
        eprintln!("video:{} url:{}", video.title, video.url)
    }
}
