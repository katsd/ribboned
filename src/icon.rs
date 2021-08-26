static RIBBONED_TEXT: &str = "RIBBONED_TEXT";
static RIBBONED_COLOR_LIGHT: &str = "RIBBONED_COLOR_LIGHT";
static RIBBONED_COLOR_DARK: &str = "RIBBONED_COLOR_DARK";

pub enum OS {
    Ios,
    Mac,
    Watch,
}

impl OS {
    fn get_svg_str(&self) -> String {
        match self {
            OS::Ios => include_str!("../assets/ios.svg").to_string(),
            OS::Mac => include_str!("../assets/mac.svg").to_string(),
            OS::Watch => include_str!("../assets/watch.svg").to_string(),
        }
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    fn to_svg_str(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
}

struct Gradient {
    light: Color,
    dark: Color,
}

impl Gradient {
    fn blue() -> Gradient { Gradient { light: Color::rgb(37, 141, 255), dark: Color::rgb(0, 122, 255) } }

    fn cyan() -> Gradient { Gradient { light: Color::rgb(91, 192, 235), dark: Color::rgb(50, 173, 230) } }

    fn green() -> Gradient { Gradient { light: Color::rgb(94, 213, 124), dark: Color::rgb(52, 199, 89) } }

    fn indigo() -> Gradient { Gradient { light: Color::rgb(107, 105, 219), dark: Color::rgb(88, 86, 214) } }

    fn orange() -> Gradient { Gradient { light: Color::rgb(255, 170, 49), dark: Color::rgb(255, 149, 0) } }

    fn pink() -> Gradient { Gradient { light: Color::rgb(255, 79, 113), dark: Color::rgb(255, 45, 85) } }
}

pub struct IconConfig {
    text: String,
    gradient: Gradient,
}

impl IconConfig {
    pub fn new(text: String) -> IconConfig {
        let sum: i32 = text.clone().into_bytes().iter().map(|v| *v as i32).sum();

        let gradient = match sum % 6 {
            0 => Gradient::blue(),
            1 => Gradient::cyan(),
            2 => Gradient::green(),
            3 => Gradient::indigo(),
            4 => Gradient::orange(),
            5 => Gradient::pink(),
            _ => Gradient::blue()
        };

        IconConfig { text: text.chars().take(3).collect::<String>(), gradient }
    }
}

pub fn get_icon_svg(os: OS, config: &IconConfig) -> Vec<u8> {
    os.get_svg_str()
        .replace(RIBBONED_TEXT, &config.text)
        .replace(RIBBONED_COLOR_LIGHT, &config.gradient.light.to_svg_str())
        .replace(RIBBONED_COLOR_DARK, &config.gradient.dark.to_svg_str())
        .into_bytes()
}
