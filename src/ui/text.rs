// src/ui/text.rs
// Text layout helpers: word wrapping of styled spans, Markdown, and syntax-highlighted code.
use ratatui::prelude::*;
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use textwrap::core::display_width;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME: LazyLock<Theme> = LazyLock::new(|| {
    let mut themes = ThemeSet::load_defaults();
    themes.themes.remove("base16-ocean.dark").unwrap_or_default()
});

pub const HEADING: Style = Style::new().fg(Color::LightYellow).add_modifier(Modifier::BOLD);
pub const BOLD: Style = Style::new().add_modifier(Modifier::BOLD);
pub const INLINE_CODE: Style = Style::new().fg(Color::LightCyan);
pub const DIM: Style = Style::new().fg(Color::DarkGray);
pub const LINK: Style = Style::new().fg(Color::LightBlue);

/// Wraps styled spans at word boundaries. Words longer than the width are split.
/// Continuation lines start with `indent`.
pub fn wrap_spans(spans: Vec<Span<'static>>, width: usize, indent: &str) -> Vec<Line<'static>> {
    let width = width.max(indent.len() + 1).max(1);
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut current: Vec<Span<'static>> = Vec::new();
    let mut current_width = 0;

    // Split into (text, style) pieces that are either whitespace or words
    let mut pieces: Vec<(String, Style)> = Vec::new();
    for span in spans {
        let style = span.style;
        let mut word = String::new();
        let mut in_space = false;
        for c in span.content.chars() {
            let is_space = c == ' ';
            if !word.is_empty() && is_space != in_space {
                pieces.push((std::mem::take(&mut word), style));
            }
            in_space = is_space;
            word.push(c);
        }
        if !word.is_empty() {
            pieces.push((word, style));
        }
    }

    let flush = |lines: &mut Vec<Line<'static>>, current: &mut Vec<Span<'static>>| {
        lines.push(Line::from(std::mem::take(current)));
    };

    for (piece, style) in pieces {
        let piece_width = display_width(&piece);
        let is_space = piece.starts_with(' ');
        if current_width + piece_width <= width {
            current_width += piece_width;
            current.push(Span::styled(piece, style));
            continue;
        }
        if is_space {
            // Break the line at whitespace
            flush(&mut lines, &mut current);
            current.push(Span::raw(indent.to_string()));
            current_width = indent.len();
            continue;
        }
        if current_width > indent.len() && piece_width <= width - indent.len() {
            flush(&mut lines, &mut current);
            current.push(Span::raw(indent.to_string()));
            current_width = indent.len();
            current.push(Span::styled(piece, style));
            current_width += piece_width;
            continue;
        }
        // Split a word that doesn't fit on a line of its own
        for c in piece.chars() {
            let char_width = display_width(c.encode_utf8(&mut [0; 4]));
            if current_width + char_width > width {
                flush(&mut lines, &mut current);
                current.push(Span::raw(indent.to_string()));
                current_width = indent.len();
            }
            current.push(Span::styled(c.to_string(), style));
            current_width += char_width;
        }
    }
    if !current.is_empty() || lines.is_empty() {
        lines.push(Line::from(current));
    }
    lines
}

/// Wraps plain text, keeping existing line breaks
pub fn wrap_text(text: &str, width: usize, style: Style) -> Vec<Line<'static>> {
    text.lines().flat_map(|line| wrap_spans(vec![Span::styled(line.to_string(), style)], width, "")).collect()
}

/// Splits inline Markdown (`**bold**`, `` `code` ``, `*italic*`) into styled spans
pub fn inline_spans(text: &str, base: Style) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        let next = [("`", INLINE_CODE), ("**", BOLD), ("__", BOLD)]
            .into_iter()
            .filter_map(|(marker, style)| rest.find(marker).map(|pos| (pos, marker, style)))
            .min_by_key(|(pos, _, _)| *pos);
        let Some((start, marker, style)) = next else {
            spans.push(Span::styled(rest.to_string(), base));
            break;
        };
        let after = &rest[start + marker.len()..];
        let Some(end) = after.find(marker) else {
            spans.push(Span::styled(rest.to_string(), base));
            break;
        };
        if start > 0 {
            spans.push(Span::styled(rest[..start].to_string(), base));
        }
        spans.push(Span::styled(after[..end].to_string(), base.patch(style)));
        rest = &after[end + marker.len()..];
    }
    spans
}

/// Renders Markdown into wrapped lines: headings, lists, quotes, inline styles and
/// fenced code blocks (highlighted as Rust)
pub fn markdown(text: &str, width: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut code = String::new();
    let mut in_code = false;

    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            if in_code {
                lines.extend(code_block(&code, width, None));
                code.clear();
            }
            in_code = !in_code;
            continue;
        }
        if in_code {
            code.push_str(line);
            code.push('\n');
            continue;
        }

        if trimmed.is_empty() {
            lines.push(Line::from(""));
        } else if let Some(heading) = trimmed.strip_prefix('#') {
            let heading = heading.trim_start_matches('#').trim();
            lines.extend(wrap_spans(inline_spans(heading, HEADING), width, ""));
        } else if let Some(item) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            let indent = " ".repeat(line.len() - trimmed.len());
            let mut spans = vec![Span::raw(format!("{}• ", indent))];
            spans.extend(inline_spans(item, Style::default()));
            lines.extend(wrap_spans(spans, width, &format!("{}  ", indent)));
        } else if let Some(quote) = trimmed.strip_prefix('>') {
            let mut spans = vec![Span::styled("▌ ", DIM)];
            spans.extend(inline_spans(quote.trim(), Style::new().add_modifier(Modifier::ITALIC)));
            lines.extend(wrap_spans(spans, width, "  "));
        } else {
            lines.extend(wrap_spans(inline_spans(line, Style::default()), width, ""));
        }
    }
    if in_code && !code.is_empty() {
        lines.extend(code_block(&code, width, None));
    }
    lines
}

