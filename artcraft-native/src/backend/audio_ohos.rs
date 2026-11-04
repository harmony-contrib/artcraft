//! Desktop audio devices are unavailable on OHOS. Open playback through the host's URL
//! handler; report waveform unavailability instead of leaving the UI waiting for a decoder.

use std::sync::Arc;

pub struct AudioPlayer {
  volume: f32,
}

/// A track's shape: peaks (0–1, loudest = 1) and its length in seconds.
#[derive(Debug, PartialEq)]
pub struct Waveform {
  pub peaks: Vec<f32>,
  pub duration: f32,
}

/// What [`AudioPlayer::waveform`] knows about a track.
#[derive(Clone)]
pub enum WaveformLookup {
  Loading,
  Ready(Arc<Waveform>),
  Failed,
}

/// Where the current track is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Playback {
  /// Still downloading.
  pub loading: bool,
  pub paused: bool,
  /// Seconds in.
  pub position: f32,
}

impl AudioPlayer {
  pub fn new(_rt: tokio::runtime::Handle, _http: reqwest::Client) -> Self {
    Self { volume: 1.0 }
  }

  pub fn toggle(&mut self, ctx: &egui::Context, _key: &str, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
  }

  pub fn toggle_track(&mut self, ctx: &egui::Context, url: &str) {
    ctx.open_url(egui::OpenUrl::new_tab(url));
  }

  pub fn seek_track(&mut self, ctx: &egui::Context, url: &str, _secs: f32) {
    self.toggle_track(ctx, url);
  }

  pub fn stop(&mut self) {}

  pub fn is_playing(&self, _key: &str) -> bool {
    false
  }

  pub fn track(&self, _url: &str) -> Option<Playback> {
    None
  }

  pub fn track_volume(&self) -> f32 {
    self.volume
  }

  pub fn set_track_volume(&mut self, volume: f32) {
    self.volume = volume.clamp(0.0, 1.0);
  }

  pub fn waveform(&mut self, _ctx: &egui::Context, _url: &str) -> WaveformLookup {
    WaveformLookup::Failed
  }

  pub fn poll(&mut self) -> Option<String> {
    None
  }
}

pub fn resample(values: &[f32], count: usize) -> Vec<f32> {
  if values.is_empty() || count == 0 {
    return vec![0.0; count];
  }
  (0..count)
    .map(|i| {
      let start = i * values.len() / count;
      let end = ((i + 1) * values.len() / count).max(start + 1).min(values.len());
      values[start..end].iter().copied().fold(0.0, f32::max)
    })
    .collect()
}
