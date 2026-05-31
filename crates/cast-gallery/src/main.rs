use std::sync::Arc;

mod patterns;

use patterns::command_palette::{CommandPaletteState, show_command_palette};
use patterns::entity_table_with_details::{
    EntityRecord, EntityTableState, show_entity_table_with_details,
};
use patterns::related_activity::show_related_activity;
use patterns::shell::{
    cast_page_scroll_area, cast_scroll_area, shell_sidebar_fill, show_shell_sidebar,
    show_shell_top_bar,
};

use cast::{
    AgentComposer, Alert, ApprovalPanel, ArtifactCard, Avatar, Badge, Button, Card,
    CastPaletteInput, CastTheme, ChatMessage, Checkbox, CodeOutputPanel, Combobox, ConfirmDialog,
    ConfirmDialogResponse, DateInput, Dialog, Dropdown, EmptyState, FormActions, FormField,
    FormSection, Intent, Kbd, Label, Link, Loader, LoaderStyle, MenuItem, MessageThread, Notice,
    NumberInput, Panel as CastPanel, Popover, ProgressBar, RadioGroup, RunPhase, RunTimeline,
    RunTimelineItem, SearchInput, SegmentedControl, Select, SemanticColorTokens, Separator, Sheet,
    Size, Skeleton, Slider, Switch, Table, Tabs, TextArea, TextInput, ThemeMode, ThemeSeed,
    TimeInput, Toast, ToastPlacement, ToastStack, ToolCall, ToolCallBlock, ToolCallStatus,
    ToolOutput, ToolOutputKind, Tooltip, TypographyTokens, ValidationIssue, ValidationSummary,
    Variant,
    egui::{self, CentralPanel, Color32, Panel as EguiPanel, RichText},
};

const LEAD_COUNT: usize = 24;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Cast Gallery",
        native_options,
        Box::new(|cc| {
            cast::install_cast_fonts(&cc.egui_ctx);
            let app = CastGallery::new();
            cast::set_theme(&cc.egui_ctx, app.theme.clone());
            Ok(Box::new(app))
        }),
    )
}

struct CastGallery {
    theme: CastTheme,
    seed: ThemeSeed,
    zoom: f32,
    search: String,
    lead_search: String,
    command: String,
    name: String,
    handle: String,
    preset_query: String,
    preset_choice: usize,
    form_validation_attention: bool,
    enabled: bool,
    notifications: bool,
    indeterminate: bool,
    form_density: usize,
    menu_choice: usize,
    dialog_open: bool,
    sheet_open: bool,
    confirm_dialog_open: bool,
    confirm_result: Option<ConfirmDialogResponse>,
    toast_preview_open: bool,
    toast_preview_toasts: Vec<Toast>,
    command_palette: CommandPaletteState,
    related_activity_open: bool,
    related_activity_group: Option<usize>,
    lead_selected: [bool; LEAD_COUNT],
    lead_expanded: [bool; LEAD_COUNT],
    lead_date_filter: usize,
    lead_user_filter: usize,
    lead_status_filter: usize,
    lead_payment_filter: usize,
    lead_rows_per_page: usize,
    lead_page: usize,
    lead_exported_count: Option<usize>,
    foundation_tab: usize,
    workflow_segment: usize,
    component_tab: usize,
    agent_model: usize,
    agent_loading: bool,
    agent_tool_open: bool,
    agent_retry_budget: f64,
    agent_due_date: String,
    agent_due_time: String,
    editable_task: String,
    editable_status: usize,
    sidebar_section: usize,
    last_scroll_route: Option<(usize, usize)>,
}

impl CastGallery {
    fn new() -> Self {
        let mode = ThemeMode::Light;
        let seed = ThemeSeed::for_mode(mode).with_typography(TypographyTokens::cast());
        let theme = seed.clone().resolve();

        Self {
            theme,
            seed,
            zoom: 1.0,
            search: String::new(),
            lead_search: String::new(),
            command: String::from("Refine the component gallery into an app-like surface"),
            name: String::from("Cast"),
            handle: String::new(),
            preset_query: String::new(),
            preset_choice: 0,
            form_validation_attention: false,
            enabled: true,
            notifications: true,
            indeterminate: false,
            form_density: 1,
            menu_choice: 0,
            dialog_open: false,
            sheet_open: false,
            confirm_dialog_open: false,
            confirm_result: None,
            toast_preview_open: false,
            toast_preview_toasts: Vec::new(),
            command_palette: CommandPaletteState::default(),
            related_activity_open: false,
            related_activity_group: None,
            lead_selected: [false; LEAD_COUNT],
            lead_expanded: [false; LEAD_COUNT],
            lead_date_filter: 1,
            lead_user_filter: 0,
            lead_status_filter: 0,
            lead_payment_filter: 0,
            lead_rows_per_page: 0,
            lead_page: 0,
            lead_exported_count: None,
            foundation_tab: 0,
            workflow_segment: 0,
            component_tab: 0,
            agent_model: 1,
            agent_loading: true,
            agent_tool_open: true,
            agent_retry_budget: 3.0,
            agent_due_date: String::from("2026-06-01"),
            agent_due_time: String::from("09:30"),
            editable_task: String::from("Review agent output table"),
            editable_status: 1,
            sidebar_section: 0,
            last_scroll_route: None,
        }
    }

    fn apply_theme(&mut self, ctx: &egui::Context) {
        self.theme = self.seed.clone().resolve();
        cast::set_theme(ctx, self.theme.clone());
    }

    fn apply_command_palette_action(&mut self, action: &str) -> bool {
        match action {
            "open-workspace" => self.sidebar_section = 0,
            "show-components" => self.sidebar_section = 2,
            "agent-components" => self.sidebar_section = 3,
            "theme-lab" => self.sidebar_section = 4,
            "export-table" => {
                self.lead_exported_count = Some(
                    self.lead_selected
                        .iter()
                        .filter(|selected| **selected)
                        .count(),
                );
            }
            "review-diagnostics" => self.sidebar_section = 1,
            "toggle-mode" => {
                let next = if self.seed.mode == ThemeMode::Light {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                };
                self.seed = self.seed.clone().with_mode(next);
                return true;
            }
            "reset-theme" => {
                self.seed =
                    ThemeSeed::for_mode(self.seed.mode).with_typography(TypographyTokens::cast());
                return true;
            }
            _ => {}
        }

        false
    }
}

