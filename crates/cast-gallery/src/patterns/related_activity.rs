use cast::{
    Accordion, AccordionItem, CastTheme, Disclosure, Intent, Size, ThemeMode,
    egui::{self, Color32},
};

pub fn show_related_activity(
    ui: &mut egui::Ui,
    related_activity_open: &mut bool,
    related_activity_group: &mut Option<usize>,
) {
    let theme = cast::theme_for_ui(ui);
    egui::Frame::new()
        .fill(Color32::TRANSPARENT)
        .stroke(egui::Stroke::new(theme.stroke.sm, theme.colors.border))
        .corner_radius(egui::CornerRadius::same(theme.radius.lg.round() as u8))
        .inner_margin(egui::Margin::same(0))
        .show(ui, |ui| {
            Disclosure::new(related_activity_open, "Related activity")
                .subtitle("Recent work across planning, review, and checks")
                .trailing_status_dot("3 active", Intent::Info)
                .show(ui, |ui| {
                    Accordion::new(
                        related_activity_group,
                        [
                            AccordionItem::new("Current run")
                                .subtitle("Planning and patch flow")
                                .trailing_status_dot("Active", Intent::Info),
                            AccordionItem::new("Review queue")
                                .subtitle("Changes waiting for inspection")
                                .trailing_status_dot("Ready", Intent::Primary),
                            AccordionItem::new("Checks")
                                .subtitle("Latest build and smoke status")
                                .trailing_status_dot("Done", Intent::Success),
                        ],
                    )
                    .size(Size::Small)
                    .show(ui, |ui, group| {
                        let events = activity_events(group);
                        let children_top = ui.cursor().top();
                        for event in events {
                            activity_event_row(ui, event);
                        }
                        ui.add_space(theme.spacing.xs);
                        paint_activity_child_guide(ui, &theme, children_top, ui.cursor().top());
                    });
                });
        });
}

#[derive(Clone, Copy, Debug)]
struct ActivityEvent {
    title: &'static str,
    detail: &'static str,
    status: &'static str,
    intent: Intent,
}

fn activity_events(group: usize) -> [ActivityEvent; 3] {
    match group {
        0 => [
            ActivityEvent {
                title: "Model planning",
                detail: "4 tool calls queued",
                status: "Active",
                intent: Intent::Info,
            },
            ActivityEvent {
                title: "Patch review",
                detail: "2 files changed",
                status: "Ready",
                intent: Intent::Primary,
            },
            ActivityEvent {
                title: "Gallery smoke",
                detail: "Last run passed",
                status: "Done",
                intent: Intent::Success,
            },
        ],
        1 => [
            ActivityEvent {
                title: "Component API",
                detail: "Disclosure and accordion",
                status: "Open",
                intent: Intent::Info,
            },
            ActivityEvent {
                title: "Visual review",
                detail: "Badge density accepted",
                status: "Done",
                intent: Intent::Success,
            },
            ActivityEvent {
                title: "Table polish",
                detail: "Selection cells centered",
                status: "Done",
                intent: Intent::Success,
            },
        ],
        _ => [
            ActivityEvent {
                title: "Unit tests",
                detail: "cast-ui passed",
                status: "Done",
                intent: Intent::Success,
            },
            ActivityEvent {
                title: "Gallery build",
                detail: "cast-gallery built",
                status: "Done",
                intent: Intent::Success,
            },
            ActivityEvent {
                title: "Formatting",
                detail: "cargo fmt clean",
                status: "Done",
                intent: Intent::Success,
            },
        ],
    }
}

fn activity_event_row(ui: &mut egui::Ui, event: ActivityEvent) {
    let theme = cast::theme_for_ui(ui);
    let row_height = 42.0;
    let width = ui.available_width().max(160.0);
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, row_height), egui::Sense::hover());

    if ui.is_rect_visible(response.rect) {
        let title = ui.painter().layout_job(activity_layout_job(
            event.title,
            theme.typography.small.clone(),
            theme.typography.letter_spacing,
        ));
        let detail = ui.painter().layout_job(activity_layout_job(
            event.detail,
            theme.typography.caption.clone(),
            theme.typography.letter_spacing,
        ));
        let status = activity_badge_layout(ui, &theme, event.status);

        let dot_size = 7.0;
        let dot_gap = theme.spacing.xs + 1.0;
        let text_x = activity_child_text_x(rect, &theme);
        let title_y = rect.center().y - (title.size().y + 2.0 + detail.size().y) / 2.0;
        let dot_center = egui::pos2(
            text_x - dot_gap - dot_size / 2.0,
            title_y + title.size().y / 2.0,
        );

        ui.painter().circle_filled(
            dot_center,
            dot_size / 2.0,
            activity_intent_color(&theme, event.intent),
        );
        ui.painter()
            .galley(egui::pos2(text_x, title_y), title, theme.colors.text);
        ui.painter().galley(
            egui::pos2(text_x, title_y + theme.typography.small.size + 3.0),
            detail,
            theme.colors.text_muted,
        );
        paint_activity_badge(
            ui,
            &theme,
            event.intent,
            egui::pos2(rect.max.x - theme.spacing.sm - status.size().x, title_y),
            status,
        );
    }
}

