//! Embeddable ArtCraft UI. Hosts can supply their own identity and writable data directory
//! with [`ArtcraftApp::with_branding`] and run it as an [`eframe::App`].

mod app;
mod backend;
mod branding;
mod feed;
mod models;
mod overlays;
mod pages;
mod prompt_box;
mod shell;
mod theme;
mod ui;

pub use app::ArtcraftApp;
pub use branding::Branding;
