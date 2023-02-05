use rust_embed::RustEmbed;

#[derive(RustEmbed, Debug)]
#[folder = "stubs/"] // Entire stubs folder
#[include = "*.html"] // Themes
#[include = "*.ttf"] // Fonts
#[include = "*.txt"] // License files
#[include = "*.md"] // Sample content
#[include = "*.yaml"] // Default configuration file
#[include = "*.jpg"]
#[include = "*.png"]
#[include = "*.css"]
pub struct Asset;
