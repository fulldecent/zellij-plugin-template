use std::collections::BTreeMap;
use std::io::{self, Write};

use zellij_tile::prelude::*;

struct State {
    foreground: PaletteColor,
    background: PaletteColor,
}

impl Default for State {
    fn default() -> Self {
        let text = Styling::default().text_unselected;
        Self {
            foreground: text.base,
            background: text.background,
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState]);
        subscribe(&[EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => {
                let text = mode_info.style.colors.text_unselected;
                self.foreground = text.base;
                self.background = text.background;
                true
            }
            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        if rows == 0 || cols == 0 {
            return;
        }

        let line = colored_slash_line(self.foreground, self.background, cols);
        // A full-width line wraps onto the next row unless autowrap is off.
        print!("\u{1b}[?7l");
        for row in 1..=rows {
            print!("\u{1b}[{row};1H{line}");
        }
        print!("\u{1b}[0m\u{1b}[?7h");
        // Plugin stdout is block-buffered. Zellij reads it when render returns.
        let _ = io::stdout().flush();
    }
}

fn colored_slash_line(foreground: PaletteColor, background: PaletteColor, cols: usize) -> String {
    format!(
        "{}{}{}",
        sgr(foreground, true),
        sgr(background, false),
        "/".repeat(cols)
    )
}

fn sgr(color: PaletteColor, foreground: bool) -> String {
    let channel = if foreground { 38 } else { 48 };
    match color {
        PaletteColor::Rgb((red, green, blue)) => {
            format!("\u{1b}[{channel};2;{red};{green};{blue}m")
        }
        PaletteColor::EightBit(index) => format!("\u{1b}[{channel};5;{index}m"),
    }
}

#[cfg(test)]
mod tests {
    use super::{colored_slash_line, sgr};
    use zellij_tile::prelude::PaletteColor;

    #[test]
    fn slash_line_fills_the_width() {
        let line = colored_slash_line(PaletteColor::EightBit(15), PaletteColor::EightBit(0), 24);
        assert!(line.ends_with(&"/".repeat(24)));
        assert_eq!(
            line.chars().filter(|character| *character == '/').count(),
            24
        );
    }

    #[test]
    fn colors_use_theme_foreground_and_background() {
        let line = colored_slash_line(
            PaletteColor::Rgb((1, 2, 3)),
            PaletteColor::Rgb((4, 5, 6)),
            2,
        );
        assert!(line.contains("\u{1b}[38;2;1;2;3m"));
        assert!(line.contains("\u{1b}[48;2;4;5;6m"));
        assert!(line.ends_with("//"));
    }

    #[test]
    fn eight_bit_colors_use_indexed_sgr() {
        assert_eq!(sgr(PaletteColor::EightBit(15), true), "\u{1b}[38;5;15m");
        assert_eq!(sgr(PaletteColor::EightBit(0), false), "\u{1b}[48;5;0m");
    }
}