impl eframe::App for CastGallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.set_zoom_factor(self.zoom);
        if ctx.input_mut(|input| input.consume_key(egui::Modifiers::COMMAND, egui::Key::K)) {
            self.command_palette.open = true;
        }

        EguiPanel::left("sidebar")
            .resizable(false)
            .default_size(248.0)
            .frame(
                egui::Frame::new()
                    .fill(shell_sidebar_fill(&self.theme))
                    .inner_margin(egui::Margin::symmetric(18, 18)),
            )
            .show_inside(ui, |ui| {
                cast_scroll_area("sidebar_scroll", &self.theme)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        show_shell_sidebar(ui, &self.theme, &mut self.sidebar_section);
                    });
            });

        let mut theme_changed = false;
        EguiPanel::top("topbar")
            .exact_size(68.0)
            .show_separator_line(false)
            .frame(
                egui::Frame::new()
                    .fill(self.theme.colors.surface)
                    .stroke(egui::Stroke::NONE)
                    .inner_margin(egui::Margin::symmetric(28, 18)),
            )
            .show_inside(ui, |ui| {
                ui.set_min_width(ui.available_width());
                theme_changed |= show_shell_top_bar(ui, &ctx, &mut self.seed, &mut self.zoom);
            });

        CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(self.theme.colors.surface)
                    .stroke(egui::Stroke::NONE)
                    .inner_margin(egui::Margin {
                        left: 28,
                        right: 28,
                        top: 0,
                        bottom: 0,
                    }),
            )
            .show_inside(ui, |ui| {
                let scroll_tab = if self.sidebar_section == 2 {
                    self.component_tab
                } else {
                    0
                };
                let scroll_route = (self.sidebar_section, scroll_tab);
                let reset_scroll = self.last_scroll_route != Some(scroll_route);
                self.last_scroll_route = Some(scroll_route);
                let mut scroll_area =
                    cast_page_scroll_area(("main_scroll", scroll_route), &self.theme);
                if reset_scroll {
                    scroll_area = scroll_area.vertical_scroll_offset(0.0);
                }

                scroll_area.auto_shrink([false, false]).show(ui, |ui| {
                    ui.add_space(self.theme.spacing.lg);
                    theme_changed |= show_workspace_view(
                        ui,
                        self.sidebar_section,
                        &self.theme,
                        &mut self.seed,
                        ctx.pixels_per_point(),
                        self.zoom,
                        &mut self.command,
                        &mut self.search,
                        &mut self.name,
                        &mut self.handle,
                        &mut self.preset_query,
                        &mut self.preset_choice,
                        &mut self.form_validation_attention,
                        &mut self.enabled,
                        &mut self.notifications,
                        &mut self.indeterminate,
                        &mut self.form_density,
                        &mut self.menu_choice,
                        &mut self.dialog_open,
                        &mut self.sheet_open,
                        &mut self.confirm_dialog_open,
                        &mut self.confirm_result,
                        &mut self.toast_preview_open,
                        &mut self.toast_preview_toasts,
                        &mut self.command_palette,
                        &mut self.lead_search,
                        &mut self.related_activity_open,
                        &mut self.related_activity_group,
                        &mut self.lead_selected,
                        &mut self.lead_expanded,
                        &mut self.lead_date_filter,
                        &mut self.lead_user_filter,
                        &mut self.lead_status_filter,
                        &mut self.lead_payment_filter,
                        &mut self.lead_rows_per_page,
                        &mut self.lead_page,
                        &mut self.lead_exported_count,
                        &mut self.foundation_tab,
                        &mut self.workflow_segment,
                        &mut self.component_tab,
                        &mut self.agent_model,
                        &mut self.agent_loading,
                        &mut self.agent_tool_open,
                        &mut self.agent_retry_budget,
                        &mut self.agent_due_date,
                        &mut self.agent_due_time,
                        &mut self.editable_task,
                        &mut self.editable_status,
                    );
                    ui.add_space(self.theme.spacing.lg);
                });
            });

        if let Some(action) = show_command_palette(&ctx, &mut self.command_palette) {
            theme_changed |= self.apply_command_palette_action(action);
        }

        if self.toast_preview_open {
            let stack_response = ToastStack::new("gallery_toast_stack", &self.toast_preview_toasts)
                .placement(ToastPlacement::TopRight)
                .width(340.0)
                .show(&ctx);

            if let Some(stack_response) = stack_response {
                for index in stack_response.inner.dismissed_indices.iter().rev() {
                    self.toast_preview_toasts.remove(*index);
                }
                if self.toast_preview_toasts.is_empty() {
                    self.toast_preview_open = false;
                }
            }
        }

        if theme_changed {
            self.apply_theme(&ctx);
            ctx.set_zoom_factor(self.zoom);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn show_workspace_view(
    ui: &mut egui::Ui,
    section: usize,
    theme: &CastTheme,
    seed: &mut ThemeSeed,
    pixels_per_point: f32,
    zoom: f32,
    command: &mut String,
    search: &mut String,
    name: &mut String,
    handle: &mut String,
    preset_query: &mut String,
    preset_choice: &mut usize,
    form_validation_attention: &mut bool,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
    form_density: &mut usize,
    menu_choice: &mut usize,
    dialog_open: &mut bool,
    sheet_open: &mut bool,
    confirm_dialog_open: &mut bool,
    confirm_result: &mut Option<ConfirmDialogResponse>,
    toast_preview_open: &mut bool,
    toast_preview_toasts: &mut Vec<Toast>,
    command_palette: &mut CommandPaletteState,
    lead_search: &mut String,
    related_activity_open: &mut bool,
    related_activity_group: &mut Option<usize>,
    lead_selected: &mut [bool; LEAD_COUNT],
    lead_expanded: &mut [bool; LEAD_COUNT],
    lead_date_filter: &mut usize,
    lead_user_filter: &mut usize,
    lead_status_filter: &mut usize,
    lead_payment_filter: &mut usize,
    lead_rows_per_page: &mut usize,
    lead_page: &mut usize,
    lead_exported_count: &mut Option<usize>,
    foundation_tab: &mut usize,
    workflow_segment: &mut usize,
    component_tab: &mut usize,
    agent_model: &mut usize,
    agent_loading: &mut bool,
    agent_tool_open: &mut bool,
    agent_retry_budget: &mut f64,
    agent_due_date: &mut String,
    agent_due_time: &mut String,
    editable_task: &mut String,
    editable_status: &mut usize,
) -> bool {
    let mut theme_changed = false;
    match section {
        0 => {
            workspace_header(
                ui,
                "Agent workspace",
                "A composed Cast surface with navigation, forms, status, and feedback.",
                Intent::Primary,
            );
            ui.add_space(12.0);
            show_workbench_preview(ui, theme, command, workflow_segment);
            ui.add_space(12.0);
            show_navigation_layout(ui, foundation_tab, workflow_segment);
            ui.add_space(12.0);
            show_forms(
                ui,
                search,
                name,
                command,
                handle,
                preset_query,
                preset_choice,
                form_validation_attention,
                enabled,
                notifications,
                indeterminate,
                form_density,
            );
        }
        1 => {
            workspace_header(
                ui,
                "Foundations",
                "Runtime theme switching, live palette editing, semantic tokens, and typography.",
                Intent::Info,
            );
            ui.add_space(12.0);
            show_theme_foundation(ui);
            ui.add_space(12.0);
            show_palette_preview(ui, theme);
            ui.add_space(12.0);
            show_typography_gallery(ui, theme);
            ui.add_space(12.0);
            show_typography_diagnostics(ui, theme, pixels_per_point, zoom);
        }
        2 => {
            workspace_header(
                ui,
                "Components",
                "Current primitives with states, variants, forms, and baseline egui comparisons.",
                Intent::Secondary,
            );
            ui.add_space(12.0);
            show_component_gallery(
                ui,
                component_tab,
                search,
                name,
                command,
                handle,
                preset_query,
                preset_choice,
                form_validation_attention,
                enabled,
                notifications,
                indeterminate,
                form_density,
                menu_choice,
                dialog_open,
                sheet_open,
                confirm_dialog_open,
                confirm_result,
                command_palette,
                lead_search,
                related_activity_open,
                related_activity_group,
                lead_selected,
                lead_expanded,
                lead_date_filter,
                lead_user_filter,
                lead_status_filter,
                lead_payment_filter,
                lead_rows_per_page,
                lead_page,
                lead_exported_count,
                toast_preview_open,
                toast_preview_toasts,
            );
        }
        3 => {
            workspace_header(
                ui,
                "Agent components",
                "Composer, transcript, tool-call, and workflow primitives for agentic desktop surfaces.",
                Intent::Primary,
            );
            ui.add_space(12.0);
            show_agent_components(
                ui,
                command,
                agent_model,
                agent_loading,
                agent_tool_open,
                agent_retry_budget,
                agent_due_date,
                agent_due_time,
                editable_task,
                editable_status,
            );
        }
        _ => {
            workspace_header(
                ui,
                "Theme lab",
                "A focused view for token derivation, live overrides, type diagnostics, and palette checks.",
                Intent::Success,
            );
            ui.add_space(12.0);
            Card::new().show(ui, |ui| {
                if show_theme_editor(ui, seed) {
                    theme_changed = true;
                }
            });
            ui.add_space(12.0);
            show_palette_preview(ui, theme);
            ui.add_space(12.0);
            show_typography_diagnostics(ui, theme, pixels_per_point, zoom);
            ui.add_space(12.0);
            show_override_preview(ui);
        }
    };
    theme_changed
}

fn workspace_header(ui: &mut egui::Ui, title: &str, subtitle: &str, intent: Intent) {
    let theme = cast::theme_for_ui(ui);
    ui.horizontal_wrapped(|ui| {
        ui.heading(RichText::new(title).size(24.0));
        ui.add(Badge::new("Live").intent(intent));
    });
    ui.label(
        RichText::new(subtitle)
            .font(theme.typography.body.clone())
            .color(theme.colors.text_muted)
            .extra_letter_spacing(theme.typography.letter_spacing),
    );
}

fn show_workbench_preview(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    command: &mut String,
    workflow_segment: &mut usize,
) {
    show_responsive_pair(
        ui,
        |ui| show_workbench_agent_thread(ui, command),
        |ui| show_workbench_interface_state(ui, theme, workflow_segment),
    );
}

fn show_workbench_agent_thread(ui: &mut egui::Ui, command: &mut String) {
    CastPanel::new().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.heading("Agent thread");
            ui.add(Badge::new("Ready").intent(Intent::Success).status_dot());
        });
        ui.add_space(8.0);
        ui.add(
            ChatMessage::user("Refine the component gallery into an app-like workspace.")
                .metadata("Just now")
                .width(ui.available_width()),
        );
        ui.add_space(8.0);
        ui.add(
            ChatMessage::assistant(
                "I will inspect the current surface, update the reusable widgets, and keep the gallery as the visual checkpoint.",
            )
            .metadata("Planning")
            .width(ui.available_width()),
        );
        ui.add_space(8.0);
        ui.add(
            ToolCall::new("cargo test -p cast-ui")
                .status(ToolCallStatus::Succeeded)
                .metadata("1.2s")
                .body("179 tests passed")
                .width(ui.available_width()),
        );
        ui.add_space(8.0);
        AgentComposer::new(command)
            .placeholder("Ask Cast to adjust this surface...")
            .send_label("Run")
            .secondary_label("Attach")
            .rows(2)
            .width(ui.available_width())
            .show(ui);
        ui.add(Separator::new().spacing(12.0));
        activity_row(ui, "01", "Inspect theme tokens", "Done", Intent::Success);
        activity_row(
            ui,
            "02",
            "Tune input and navigation states",
            "Active",
            Intent::Info,
        );
        activity_row(
            ui,
            "03",
            "Review widget-specific feedback",
            "Next",
            Intent::Neutral,
        );
    });
}

fn show_workbench_interface_state(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    workflow_segment: &mut usize,
) {
    CastPanel::new().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.heading("Interface state");
            ui.add(SegmentedControl::new(
                workflow_segment,
                ["Design", "Build", "Ship"],
            ));
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            metric_tile(ui, "Accent", "Primary", theme.colors.primary_family.base);
            metric_tile(
                ui,
                "Radius",
                format!("{:.0}px", theme.radius.md),
                theme.colors.border,
            );
            metric_tile(
                ui,
                "Type",
                format!("{:.0}px", theme.typography.body.size),
                theme.colors.secondary_family.base,
            );
        });
        ui.add(Separator::new().spacing(12.0));
        ui.add(
            Alert::new("Theme-safe by default")
                .body("The preview is using the same runtime tokens exposed in the editor.")
                .intent(Intent::Info),
        );
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("Accessible").intent(Intent::Success));
            ui.add(Badge::new("Immediate mode").intent(Intent::Secondary));
            ui.add(Badge::new("Runtime theme").intent(Intent::Primary));
        });
    });
}

fn show_responsive_pair<L, R>(ui: &mut egui::Ui, left: L, right: R)
where
    L: FnOnce(&mut egui::Ui),
    R: FnOnce(&mut egui::Ui),
{
    let theme = cast::theme_for_ui(ui);
    let available = ui.available_width();
    let gap = theme.spacing.md;

    if available < 720.0 {
        left(ui);
        ui.add_space(gap);
        right(ui);
        return;
    }

    let column_width = ((available - gap) / 2.0).max(260.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.sm;
            ui.set_width(column_width);
            ui.set_max_width(column_width);
            left(ui);
        });
        ui.add_space(gap);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.sm;
            ui.set_width(column_width);
            ui.set_max_width(column_width);
            right(ui);
        });
    });
}

fn activity_row(ui: &mut egui::Ui, number: &str, label: &str, status: &str, intent: Intent) {
    let theme = cast::theme_for_ui(ui);
    ui.horizontal_wrapped(|ui| {
        ui.add_sized(
            [28.0, 22.0],
            egui::Label::new(
                RichText::new(number)
                    .font(theme.typography.caption.clone())
                    .color(theme.colors.text_subtle)
                    .extra_letter_spacing(theme.typography.letter_spacing),
            ),
        );
        ui.label(
            RichText::new(label)
                .font(theme.typography.body.clone())
                .color(theme.colors.text)
                .extra_letter_spacing(theme.typography.letter_spacing),
        );
        ui.add(Badge::new(status).intent(intent).size(Size::Small));
    });
}

fn metric_tile(ui: &mut egui::Ui, label: &str, value: impl Into<String>, color: Color32) {
    let theme = cast::theme_for_ui(ui);
    let width = 96.0;
    let height = 58.0;
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        ui.painter().rect(
            rect,
            egui::CornerRadius::same(theme.radius.md as u8),
            theme.colors.surface,
            egui::Stroke::new(theme.stroke.sm, theme.colors.border),
            egui::StrokeKind::Outside,
        );
        let swatch = egui::Rect::from_min_size(
            rect.min + egui::vec2(theme.spacing.sm, theme.spacing.sm),
            egui::vec2(10.0, 10.0),
        );
        ui.painter()
            .rect_filled(swatch, egui::CornerRadius::same(2), color);
        ui.painter().text(
            rect.min + egui::vec2(theme.spacing.sm + 16.0, theme.spacing.xs + 2.0),
            egui::Align2::LEFT_TOP,
            label,
            theme.typography.caption.clone(),
            theme.colors.text_subtle,
        );
        ui.painter().text(
            rect.min + egui::vec2(theme.spacing.sm, 30.0),
            egui::Align2::LEFT_TOP,
            value.into(),
            theme.typography.strong.clone(),
            theme.colors.text,
        );
    }
}

