# Components

Cast components are ordinary `egui` widgets with builder-style APIs. They are designed to be composed directly in immediate-mode Rust code, while drawing styling from the runtime Cast theme stored in the `egui` context.

## Naming

The repository is `cast`, the published package is `cast-ui`, and the recommended local crate name is `cast`:

```toml
[dependencies]
cast = { package = "cast-ui", version = "0.1" }
```

Examples should use `cast::Button` rather than `cast_ui::Button`. If a consumer chooses not to rename the dependency, the Rust module path remains `cast_ui`.

## Common Conventions

Most components support some combination of:

- `intent(...)` for semantic color role: neutral, primary, secondary, success, warning, danger, or info.
- `variant(...)` for visual treatment: solid, outline, ghost, or subtle where relevant.
- `size(...)` for small, medium, or large control sizing.
- `width(...)` for explicit layout width when `egui` needs help.
- Host-owned state through `&mut` references rather than internal retained state.

Cast intentionally keeps state ownership with the application. This matches `egui` and makes components predictable in agentic or data-heavy interfaces where state often comes from a model, task run, or server update.

## Core Controls

Core controls include:

- `Button`
- `Badge`
- `Link`
- `Label`
- `Kbd`
- `Checkbox`
- `Radio` / `RadioGroup`
- `Switch`
- `Slider`
- `TextInput`
- `SearchInput`
- `TextArea`
- `NumberInput`
- `DateInput`
- `TimeInput`
- `Select`
- `Dropdown`
- `Combobox`

Use semantic intent for meaning, not decoration:

```rust
ui.add(cast::Button::new("Save"));
ui.add(cast::Button::new("Delete").intent(cast::Intent::Danger));
ui.add(cast::Badge::new("Running").intent(cast::Intent::Info).status_dot());
```

## Navigation And Layout

Navigation and layout primitives include:

- `Tabs`
- `SegmentedControl`
- `Breadcrumb`
- `Sidebar`
- `SidebarItem`
- `Pagination`
- `ControlGroup`
- `ResponsiveColumns`
- `ResizablePanels`
- `Carousel`
- `Calendar`
- `Separator`

`ResponsiveColumns` is meant for simple two-column app layouts. It falls back to vertical layout when the available width cannot satisfy both minimum column widths.

## Surfaces And Overlays

Surfaces and overlays include:

- `Card`
- `Panel`
- `Alert`
- `Notice`
- `Tooltip`
- `Popover`
- `HoverCard`
- `Dialog`
- `Sheet`
- `Disclosure`
- `Accordion`
- `EmptyState`

Cards, panels, dialogs, sheets, and popovers can use sectioned headers and footers. Section chrome is token-driven through `SurfaceSectionTokens`, so a theme can make headers flat, muted, or very subtle without changing content structure.

## Tables

Cast has two table shapes:

- `Table`: rich widget-capable rows. Use this for application data, selectable rows, checkboxes, badges, editable cells, actions, and expandable details.
- `TextTable`: string-only table. Use this for markdown-like output, logs, generated tabular results, or LLM-produced tables where rich widgets are not needed.

Use `Table` when cells need interactive components:

```rust
cast::Table::new(["", "Task", "State", "Action"])
    .column_weights([0.2, 2.4, 1.0, 1.0])
    .show(ui, tasks.len(), |row, index| {
        row.centered_cell(|ui| {
            ui.add(cast::Checkbox::new(&mut tasks[index].selected, ""));
        });
        row.text(&tasks[index].title);
        row.cell(|ui| {
            ui.add(cast::Badge::new(&tasks[index].state).status_dot());
        });
        row.cell(|ui| {
            ui.add(cast::Button::new("Open").size(cast::Size::Small));
        });
    });
```

Row selection should usually be model-driven. If a table has checkboxes in the first column, the checkbox should own selection and the row background should reflect that state.

## Feedback And Loading

Feedback primitives include:

- `Toast`
- `ToastStack`
- `ProgressBar`
- `ProgressMetric`
- `Loader`
- `Spinner`
- `Skeleton`

`Loader` is the preferred generic name. `Spinner` aliases remain available for conventional spinner-style usage.

## Markdown And Output

Agentic and documentation-heavy apps often need selectable text, markdown, and output panels:

- `Markdown`
- `CodeOutputPanel`
- `ToolOutput`

`Markdown` is intentionally pragmatic rather than a full browser renderer. It exists so chat messages and generated reports can preserve headings, lists, code blocks, and basic formatting inside native `egui` UI.

## Agent Workflow Components

Agent-oriented primitives include:

- `AgentComposer`: multiline prompt input with send/stop actions, attachment/tool slots, model selector, loading state, and Enter / Shift+Enter behavior.
- `MessageThread` and `ChatMessage`: user, assistant, system, and tool message blocks with copy actions and rich content slots.
- `ToolCall` and `ToolCallBlock`: compact and collapsible tool-call display with queued, running, succeeded, and failed states.
- `RunTimeline`: vertical timeline for planning, tool calls, patches, tests, review, and final response.
- `CodeOutputPanel`: scrollable monospace output for logs, shell output, errors, diffs, and tool results.
- `ArtifactCard`: generated file, report, screenshot, or review-item card with common actions.
- `ApprovalPanel`: workflow approval surface for patch, command, or permission decisions.
- `ContextPanel`, `PlanList`, and `PatchReviewPanel`: higher-level building blocks for agent workbenches.

These components are useful for Turin-style workflows, but they are not Turin-specific. They should remain generic enough for any app that coordinates agents, tools, logs, approvals, and structured output.