/// Converts syntect highlighting of one line into spans
fn highlight_line(highlighter: &mut HighlightLines, line: &str) -> Vec<Span<'static>> {
    let with_newline = format!("{}\n", line);
    match highlighter.highlight_line(&with_newline, &SYNTAX_SET) {
        Ok(ranges) => ranges
            .into_iter()
            .map(|(style, text)| {
                let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                Span::styled(text.trim_end_matches('\n').replace('\t', "    "), Style::new().fg(fg))
            })
            .filter(|span| !span.content.is_empty())
            .collect(),
        Err(_) => vec![Span::raw(line.to_string())],
    }
}

/// Splits highlighted spans into chunks no wider than `width` (hard wrap, keeps indentation)
fn hard_wrap(spans: Vec<Span<'static>>, width: usize) -> Vec<Vec<Span<'static>>> {
    let width = width.max(1);
    let mut rows = vec![Vec::new()];
    let mut row_width = 0;
    for span in spans {
        let style = span.style;
        let mut piece = String::new();
        for c in span.content.chars() {
            let char_width = display_width(c.encode_utf8(&mut [0; 4]));
            if row_width + char_width > width {
                if !piece.is_empty() {
                    rows.last_mut().unwrap().push(Span::styled(std::mem::take(&mut piece), style));
                }
                rows.push(Vec::new());
                row_width = 0;
            }
            piece.push(c);
            row_width += char_width;
        }
        if !piece.is_empty() {
            rows.last_mut().unwrap().push(Span::styled(piece, style));
        }
    }
    rows
}

/// A syntax-highlighted Rust code block in a box, with an optional title
pub fn code_block(code: &str, width: usize, title: Option<&str>) -> Vec<Line<'static>> {
    let width = width.max(8);
    let inner = width - 4; // "│ " + " │"
    let syntax = SYNTAX_SET.find_syntax_by_extension("rs").unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());
    let mut highlighter = HighlightLines::new(syntax, &THEME);

    let title = title.map(|t| format!(" {} ", t)).unwrap_or_default();
    let title: String = title.chars().take(inner).collect();
    let mut lines = vec![Line::from(Span::styled(
        format!("┌─{}{}┐", title, "─".repeat(width.saturating_sub(3 + display_width(&title)))),
        DIM,
    ))];
    for line in code.trim_end().lines() {
        for row in hard_wrap(highlight_line(&mut highlighter, line), inner) {
            let row_width: usize = row.iter().map(|s| display_width(&s.content)).sum();
            let mut spans = vec![Span::styled("│ ", DIM)];
            spans.extend(row);
            spans.push(Span::raw(" ".repeat(inner.saturating_sub(row_width))));
            spans.push(Span::styled(" │", DIM));
            lines.push(Line::from(spans));
        }
    }
    lines.push(Line::from(Span::styled(format!("└{}┘", "─".repeat(width - 2)), DIM)));
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_of(line: &Line) -> String {
        line.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn wraps_at_word_boundaries() {
        let lines = wrap_spans(vec![Span::raw("the quick brown fox")], 10, "");
        let texts: Vec<_> = lines.iter().map(text_of).collect();
        assert_eq!(texts, ["the quick ", "brown fox"]);
        assert!(lines.iter().all(|l| l.width() <= 10));
    }

    #[test]
    fn splits_long_words() {
        let lines = wrap_spans(vec![Span::raw("abcdefghij")], 4, "");
        assert_eq!(lines.iter().map(text_of).collect::<Vec<_>>(), ["abcd", "efgh", "ij"]);
    }

    #[test]
    fn inline_styles() {
        let spans = inline_spans("use `Vec` for **lists**", Style::default());
        let texts: Vec<_> = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(texts, ["use ", "Vec", " for ", "lists"]);
        assert_eq!(spans[1].style.fg, Some(Color::LightCyan));
    }

    #[test]
    fn markdown_renders_code_blocks_and_lists() {
        let lines = markdown("# Title\n- item\n```rust\nlet x = 1;\n```", 40);
        let texts: Vec<_> = lines.iter().map(text_of).collect();
        assert_eq!(texts[0], "Title");
        assert_eq!(texts[1], "• item");
        assert!(texts[2].starts_with('┌'));
        assert!(texts[3].contains("let x = 1;"));
        assert!(lines.iter().all(|l| l.width() <= 40));
    }

    #[test]
    fn code_block_fits_width() {
        let code = "fn main() {\n    println!(\"a very long line that needs to be wrapped somewhere\");\n}";
        let lines = code_block(code, 30, Some("Example"));
        assert!(lines.iter().all(|l| l.width() == 30), "{:?}", lines.iter().map(|l| l.width()).collect::<Vec<_>>());
        assert!(text_of(&lines[0]).contains("Example"));
    }
}