fn show_theme_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    ui.heading("Theme");

    egui::CollapsingHeader::new("Palette")
        .default_open(true)
        .show(ui, |ui| changed |= show_palette_editor(ui, seed));
    egui::CollapsingHeader::new("Tokens")
        .default_open(true)
        .show(ui, |ui| changed |= show_token_editor(ui, seed));
    egui::CollapsingHeader::new("Typography")
        .default_open(false)
        .show(ui, |ui| changed |= show_typography_editor(ui, seed));
    egui::CollapsingHeader::new("Motion")
        .default_open(false)
        .show(ui, |ui| changed |= show_motion_editor(ui, seed));
    egui::CollapsingHeader::new("Presets")
        .default_open(false)
        .show(ui, |ui| changed |= show_preset_editor(ui, seed));
    egui::CollapsingHeader::new("Overrides")
        .default_open(false)
        .show(ui, |ui| changed |= show_override_editor(ui, seed));

    ui.horizontal(|ui| {
        if ui.button("Reset").clicked() {
            *seed = ThemeSeed::for_mode(seed.mode).with_typography(TypographyTokens::cast());
            changed = true;
        }
        if ui.button("Primary only").clicked() {
            seed.palette = CastPaletteInput::from_primary(seed.palette.primary);
            changed = true;
        }
    });

    changed
}

fn show_palette_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    changed |= color_row(ui, "Primary", &mut seed.palette.primary);
    changed |= optional_color_row(
        ui,
        "Secondary",
        &mut seed.palette.secondary,
        CastPaletteInput::default_for(seed.mode)
            .secondary
            .unwrap_or(Color32::from_rgb(124, 58, 237)),
    );
    changed |= optional_color_row(
        ui,
        "Neutral",
        &mut seed.palette.neutral,
        CastPaletteInput::default_for(seed.mode)
            .neutral
            .unwrap_or(Color32::from_rgb(100, 116, 139)),
    );
    changed |= optional_color_row(
        ui,
        "Success",
        &mut seed.palette.success,
        CastPaletteInput::default_for(seed.mode)
            .success
            .unwrap_or(Color32::from_rgb(22, 163, 74)),
    );
    changed |= optional_color_row(
        ui,
        "Warning",
        &mut seed.palette.warning,
        CastPaletteInput::default_for(seed.mode)
            .warning
            .unwrap_or(Color32::from_rgb(217, 119, 6)),
    );
    changed |= optional_color_row(
        ui,
        "Danger",
        &mut seed.palette.danger,
        CastPaletteInput::default_for(seed.mode)
            .danger
            .unwrap_or(Color32::from_rgb(220, 38, 38)),
    );
    changed |= optional_color_row(
        ui,
        "Info",
        &mut seed.palette.info,
        CastPaletteInput::default_for(seed.mode)
            .info
            .unwrap_or(Color32::from_rgb(8, 145, 178)),
    );
    changed
}

fn show_token_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    let spacing_changed = theme_slider(ui, "Spacing", &mut seed.spacing.md, 8.0..=20.0);
    changed |= spacing_changed;
    if spacing_changed {
        seed.spacing.xs = seed.spacing.md / 3.0;
        seed.spacing.sm = seed.spacing.md * 2.0 / 3.0;
        seed.spacing.lg = seed.spacing.md * 4.0 / 3.0;
        seed.spacing.xl = seed.spacing.md * 2.0;
    }
    let radius_changed = theme_slider(ui, "Radius", &mut seed.radius.md, 0.0..=16.0);
    changed |= radius_changed;
    if radius_changed {
        seed.set_radius(seed.radius.md);
    }
    let stroke_changed = theme_slider(ui, "Border", &mut seed.stroke.sm, 0.0..=3.0);
    changed |= stroke_changed;
    if stroke_changed {
        seed.stroke.md = seed.stroke.sm + 0.5;
        seed.stroke.lg = seed.stroke.sm + 1.0;
    }
    let controls_changed = theme_slider(ui, "Control", &mut seed.controls.min_height, 26.0..=44.0);
    changed |= controls_changed;
    if controls_changed {
        seed.set_density(seed.controls.min_height, seed.spacing.md);
    }
    changed
}

fn show_typography_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;

    ui.horizontal_wrapped(|ui| {
        if ui.button("Cast").clicked() {
            seed.typography = TypographyTokens::cast().with_body_size(seed.typography.body.size);
            changed = true;
        }
        if ui.button("Inter").clicked() {
            seed.typography = TypographyTokens::inter().with_body_size(seed.typography.body.size);
            changed = true;
        }
        if ui.button("System").clicked() {
            seed.typography = TypographyTokens::default().with_body_size(seed.typography.body.size);
            changed = true;
        }
        if ui.button("Compact").clicked() {
            seed.typography.set_body_size(13.0);
            changed = true;
        }
        if ui.button("Comfortable").clicked() {
            seed.typography.set_body_size(14.0);
            changed = true;
        }
        if ui.button("Large").clicked() {
            seed.typography.set_body_size(16.0);
            changed = true;
        }
    });

    let mut body_size = seed.typography.body.size;
    if theme_slider(ui, "Body size", &mut body_size, 12.0..=20.0) {
        seed.typography.set_body_size(body_size);
        changed = true;
    }
    changed |= theme_slider(
        ui,
        "Letter spacing",
        &mut seed.typography.letter_spacing,
        -0.25..=0.25,
    );

    if let Some(family) = font_family_selector(ui, "Body", &seed.typography.body.family) {
        seed.typography.set_body_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Heading", &seed.typography.heading.family) {
        seed.typography.set_heading_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Controls", &seed.typography.button.family) {
        seed.typography.set_button_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Strong", &seed.typography.strong.family) {
        seed.typography.set_strong_family(family);
        changed = true;
    }
    if let Some(family) = font_family_selector(ui, "Code", &seed.typography.code.family) {
        seed.typography.set_code_family(family);
        changed = true;
    }

    ui.add_space(4.0);
    ui.monospace(cast::FontStack::google_fonts_css2_url_for_names(&[
        "Inter",
        "JetBrains Mono",
    ]));

    changed
}

fn show_motion_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    changed |= ui
        .checkbox(&mut seed.animation.enabled, "Animations")
        .changed();
    changed |= ui
        .checkbox(&mut seed.animation.reduced_motion, "Reduced motion")
        .changed();
    changed |= theme_slider(
        ui,
        "Duration",
        &mut seed.animation.duration_scale,
        0.0..=2.0,
    );
    changed |= ui
        .checkbox(&mut seed.scroll.drag_to_scroll, "Drag to scroll")
        .changed();
    changed |= theme_slider(
        ui,
        "Touchpad speed",
        &mut seed.scroll.wheel_multiplier,
        0.75..=4.0,
    );
    changed |= theme_slider(
        ui,
        "Wheel line speed",
        &mut seed.scroll.line_scroll_speed,
        20.0..=80.0,
    );
    changed
}

fn show_preset_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    ui.horizontal_wrapped(|ui| {
        if ui.button("Compact").clicked() {
            seed.set_density(28.0, 10.0);
            changed = true;
        }
        if ui.button("Comfortable").clicked() {
            seed.set_density(36.0, 14.0);
            changed = true;
        }
        if ui.button("Sharp").clicked() {
            seed.set_radius(2.0);
            changed = true;
        }
        if ui.button("Soft").clicked() {
            seed.set_radius(10.0);
            changed = true;
        }
    });
    changed
}

fn show_override_editor(ui: &mut egui::Ui, seed: &mut ThemeSeed) -> bool {
    let mut changed = false;
    changed |= optional_theme_slider(
        ui,
        "Button radius",
        &mut seed.component_overrides.button.radius,
        seed.radius.md,
        0.0..=20.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Button border",
        &mut seed.component_overrides.button.border_width,
        seed.stroke.sm,
        0.0..=4.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Badge radius",
        &mut seed.component_overrides.badge.radius,
        seed.radius.md * 2.0,
        0.0..=28.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Badge height",
        &mut seed.component_overrides.badge.min_height,
        seed.controls.min_height - 6.0,
        16.0..=44.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Input radius",
        &mut seed.component_overrides.input.radius,
        seed.radius.md,
        0.0..=20.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Input height",
        &mut seed.component_overrides.input.min_height,
        seed.controls.min_height,
        24.0..=52.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Card padding",
        &mut seed.component_overrides.card.padding,
        seed.spacing.lg,
        8.0..=32.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Card radius",
        &mut seed.component_overrides.card.radius,
        seed.radius.lg,
        0.0..=24.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Panel padding",
        &mut seed.component_overrides.panel.padding,
        seed.spacing.lg,
        8.0..=32.0,
    );
    changed |= optional_theme_slider(
        ui,
        "Alert padding",
        &mut seed.component_overrides.alert.padding,
        seed.spacing.md,
        8.0..=32.0,
    );
    if !seed.component_overrides.is_empty() && ui.button("Clear overrides").clicked() {
        seed.component_overrides = Default::default();
        changed = true;
    }
    changed
}

fn theme_slider(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) -> bool {
    ui.add(Slider::new(value, range).text(label)).changed()
}

fn optional_theme_slider(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<f32>,
    fallback: f32,
    range: std::ops::RangeInclusive<f32>,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let mut enabled = value.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *value = enabled.then_some(fallback);
            changed = true;
        }

        if let Some(value) = value {
            changed |= ui
                .add(Slider::new(value, range).show_value(false).width(148.0))
                .changed();
        }
    });

    changed
}

fn font_family_selector(
    ui: &mut egui::Ui,
    label: &str,
    current: &egui::FontFamily,
) -> Option<egui::FontFamily> {
    let mut selected = current.clone();
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(format!("font_family_{label}"))
            .selected_text(font_family_label(&selected))
            .show_ui(ui, |ui| {
                font_family_option(
                    ui,
                    &mut selected,
                    egui::FontFamily::Proportional,
                    "System UI",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    egui::FontFamily::Monospace,
                    "System Mono",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::INTER_BODY_FAMILY),
                    "Inter Regular",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::INTER_BUTTON_FAMILY),
                    "Inter Medium",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::INTER_STRONG_FAMILY),
                    "Inter SemiBold",
                );
                font_family_option(
                    ui,
                    &mut selected,
                    inter_family(cast::FontStack::JETBRAINS_MONO_FAMILY),
                    "JetBrains Mono",
                );
            });
    });

    (selected != *current).then_some(selected)
}

fn font_family_option(
    ui: &mut egui::Ui,
    selected: &mut egui::FontFamily,
    family: egui::FontFamily,
    label: &str,
) {
    ui.selectable_value(selected, family, label);
}

fn inter_family(name: &'static str) -> egui::FontFamily {
    egui::FontFamily::Name(Arc::from(name))
}

fn font_family_label(family: &egui::FontFamily) -> String {
    match family {
        egui::FontFamily::Proportional => "System UI".to_owned(),
        egui::FontFamily::Monospace => "System Mono".to_owned(),
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::INTER_BODY_FAMILY => {
            "Inter Regular".to_owned()
        }
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::INTER_BUTTON_FAMILY => {
            "Inter Medium".to_owned()
        }
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::INTER_STRONG_FAMILY => {
            "Inter SemiBold".to_owned()
        }
        egui::FontFamily::Name(name) if name.as_ref() == cast::FontStack::JETBRAINS_MONO_FAMILY => {
            "JetBrains Mono".to_owned()
        }
        egui::FontFamily::Name(name) => name.to_string(),
    }
}

fn color_row(ui: &mut egui::Ui, label: &str, color: &mut Color32) -> bool {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.color_edit_button_srgba(color).changed()
    })
    .inner
}

fn optional_color_row(
    ui: &mut egui::Ui,
    label: &str,
    color: &mut Option<Color32>,
    fallback: Color32,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        let mut enabled = color.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            *color = enabled.then_some(fallback);
            changed = true;
        }

        if let Some(color) = color {
            changed |= ui.color_edit_button_srgba(color).changed();
        }
    });

    changed
}

