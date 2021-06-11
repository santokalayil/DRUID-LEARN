

use druid::widget::{SvgData, Svg}; // Svg

// #[derive(RustEmbed)]
// #[folder = "src/assets/"]
// pub struct Assets;

pub fn logo() -> Svg {
    // let logo_url = Assets::get("logo.svg").unwrap();
    Svg::new(
        match include_str!("..\\assets\\logo.svg").parse::<SvgData>() {  // "..\\assets\\logo.svg"
            Ok(svg) => svg,
            Err(err) => {
                println!("{}", err);
                println!("Using an empty SVG instead.");
                SvgData::default()
            }
        }
    )

}
