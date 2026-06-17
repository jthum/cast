# Patterns

Patterns are composed UI blocks built from Cast primitives. They live in the gallery while their APIs, layout, and visual hierarchy are still being tested against realistic screens.

Patterns are not the same as low-level components. A component should be broadly reusable and have a stable API. A pattern can be more opinionated, combine several components, and change more freely as the design language matures.

## Where Patterns Live

Current patterns live in:

```text
crates/cast-gallery/src/patterns
```

This keeps experimental composition close to the visual gallery without forcing every arrangement into the public `cast-ui` crate too early.

## Current Pattern Types

The gallery currently includes patterns for:

- App shell and sidebar navigation.
- Command palette.
- Related activity / collapsible activity rows.
- Entity tables with expandable details.

These patterns are useful references for product applications, but they should graduate into `cast-ui` only when the API is clear enough to support outside consumers.

## When To Promote A Pattern

Promote a pattern into `crates/cast-ui` when:

- Multiple screens need the same structure.
- The structure has stable state ownership.
- The component boundary is clear.
- The pattern can be named without referencing one app's domain.
- The behavior is useful beyond the gallery.
- Styling can be controlled through existing tokens or a small, meaningful extension to the token model.

Do not promote a pattern just because it looks good once. Cast should avoid hardcoding one-off layouts into the library crate.

## App Shell Pattern

The shell pattern is a good example of a reusable composition that may eventually become a first-class component or remain a documented block. It combines:

- Sidebar navigation.
- Top bar controls.
- Compact slide-out navigation.
- Scrollable content pane.
- Route-specific content.

The gallery shell should stay useful as an implementation reference for applications that want a dense desktop layout without inventing their own chrome from scratch.

## Agent Workspace Pattern

The Turin-style screen in the gallery is intentionally a composition, not a single component. It combines:

- Run summary.
- Message thread.
- Tool-call disclosure.
- Composer.
- Run state.
- Timeline.
- Approval panel.
- Review table.

This is a realistic product screen that tests whether primitives work together without becoming visually noisy. It should guide component polish, but the app still owns the exact workflow semantics.

## Documentation Blocks

Patterns can also be used as documentation blocks: a card with code, a component example with variants, a command palette demo, or a layout recipe. These are useful in the gallery and docs, but they should remain separate from the core component API unless they become generally useful.

## Rule Of Thumb

If the caller needs to bring domain data and decide the workflow, keep it as a pattern. If the caller only needs to provide state and content slots, it may be ready to become a component.
