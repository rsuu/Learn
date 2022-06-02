use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::ops::Deref;

fn main() {
    let markdown_input: &str = r#"aaaa aaaaa
bbb [1|https.com] aaa [1|https.com]
ppp
awdawd
"#;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options).map(|event| match event {
        // newline
        Event::Text(text) => {
            let mut chars = text.chars().peekable();
            let mut buf = String::with_capacity(text.len());
            let mut start = false;

            'l1: loop {
                if let Some(c) = chars.peek() {
                    if *c == '[' {
                        buf.push_str("<div calss=\"reflink\">");
                    } else if *c == ']' {
                        buf.push_str("</div>");
                        break 'l1;
                    } else if *c == '\r' {
                        buf.push_str("</p><p>");
                    } else {
                        println!("{}", c);

                        buf.push(*c);
                    }
                    chars.next();
                } else {
                    break 'l1;
                }
            }

            Event::Html(buf.into()).into()
        }

        _ => event,
    });

    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    println!("{}", &html_output);
}