fn show_theme_foundation(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Theme boundary");
        ui.label("Cast widgets resolve semantic styling from CastTheme.");
        ui.label("Raw egui widgets inherit the derived egui::Style fallback.");
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("CastTheme").intent(Intent::Primary));
            ui.add(Badge::new("egui::Style fallback").intent(Intent::Neutral));
            ui.add(Badge::new("runtime switching").intent(Intent::Info));
            ui.add(Badge::new("secondary").intent(Intent::Secondary));
        });
    });
}

fn show_palette_preview(ui: &mut egui::Ui, theme: &CastTheme) {
    Card::new().show(ui, |ui| {
        ui.heading("Derived palette");
        palette_family_row(ui, "Neutral", theme.colors.neutral_family);
        palette_family_row(ui, "Primary", theme.colors.primary_family);
        palette_family_row(ui, "Secondary", theme.colors.secondary_family);
        palette_family_row(ui, "Success", theme.colors.success_family);
        palette_family_row(ui, "Warning", theme.colors.warning_family);
        palette_family_row(ui, "Danger", theme.colors.danger_family);
        palette_family_row(ui, "Info", theme.colors.info_family);
    });
}

fn show_typography_gallery(ui: &mut egui::Ui, theme: &CastTheme) {
    Card::new().show(ui, |ui| {
        ui.heading("Typography");
        typography_sample(
            ui,
            "Heading large",
            "Build dense Rust interfaces with readable text.",
            theme.typography.heading_lg.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Heading",
            "Themeable components for immediate-mode apps.",
            theme.typography.heading.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Heading small",
            "Forms, surfaces, feedback, and navigation.",
            theme.typography.heading_sm.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Body",
            "Cast uses role-based font tokens so apps can choose separate faces for body text, headings, controls, and code.",
            theme.typography.body.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Body strong",
            "Important text can use a stronger face without changing size.",
            theme.typography.body_strong.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Small",
            "Secondary details should remain legible at small sizes.",
            theme.typography.small.clone(),
            theme.colors.text_muted,
        );
        typography_sample(
            ui,
            "Caption",
            "Caption text, metadata, and dense row annotations.",
            theme.typography.caption.clone(),
            theme.colors.text_subtle,
        );
        typography_sample(
            ui,
            "Code",
            "let theme = ThemeSeed::for_mode(mode).with_typography(TypographyTokens::inter());",
            theme.typography.code.clone(),
            theme.colors.text,
        );
    });
}

fn show_typography_diagnostics(
    ui: &mut egui::Ui,
    theme: &CastTheme,
    pixels_per_point: f32,
    zoom: f32,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Text diagnostics");
        egui::Grid::new("typography_diagnostics_grid")
            .num_columns(2)
            .spacing(egui::vec2(12.0, 4.0))
            .show(ui, |ui| {
                diagnostic_row(ui, "Zoom", format!("{zoom:.2}"));
                diagnostic_row(ui, "Pixels/point", format!("{pixels_per_point:.2}"));
                diagnostic_row(
                    ui,
                    "Body size",
                    format!("{:.1}", theme.typography.body.size),
                );
                diagnostic_row(
                    ui,
                    "Caption size",
                    format!("{:.1}", theme.typography.caption.size),
                );
                diagnostic_row(
                    ui,
                    "Code size",
                    format!("{:.1}", theme.typography.code.size),
                );
                diagnostic_row(
                    ui,
                    "Letter spacing",
                    format!("{:.2}", theme.typography.letter_spacing),
                );
                diagnostic_row(
                    ui,
                    "Touchpad speed",
                    format!("{:.2}", theme.scroll.wheel_multiplier),
                );
                diagnostic_row(
                    ui,
                    "Wheel line speed",
                    format!("{:.1}", theme.scroll.line_scroll_speed),
                );
                diagnostic_row(
                    ui,
                    "Body family",
                    font_family_label(&theme.typography.body.family),
                );
                diagnostic_row(
                    ui,
                    "Code family",
                    font_family_label(&theme.typography.code.family),
                );
            });

        ui.add(Separator::new().spacing(10.0));
        typography_sample(
            ui,
            "Small",
            "Small text: abcdefghijklmnopqrstuvwxyz 0123456789",
            theme.typography.small.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Tracking",
            "Tighter letter spacing is a typography token, not a renderer fix.",
            theme.typography.body.clone(),
            theme.colors.text,
        );
        typography_sample(
            ui,
            "Caption",
            "Dense metadata: 2026-05-28 14:32:08 / queued / retry 02",
            theme.typography.caption.clone(),
            theme.colors.text_muted,
        );
        typography_sample(
            ui,
            "Mono",
            "fn main() { println!(\"Cast\"); }",
            theme.typography.code.clone(),
            theme.colors.text,
        );

        ui.add_space(6.0);
        for index in 0..3 {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [28.0, 18.0],
                    egui::Label::new(
                        RichText::new(format!("{:02}", index + 1))
                            .font(theme.typography.caption.clone())
                            .color(theme.colors.text_subtle)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    ),
                );
                ui.label(
                    RichText::new("Render row with mixed weight, muted text, and stable spacing.")
                        .font(theme.typography.body.clone())
                        .color(theme.colors.text)
                        .extra_letter_spacing(theme.typography.letter_spacing),
                );
            });
        }
    });
}

fn diagnostic_row(ui: &mut egui::Ui, label: &str, value: impl Into<String>) {
    let letter_spacing = theme_letter_spacing(ui);
    ui.label(
        RichText::new(label)
            .size(11.0)
            .extra_letter_spacing(letter_spacing),
    );
    ui.label(value.into());
    ui.end_row();
}

fn typography_sample(
    ui: &mut egui::Ui,
    label: &str,
    text: &str,
    font: egui::FontId,
    color: Color32,
) {
    let letter_spacing = theme_letter_spacing(ui);
    ui.horizontal_wrapped(|ui| {
        ui.add_sized(
            [92.0, 18.0],
            egui::Label::new(
                RichText::new(label)
                    .size(11.0)
                    .extra_letter_spacing(letter_spacing),
            ),
        );
        ui.label(
            RichText::new(text)
                .font(font)
                .color(color)
                .extra_letter_spacing(letter_spacing),
        );
    });
}

fn theme_letter_spacing(ui: &egui::Ui) -> f32 {
    cast::theme_for_ui(ui).typography.letter_spacing
}

fn palette_family_row(ui: &mut egui::Ui, label: &str, family: SemanticColorTokens) {
    ui.horizontal(|ui| {
        ui.label(label);
        color_swatch(ui, family.base, "base");
        color_swatch(ui, family.fg, "foreground");
        color_swatch(ui, family.subtle, "subtle");
        color_swatch(ui, family.muted, "muted");
        color_swatch(ui, family.emphasis, "emphasis");
        color_swatch(ui, family.border, "border");
        color_swatch(ui, family.hover, "hover");
        color_swatch(ui, family.active, "active");
    });
}

fn color_swatch(ui: &mut egui::Ui, color: Color32, label: &str) {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(24.0, 18.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 3, color);
    response.on_hover_text(label);
}

fn show_navigation_layout(
    ui: &mut egui::Ui,
    foundation_tab: &mut usize,
    workflow_segment: &mut usize,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Navigation and layout");
        ui.add(Tabs::new(
            foundation_tab,
            ["Overview", "Components", "Theme", "Diagnostics"],
        ));
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(SegmentedControl::new(
                workflow_segment,
                ["Design", "Build", "Ship"],
            ));
            ui.add(Badge::new(match *workflow_segment {
                0 => "Design review",
                1 => "Implementation",
                _ => "Release check",
            }));
        });
        ui.add_space(10.0);
        CastPanel::new().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    Badge::new(match *foundation_tab {
                        0 => "Overview",
                        1 => "Components",
                        2 => "Theme",
                        _ => "Diagnostics",
                    })
                    .intent(Intent::Info),
                );
                ui.label(match *foundation_tab {
                    0 => "A compact route bar for switching between app sections.",
                    1 => "Tabs and segmented controls share sizing, tracking, and colors.",
                    2 => "Navigation follows runtime theme changes without extra setup.",
                    _ => "Focused state and hover affordances are painted by Cast.",
                });
            });
        });
    });
}

