pub mod error;

use error::{ColorSequenceParsingError, StyleSequenceParsingError};

use log::warn;
use yansi::{Color, Style};

/// Parses SGR-sequence of ASCII escape characters into a terminal text style.
///
/// # Errors
///
/// If the string is not a valid SGR-sequence, raises a [`StyleSequenceParsingError`].
///
pub fn style_from(sgr_sequence: &str) -> Result<Style, StyleSequenceParsingError> {
    let mut style = Style::new();
    let mut itr = sgr_sequence.split(';');
    while let Some(token) = itr.next() {
        if token.is_empty() {
            continue;
        }

        let code = token
            .parse::<u8>()
            .map_err(|err| StyleSequenceParsingError::NotACode(token.to_owned(), err))?;
        match code {
            0 => {}
            1 => style = style.bold(),
            2 => style = style.dim(),
            3 => style = style.italic(),
            4 => style = style.underline(),
            5 | 6 => {
                warn!("Slow and rapid blinks are treated the same way");
                style = style.blink();
            }
            7 => style = style.invert(),
            8 => style = style.conceal(),
            9 => style = style.strike(),
            30..=39 => {
                style = style.fg(color_from(code, &mut itr)?);
            }
            40..=49 => {
                style = style.bg(color_from(code, &mut itr)?);
            }
            10..=29 | 50..=107 => return Err(StyleSequenceParsingError::UnsupportedCode(code)),
            _ => return Err(StyleSequenceParsingError::BadCode(code)),
        }
    }

    Ok(style)
}

