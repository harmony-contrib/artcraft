//! Product identity supplied by the embedding host.

/// Application branding. API/provider names and URLs still identify the upstream service.
#[derive(Clone, Copy, Debug)]
pub struct Branding {
  pub name: &'static str,
  pub icon_png: &'static [u8],
  pub attribution: &'static str,
}

impl Branding {
  /// The standalone desktop application's identity.
  #[cfg(feature = "upstream-branding")]
  pub const UPSTREAM: Self = Self { name: "ArtCraft", icon_png: include_bytes!("../../crates/desktop/artcraft/icons/icon.png"), attribution: "ArtCraft — https://github.com/storytold/artcraft" };
}
