use cast::{
    Badge, Button, Checkbox, Dropdown, Intent, Notice, Size, Table, Variant,
    egui::{self, RichText},
};

#[derive(Clone, Copy, Debug)]
pub struct EntityRecord {
    pub name: &'static str,
    pub status: &'static str,
    pub interest: &'static str,
    pub source: &'static str,
    pub deal_value: &'static str,
    pub payment: &'static str,
    pub assigned_to: &'static str,
    pub interacted: &'static str,
    pub days_ago: u8,
}

pub struct EntityTableState<'a> {
    pub selected: &'a mut [bool],
    pub expanded: &'a mut [bool],
    pub rows_per_page: &'a mut usize,
    pub page: &'a mut usize,
    pub exported_count: &'a mut Option<usize>,
}

pub fn show_entity_table_with_details(
    ui: &mut egui::Ui,
    records: &[EntityRecord],
    state: EntityTableState<'_>,
) {
    let filtered_records = records.iter().enumerate().collect::<Vec<_>>();
    let filtered_count = filtered_records.len();
    let row_limit = rows_per_page_limit(*state.rows_per_page);
    let page_count = filtered_count.div_ceil(row_limit).max(1);
    if *state.page >= page_count {
        *state.page = page_count - 1;
    }
    let row_offset = *state.page * row_limit;
    let visible_rows = filtered_records
        .iter()
        .copied()
        .skip(row_offset)
        .take(row_limit)
        .collect::<Vec<_>>();
    let visible_count = visible_rows.len();
    let selected_rows = visible_rows
        .iter()
        .enumerate()
        .filter_map(|(visible_index, (record_index, _))| {
            state
                .selected
                .get(*record_index)
                .copied()
                .unwrap_or(false)
                .then_some(visible_index)
        })
        .collect::<Vec<_>>();
    let expanded_rows = visible_rows
        .iter()
        .enumerate()
        .filter_map(|(visible_index, (record_index, _))| {
            state
                .expanded
                .get(*record_index)
                .copied()
                .unwrap_or(false)
                .then_some(visible_index)
        })
        .collect::<Vec<_>>();

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading(format!("Leads [{filtered_count}]"));
            ui.label("Lead records rendered directly by the Cast table component.");
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui
                .add(
                    Button::new("Export as CSV")
                        .intent(Intent::Neutral)
                        .variant(Variant::Outline)
                        .leading_icon("[^]"),
                )
                .clicked()
            {
                *state.exported_count = Some(filtered_count);
            }
        });
    });
    if let Some(count) = *state.exported_count {
        ui.add_space(8.0);
        ui.add(
            Notice::new(format!("{count} leads exported"))
                .body("Export feedback is wired to the current table rows.")
                .intent(Intent::Success),
        );
    }
    ui.add_space(14.0);
    if visible_count == 0 {
        ui.add(
            Notice::new("No matching leads")
                .body("Change the search query or filters to widen the result set."),
        );
    } else {
        Table::new([
            "",
            "Lead name",
            "Status",
            "Interest",
            "Source",
            "Deal value",
            "Assigned to",
            "Interacted",
        ])
        .size(Size::Small)
        .column_weights([0.24, 1.35, 1.20, 1.10, 1.30, 1.10, 1.10, 1.0])
        .right_aligned_columns([5])
        .selected_rows(selected_rows)
        .expanded_rows(expanded_rows)
        .expanded_row_height(96.0)
        .sticky_header(320.0)
        .show_with_details(
            ui,
            visible_count,
            |row, row_index| {
                let (record_index, record) = visible_rows[row_index];
                row.centered_cell(|ui| {
                    if let Some(selected) = state.selected.get_mut(record_index) {
                        ui.add(Checkbox::new(selected, "").size(Size::Small));
                    }
                });
                row.cell(|ui| {
                    let expanded = state.expanded.get(record_index).copied().unwrap_or(false);
                    if ui
                        .add(
                            Button::new(if expanded { "-" } else { "+" })
                                .intent(Intent::Neutral)
                                .variant(Variant::Ghost)
                                .size(Size::Small),
                        )
                        .clicked()
                    {
                        if let Some(expanded) = state.expanded.get_mut(record_index) {
                            *expanded = !*expanded;
                        }
                    }
                    let name = if expanded {
                        RichText::new(record.name).strong()
                    } else {
                        RichText::new(record.name)
                    };
                    ui.label(name);
                });
                row.cell(|ui| {
                    ui.add(
                        Badge::new(record.status)
                            .intent(entity_status_intent(record.status))
                            .status_dot()
                            .size(Size::Small),
                    );
                });
                row.cell(|ui| {
                    ui.add(
                        Badge::new(record.interest)
                            .intent(entity_interest_intent(record.interest))
                            .status_dot()
                            .size(Size::Small),
                    );
                });
                row.text(record.source);
                row.text(record.deal_value);
                row.text(record.assigned_to);
                row.text(record.interacted);
            },
            |detail, row_index| {
                let (_, record) = visible_rows[row_index];
                detail.show(|ui| {
                    entity_detail_content(ui, record);
                });
            },
        );
    }
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Rows per page:");
        let rows_response = ui.add(
            Dropdown::new(state.rows_per_page, ["5", "10", "25"])
                .width(82.0)
                .size(Size::Small),
        );
        if rows_response.changed() {
            *state.page = 0;
        }
        let visible_start = if visible_count == 0 {
            0
        } else {
            row_offset + 1
        };
        let visible_end = row_offset + visible_count;
        ui.add(
            Badge::new(format!("Showing {visible_start}-{visible_end}")).intent(Intent::Neutral),
        );
        ui.add(Badge::new(format!("of {filtered_count}")).intent(Intent::Neutral));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(
                    Button::new("Next")
                        .size(Size::Small)
                        .variant(Variant::Outline)
                        .enabled(*state.page + 1 < page_count && visible_count > 0),
                )
                .clicked()
            {
                *state.page += 1;
            }
            ui.add(Badge::new(format!(
                "Page {} of {page_count}",
                *state.page + 1
            )));
            if ui
                .add(
                    Button::new("Previous")
                        .size(Size::Small)
                        .variant(Variant::Outline)
                        .enabled(*state.page > 0 && visible_count > 0),
                )
                .clicked()
            {
                *state.page -= 1;
            }
        });
    });
}

