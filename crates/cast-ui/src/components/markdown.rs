use egui::{Color32, Response, RichText, Sense, Ui, Widget};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::theme::{CastTheme, theme_for_ui};

#[derive(Clone, Debug)]
pub struct Markdown {
    source: String,
    width: Option<f32>,
    selectable: bool,
}

impl Markdown {
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            width: None,
            selectable: true,
        }
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(120.0));
        self
    }

    #[must_use]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }
}

impl Widget for Markdown {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme_for_ui(ui);
        if let Some(width) = self.width {
            ui.set_max_width(width);
        }

        let blocks = parse_markdown_blocks(&self.source);
        let mut combined: Option<Response> = None;
        for (index, block) in blocks.iter().enumerate() {
            if index > 0 {
                ui.add_space(markdown_block_gap(&theme, block));
            }
            let response = paint_markdown_block(ui, &theme, block, self.selectable);
            combined = Some(match combined {
                Some(existing) => existing.union(response),
                None => response,
            });
        }

        combined.unwrap_or_else(|| ui.allocate_response(egui::Vec2::ZERO, Sense::hover()))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MarkdownBlock {
    Heading {
        level: usize,
        text: String,
    },
    Paragraph(String),
    Quote(String),
    ListItem {
        text: String,
        ordered: bool,
        number: Option<u64>,
        checked: Option<bool>,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Rule,
}

#[derive(Clone, Debug)]
enum BlockKind {
    Paragraph,
    Heading(usize),
    Quote,
    ListItem {
        ordered: bool,
        number: Option<u64>,
        checked: Option<bool>,
    },
    CodeBlock(Option<String>),
}

#[derive(Clone, Debug)]
struct BlockBuilder {
    kind: BlockKind,
    text: String,
}

#[derive(Clone, Debug)]
struct ListState {
    start: Option<u64>,
    index: u64,
}

fn parse_markdown_blocks(source: &str) -> Vec<MarkdownBlock> {
    let mut blocks = Vec::new();
    let mut current: Option<BlockBuilder> = None;
    let mut lists: Vec<ListState> = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    for event in Parser::new_ext(source, options) {
        match event {
            Event::Start(Tag::Paragraph) => {
                if current.is_none() {
                    current = Some(BlockBuilder::new(BlockKind::Paragraph));
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if matches!(
                    current.as_ref().map(|builder| &builder.kind),
                    Some(BlockKind::Paragraph)
                ) {
                    finish_block(&mut blocks, current.take());
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                current = Some(BlockBuilder::new(BlockKind::Heading(heading_level(level))));
            }
            Event::End(TagEnd::Heading(_)) => finish_block(&mut blocks, current.take()),
            Event::Start(Tag::BlockQuote(_)) => {
                current = Some(BlockBuilder::new(BlockKind::Quote));
            }
            Event::End(TagEnd::BlockQuote(_)) => finish_block(&mut blocks, current.take()),
            Event::Start(Tag::CodeBlock(kind)) => {
                current = Some(BlockBuilder::new(BlockKind::CodeBlock(code_language(kind))));
            }
            Event::End(TagEnd::CodeBlock) => finish_block(&mut blocks, current.take()),
            Event::Start(Tag::List(start)) => {
                lists.push(ListState { start, index: 0 });
            }
            Event::End(TagEnd::List(_)) => {
                lists.pop();
            }
            Event::Start(Tag::Item) => {
                let list = lists.last_mut();
                let (ordered, number) = match list {
                    Some(list) => {
                        let number = list.start.map(|start| start + list.index);
                        list.index += 1;
                        (list.start.is_some(), number)
                    }
                    None => (false, None),
                };
                current = Some(BlockBuilder::new(BlockKind::ListItem {
                    ordered,
                    number,
                    checked: None,
                }));
            }
            Event::End(TagEnd::Item) => finish_block(&mut blocks, current.take()),
            Event::Text(text) | Event::Html(text) | Event::InlineHtml(text) => {
                append_markdown_text(&mut current, &text);
            }
            Event::Code(code) | Event::InlineMath(code) | Event::DisplayMath(code) => {
                append_markdown_text(&mut current, "`");
                append_markdown_text(&mut current, &code);
                append_markdown_text(&mut current, "`");
            }
            Event::FootnoteReference(reference) => {
                append_markdown_text(&mut current, "[^");
                append_markdown_text(&mut current, &reference);
                append_markdown_text(&mut current, "]");
            }
            Event::SoftBreak | Event::HardBreak => append_markdown_text(&mut current, "\n"),
            Event::Rule => blocks.push(MarkdownBlock::Rule),
            Event::TaskListMarker(checked) => {
                if let Some(BlockBuilder {
                    kind:
                        BlockKind::ListItem {
                            checked: item_checked,
                            ..
                        },
                    ..
                }) = current.as_mut()
                {
                    *item_checked = Some(checked);
                }
            }
            Event::Start(_) | Event::End(_) => {}
        }
    }
    finish_block(&mut blocks, current);

    blocks
}

impl BlockBuilder {
    fn new(kind: BlockKind) -> Self {
        Self {
            kind,
            text: String::new(),
        }
    }
}

fn append_markdown_text(current: &mut Option<BlockBuilder>, text: &str) {
    if current.is_none() {
        *current = Some(BlockBuilder::new(BlockKind::Paragraph));
    }
    if let Some(current) = current {
        current.text.push_str(text);
    }
}

fn finish_block(blocks: &mut Vec<MarkdownBlock>, builder: Option<BlockBuilder>) {
    let Some(builder) = builder else {
        return;
    };
    let text = match builder.kind {
        BlockKind::CodeBlock(_) => builder.text.trim_end().to_owned(),
        _ => builder.text.trim().to_owned(),
    };
    if text.is_empty() {
        return;
    }

    blocks.push(match builder.kind {
        BlockKind::Paragraph => MarkdownBlock::Paragraph(text),
        BlockKind::Heading(level) => MarkdownBlock::Heading { level, text },
        BlockKind::Quote => MarkdownBlock::Quote(text),
        BlockKind::ListItem {
            ordered,
            number,
            checked,
        } => MarkdownBlock::ListItem {
            text,
            ordered,
            number,
            checked,
        },
        BlockKind::CodeBlock(language) => MarkdownBlock::CodeBlock {
            language,
            code: text,
        },
    });
}

fn heading_level(level: HeadingLevel) -> usize {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn code_language(kind: CodeBlockKind<'_>) -> Option<String> {
    match kind {
        CodeBlockKind::Indented => None,
        CodeBlockKind::Fenced(language) => {
            let language = language.trim();
            (!language.is_empty()).then(|| language.to_owned())
        }
    }
}

fn paint_markdown_block(
    ui: &mut Ui,
    theme: &CastTheme,
    block: &MarkdownBlock,
    selectable: bool,
) -> Response {
    match block {
        MarkdownBlock::Heading { level, text } => {
            let mut font = theme.typography.strong.clone();
            font.size = match level {
                1 => theme.typography.heading.size,
                2 => theme.typography.body_strong.size + 2.0,
                _ => theme.typography.body_strong.size,
            };
            ui.add(markdown_label(
                RichText::new(text)
                    .font(font)
                    .color(theme.colors.text)
                    .extra_letter_spacing(theme.typography.letter_spacing),
                selectable,
            ))
        }
        MarkdownBlock::Paragraph(text) => ui.add(markdown_label(
            RichText::new(text)
                .font(theme.typography.body.clone())
                .color(theme.colors.text)
                .extra_letter_spacing(theme.typography.letter_spacing),
            selectable,
        )),
        MarkdownBlock::Quote(text) => {
            egui::Frame::new()
                .fill(theme.colors.surface_muted)
                .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
                .corner_radius(egui::CornerRadius::same(theme.radius.md.round() as u8))
                .inner_margin(egui::Margin::symmetric(
                    theme.spacing.md as i8,
                    theme.spacing.sm as i8,
                ))
                .show(ui, |ui| {
                    ui.add(markdown_label(
                        RichText::new(text)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_muted)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                        selectable,
                    ))
                })
                .inner
        }
        MarkdownBlock::ListItem {
            text,
            ordered,
            number,
            checked,
        } => {
            ui.horizontal_top(|ui| {
                let marker = list_marker(*ordered, *number, *checked);
                ui.label(
                    RichText::new(marker)
                        .font(theme.typography.body.clone())
                        .color(theme.colors.text_muted),
                );
                ui.add(markdown_label(
                    RichText::new(text)
                        .font(theme.typography.body.clone())
                        .color(theme.colors.text)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                    selectable,
                ));
            })
            .response
        }
        MarkdownBlock::CodeBlock { language, code } => {
            egui::Frame::new()
                .fill(markdown_code_fill(theme))
                .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
                .corner_radius(egui::CornerRadius::same(theme.radius.md.round() as u8))
                .inner_margin(egui::Margin::same(theme.spacing.sm as i8))
                .show(ui, |ui| {
                    if let Some(language) = language {
                        ui.label(
                            RichText::new(language)
                                .font(theme.typography.caption.clone())
                                .color(theme.colors.text_subtle),
                        );
                        ui.add_space(theme.spacing.xs);
                    }
                    ui.add(markdown_label(
                        RichText::new(code)
                            .font(theme.typography.code.clone())
                            .color(theme.colors.text)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                        selectable,
                    ))
                })
                .inner
        }
        MarkdownBlock::Rule => {
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), Sense::hover());
            ui.painter().line_segment(
                [rect.left_center(), rect.right_center()],
                egui::Stroke::new(theme.stroke.sm, theme.colors.border),
            );
            response
        }
    }
}

fn markdown_label(text: RichText, selectable: bool) -> egui::Label {
    let label = egui::Label::new(text).wrap();
    if selectable {
        label.selectable(true)
    } else {
        label
    }
}

fn markdown_code_fill(theme: &CastTheme) -> Color32 {
    match theme.mode {
        crate::ThemeMode::Light => theme.colors.surface_muted,
        crate::ThemeMode::Dark => crate::mix_with_transparent(theme.colors.text, 0.04),
    }
}

fn markdown_block_gap(theme: &CastTheme, block: &MarkdownBlock) -> f32 {
    match block {
        MarkdownBlock::ListItem { .. } => theme.spacing.xs,
        MarkdownBlock::Rule => theme.spacing.sm,
        _ => theme.spacing.sm,
    }
}

fn list_marker(ordered: bool, number: Option<u64>, checked: Option<bool>) -> String {
    if let Some(checked) = checked {
        return if checked { "[x]" } else { "[ ]" }.to_owned();
    }
    if ordered {
        format!("{}.", number.unwrap_or(1))
    } else {
        "-".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_parser_extracts_headings_lists_and_code() {
        let blocks = parse_markdown_blocks(
            "# Result\n\n- Inspect output\n- [x] Patch\n\n```rust\nlet ok = true;\n```",
        );

        assert_eq!(
            blocks[0],
            MarkdownBlock::Heading {
                level: 1,
                text: "Result".to_owned()
            }
        );
        assert_eq!(
            blocks[1],
            MarkdownBlock::ListItem {
                text: "Inspect output".to_owned(),
                ordered: false,
                number: None,
                checked: None
            }
        );
        assert_eq!(
            blocks[2],
            MarkdownBlock::ListItem {
                text: "Patch".to_owned(),
                ordered: false,
                number: None,
                checked: Some(true)
            }
        );
        assert_eq!(
            blocks[3],
            MarkdownBlock::CodeBlock {
                language: Some("rust".to_owned()),
                code: "let ok = true;".to_owned()
            }
        );
    }

    #[test]
    fn ordered_list_keeps_start_number() {
        let blocks = parse_markdown_blocks("3. Plan\n4. Patch");

        assert_eq!(
            blocks[0],
            MarkdownBlock::ListItem {
                text: "Plan".to_owned(),
                ordered: true,
                number: Some(3),
                checked: None
            }
        );
    }

    #[test]
    fn markdown_width_has_floor() {
        let markdown = Markdown::new("Text").width(40.0).selectable(false);

        assert_eq!(markdown.width, Some(120.0));
        assert!(!markdown.selectable);
    }
}
