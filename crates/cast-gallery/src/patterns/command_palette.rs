use cast::{
    Dialog, Intent, Kbd, SearchInput, Size, Variant,
    egui::{self, Color32, RichText, Sense, StrokeKind, text::LayoutJob},
};

const COMMANDS: [CommandPaletteItem; 7] = [
    CommandPaletteItem {
        id: "open-workspace",
        title: "Open workspace",
        detail: "Jump to the main agent workspace",
        shortcut: "1",
        intent: Intent::Primary,
    },
    CommandPaletteItem {
        id: "show-components",
        title: "Show components",
        detail: "Review primitive Cast widgets",
        shortcut: "2",
        intent: Intent::Secondary,
    },
    CommandPaletteItem {
        id: "theme-lab",
        title: "Open theme lab",
        detail: "Tune tokens and runtime theme values",
        shortcut: "3",
        intent: Intent::Info,
    },
    CommandPaletteItem {
        id: "toggle-mode",
        title: "Toggle light or dark mode",
        detail: "Switch the gallery between theme modes",
        shortcut: "T",
        intent: Intent::Neutral,
    },
    CommandPaletteItem {
        id: "export-table",
        title: "Export current table",
        detail: "Run the table export action",
        shortcut: "E",
        intent: Intent::Success,
    },
    CommandPaletteItem {
        id: "review-diagnostics",
        title: "Review diagnostics",
        detail: "Inspect typography and rendering metrics",
        shortcut: "D",
        intent: Intent::Warning,
    },
    CommandPaletteItem {
        id: "reset-theme",
        title: "Reset theme overrides",
        detail: "Return the gallery to its default theme seed",
        shortcut: "R",
        intent: Intent::Danger,
    },
];

#[derive(Debug, Default)]
pub struct CommandPaletteState {
    pub open: bool,
    pub query: String,
    pub selected: usize,
    pub last_action: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CommandPaletteItem {
    pub id: &'static str,
    pub title: &'static str,
    pub detail: &'static str,
    pub shortcut: &'static str,
    pub intent: Intent,
}

pub fn show_command_palette(
    ctx: &egui::Context,
    state: &mut CommandPaletteState,
) -> Option<&'static str> {
    let CommandPaletteState {
        open,
        query,
        selected,
        last_action,
    } = state;
    let mut action = None;

    Dialog::new(open, "gallery_command_palette")
        .title("Command palette")
        .description("Search actions, jump between surfaces, and trigger workflow commands.")
        .width(560.0)
        .show(ctx, |ui, dialog| {
            ui.add(
                SearchInput::new(query)
                    .hint_text("Search commands...")
                    .size(Size::Small)
                    .variant(Variant::Subtle)
                    .width(ui.available_width()),
            );
            ui.add_space(10.0);

            let matches = filtered_command_indices(query);
            clamp_selected(selected, matches.len());
            handle_palette_keys(ui.ctx(), selected, &matches, &mut action);

            if matches.is_empty() {
                ui.label(
                    RichText::new("No commands found")
                        .font(cast::theme_for_ui(ui).typography.small.clone())
                        .color(cast::theme_for_ui(ui).colors.text_muted),
                );
                return;
            }

            for (position, command_index) in matches.iter().copied().enumerate() {
                let command = COMMANDS[command_index];
                let response = command_row(ui, command, position == *selected);
                if response.clicked() {
                    *selected = position;
                    action = Some(command.id);
                    dialog.close();
                }
            }

            if let Some(id) = action {
                *last_action = Some(id);
                dialog.close();
            }
        });

    action
}

fn handle_palette_keys(
    ctx: &egui::Context,
    selected: &mut usize,
    matches: &[usize],
    action: &mut Option<&'static str>,
) {
    let match_count = matches.len();
    if match_count == 0 {
        *selected = 0;
        return;
    }

    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)) {
        *selected = (*selected + 1).min(match_count - 1);
    }
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
        *selected = selected.saturating_sub(1);
    }
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
        *action = Some(COMMANDS[matches[*selected]].id);
    }
}

fn command_row(ui: &mut egui::Ui, command: CommandPaletteItem, selected: bool) -> egui::Response {
    let theme = cast::theme_for_ui(ui);
    let height = 52.0;
    let width = ui.available_width().max(220.0);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = response.hovered();
        let accent = match command.intent {
            Intent::Primary => theme.colors.primary_family.base,
            Intent::Secondary => theme.colors.secondary_family.base,
            Intent::Success => theme.colors.success_family.base,
            Intent::Warning => theme.colors.warning_family.base,
            Intent::Danger => theme.colors.danger_family.base,
            Intent::Info => theme.colors.info_family.base,
            Intent::Neutral => theme.colors.text_muted,
        };
        let fill = if selected {
            cast::mix_with_transparent(accent, 0.08)
        } else if hovered {
            theme.colors.surface_muted
        } else {
            Color32::TRANSPARENT
        };
        let border = if selected {
            cast::mix_with_transparent(accent, 0.24)
        } else {
            Color32::TRANSPARENT
        };

        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md as u8),
            fill,
            egui::Stroke::new(theme.stroke.sm, border),
            StrokeKind::Outside,
        );

        let title = ui.painter().layout_job(command_layout_job(
            command.title,
            theme.typography.button.clone(),
            theme.colors.text,
            theme.typography.letter_spacing,
        ));
        let detail = ui.painter().layout_job(command_layout_job(
            command.detail,
            theme.typography.small.clone(),
            theme.colors.text_muted,
            theme.typography.letter_spacing,
        ));
        let text_x = rect.min.x + theme.spacing.md;
        let text_y = rect.center().y - (title.size().y + 3.0 + detail.size().y) / 2.0;
        ui.painter()
            .galley(egui::pos2(text_x, text_y), title, theme.colors.text);
        ui.painter().galley(
            egui::pos2(text_x, text_y + theme.typography.button.size + 3.0),
            detail,
            theme.colors.text_muted,
        );

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(theme.spacing.md);
                ui.add(Kbd::new(command.shortcut).size(Size::Small));
            });
        });
    }

    response
}

fn filtered_command_indices(query: &str) -> Vec<usize> {
    let query = query.trim().to_lowercase();
    COMMANDS
        .iter()
        .enumerate()
        .filter_map(|(index, command)| {
            if query.is_empty()
                || command.title.to_lowercase().contains(&query)
                || command.detail.to_lowercase().contains(&query)
            {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

fn clamp_selected(selected: &mut usize, match_count: usize) {
    if match_count == 0 {
        *selected = 0;
    } else {
        *selected = (*selected).min(match_count - 1);
    }
}

fn command_layout_job(
    text: &'static str,
    font_id: egui::FontId,
    color: Color32,
    letter_spacing: f32,
) -> LayoutJob {
    LayoutJob::single_section(
        text.to_owned(),
        egui::text::TextFormat {
            font_id,
            color,
            extra_letter_spacing: letter_spacing,
            ..Default::default()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_filter_matches_title_and_detail() {
        assert_eq!(filtered_command_indices("theme"), vec![2, 3, 6]);
        assert_eq!(filtered_command_indices("diagnostics"), vec![5]);
    }

    #[test]
    fn command_filter_returns_all_commands_for_empty_query() {
        assert_eq!(filtered_command_indices("").len(), COMMANDS.len());
    }

    #[test]
    fn selected_index_clamps_to_available_matches() {
        let mut selected = 10;

        clamp_selected(&mut selected, 3);
        assert_eq!(selected, 2);

        clamp_selected(&mut selected, 0);
        assert_eq!(selected, 0);
    }
}