#[allow(clippy::too_many_arguments)]
fn show_component_gallery(
    ui: &mut egui::Ui,
    component_tab: &mut usize,
    search: &mut String,
    name: &mut String,
    command: &mut String,
    handle: &mut String,
    preset_query: &mut String,
    preset_choice: &mut usize,
    form_validation_attention: &mut bool,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
    form_density: &mut usize,
    menu_choice: &mut usize,
    dialog_open: &mut bool,
    sheet_open: &mut bool,
    confirm_dialog_open: &mut bool,
    confirm_result: &mut Option<ConfirmDialogResponse>,
    command_palette: &mut CommandPaletteState,
    lead_search: &mut String,
    related_activity_open: &mut bool,
    related_activity_group: &mut Option<usize>,
    lead_selected: &mut [bool; LEAD_COUNT],
    lead_expanded: &mut [bool; LEAD_COUNT],
    lead_date_filter: &mut usize,
    lead_user_filter: &mut usize,
    lead_status_filter: &mut usize,
    lead_payment_filter: &mut usize,
    lead_rows_per_page: &mut usize,
    lead_page: &mut usize,
    lead_exported_count: &mut Option<usize>,
    toast_preview_open: &mut bool,
    toast_preview_toasts: &mut Vec<Toast>,
) {
    Card::new().show(ui, |ui| {
        ui.add(Tabs::new(
            component_tab,
            ["Core", "Inputs", "Menus", "Data", "Feedback", "Surfaces"],
        ));
    });
    ui.add_space(12.0);

    match *component_tab {
        0 => {
            show_override_preview(ui);
            ui.add_space(12.0);
            show_buttons_and_badges(ui);
            ui.add_space(12.0);
            let mut nav_tab = 0;
            let mut nav_segment = 0;
            show_navigation_layout(ui, &mut nav_tab, &mut nav_segment);
        }
        1 => show_forms(
            ui,
            search,
            name,
            command,
            handle,
            preset_query,
            preset_choice,
            form_validation_attention,
            enabled,
            notifications,
            indeterminate,
            form_density,
        ),
        2 => show_menus(
            ui,
            menu_choice,
            dialog_open,
            sheet_open,
            confirm_dialog_open,
            confirm_result,
            command_palette,
        ),
        3 => show_lists_and_tables(
            ui,
            lead_search,
            related_activity_open,
            related_activity_group,
            lead_selected,
            lead_expanded,
            lead_date_filter,
            lead_user_filter,
            lead_status_filter,
            lead_payment_filter,
            lead_rows_per_page,
            lead_page,
            lead_exported_count,
        ),
        4 => show_text_and_feedback(ui, toast_preview_open, toast_preview_toasts),
        _ => {
            show_surfaces(ui);
            ui.add_space(12.0);
            show_raw_egui_controls(ui, search, enabled);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn show_agent_components(
    ui: &mut egui::Ui,
    command: &mut String,
    agent_model: &mut usize,
    agent_loading: &mut bool,
    agent_tool_open: &mut bool,
    agent_retry_budget: &mut f64,
    agent_due_date: &mut String,
    agent_due_time: &mut String,
    editable_task: &mut String,
    editable_status: &mut usize,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Conversation primitives");
        ui.add_space(8.0);
        show_responsive_pair(
            ui,
            show_agent_transcript_examples,
            show_agent_tool_call_examples,
        );
    });

    ui.add_space(12.0);
    Card::new().show(ui, |ui| {
        ui.heading("Composer");
        ui.label("A framed multiline prompt box with attachments, tools, model choice, loading state, and Enter-to-send behavior.");
        ui.add_space(8.0);
        let composer = AgentComposer::new(command)
            .placeholder("Ask the agent to inspect, patch, or explain...")
            .send_label("Run")
            .stop_label("Stop")
            .attachment_label("Attach")
            .tool_label("Tools")
            .model_selector(agent_model, ["Swift", "Balanced", "Deep review"])
            .loading(*agent_loading)
            .rows(4)
            .width(ui.available_width())
            .show(ui);
        if composer.inner.submitted {
            *agent_loading = true;
        }
        if composer.inner.stopped {
            *agent_loading = false;
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            NumberInput::new(agent_retry_budget)
                .label("Retries")
                .range(0.0, 12.0)
                .width(92.0)
                .size(Size::Small)
                .show(ui);
            ui.add(DateInput::new(agent_due_date).label("Due date"));
            ui.add(TimeInput::new(agent_due_time).label("Due time"));
        });
    });

    ui.add_space(12.0);
    Card::new().show(ui, |ui| {
        ui.heading("Workflow blocks");
        ui.label("Collapsible calls, timelines, and output regions for agent execution state.");
        ui.add_space(8.0);
        show_responsive_pair(
            ui,
            |ui| {
                ui.add(
                    ToolCallBlock::new("cargo test -p cast-ui", agent_tool_open)
                        .status(ToolCallStatus::Running)
                        .arguments("package: cast-ui, profile: test")
                        .elapsed("14.2s")
                        .preview("running 191 tests\ncomponents::agent::tests::workflow_components_store_state ... ok\ncomponents::text_input::tests::number_input_stores_typed_constraints ... ok")
                        .width(ui.available_width()),
                );
                ui.add_space(8.0);
                ui.add(
                    RunTimeline::new()
                        .item(
                            RunTimelineItem::new(RunPhase::Planning, "Plan component API")
                                .detail("Map Turin agent states to Cast primitives")
                                .metadata("done"),
                        )
                        .item(
                            RunTimelineItem::new(RunPhase::ToolCall, "Inspect current widgets")
                                .status(ToolCallStatus::Succeeded)
                                .metadata("120ms"),
                        )
                        .item(
                            RunTimelineItem::new(RunPhase::Patch, "Add workflow components")
                                .status(ToolCallStatus::Succeeded)
                                .metadata("done"),
                        )
                        .item(
                            RunTimelineItem::new(RunPhase::Test, "Run focused tests")
                                .status(ToolCallStatus::Running)
                                .metadata("active"),
                        )
                        .width(ui.available_width()),
                );
            },
            |ui| {
                ui.add(
                    CodeOutputPanel::new(
                        "Shell output",
                        "cargo test -p cast-ui\n\nrunning 191 tests\nagent workflow primitives ... ok\ntext input typed wrappers ... ok\n\nresult: ok",
                    )
                    .kind(ToolOutputKind::Log)
                    .metadata("stdout")
                    .height(172.0)
                    .width(ui.available_width()),
                );
                ui.add_space(8.0);
                ArtifactCard::new("agent-workflow-primitives.md")
                    .kind("Report")
                    .description("Generated review notes for composer, timeline, tool calls, output panels, and approvals.")
                    .metadata("Markdown")
                    .intent(Intent::Info)
                    .width(ui.available_width())
                    .show(ui);
            },
        );
    });

    ui.add_space(12.0);
    Card::new().show(ui, |ui| {
        ui.heading("Review and editable output");
        ui.label("Approval surfaces and rich table cells for agent-produced settings or structured output.");
        ui.add_space(8.0);
        show_responsive_pair(
            ui,
            |ui| {
                ApprovalPanel::new(
                    "Approve patch",
                    "Applies changes to Cast UI agent components and gallery examples.",
                )
                .risk("Touches reusable component APIs, so downstream callers should adopt the new names deliberately.")
                .primary_label("Approve patch")
                .secondary_label("Hold")
                .intent(Intent::Warning)
                .width(ui.available_width())
                .show(ui);
            },
            |ui| {
                Table::new(["Task", "Status", "Owner"])
                    .column_weights([2.0, 1.0, 1.0])
                    .min_column_width(96.0)
                    .show(ui, 2, |row, index| {
                        if index == 0 {
                            row.editable_text(editable_task);
                            row.select(editable_status, ["Queued", "In progress", "Done"]);
                            row.text("Agent");
                        } else {
                            row.text("Contrast audit");
                            row.text("Pending");
                            row.text("Design");
                        }
                    });
            },
        );
    });

    ui.add_space(12.0);
    Card::new().show(ui, |ui| {
        ui.heading("Legacy output blocks");
        ui.label("The lower-level ToolOutput widget is still useful for compact inline previews.");
        ui.add_space(8.0);
        show_responsive_pair(
            ui,
            |ui| {
                ui.add(
                    ToolOutput::new(
                        "Structured result",
                        "{ \"tests\": \"passed\", \"components\": [\"MessageThread\", \"ToolCallBlock\", \"CodeOutputPanel\"] }",
                    )
                    .kind(ToolOutputKind::Json)
                    .metadata("result.json")
                    .width(ui.available_width()),
                );
            },
            |ui| {
                ui.add(
                    ToolOutput::new(
                        "Generated snippet",
                        "AgentComposer::new(&mut prompt)\n    .attachment_label(\"Attach\")\n    .tool_label(\"Tools\")\n    .model_selector(&mut model, [\"Swift\", \"Deep review\"])\n    .show(ui);",
                    )
                    .kind(ToolOutputKind::Code)
                    .metadata("rust")
                    .width(ui.available_width()),
                );
            },
        );
    });
}

fn show_agent_transcript_examples(ui: &mut egui::Ui) {
    CastPanel::new().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("Transcript").intent(Intent::Primary).status_dot());
            ui.add(Badge::new("Streaming-ready").intent(Intent::Info));
        });
        ui.add_space(8.0);
        MessageThread::new()
            .width(ui.available_width())
            .show(ui, |thread| {
                thread.message(
                    ChatMessage::system("Use compact, theme-aware surfaces for agent state.")
                        .metadata("Policy"),
                );
                thread.message(
                    ChatMessage::user("Compare the table states and propose the next polish pass.")
                        .metadata("You"),
                );
                thread.rich_message(
                    ChatMessage::assistant(
                        "I will review selection, hover, dark-mode contrast, and the expandable-row pattern before changing code.",
                    )
                    .metadata("Assistant")
                    .streaming(true),
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.add(Badge::new("Table").intent(Intent::Info).status_dot());
                            ui.add(Badge::new("Dark mode").intent(Intent::Secondary).status_dot());
                            ui.add(Badge::new("Selection").intent(Intent::Success).status_dot());
                        });
                    },
                );
            });
    });
}

fn show_agent_tool_call_examples(ui: &mut egui::Ui) {
    CastPanel::new().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.add(
                Badge::new("Tool calls")
                    .intent(Intent::Success)
                    .status_dot(),
            );
            ui.add(Badge::new("Composable").intent(Intent::Secondary));
        });
        ui.add_space(8.0);
        ui.add(
            ToolCall::new("rg selected_row")
                .status(ToolCallStatus::Succeeded)
                .metadata("120ms")
                .body("Found table selection and row hover helpers.")
                .width(ui.available_width()),
        );
        ui.add_space(8.0);
        ui.add(
            ToolCall::new("cargo test -p cast-ui")
                .status(ToolCallStatus::Running)
                .metadata("active")
                .body("Focused component tests are running.")
                .width(ui.available_width()),
        );
        ui.add_space(8.0);
        ui.add(
            ToolCall::new("visual snapshot")
                .status(ToolCallStatus::Queued)
                .metadata("next")
                .body("Capture gallery state after the build finishes.")
                .width(ui.available_width()),
        );
        ui.add_space(8.0);
        ui.add(
            ToolCall::new("deploy preview")
                .status(ToolCallStatus::Failed)
                .metadata("blocked")
                .body("Missing release token in the local environment.")
                .width(ui.available_width()),
        );
    });
}

fn show_buttons_and_badges(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Buttons");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Primary"));
            ui.add(Button::new("Secondary").intent(Intent::Secondary));
            ui.add(Button::new("Success").intent(Intent::Success));
            ui.add(Button::new("Warning").intent(Intent::Warning));
            ui.add(Button::new("Danger").intent(Intent::Danger));
            ui.add(
                Button::new("Outline")
                    .intent(Intent::Primary)
                    .variant(Variant::Outline),
            );
            ui.add(
                Button::new("Ghost")
                    .intent(Intent::Primary)
                    .variant(Variant::Ghost),
            );
        });

        ui.add_space(8.0);
        ui.heading("States");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("With icon").leading_icon("[+]"));
            ui.add(Button::new("Next").trailing_icon("[>]"));
            ui.add(Button::new("Saving").loading(true));
            ui.add(Button::new("Disabled").disabled());
            Tooltip::new("Tooltips inherit Cast theme colors, type, radius, and elevation.")
                .title("Tooltip")
                .show(ui, |ui| {
                    ui.add(
                        Button::new("Hover details")
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline),
                    );
                });
        });

        ui.add_space(8.0);
        ui.heading("Sizes");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Small").size(Size::Small));
            ui.add(Button::new("Medium").size(Size::Medium));
            ui.add(Button::new("Large").size(Size::Large));
        });

        ui.add_space(8.0);
        ui.heading("Badges");
        ui.horizontal_wrapped(|ui| {
            ui.add(Badge::new("Neutral"));
            ui.add(Badge::new("Primary").intent(Intent::Primary));
            ui.add(Badge::new("Secondary").intent(Intent::Secondary));
            ui.add(Badge::new("Success").intent(Intent::Success));
            ui.add(Badge::new("Warning").intent(Intent::Warning));
            ui.add(Badge::new("Danger").intent(Intent::Danger));
            ui.add(Badge::new("Info").intent(Intent::Info));
            ui.add(
                Badge::new("Outline")
                    .intent(Intent::Primary)
                    .variant(Variant::Outline),
            );
        });

        ui.add_space(8.0);
        ui.heading("Avatars");
        ui.horizontal_wrapped(|ui| {
            ui.add(Avatar::new("Sarah Parker").intent(Intent::Primary));
            ui.add(Avatar::new("Mike Brown").intent(Intent::Info));
            ui.add(Avatar::new("Linda Chen").intent(Intent::Secondary));
            ui.add(Avatar::new("A").intent(Intent::Neutral).size(Size::Small));
            ui.add(
                Avatar::new("Cast UI")
                    .intent(Intent::Success)
                    .size(Size::Large),
            );
        });
    });
}

