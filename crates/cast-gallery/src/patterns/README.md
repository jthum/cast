# Gallery Patterns

Patterns are composed UI blocks built from Cast primitives. They live in the gallery while their interaction model, naming, and visual language are still being proven, and they can graduate into `cast-ui` only when the reusable API is clear.

Current patterns:

- `command_palette.rs`: modal command surface with search, keyboard selection state, action dispatch, and compact command rows.
- `entity_table_with_details.rs`: product-style table pattern with selectable rows, status badges, pagination state, sticky header behavior, and expandable details.
- `related_activity.rs`: disclosure and accordion arrangement for nested activity streams.
- `shell.rs`: reusable gallery shell chrome including sidebar, top bar, scroll settings, and shell colors.

Use patterns as starting points for real app screens, not as stable public APIs. If a pattern becomes broadly useful across projects, extract the stable pieces into `cast-ui` and keep app-specific data and actions in the consuming app.
