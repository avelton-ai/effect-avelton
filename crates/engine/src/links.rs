//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Discord button).

pub const APP_NAME: &str = "Effect Avelton";
pub const APP_SLUG: &str = "effectcraft";
pub const DISCORD: &str = "https://github.com/avelton-ai/effect-avelton/issues";
pub const WEBSITE: &str = "https://avelton.online";
pub const APP_PAGE: &str = "https://effect.avelton.online";
pub const GITHUB: &str = "https://github.com/avelton-ai/effect-avelton";
pub const ISSUES: &str = "https://github.com/avelton-ai/effect-avelton/issues";

/// Sibling apps of the Avelton family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoCraft", "photocraft"),
    ("VectorCraft", "vectorcraft"),
    ("FilmCraft", "filmcraft"),
    ("LightCraft", "lightcraft"),
    ("PdfCraft", "pdfcraft"),
    ("DesignCraft", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://avelton.online/apps/{slug}")
}
