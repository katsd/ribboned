use std::fs::File;
use std::io::Read;

static RIBBONED_TEXT: &str = "RIBBONED_TEXT";
static RIBBONED_COLOR_LIGHT: &str = "RIBBONED_COLOR_LIGHT";
static RIBBONED_COLOR_DARK: &str = "RIBBONED_COLOR_DARK";

pub enum OS {
    ios,
    mac,
    watch,
}

impl OS {
    fn to_file_path(&self) -> String {
        format!("assets/{}.svg",
                match self {
                    ios => "ios",
                    mac => "mac",
                    watch => "watch"
                }
        )
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn to_svg_str(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
}

pub struct IconConfig {
    pub text: String,
    pub color_light: Color,
    pub color_dark: Color,
}

pub fn ios_icon_svg_str(os: &OS, config: &IconConfig) -> String {
    let mut f = File::open(os.to_file_path()).expect("file not found");
    let mut svg_str = String::new();
    f.read_to_string(&mut svg_str).expect("failed to read the file");

    svg_str
        .replace(RIBBONED_TEXT, &config.text)
        .replace(RIBBONED_COLOR_LIGHT, &config.color_light.to_svg_str())
        .replace(RIBBONED_COLOR_DARK, &config.color_dark.to_svg_str())
}
