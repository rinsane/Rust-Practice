use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

fn main() {
    let hex_colors = vec![
        "#83a598", // Accent
        "#1a1a1a", // Background
        "#d5c4a1", // Foreground
        "#665c54", // Bright Black
        "#bdae93", // Bright Blue
        "#d65d0e", // Bright Cyan
        "#3c3836", // Bright Green
        "#ebdbb2", // Bright Magenta
        "#fe8019", // Bright Red
        "#fbf1c7", // Bright White
        "#504945", // Bright Yellow
        "#1d2021", // Normal Black
        "#83a598", // Normal Blue
        "#8ec07c", // Normal Cyan
        "#b8bb26", // Normal Green
        "#d3869b", // Normal Magenta
        "#fb4934", // Normal Red
        "#d5c4a1", // Normal White
        "#fabd2f", // Normal Yellow
    ];

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for hex in hex_colors {
        let (r, g, b) = hex_to_rgb(&hex[1..]);
        let color = Color::Rgb(r, g, b);
        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(Some(color)).set_bold(true);

        stdout.set_color(&color_spec).unwrap();
        writeln!(&mut stdout, "{}", hex).unwrap();
        stdout.reset().unwrap();
    }
}