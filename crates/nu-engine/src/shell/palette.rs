use nu_ansi_term::{Color, Style};
use nu_protocol::hir::FlatShape;
use nu_source::{Span, Spanned};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::ops::Deref;
use std::str::Bytes;
use std::{fmt, io};

pub trait Palette {
    fn styles_for_shape(&self, shape: &Spanned<FlatShape>) -> Vec<Spanned<Style>>;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultPalette {}

impl Palette for DefaultPalette {
    fn styles_for_shape(&self, shape: &Spanned<FlatShape>) -> Vec<Spanned<Style>> {
        match &shape.item {
            FlatShape::BareMember => single_style_span(Color::Yellow.bold(), shape.span),
            FlatShape::CloseDelimiter(_) => single_style_span(Color::White.normal(), shape.span),
            FlatShape::Comment => single_style_span(Color::Green.bold(), shape.span),
            FlatShape::Decimal => single_style_span(Color::Purple.bold(), shape.span),
            FlatShape::Dot => single_style_span(Style::new().fg(Color::White), shape.span),
            FlatShape::DotDot => single_style_span(Color::Yellow.bold(), shape.span),
            FlatShape::DotDotLeftAngleBracket => {
                single_style_span(Color::Yellow.bold(), shape.span)
            }
            FlatShape::ExternalCommand => single_style_span(Color::Cyan.normal(), shape.span),
            FlatShape::ExternalWord => single_style_span(Color::Green.bold(), shape.span),
            FlatShape::Flag => single_style_span(Color::Blue.bold(), shape.span),
            FlatShape::Garbage => {
                single_style_span(Style::new().fg(Color::White).on(Color::Red), shape.span)
            }
            FlatShape::GlobPattern => single_style_span(Color::Cyan.bold(), shape.span),
            FlatShape::Identifier => single_style_span(Color::Purple.normal(), shape.span),
            FlatShape::Int => single_style_span(Color::Purple.bold(), shape.span),
            FlatShape::InternalCommand => single_style_span(Color::Cyan.bold(), shape.span),
            FlatShape::ItVariable => single_style_span(Color::Purple.bold(), shape.span),
            FlatShape::Keyword => single_style_span(Color::Purple.bold(), shape.span),
            FlatShape::OpenDelimiter(_) => single_style_span(Color::White.normal(), shape.span),
            FlatShape::Operator => single_style_span(Color::Yellow.normal(), shape.span),
            FlatShape::Path => single_style_span(Color::Cyan.normal(), shape.span),
            FlatShape::Pipe => single_style_span(Color::Purple.bold(), shape.span),
            FlatShape::Separator => single_style_span(Color::White.normal(), shape.span),
            FlatShape::ShorthandFlag => single_style_span(Color::Blue.bold(), shape.span),
            FlatShape::Size { number, unit } => vec![
                Spanned::<Style> {
                    span: *number,
                    item: Color::Purple.bold(),
                },
                Spanned::<Style> {
                    span: *unit,
                    item: Color::Cyan.bold(),
                },
            ],
            FlatShape::String => single_style_span(Color::Green.normal(), shape.span),
            FlatShape::StringMember => single_style_span(Color::Yellow.bold(), shape.span),
            FlatShape::Type => single_style_span(Color::Blue.bold(), shape.span),
            FlatShape::Variable => single_style_span(Color::Purple.normal(), shape.span),
            FlatShape::Whitespace => single_style_span(Color::White.normal(), shape.span),
            FlatShape::Word => single_style_span(Color::Green.normal(), shape.span),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ThemedPalette {
    theme: Theme,
}

impl ThemedPalette {
    pub fn new<R: io::Read>(reader: &mut R) -> Result<ThemedPalette, ThemeError> {
        let theme = serde_json::from_reader(reader)?;
        Ok(ThemedPalette { theme })
    }

    pub fn default() -> ThemedPalette {
        let theme = Theme::default();
        ThemedPalette { theme }
    }
}

impl Palette for ThemedPalette {
    fn styles_for_shape(&self, shape: &Spanned<FlatShape>) -> Vec<Spanned<Style>> {
        match &shape.item {
            FlatShape::OpenDelimiter(_) => {
                single_style_span(self.theme.open_delimiter.normal(), shape.span)
            }
            FlatShape::CloseDelimiter(_) => {
                single_style_span(self.theme.close_delimiter.normal(), shape.span)
            }
            FlatShape::ItVariable => single_style_span(self.theme.it_variable.bold(), shape.span),
            FlatShape::Keyword => single_style_span(self.theme.keyword.bold(), shape.span),
            FlatShape::Variable => single_style_span(self.theme.variable.normal(), shape.span),
            FlatShape::Identifier => single_style_span(self.theme.identifier.normal(), shape.span),
            FlatShape::Type => single_style_span(self.theme.r#type.bold(), shape.span),
            FlatShape::Operator => single_style_span(self.theme.operator.normal(), shape.span),
            FlatShape::DotDotLeftAngleBracket => {
                single_style_span(self.theme.dot_dot.bold(), shape.span)
            }
            FlatShape::DotDot => single_style_span(self.theme.dot_dot.bold(), shape.span),
            FlatShape::Dot => single_style_span(Style::new().fg(*self.theme.dot), shape.span),
            FlatShape::InternalCommand => {
                single_style_span(self.theme.internal_command.bold(), shape.span)
            }
            FlatShape::ExternalCommand => {
                single_style_span(self.theme.external_command.normal(), shape.span)
            }
            FlatShape::ExternalWord => {
                single_style_span(self.theme.external_word.bold(), shape.span)
            }
            FlatShape::BareMember => single_style_span(self.theme.bare_member.bold(), shape.span),
            FlatShape::StringMember => {
                single_style_span(self.theme.string_member.bold(), shape.span)
            }
            FlatShape::String => single_style_span(self.theme.string.normal(), shape.span),
            FlatShape::Path => single_style_span(self.theme.path.normal(), shape.span),
            FlatShape::GlobPattern => single_style_span(self.theme.glob_pattern.bold(), shape.span),
            FlatShape::Word => single_style_span(self.theme.word.normal(), shape.span),
            FlatShape::Pipe => single_style_span(self.theme.pipe.bold(), shape.span),
            FlatShape::Flag => single_style_span(self.theme.flag.bold(), shape.span),
            FlatShape::ShorthandFlag => {
                single_style_span(self.theme.shorthand_flag.bold(), shape.span)
            }
            FlatShape::Int => single_style_span(self.theme.int.bold(), shape.span),
            FlatShape::Decimal => single_style_span(self.theme.decimal.bold(), shape.span),
            FlatShape::Whitespace => single_style_span(self.theme.whitespace.normal(), shape.span),
            FlatShape::Separator => single_style_span(self.theme.separator.normal(), shape.span),
            FlatShape::Comment => single_style_span(self.theme.comment.bold(), shape.span),
            FlatShape::Garbage => single_style_span(
                Style::new().fg(*self.theme.garbage).on(Color::Red),
                shape.span,
            ),
            FlatShape::Size { number, unit } => vec![
                Spanned::<Style> {
                    span: *number,
                    item: self.theme.size_number.bold(),
                },
                Spanned::<Style> {
                    span: *unit,
                    item: self.theme.size_unit.bold(),
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct ThemeError {
    serde_err: serde_json::error::Error,
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failure to load theme")
    }
}

impl Error for ThemeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.serde_err)
    }
}

impl From<serde_json::error::Error> for ThemeError {
    fn from(serde_err: serde_json::error::Error) -> Self {
        ThemeError { serde_err }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Theme {
    bare_member: ThemeColor,
    close_delimiter: ThemeColor,
    comment: ThemeColor,
    decimal: ThemeColor,
    dot: ThemeColor,
    dot_dot: ThemeColor,
    dot_dot_left_angle_bracket: ThemeColor,
    external_command: ThemeColor,
    external_word: ThemeColor,
    flag: ThemeColor,
    garbage: ThemeColor,
    glob_pattern: ThemeColor,
    identifier: ThemeColor,
    int: ThemeColor,
    internal_command: ThemeColor,
    it_variable: ThemeColor,
    keyword: ThemeColor,
    open_delimiter: ThemeColor,
    operator: ThemeColor,
    path: ThemeColor,
    pipe: ThemeColor,
    separator: ThemeColor,
    shorthand_flag: ThemeColor,
    size_number: ThemeColor,
    size_unit: ThemeColor,
    string: ThemeColor,
    string_member: ThemeColor,
    r#type: ThemeColor,
    variable: ThemeColor,
    whitespace: ThemeColor,
    word: ThemeColor,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            bare_member: ThemeColor(Color::Yellow),
            close_delimiter: ThemeColor(Color::White),
            comment: ThemeColor(Color::Green),
            decimal: ThemeColor(Color::Purple),
            dot: ThemeColor(Color::White),
            dot_dot: ThemeColor(Color::Yellow),
            dot_dot_left_angle_bracket: ThemeColor(Color::Yellow),
            external_command: ThemeColor(Color::Cyan),
            external_word: ThemeColor(Color::Green),
            flag: ThemeColor(Color::Blue),
            garbage: ThemeColor(Color::White),
            glob_pattern: ThemeColor(Color::Cyan),
            identifier: ThemeColor(Color::Purple),
            int: ThemeColor(Color::Purple),
            internal_command: ThemeColor(Color::Cyan),
            it_variable: ThemeColor(Color::Purple),
            keyword: ThemeColor(Color::Purple),
            open_delimiter: ThemeColor(Color::White),
            operator: ThemeColor(Color::Yellow),
            path: ThemeColor(Color::Cyan),
            pipe: ThemeColor(Color::Purple),
            separator: ThemeColor(Color::Red),
            shorthand_flag: ThemeColor(Color::Blue),
            size_number: ThemeColor(Color::Purple),
            size_unit: ThemeColor(Color::Cyan),
            string: ThemeColor(Color::Green),
            string_member: ThemeColor(Color::Yellow),
            r#type: ThemeColor(Color::Blue),
            variable: ThemeColor(Color::Purple),
            whitespace: ThemeColor(Color::White),
            word: ThemeColor(Color::Green),
            // These should really be Styles and not colors
            // leave this here for the next change to make
            // ThemeColor, ThemeStyle.
            // open_delimiter: ThemeColor(Color::White.normal()),
            // close_delimiter: ThemeColor(Color::White.normal()),
            // it_variable: ThemeColor(Color::Purple.bold()),
            // variable: ThemeColor(Color::Purple.normal()),
            // r#type: ThemeColor(Color::Blue.bold()),
            // identifier: ThemeColor(Color::Purple.normal()),
            // operator: ThemeColor(Color::Yellow.normal()),
            // dot: ThemeColor(Color::White),
            // dot_dot: ThemeColor(Color::Yellow.bold()),
            // //missing DotDotLeftAngleBracket
            // internal_command: ThemeColor(Color::Cyan.bold()),
            // external_command: ThemeColor(Color::Cyan.normal()),
            // external_word: ThemeColor(Color::Green.bold()),
            // bare_member: ThemeColor(Color::Yellow.bold()),
            // string: ThemeColor(Color::Green.normal()),
            // string_member: ThemeColor(Color::Yellow.bold()),
            // path: ThemeColor(Color::Cyan.normal()),
            // glob_pattern: ThemeColor(Color::Cyan.bold()),
            // word: ThemeColor(Color::Green.normal()),
            // keyword: ThemeColor(Color::Purple.bold()),
            // pipe: ThemeColor(Color::Purple.bold()),
            // flag: ThemeColor(Color::Blue.bold()),
            // shorthand_flag: ThemeColor(Color::Blue.bold()),
            // int: ThemeColor(Color::Purple.bold()),
            // decimal: ThemeColor(Color::Purple.bold()),
            // garbage: ThemeColor(Style::new().fg(Color::White).on(Color::Red)),
            // whitespace: ThemeColor(Color::White.normal()),
            // separator: ThemeColor(Color::Red),
            // comment: ThemeColor(Color::Green.bold()),
            // size_number: ThemeColor(Color::Purple.bold()),
            // size_unit: ThemeColor(Color::Cyan.bold()),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct ThemeColor(Color);

impl Deref for ThemeColor {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for ThemeColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("TODO: IMPLEMENT SERIALIZATION")
    }
}

impl<'de> Deserialize<'de> for ThemeColor {
    fn deserialize<D>(deserializer: D) -> Result<ThemeColor, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ThemeColor::from_str(&s)
    }
}

impl ThemeColor {
    fn from_str<E>(s: &str) -> Result<ThemeColor, E>
    where
        E: serde::de::Error,
    {
        let mut bytes = s.bytes();
        let r = ThemeColor::xtoi(&mut bytes)?;
        let g = ThemeColor::xtoi(&mut bytes)?;
        let b = ThemeColor::xtoi(&mut bytes)?;
        Ok(ThemeColor(Color::Rgb(r, g, b)))
    }

    fn xtoi<E>(b: &mut Bytes) -> Result<u8, E>
    where
        E: serde::de::Error,
    {
        let upper = b
            .next()
            .ok_or_else(|| E::custom("color string too short"))?;
        let lower = b
            .next()
            .ok_or_else(|| E::custom("color string too short"))?;
        let mut val = ThemeColor::numerical_value(upper)?;
        val = (val << 4) | ThemeColor::numerical_value(lower)?;
        Ok(val)
    }

    fn numerical_value<E>(character: u8) -> Result<u8, E>
    where
        E: serde::de::Error,
    {
        match character {
            b'0'..=b'9' => Ok(character - b'0'),
            b'a'..=b'z' => Ok(character - (b'a' - 10)),
            _ => Err(E::custom(format!("invalid character {}", character))),
        }
    }
}

fn single_style_span(style: Style, span: Span) -> Vec<Spanned<Style>> {
    vec![Spanned::<Style> { span, item: style }]
}

#[cfg(test)]
mod tests {
    use super::{Palette, ThemedPalette};
    use nu_ansi_term::Color;
    use nu_protocol::hir::FlatShape;
    use nu_source::{Span, Spanned};
    use std::io::Cursor;

    #[test]
    fn create_themed_palette() {
        let json = r#"
{
    "bare_member": "a359cc",
    "close_delimiter": "a359cc",
    "comment": "a359cc",
    "decimal": "a359cc",
    "dot": "a359cc",
    "dot_dot": "a359cc",
    "dot_dot_left_angle_bracket": "a359cc",
    "external_command": "a359cc",
    "external_word": "a359cc",
    "flag": "a359cc",
    "garbage": "a359cc",
    "glob_pattern": "a359cc",
    "identifier": "a359cc",
    "int": "a359cc",
    "internal_command": "a359cc",
    "it_variable": "a359cc",
    "keyword": "a359cc",
    "open_delimiter": "a359cc",
    "operator": "a359cc",
    "path": "a359cc",
    "pipe": "a359cc",
    "separator": "a359cc",
    "shorthand_flag": "a359cc",
    "size_number": "a359cc",
    "size_unit": "a359cc",
    "string": "a359cc",
    "string_member": "a359cc",
    "type": "a359cc",
    "variable": "a359cc",
    "whitespace": "a359cc",
    "word": "a359cc"
}"#;
        let mut json_reader = Cursor::new(json);
        let themed_palette = ThemedPalette::new(&mut json_reader).unwrap();
        let test_shape = Spanned {
            item: FlatShape::Type,
            span: Span::new(4, 9),
        };
        let styled = themed_palette.styles_for_shape(&test_shape);
        assert_eq!(styled.len(), 1);
        assert_eq!(
            styled[0],
            Spanned {
                item: Color::Rgb(163, 89, 204).bold(),
                span: Span::new(4, 9),
            },
        );
    }
}