fn show_override_preview(ui: &mut egui::Ui) {
    let mut preview_input = String::from("Input");

    Card::new().show(ui, |ui| {
        ui.heading("Live override preview");
        ui.horizontal_wrapped(|ui| {
            ui.add(Button::new("Button").leading_icon("[+]"));
            ui.add(Badge::new("Badge").intent(Intent::Info));
            ui.add(TextInput::new(&mut preview_input).width(160.0));
        });
        ui.add_space(8.0);
        CastPanel::new().show(ui, |ui| {
            ui.label("Panel padding and radius update here.");
        });
        ui.add_space(8.0);
        ui.add(Alert::new("Alert preview").body("Alert padding and radius update here."));
    });
}

fn show_surfaces(ui: &mut egui::Ui) {
    Card::new().show(ui, |ui| {
        ui.heading("Surfaces");
        ui.label("Card frames primary content. Panel frames secondary or raised content.");
        ui.add(Separator::new());
        show_context_card_example(ui);
        ui.add_space(8.0);
        CastPanel::new().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Panel").intent(Intent::Neutral));
                ui.label("A raised surface for dense app UI regions.");
            });
        });
        ui.add_space(8.0);
        CastPanel::new().show(ui, |ui| {
            EmptyState::new("No runs yet")
                .body("Start a run to populate this workspace with activity, results, and checks.")
                .icon("+")
                .intent(Intent::Primary)
                .show(ui, |ui| {
                    ui.add(Button::new("Start run").size(Size::Small));
                });
        });
    });
}

fn show_context_card_example(ui: &mut egui::Ui) {
    let theme = cast::theme_for_ui(ui);
    let outer_width = ui.available_width().min(620.0);
    let outer_fill = theme.colors.surface_muted;
    let inner_fill = theme.colors.surface;
    let border = theme.colors.border;

    ui.allocate_ui_with_layout(
        egui::vec2(outer_width, 0.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            egui::Frame::new()
                .fill(outer_fill)
                .stroke(egui::Stroke::new(theme.stroke.sm, border))
                .corner_radius(egui::CornerRadius::same((theme.radius.lg + 10.0) as u8))
                .shadow(egui::epaint::Shadow {
                    offset: [0, 8],
                    blur: 20,
                    spread: 0,
                    color: Color32::from_black_alpha(26),
                })
                .inner_margin(egui::Margin::symmetric(18, 16))
                .show(ui, |ui| {
                    ui.set_width((outer_width - 2.0).max(280.0));
                    ui.horizontal(|ui| {
                        ui.heading(
                            RichText::new("Context")
                                .font(theme.typography.heading_sm.clone())
                                .color(theme.colors.text)
                                .extra_letter_spacing(theme.typography.letter_spacing),
                        );
                        ui.add(Badge::new("860 / 200k").intent(Intent::Neutral));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            context_header_meter(ui);
                        });
                    });

                    ui.add_space(14.0);

                    egui::Frame::new()
                        .fill(inner_fill)
                        .stroke(egui::Stroke::new(theme.stroke.sm, border))
                        .corner_radius(egui::CornerRadius::same((theme.radius.lg + 4.0) as u8))
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            context_message_row(
                                ui,
                                "YOU",
                                "Build is failing on main after the React 19 upgrade.",
                                "318",
                                Intent::Neutral,
                            );
                            ui.add_space(10.0);
                            context_message_row(
                                ui,
                                "CLAUDE",
                                "Let me read the build log and trace the failing module.",
                                "542",
                                Intent::Warning,
                            );
                        });

                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("3 in window")
                                .font(theme.typography.body_strong.clone())
                                .color(theme.colors.text),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new("Auto-compacts at 6")
                                    .font(theme.typography.body_strong.clone())
                                    .color(theme.colors.text_subtle),
                            );
                        });
                    });
                });
        },
    );
}

fn context_header_meter(ui: &mut egui::Ui) {
    let theme = cast::theme_for_ui(ui);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(120.0, 28.0), egui::Sense::hover());
    let painter = ui.painter();
    let line_y = rect.center().y;
    let line_start = egui::pos2(rect.min.x + 4.0, line_y);
    let line_end = egui::pos2(rect.min.x + 78.0, line_y);
    let progress_end = egui::pos2(rect.min.x + 56.0, line_y);
    let circle_center = egui::pos2(rect.max.x - 16.0, line_y);

    painter.line_segment(
        [line_start, line_end],
        egui::Stroke::new(4.0, theme.colors.border),
    );
    painter.line_segment(
        [line_start, progress_end],
        egui::Stroke::new(4.0, theme.colors.border_strong),
    );
    painter.circle_stroke(
        circle_center,
        11.0,
        egui::Stroke::new(2.0, theme.colors.border),
    );
    painter.circle_stroke(
        circle_center,
        11.0,
        egui::Stroke::new(2.0, theme.colors.text_subtle),
    );
    painter.circle_filled(
        egui::pos2(circle_center.x - 3.5, circle_center.y - 8.5),
        2.0,
        theme.colors.text_subtle,
    );
}

fn context_message_row(
    ui: &mut egui::Ui,
    speaker: &str,
    message: &str,
    count: &str,
    intent: Intent,
) {
    let theme = cast::theme_for_ui(ui);
    let family = match intent {
        Intent::Warning => theme.colors.warning_family,
        Intent::Primary => theme.colors.primary_family,
        Intent::Secondary => theme.colors.secondary_family,
        Intent::Success => theme.colors.success_family,
        Intent::Danger => theme.colors.danger_family,
        Intent::Info => theme.colors.info_family,
        Intent::Neutral => theme.colors.neutral_family,
    };
    let row_fill = if intent == Intent::Neutral {
        theme.colors.surface
    } else {
        family.subtle
    };
    let row_border = if intent == Intent::Neutral {
        theme.colors.border
    } else {
        family.border
    };
    let row_width = ui.available_width();

    egui::Frame::new()
        .fill(row_fill)
        .stroke(egui::Stroke::new(theme.stroke.sm, row_border))
        .corner_radius(egui::CornerRadius::same(theme.radius.lg as u8))
        .inner_margin(egui::Margin::symmetric(14, 12))
        .show(ui, |ui| {
            ui.set_width(row_width);
            ui.horizontal_top(|ui| {
                context_message_icon(ui, intent);
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(speaker)
                            .font(theme.typography.body_strong.clone())
                            .color(if intent == Intent::Neutral {
                                theme.colors.text_muted
                            } else {
                                family.emphasis
                            })
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                    ui.label(
                        RichText::new(message)
                            .font(theme.typography.body.clone())
                            .color(theme.colors.text)
                            .extra_letter_spacing(theme.typography.letter_spacing),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(
                        RichText::new(count)
                            .font(theme.typography.small.clone())
                            .color(theme.colors.text_subtle),
                    );
                });
            });
        });
}

fn context_message_icon(ui: &mut egui::Ui, intent: Intent) {
    let theme = cast::theme_for_ui(ui);
    let color = match intent {
        Intent::Warning => theme.colors.warning_family.emphasis,
        Intent::Primary => theme.colors.primary_family.emphasis,
        Intent::Secondary => theme.colors.secondary_family.emphasis,
        Intent::Success => theme.colors.success_family.emphasis,
        Intent::Danger => theme.colors.danger_family.emphasis,
        Intent::Info => theme.colors.info_family.emphasis,
        Intent::Neutral => theme.colors.text_muted,
    };
    let (rect, _) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::hover());
    let painter = ui.painter();
    let center = rect.center();
    let stroke = egui::Stroke::new(1.5, color);

    if intent == Intent::Warning {
        let body = egui::Rect::from_center_size(center, egui::vec2(13.0, 9.0));
        painter.rect_filled(body, 2.0, color);
        painter.line_segment(
            [
                egui::pos2(body.min.x + 3.0, body.min.y),
                egui::pos2(body.min.x + 3.0, body.min.y - 3.0),
            ],
            stroke,
        );
        painter.line_segment(
            [
                egui::pos2(body.max.x - 3.0, body.min.y),
                egui::pos2(body.max.x - 3.0, body.min.y - 3.0),
            ],
            stroke,
        );
        painter.circle_filled(
            egui::pos2(body.min.x + 4.0, body.center().y),
            1.2,
            theme.colors.surface,
        );
        painter.circle_filled(
            egui::pos2(body.max.x - 4.0, body.center().y),
            1.2,
            theme.colors.surface,
        );
    } else {
        painter.circle_stroke(egui::pos2(center.x, center.y - 4.0), 4.0, stroke);
        painter.line_segment(
            [
                egui::pos2(center.x - 7.0, center.y + 7.0),
                egui::pos2(center.x + 7.0, center.y + 7.0),
            ],
            stroke,
        );
        painter.line_segment(
            [
                egui::pos2(center.x - 7.0, center.y + 7.0),
                egui::pos2(center.x - 4.0, center.y + 1.0),
            ],
            stroke,
        );
        painter.line_segment(
            [
                egui::pos2(center.x + 7.0, center.y + 7.0),
                egui::pos2(center.x + 4.0, center.y + 1.0),
            ],
            stroke,
        );
    }
}

fn show_menus(
    ui: &mut egui::Ui,
    menu_choice: &mut usize,
    dialog_open: &mut bool,
    sheet_open: &mut bool,
    confirm_dialog_open: &mut bool,
    confirm_result: &mut Option<ConfirmDialogResponse>,
    command_palette: &mut CommandPaletteState,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Menus and dropdowns");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                Dropdown::new(
                    menu_choice,
                    ["Overview", "Components", "Theme lab", "Diagnostics"],
                )
                .width(220.0),
            );
            ui.add(
                Dropdown::new(menu_choice, ["Small", "Medium", "Large"])
                    .size(Size::Small)
                    .width(144.0),
            );
        });
        ui.add_space(8.0);
        CastPanel::new().show(ui, |ui| {
            ui.label("Menu items");
            ui.add_space(4.0);
            if ui
                .add(MenuItem::new("Open command palette").selected(true))
                .clicked()
            {
                command_palette.open = true;
            }
            ui.add(MenuItem::new("Duplicate theme"));
            ui.add(MenuItem::new("Archive preset").intent(Intent::Warning));
            ui.add(MenuItem::new("Delete preset").intent(Intent::Danger));
            ui.add(MenuItem::new("Unavailable action").disabled());
        });
        ui.add_space(8.0);
        Popover::new()
            .title("Popover")
            .body("A richer anchored overlay for compact settings and contextual actions.")
            .width(280.0)
            .muted_sections()
            .show_with_footer(
                ui,
                |ui| {
                    ui.add(
                        Button::new("Open popover")
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline),
                    )
                },
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.add(Badge::new("Anchored").intent(Intent::Info));
                        ui.add(Badge::new("Composable").intent(Intent::Secondary));
                    });
                    ui.add_space(6.0);
                    ui.add(Button::new("Apply").size(Size::Small));
                },
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Esc closes");
                        ui.add(Badge::new("Local").intent(Intent::Neutral));
                    });
                },
            );
        ui.add_space(8.0);
        if ui
            .add(
                Button::new("Open dialog")
                    .intent(Intent::Neutral)
                    .variant(Variant::Outline),
            )
            .clicked()
        {
            *dialog_open = true;
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(
                    Button::new("Open sheet")
                        .intent(Intent::Neutral)
                        .variant(Variant::Outline),
                )
                .clicked()
            {
                *sheet_open = true;
            }

            if ui
                .add(
                    Button::new("Open confirm dialog")
                        .intent(Intent::Danger)
                        .variant(Variant::Outline),
                )
                .clicked()
            {
                *confirm_dialog_open = true;
            }

            if let Some(result) = *confirm_result {
                let (label, intent) = match result {
                    ConfirmDialogResponse::Confirmed => ("Confirmed", Intent::Success),
                    ConfirmDialogResponse::Cancelled => ("Cancelled", Intent::Neutral),
                };
                ui.add(Badge::new(label).intent(intent));
            }
        });
    });

    Sheet::new(sheet_open, "gallery_sheet")
        .title("Run settings")
        .description("A non-blocking side surface for secondary tasks and detailed controls.")
        .width(380.0)
        .muted_sections()
        .show_with_footer(
            ui.ctx(),
            |ui, _sheet| {
                ui.label("Sheets keep the workspace visible while exposing a focused panel.");
                ui.add_space(12.0);
                ui.add(Badge::new("Right edge").intent(Intent::Info));
                ui.add_space(12.0);
                ui.add(Separator::new());
                ui.add_space(12.0);
            },
            |ui, sheet| {
                if ui.add(Button::new("Done").size(Size::Small)).clicked() {
                    sheet.close();
                }
            },
        );

    Dialog::new(dialog_open, "gallery_dialog")
        .title("Dialog")
        .description("A blocking surface for focused decisions, confirmations, and short forms.")
        .width(440.0)
        .muted_sections()
        .show_with_footer(
            ui.ctx(),
            |ui, _dialog| {
            ui.label("Use dialogs when the surrounding workspace should pause until the user makes a choice.");
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.add(Badge::new("Esc closes").intent(Intent::Neutral));
                ui.add(Badge::new("Backdrop closes").intent(Intent::Info));
            });
            },
            |ui, dialog| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add(Button::new("Confirm").size(Size::Small)).clicked() {
                    dialog.close();
                }
                if ui
                    .add(
                        Button::new("Cancel")
                            .intent(Intent::Neutral)
                            .variant(Variant::Outline)
                            .size(Size::Small),
                    )
                    .clicked()
                {
                    dialog.close();
                }
            });
            },
        );

    if let Some(result) = ConfirmDialog::new(confirm_dialog_open, "gallery_confirm_dialog")
        .title("Delete generated report?")
        .description(
            "This keeps the project intact, but removes the exported report from the current run.",
        )
        .confirm_label("Delete report")
        .cancel_label("Keep report")
        .width(420.0)
        .show(ui.ctx())
    {
        *confirm_result = Some(result);
    }
}