fn color_from<'src>(
    code: u8,
    itr: &mut impl Iterator<Item = &'src str>,
) -> Result<Color, ColorSequenceParsingError> {
    let code_suffix = code % 10;
    match code_suffix {
        0 => Ok(Color::Black),
        1 => Ok(Color::Red),
        2 => Ok(Color::Green),
        3 => Ok(Color::Yellow),
        4 => Ok(Color::Blue),
        5 => Ok(Color::Magenta),
        6 => Ok(Color::Cyan),
        7 => Ok(Color::White),
        8 => {
            if let Some(differentiator) = itr.next() {
                let differentiator = differentiator.parse::<u8>().map_err(|err| {
                    ColorSequenceParsingError::NotACode(differentiator.to_owned(), err)
                })?;
                match differentiator {
                    2 => match (itr.next(), itr.next(), itr.next()) {
                        (Some(red), Some(green), Some(blue)) => {
                            let red = red.parse::<u8>().map_err(|err| {
                                ColorSequenceParsingError::NotACode(red.to_owned(), err)
                            })?;
                            let green = green.parse::<u8>().map_err(|err| {
                                ColorSequenceParsingError::NotACode(green.to_owned(), err)
                            })?;
                            let blue = blue.parse::<u8>().map_err(|err| {
                                ColorSequenceParsingError::NotACode(blue.to_owned(), err)
                            })?;
                            Ok(Color::Rgb(red, green, blue))
                        }
                        _ => Err(ColorSequenceParsingError::BadTrueColor),
                    },
                    5 => {
                        if let Some(val) = itr.next() {
                            let val = val.parse::<u8>().map_err(|err| {
                                ColorSequenceParsingError::NotACode(val.to_owned(), err)
                            })?;
                            Ok(Color::Fixed(val))
                        } else {
                            Err(ColorSequenceParsingError::BadFixedColor)
                        }
                    }
                    _ => Err(ColorSequenceParsingError::BadColorType(differentiator)),
                }
            } else {
                Err(ColorSequenceParsingError::IncompleteSequence)
            }
        }
        9 => Ok(Color::Primary),
        #[expect(clippy::unreachable, reason = "It is mathematically impossible")]
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_reset() {
        let sequence = "0";
        assert_eq!(style_from(sequence).unwrap(), Style::new());
    }

    #[test]
    fn style_bold() {
        let sequence = "1";
        assert_eq!(style_from(sequence).unwrap(), Style::new().bold());
    }

    #[test]
    fn style_dim() {
        let sequence = "2";
        assert_eq!(style_from(sequence).unwrap(), Style::new().dim());
    }

    #[test]
    fn style_italic() {
        let sequence = "3";
        assert_eq!(style_from(sequence).unwrap(), Style::new().italic());
    }

    #[test]
    fn style_underline() {
        let sequence = "4";
        assert_eq!(style_from(sequence).unwrap(), Style::new().underline());
    }

    #[test]
    fn style_slow_blink() {
        let sequence = "5";
        assert_eq!(style_from(sequence).unwrap(), Style::new().blink());
    }

    #[test]
    fn style_rapid_blink() {
        let sequence = "6";
        assert_eq!(style_from(sequence).unwrap(), Style::new().blink());
    }

    #[test]
    fn style_invert() {
        let sequence = "7";
        assert_eq!(style_from(sequence).unwrap(), Style::new().invert());
    }

    #[test]
    fn style_hide() {
        let sequence = "8";
        assert_eq!(style_from(sequence).unwrap(), Style::new().conceal());
    }

    #[test]
    fn style_strike() {
        let sequence = "9";
        assert_eq!(style_from(sequence).unwrap(), Style::new().strike());
    }

    #[test]
    fn style_fg_color_black() {
        let sequence = "30";
        assert_eq!(style_from(sequence).unwrap(), Style::new().black());
    }

    #[test]
    fn style_fg_color_red() {
        let sequence = "31";
        assert_eq!(style_from(sequence).unwrap(), Style::new().red());
    }

    #[test]
    fn style_fg_color_green() {
        let sequence = "32";
        assert_eq!(style_from(sequence).unwrap(), Style::new().green());
    }

    #[test]
    fn style_fg_color_yellow() {
        let sequence = "33";
        assert_eq!(style_from(sequence).unwrap(), Style::new().yellow());
    }
    #[test]
    fn style_fg_color_blue() {
        let sequence = "34";
        assert_eq!(style_from(sequence).unwrap(), Style::new().blue());
    }

    #[test]
    fn style_fg_color_magenta() {
        let sequence = "35";
        assert_eq!(style_from(sequence).unwrap(), Style::new().magenta());
    }

    #[test]
    fn style_fg_color_cyan() {
        let sequence = "36";
        assert_eq!(style_from(sequence).unwrap(), Style::new().cyan());
    }

    #[test]
    fn style_fg_color_white() {
        let sequence = "37";
        assert_eq!(style_from(sequence).unwrap(), Style::new().white());
    }

    #[test]
    fn style_fg_color_8bit() {
        let sequence = "38;5;120";
        assert_eq!(style_from(sequence).unwrap(), Style::new().fixed(120));
    }

    #[test]
    fn style_fg_color_24bit() {
        let sequence = "38;2;192;255;238";
        assert_eq!(
            style_from(sequence).unwrap(),
            Style::new().rgb(192, 255, 238)
        );
    }

    #[test]
    fn style_fg_color_default() {
        let sequence = "39";
        assert_eq!(style_from(sequence).unwrap(), Style::new().primary());
    }

    #[test]
    fn style_bg_color_black() {
        let sequence = "40";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_black());
    }

    #[test]
    fn style_bg_color_red() {
        let sequence = "41";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_red());
    }

    #[test]
    fn style_bg_color_green() {
        let sequence = "42";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_green());
    }

    #[test]
    fn style_bg_color_yellow() {
        let sequence = "43";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_yellow());
    }
    #[test]
    fn style_bg_color_blue() {
        let sequence = "44";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_blue());
    }

    #[test]
    fn style_bg_color_magenta() {
        let sequence = "45";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_magenta());
    }

    #[test]
    fn style_bg_color_cyan() {
        let sequence = "46";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_cyan());
    }

    #[test]
    fn style_bg_color_white() {
        let sequence = "47";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_white());
    }

    #[test]
    fn style_bg_color_8bit() {
        let sequence = "48;5;120";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_fixed(120));
    }

    #[test]
    fn style_bg_color_24bit() {
        let sequence = "48;2;192;255;238";
        assert_eq!(
            style_from(sequence).unwrap(),
            Style::new().on_rgb(192, 255, 238)
        );
    }

    #[test]
    fn style_bg_color_default() {
        let sequence = "49";
        assert_eq!(style_from(sequence).unwrap(), Style::new().on_primary());
    }

    #[test]
    fn style_multiple_styles() {
        let sequence = "33;3;4;48;2;192;255;238";
        assert_eq!(
            style_from(sequence).unwrap(),
            Style::new()
                .yellow()
                .on_rgb(192, 255, 238)
                .italic()
                .underline()
        );
    }
}
