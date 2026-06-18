/*
 * This formatter will handle various format conversion
 * One of the import format conversion needs to be handled is markdown
 * to html version.
 */
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use teloxide::utils::html;

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => output.push_str("\n\n"),

            Event::Start(Tag::Heading { .. }) => output.push_str("<b>"),
            Event::End(TagEnd::Heading(_)) => output.push_str("</b>\n\n"),

            Event::Start(Tag::BlockQuote(_)) => output.push_str("<blockquote>"),
            Event::End(TagEnd::BlockQuote(_)) => output.push_str("</blockquote>"),

            Event::Start(Tag::CodeBlock(kind)) => match kind {
                CodeBlockKind::Fenced(lang) if !lang.is_empty() => {
                    output.push_str(&format!(
                        "<pre><code class=\"language-{}\">",
                        html::escape(&lang)
                    ));
                }
                _ => output.push_str("<pre><code>"),
            },
            Event::End(TagEnd::CodeBlock) => {
                output.push_str("</code></pre>\n");
            }
            Event::Start(Tag::List(None)) => {}
            Event::End(TagEnd::List(_)) => output.push('\n'),
            Event::Start(Tag::Item) => output.push_str("• "),
            Event::End(TagEnd::Item) => output.push('\n'),

            Event::Start(Tag::Strong) => output.push_str("<b>"),
            Event::End(TagEnd::Strong) => output.push_str("</b>"),

            Event::Start(Tag::Emphasis) => output.push_str("<i>"),
            Event::End(TagEnd::Emphasis) => output.push_str("</i>"),

            Event::Start(Tag::Strikethrough) => output.push_str("<s>"),
            Event::End(TagEnd::Strikethrough) => output.push_str("</s>"),

            Event::Start(Tag::Link { dest_url, .. }) => {
                output.push_str(&format!("<a href=\"{}\">", html::escape(&dest_url)));
            }
            Event::End(TagEnd::Link) => output.push_str("</a>"),

            Event::Code(code) => {
                output.push_str(&format!("<code>{}</code>", html::escape(&code)));
            }

            Event::Text(text) => {
                output.push_str(&html::escape(&text));
            }

            Event::SoftBreak => output.push('\n'),
            Event::HardBreak => output.push_str("\n\n"),
            Event::Rule => output.push_str("\n—\n\n"),

            _ => {}
        }
    }
    output.trim_end().to_string()
}