#[allow(clippy::too_many_arguments)]
fn show_lists_and_tables(
    ui: &mut egui::Ui,
    _lead_search: &mut String,
    related_activity_open: &mut bool,
    related_activity_group: &mut Option<usize>,
    lead_selected: &mut [bool; LEAD_COUNT],
    lead_expanded: &mut [bool; LEAD_COUNT],
    _date_filter: &mut usize,
    _user_filter: &mut usize,
    _status_filter: &mut usize,
    _payment_filter: &mut usize,
    rows_per_page: &mut usize,
    page: &mut usize,
    exported_count: &mut Option<usize>,
) {
    Card::new().show(ui, |ui| {
        show_entity_table_with_details(
            ui,
            &LEADS,
            EntityTableState {
                selected: lead_selected,
                expanded: lead_expanded,
                rows_per_page,
                page,
                exported_count,
            },
        );
        ui.add_space(12.0);
        show_related_activity(ui, related_activity_open, related_activity_group);
    });
}

type LeadRecord = EntityRecord;

#[cfg(test)]
const LEAD_USER_FILTERS: [&str; 6] = [
    "All users",
    "Sarah P.",
    "Alex W.",
    "Jane D.",
    "John S.",
    "Ali M.",
];
#[cfg(test)]
const LEAD_STATUS_FILTERS: [&str; 7] = [
    "Any status",
    "Won",
    "Call booked",
    "Qualified",
    "Unqualified",
    "Lost",
    "No show",
];
#[cfg(test)]
const LEAD_PAYMENT_FILTERS: [&str; 4] = ["All payments", "Paid", "Pending", "No value"];

const LEADS: [LeadRecord; 24] = [
    LeadRecord {
        name: "Sarah Parker",
        status: "Won",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "$ 2,500.00",
        payment: "Paid",
        assigned_to: "Sarah P.",
        interacted: "2 days ago",
        days_ago: 2,
    },
    LeadRecord {
        name: "Mike Brown",
        status: "Call booked",
        interest: "Broke",
        source: "Direct message",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Alex W.",
        interacted: "3 hours ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Linda Chen",
        status: "Unqualified",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Jane D.",
        interacted: "1 week ago",
        days_ago: 7,
    },
    LeadRecord {
        name: "David Lee",
        status: "Won",
        interest: "Interested",
        source: "Story replies",
        deal_value: "$ 5,000.00",
        payment: "Paid",
        assigned_to: "John S.",
        interacted: "4 days ago",
        days_ago: 4,
    },
    LeadRecord {
        name: "Emily White",
        status: "No show",
        interest: "Interested",
        source: "Direct message",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "15 mins ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Jessica Wong",
        status: "Won",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "$ 3,000.00",
        payment: "Paid",
        assigned_to: "Sarah P.",
        interacted: "1 week ago",
        days_ago: 7,
    },
    LeadRecord {
        name: "Kevin Harris",
        status: "Qualified",
        interest: "N/A",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Jane D.",
        interacted: "1 day ago",
        days_ago: 1,
    },
    LeadRecord {
        name: "Tom Clark",
        status: "Lost",
        interest: "Broke",
        source: "Direct message",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "John S.",
        interacted: "6 days ago",
        days_ago: 6,
    },
    LeadRecord {
        name: "Laura Lewis",
        status: "Qualified",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "30 mins ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Brian Walker",
        status: "Call booked",
        interest: "Interested",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Alex W.",
        interacted: "2 days ago",
        days_ago: 2,
    },
    LeadRecord {
        name: "Daniel Hall",
        status: "Won",
        interest: "Interested",
        source: "Direct message",
        deal_value: "$ 1,500.00",
        payment: "Paid",
        assigned_to: "John S.",
        interacted: "5 days ago",
        days_ago: 5,
    },
    LeadRecord {
        name: "Nancy Allen",
        status: "Unqualified",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Jane D.",
        interacted: "2 weeks ago",
        days_ago: 14,
    },
    LeadRecord {
        name: "Paul Young",
        status: "Qualified",
        interest: "N/A",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "1 hour ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Sandra King",
        status: "Won",
        interest: "Broke",
        source: "Direct message",
        deal_value: "$ 4,200.00",
        payment: "Paid",
        assigned_to: "Jane D.",
        interacted: "6 days ago",
        days_ago: 6,
    },
    LeadRecord {
        name: "Omar Diaz",
        status: "Call booked",
        interest: "Interested",
        source: "Sponsored ad",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Sarah P.",
        interacted: "20 mins ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Priya Nair",
        status: "Lost",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Alex W.",
        interacted: "8 days ago",
        days_ago: 8,
    },
    LeadRecord {
        name: "Hannah Brooks",
        status: "Won",
        interest: "Interested",
        source: "Direct message",
        deal_value: "$ 6,750.00",
        payment: "Paid",
        assigned_to: "John S.",
        interacted: "1 day ago",
        days_ago: 1,
    },
    LeadRecord {
        name: "Victor Stone",
        status: "No show",
        interest: "Broke",
        source: "Sponsored ad",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Ali M.",
        interacted: "12 hours ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Mina Patel",
        status: "Qualified",
        interest: "Achiever",
        source: "Story replies",
        deal_value: "$ 900.00",
        payment: "Paid",
        assigned_to: "Jane D.",
        interacted: "3 days ago",
        days_ago: 3,
    },
    LeadRecord {
        name: "Ethan Ross",
        status: "Unqualified",
        interest: "Interested",
        source: "Direct message",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Alex W.",
        interacted: "2 weeks ago",
        days_ago: 14,
    },
    LeadRecord {
        name: "Grace Kim",
        status: "Won",
        interest: "Interested",
        source: "Story replies",
        deal_value: "$ 2,200.00",
        payment: "Paid",
        assigned_to: "Sarah P.",
        interacted: "5 days ago",
        days_ago: 5,
    },
    LeadRecord {
        name: "Noah Smith",
        status: "Call booked",
        interest: "N/A",
        source: "Sponsored ad",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "John S.",
        interacted: "4 hours ago",
        days_ago: 0,
    },
    LeadRecord {
        name: "Iris Chen",
        status: "Lost",
        interest: "Broke",
        source: "Direct message",
        deal_value: "N/A",
        payment: "No value",
        assigned_to: "Ali M.",
        interacted: "9 days ago",
        days_ago: 9,
    },
    LeadRecord {
        name: "Leo Martin",
        status: "Qualified",
        interest: "Interested",
        source: "Story replies",
        deal_value: "Pending...",
        payment: "Pending",
        assigned_to: "Jane D.",
        interacted: "7 days ago",
        days_ago: 7,
    },
];

const _: [(); LEAD_COUNT] = [(); LEADS.len()];

#[cfg(test)]
fn filtered_leads(
    query: &str,
    date_filter: usize,
    user_filter: usize,
    status_filter: usize,
    payment_filter: usize,
) -> Vec<&'static LeadRecord> {
    let query = query.trim().to_lowercase();
    LEADS
        .iter()
        .filter(|lead| lead_matches_date_filter(lead, date_filter))
        .filter(|lead| lead_matches_choice(lead.assigned_to, LEAD_USER_FILTERS, user_filter))
        .filter(|lead| lead_matches_choice(lead.status, LEAD_STATUS_FILTERS, status_filter))
        .filter(|lead| lead_matches_choice(lead.payment, LEAD_PAYMENT_FILTERS, payment_filter))
        .filter(|lead| {
            query.is_empty()
                || [
                    lead.name,
                    lead.status,
                    lead.interest,
                    lead.source,
                    lead.deal_value,
                    lead.assigned_to,
                    lead.interacted,
                ]
                .iter()
                .any(|value| value.to_lowercase().contains(&query))
        })
        .collect()
}

#[cfg(test)]
fn lead_matches_date_filter(lead: &LeadRecord, filter: usize) -> bool {
    match filter {
        1 => lead.days_ago <= 7,
        2 => lead.days_ago == 0,
        _ => true,
    }
}

#[cfg(test)]
fn lead_matches_choice<const N: usize>(value: &str, labels: [&str; N], index: usize) -> bool {
    index == 0 || labels.get(index).is_some_and(|label| *label == value)
}

fn gallery_toasts() -> Vec<Toast> {
    vec![
        Toast::new("Run complete")
            .body("The latest component pass is ready to review.")
            .intent(Intent::Success),
        Toast::new("Theme changed")
            .body("Runtime tokens were re-applied to the gallery.")
            .intent(Intent::Info),
        Toast::new("Action queued")
            .body("The host app owns timeout and dismissal behavior.")
            .intent(Intent::Neutral),
    ]
}