fn activity_child_text_x(rect: egui::Rect, theme: &CastTheme) -> f32 {
    rect.min.x + theme.spacing.sm * 2.0 + 20.0
}

fn activity_child_guide_x(rect: egui::Rect, theme: &CastTheme) -> f32 {
    rect.min.x + theme.spacing.sm + 6.0
}

fn paint_activity_child_guide(ui: &egui::Ui, theme: &CastTheme, top: f32, bottom: f32) {
    if bottom <= top {
        return;
    }

    let x = activity_child_guide_x(ui.max_rect(), theme);
    ui.painter().line_segment(
        [
            egui::pos2(x, top - theme.spacing.lg),
            egui::pos2(x, bottom - theme.spacing.md),
        ],
        egui::Stroke::new(theme.stroke.sm, activity_child_guide_color(theme)),
    );
}

struct ActivityBadgeLayout {
    label: std::sync::Arc<egui::Galley>,
    size: egui::Vec2,
}

impl ActivityBadgeLayout {
    fn size(&self) -> egui::Vec2 {
        self.size
    }
}

fn activity_badge_layout(
    ui: &egui::Ui,
    theme: &CastTheme,
    label: &'static str,
) -> ActivityBadgeLayout {
    let label = ui.painter().layout_job(activity_layout_job(
        label,
        theme.typography.caption.clone(),
        theme.typography.letter_spacing,
    ));
    let padding = egui::vec2(theme.spacing.sm, theme.spacing.xs * 0.5);
    let dot_size = 7.0;
    let dot_gap = theme.spacing.xs + 1.0;

    ActivityBadgeLayout {
        size: egui::vec2(
            label.size().x + padding.x * 2.0 + dot_size + dot_gap,
            label.size().y + padding.y * 2.0,
        ),
        label,
    }
}

fn paint_activity_badge(
    ui: &egui::Ui,
    theme: &CastTheme,
    intent: Intent,
    pos: egui::Pos2,
    badge: ActivityBadgeLayout,
) {
    let rect = egui::Rect::from_min_size(pos, badge.size);
    let border = match theme.mode {
        ThemeMode::Light => theme.colors.border,
        ThemeMode::Dark => cast::mix_with_transparent(theme.colors.text_muted, 0.28),
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(theme.components.badge.radius as u8),
        Color32::TRANSPARENT,
        egui::Stroke::new(theme.components.badge.border_width, border),
        egui::StrokeKind::Outside,
    );

    let dot_size = 7.0;
    let dot_gap = theme.spacing.xs + 1.0;
    let x = rect.min.x + theme.spacing.sm;
    ui.painter().circle_filled(
        egui::pos2(x + dot_size / 2.0, rect.center().y),
        dot_size / 2.0,
        activity_intent_color(theme, intent),
    );
    ui.painter().galley(
        egui::pos2(
            x + dot_size + dot_gap,
            rect.center().y - badge.label.size().y / 2.0,
        ),
        badge.label,
        theme.colors.text,
    );
}

fn activity_child_guide_color(theme: &CastTheme) -> Color32 {
    match theme.mode {
        ThemeMode::Light => cast::mix_with_transparent(theme.colors.primary_family.base, 0.18),
        ThemeMode::Dark => cast::mix_with_transparent(theme.colors.text_muted, 0.32),
    }
}

fn activity_layout_job(
    text: &'static str,
    font_id: egui::FontId,
    letter_spacing: f32,
) -> egui::text::LayoutJob {
    egui::text::LayoutJob::single_section(
        text.to_owned(),
        egui::text::TextFormat {
            font_id,
            extra_letter_spacing: letter_spacing,
            color: Color32::PLACEHOLDER,
            ..Default::default()
        },
    )
}

fn activity_intent_color(theme: &CastTheme, intent: Intent) -> Color32 {
    match intent {
        Intent::Primary => theme.colors.primary_family.base,
        Intent::Secondary => theme.colors.secondary_family.base,
        Intent::Success => theme.colors.success_family.base,
        Intent::Warning => theme.colors.warning_family.base,
        Intent::Danger => theme.colors.danger_family.base,
        Intent::Info => theme.colors.info_family.base,
        Intent::Neutral => theme.colors.text_muted,
    }
}
