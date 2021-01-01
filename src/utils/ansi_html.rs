use yew::{html, Html, services::ConsoleService};
use regex::Regex;
use lazy_static::{lazy_static, initialize};

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"\u{1b}\[(?P<fg>\d{1,2})(;\d{1,2})?m").unwrap();
}

pub fn start_regex() {
    initialize(&COLOR_REGEX);
}

#[derive(Debug, Clone, Copy)]
enum Color {
    None,
    Red,
    Green,
    Yellow,
}

impl<'a> From<&'a str> for Color {
    fn from(text: &str) -> Self {
        match text {
            "31" => Color::Red,
            "32" => Color::Green,
            "33" => Color::Yellow,
            _ => Color::None,
        }
    }
}

pub fn convert(ansi_str: &str) -> Html {
    let mut chunks: Vec<(Color, &str)> = Vec::new();
    let mut last_text = 0;
    let mut last_color = Color::None;

    COLOR_REGEX.captures_iter(ansi_str)
        .zip(COLOR_REGEX.find_iter(ansi_str))
        .for_each(|(capture, m)| {
            if m.start() > last_text {
                chunks.push((last_color, &ansi_str[last_text .. m.start()]));
            }
            let color = Color::from(capture.name("fg").unwrap().as_str());
            
            last_text = m.end();
            last_color = color;
        });
    // append remainder of string
    if last_text < ansi_str.len() {
        chunks.push((last_color, &ansi_str[last_text..]));
    }

    // get start and end ranges for colors

    chunks.iter()
        .map(|(color, text)| {
            let text_html = text.chars()
                .map(|c| {
                    match c {
                        '\n' => html! {<br />},
                        '\t' => html! {"\u{a0}\u{a0}\u{a0}\u{a0}"},
                        ' ' => html! {"\u{a0}"},
                        _ => html! {c},
                    }
                });
            match color {
                Color::None => html! {for text_html},
                Color::Red => html! {<span class="highlight-red">{for text_html}</span>},
                Color::Green => html! {<span class="highlight-green">{for text_html}</span>},
                Color::Yellow => html! {<span class="highlight-yellow">{for text_html}</span>},
            }
        })
        .collect()
}

#[test]
fn test_regex() {
    let test_string = "error: Found argument \'--hel\' which wasn\'t expected, or isn\'t valid in this context\n\tDid you mean \u{1b}[32m--\u{1b}[0m\u{1b}[32mhelp\u{1b}[0m?\n\nUSAGE:\n    portfolio --help\n\nFor more information try --help";
    assert!(COLOR_REGEX.is_match(test_string));
    COLOR_REGEX.captures_iter(test_string)
        .for_each(|c| {
            let color = c.name("fg").unwrap();
            println!("{}", &test_string[color.start()..color.end()]);
        });
}