fn show_text_and_feedback(
    ui: &mut egui::Ui,
    toast_preview_open: &mut bool,
    toast_preview_toasts: &mut Vec<Toast>,
) {
    Card::new().show(ui, |ui| {
        ui.heading("Text and feedback");
        ui.horizontal_wrapped(|ui| {
            ui.add(Label::new("Default label"));
            ui.add(Label::new("Muted label").muted());
            ui.add(Label::new("Small label").size(Size::Small));
            ui.add(Link::new("Action link"));
            ui.add(Link::new("egui").to("https://github.com/emilk/egui"));
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(Kbd::new("K").size(Size::Small));
            ui.add(Kbd::shortcut(["Ctrl", "K"]).size(Size::Small));
            ui.add(Kbd::shortcut(["Shift", "Enter"]));
        });
        ui.add(Separator::new().spacing(10.0));
        ui.add(
            Alert::new("Build succeeded")
                .body("The latest component pass compiled and passed focused checks.")
                .intent(Intent::Success),
        );
        ui.add_space(8.0);
        ui.add(
            Alert::new("Review warning")
                .body("Palette derivation still needs dedicated contrast work.")
                .intent(Intent::Warning),
        );
        ui.add_space(8.0);
        ui.add(Notice::new("Neutral notice").body("Notices use the same feedback foundation."));
        ui.add(Separator::new().spacing(10.0));
        ui.heading("Toasts");
        ui.horizontal_wrapped(|ui| {
            ui.add(
                Toast::new("Run complete")
                    .body("Patch, format, and focused checks finished.")
                    .intent(Intent::Success)
                    .width(280.0),
            );
            ui.add(
                Toast::new("Review needed")
                    .body("Two generated files are waiting for inspection.")
                    .intent(Intent::Warning)
                    .width(280.0),
            );
            Toast::new("Changes archived")
                .body("A toast can render Cast controls in its action area.")
                .intent(Intent::Info)
                .width(280.0)
                .show_with(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.add(Button::new("Undo").size(Size::Small));
                        ui.add(
                            Button::new("View")
                                .size(Size::Small)
                                .variant(Variant::Ghost),
                        );
                    });
                });
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.add(Button::new("Show toast stack")).clicked() {
                *toast_preview_toasts = gallery_toasts();
                *toast_preview_open = true;
            }
            if ui.add(Button::new("Hide")).clicked() {
                *toast_preview_open = false;
                toast_preview_toasts.clear();
            }
        });
        ui.add(Separator::new().spacing(10.0));
        ui.heading("Loading");
        ui.horizontal_wrapped(|ui| {
            ui.add(Loader::new().size(Size::Small));
            ui.add(Loader::new().intent(Intent::Info));
            ui.add(Loader::new().intent(Intent::Success).size(Size::Large));
            ui.add(
                Loader::new()
                    .intent(Intent::Primary)
                    .style(LoaderStyle::Signal)
                    .size(Size::Large),
            );
            ui.add(
                Loader::new()
                    .intent(Intent::Info)
                    .style(LoaderStyle::Signal),
            );
            ui.add(
                Loader::new()
                    .intent(Intent::Primary)
                    .style(LoaderStyle::PixelEqualizer)
                    .size(Size::Large),
            );
            ui.add(
                Loader::new()
                    .intent(Intent::Info)
                    .style(LoaderStyle::PixelEqualizer),
            );
            ui.add(
                Loader::new()
                    .intent(Intent::Success)
                    .style(LoaderStyle::PulseGrid)
                    .size(Size::Large),
            );
            ui.add(
                Loader::new()
                    .intent(Intent::Primary)
                    .style(LoaderStyle::PulseGrid),
            );
            ui.label("Async work can use loader and progress primitives together.");
        });
        ui.add_space(8.0);
        ui.add(ProgressBar::new(0.64).intent(Intent::Primary));
        ui.add_space(6.0);
        ui.add(
            ProgressBar::new(0.38)
                .intent(Intent::Success)
                .size(Size::Small),
        );
        ui.add_space(6.0);
        ui.add(
            ProgressBar::new(0.82)
                .intent(Intent::Warning)
                .size(Size::Large),
        );
        ui.add(Separator::new().spacing(10.0));
        ui.heading("Skeleton");
        ui.vertical(|ui| {
            ui.add(Skeleton::new().width(220.0));
            ui.add_space(6.0);
            ui.add(Skeleton::new().width(320.0).size(Size::Small));
            ui.add_space(6.0);
            ui.add(Skeleton::new().width(180.0).size(Size::Small));
        });
    });
}

#[allow(clippy::too_many_arguments)]
fn show_forms(
    ui: &mut egui::Ui,
    search: &mut String,
    name: &mut String,
    notes: &mut String,
    handle: &mut String,
    preset_query: &mut String,
    preset_choice: &mut usize,
    form_validation_attention: &mut bool,
    enabled: &mut bool,
    notifications: &mut bool,
    indeterminate: &mut bool,
    form_density: &mut usize,
) {
    let clear_validation_attention = *form_validation_attention;

    Card::new().show(ui, |ui| {
        ui.heading("Forms");

        FormSection::new("Project details")
            .description("A grouped form area with related controls and field-level messages.")
            .show(ui, |ui| {
                ui.add(
                    ValidationSummary::new("Review before publishing")
                        .issue(ValidationIssue::new("Required before publishing.").field("Handle"))
                        .issue(ValidationIssue::new("Pick a preset for repeatable previews."))
                        .intent(Intent::Warning)
                        .width(520.0)
                        .attention(*form_validation_attention)
                        .scroll_to(*form_validation_attention),
                );
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    FormField::new("Project name")
                        .description("Shown in window titles and saved presets.")
                        .required(true)
                        .width(240.0)
                        .show(ui, |ui| {
                            ui.add(TextInput::new(name).hint_text("Project name").width(240.0));
                        });
                    FormField::new("Search")
                        .description("Filters the current gallery view.")
                        .width(240.0)
                        .show(ui, |ui| {
                            ui.add(SearchInput::new(search).width(240.0));
                        });
                });
                ui.add_space(8.0);
                FormField::new("Instructions")
                    .description(
                        "Multiline text areas use the same input frame, halo, and status treatment.",
                    )
                    .width(520.0)
                    .show(ui, |ui| {
                        ui.add(
                            TextArea::new(notes)
                                .hint_text("Describe what the agent should do...")
                                .rows(4)
                                .width(520.0),
                        );
                    });
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    FormField::new("Handle")
                        .required(true)
                        .error("Required before publishing.")
                        .width(220.0)
                        .show(ui, |ui| {
                            ui.add(
                                TextInput::new(handle)
                                    .hint_text("theme-handle")
                                    .variant(Variant::Subtle)
                                    .width(220.0),
                            );
                        });
                    FormField::new("Outline")
                        .success("Looks ready.")
                        .width(220.0)
                        .show(ui, |ui| {
                            ui.add(
                                TextInput::new(name)
                                    .hint_text("Outline input")
                                    .variant(Variant::Outline)
                                    .width(220.0),
                            );
                        });
                    FormField::new("Ghost")
                        .warning("Use sparingly in dense forms.")
                        .width(220.0)
                        .show(ui, |ui| {
                            ui.add(
                                TextInput::new(search)
                                    .hint_text("Ghost input")
                                    .variant(Variant::Ghost)
                                    .width(220.0),
                            );
                        });
                    FormField::new("Disabled")
                        .description("Disabled state remains legible.")
                        .width(220.0)
                        .show(ui, |ui| {
                            ui.add(
                                TextInput::new(name)
                                    .hint_text("Disabled input")
                                    .disabled()
                                    .width(220.0),
                            );
                        });
                });
            });

        ui.add(Separator::new().spacing(12.0));
        FormSection::new("Preferences")
            .description("Choice controls can be composed as grouped fields or standalone toggles.")
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.add(Checkbox::new(enabled, "Enabled"));
                    ui.add(Checkbox::new(indeterminate, "Mixed").indeterminate(true));
                    ui.add(Checkbox::new(notifications, "Disabled").disabled());
                });
                FormField::new("Density")
                    .description("RadioGroup keeps related choices together.")
                    .width(360.0)
                    .show(ui, |ui| {
                        ui.add(
                            RadioGroup::new(
                                form_density,
                                [(0, "Compact"), (1, "Comfortable"), (2, "Spacious")],
                            )
                            .size(Size::Small),
                        );
                    });
                ui.add_space(6.0);
                FormField::new("Density select")
                    .description("Select gives dropdown behavior a form-control name.")
                    .width(220.0)
                    .show(ui, |ui| {
                        ui.add(
                            Select::new(form_density, ["Compact", "Comfortable", "Spacious"])
                                .placeholder("Density")
                                .width(220.0),
                        );
                    });
                ui.add_space(6.0);
                FormField::new("Preset combobox")
                    .description("Combobox filters larger option sets before choosing.")
                    .width(260.0)
                    .show(ui, |ui| {
                        ui.add(
                            Combobox::new(
                                preset_choice,
                                preset_query,
                                [
                                    "Compact",
                                    "Comfortable",
                                    "Spacious",
                                    "Agent workspace",
                                    "Dense table",
                                    "Presentation",
                                ],
                            )
                            .placeholder("Preset")
                            .search_hint("Search presets")
                            .width(260.0),
                        );
                    });
                ui.horizontal(|ui| {
                    ui.add(Switch::new(notifications));
                    ui.label("Notifications");
                });
                ui.horizontal_wrapped(|ui| {
                    ui.add(Switch::new(enabled).size(Size::Small));
                    ui.add(Switch::new(enabled).size(Size::Medium));
                    ui.add(Switch::new(enabled).size(Size::Large));
                });
        });

        FormActions::new().show(ui, |ui| {
            if ui.add(Button::new("Save").size(Size::Small)).clicked() {
                *form_validation_attention = true;
            }
            ui.add(
                Button::new("Reset")
                    .intent(Intent::Neutral)
                    .variant(Variant::Outline)
                    .size(Size::Small),
            );
        });
    });

    if clear_validation_attention {
        *form_validation_attention = false;
    }
}

fn show_raw_egui_controls(ui: &mut egui::Ui, search: &mut String, enabled: &mut bool) {
    Card::new().show(ui, |ui| {
        ui.heading("Raw egui controls");
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.text_edit_singleline(search);
        });
        ui.checkbox(enabled, "Enabled");
        ui.horizontal_wrapped(|ui| {
            let _ = ui.button("egui button");
            ui.hyperlink_to("egui link", "https://github.com/emilk/egui");
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lead_filters_apply_status_and_payment() {
        let filtered = filtered_leads("", 0, 0, 1, 1);

        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|lead| lead.status == "Won"));
        assert!(filtered.iter().all(|lead| lead.payment == "Paid"));
    }

    #[test]
    fn lead_search_matches_visible_table_fields() {
        let filtered = filtered_leads("alex", 0, 0, 0, 0);

        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|lead| lead.assigned_to == "Alex W."));
    }

    #[test]
    fn last_twenty_four_hours_filter_uses_recent_rows() {
        let filtered = filtered_leads("", 2, 0, 0, 0);

        assert!(!filtered.is_empty());
        assert!(filtered.iter().all(|lead| lead.days_ago == 0));
    }

    #[test]
    fn rows_per_page_limit_is_state_backed() {
        assert_eq!(
            patterns::entity_table_with_details::rows_per_page_limit(0),
            5
        );
        assert_eq!(
            patterns::entity_table_with_details::rows_per_page_limit(1),
            10
        );
        assert_eq!(
            patterns::entity_table_with_details::rows_per_page_limit(2),
            25
        );
    }
}
