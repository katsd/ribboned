mod icon;

use clap::{Arg, App};
use icon::*;
use svg2appicon;

fn main() {
    match svg2appicon::generate_icons(&get_config()) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e)
    };
}

fn get_config() -> svg2appicon::Config {
    let appiconset_key = "APPICONSET";
    let text_key = "TEXT";
    let ios_key = "IOS";
    let mac_key = "MAC";
    let watch_key = "WATCH";


    let matches = App::new("ribboned")
        .version("0.1.0")
        .author("Katsu Matsuda")
        .about("Make default app icons identifiable")
        .arg(Arg::with_name(appiconset_key)
            .help("Path to .appiconset (e.g. /path/to/Assets.xcassets/AppIcon.appiconset)")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name(text_key)
            .help("Icon text (Up to three characters will be displayed)")
            .required(true)
            .index(2)
        )
        .arg(Arg::with_name(ios_key)
            .help("Generate icons for iOS")
            .long("ios")
        )
        .arg(Arg::with_name(mac_key)
            .help("Generate icons for macOS")
            .long("mac")
        )
        .arg(Arg::with_name(watch_key)
            .help("Generate icons for watchOS")
            .long("watch")
        )
        .get_matches();


    let appiconset_path = matches.value_of(appiconset_key).unwrap().to_string();
    let text = matches.value_of(text_key).unwrap().to_string();

    let icon_config = IconConfig::new(text);

    let svg_ios =
        if matches.is_present(ios_key) || (!matches.is_present(mac_key) && !matches.is_present(watch_key)) {
            Some(svg2appicon::SVG::Data(get_icon_svg(OS::Ios, &icon_config)))
        } else { None };
    let svg_mac =
        if matches.is_present(mac_key) {
            Some(svg2appicon::SVG::Data(get_icon_svg(OS::Mac, &icon_config)))
        } else { None };
    let svg_watch =
        if matches.is_present(watch_key) {
            Some(svg2appicon::SVG::Data(get_icon_svg(OS::Watch, &icon_config)))
        } else { None };


    svg2appicon::Config { assets_path: appiconset_path, svg_ios, svg_mac, svg_watch }
}