pub fn rows_per_page_limit(index: usize) -> usize {
    match index {
        1 => 10,
        2 => 25,
        _ => 5,
    }
}

fn entity_detail_content(ui: &mut egui::Ui, record: &EntityRecord) {
    let theme = cast::theme_for_ui(ui);
    ui.vertical(|ui| {
        ui.label(
            RichText::new("Related information")
                .strong()
                .color(theme.colors.text),
        );
        ui.add_space(theme.spacing.xs);
        ui.columns(3, |columns| {
            detail_field(
                &mut columns[0],
                "Owner",
                record.assigned_to,
                Intent::Neutral,
            );
            detail_field(&mut columns[1], "Source", record.source, Intent::Info);
            detail_field(
                &mut columns[2],
                "Payment",
                record.payment,
                payment_intent(record.payment),
            );
        });
        ui.add_space(theme.spacing.xs);
        ui.label(
            RichText::new(format!(
                "{} last interacted; current interest is {}.",
                record.interacted, record.interest
            ))
            .color(theme.colors.text_muted),
        );
    });
}

fn detail_field(ui: &mut egui::Ui, label: &str, value: &str, intent: Intent) {
    let theme = cast::theme_for_ui(ui);
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).small().color(theme.colors.text_muted));
        ui.add(
            Badge::new(value)
                .intent(intent)
                .status_dot()
                .size(Size::Small),
        );
    });
}

fn entity_status_intent(status: &str) -> Intent {
    match status {
        "Won" | "Qualified" => Intent::Success,
        "Call booked" => Intent::Info,
        "Lost" | "Unqualified" | "No show" => Intent::Danger,
        _ => Intent::Neutral,
    }
}

fn entity_interest_intent(interest: &str) -> Intent {
    match interest {
        "Interested" => Intent::Primary,
        "Achiever" => Intent::Success,
        "Broke" => Intent::Warning,
        _ => Intent::Neutral,
    }
}

fn payment_intent(payment: &str) -> Intent {
    match payment {
        "Paid" => Intent::Success,
        "Pending" => Intent::Warning,
        "No value" => Intent::Neutral,
        _ => Intent::Neutral,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_per_page_limit_is_state_backed() {
        assert_eq!(rows_per_page_limit(0), 5);
        assert_eq!(rows_per_page_limit(1), 10);
        assert_eq!(rows_per_page_limit(2), 25);
    }

    #[test]
    fn entity_badge_intents_follow_record_semantics() {
        assert_eq!(entity_status_intent("Won"), Intent::Success);
        assert_eq!(entity_status_intent("No show"), Intent::Danger);
        assert_eq!(entity_interest_intent("Broke"), Intent::Warning);
        assert_eq!(payment_intent("Paid"), Intent::Success);
        assert_eq!(payment_intent("Pending"), Intent::Warning);
    }
}
