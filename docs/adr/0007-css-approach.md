# ADR 0007 — CSS approach: custom styles for game pages, PicoCSS for admin pages

## Status

Accepted

## Context

Two distinct page families have emerged:

- **Game pages** (question, results) and **user interaction pages** (submission form, pool exhaustion, success/error) — require deliberate spatial design. The signature layout is a full-viewport split where each half is a vote button. No CSS framework anticipates this; any classless framework would clobber it.
- **Admin and moderation pages** (future: question queue, moderator actions) — need no specific visual design. Functional, readable, accessible defaults are enough.

PicoCSS was evaluated as a CSS baseline. In classless mode it styles `<button>` and other elements directly — a problem if it shares a page with the full-viewport half-buttons on game pages, where every default would need resetting.

With clean template separation the problem disappears. Admin pages contain only conventional semantic HTML (tables, forms, approve/reject buttons). Classless PicoCSS styles all of that with zero markup overhead and no conflict. Class-based mode would add class ceremony for no benefit on this page family.

## Decision

Two base templates in the same binary and template tree:

- `base.html` — used by game and user interaction pages. Custom CSS only. No PicoCSS.
- `base_admin.html` — used by admin and moderation pages. PicoCSS classless. Minimal or no custom overrides.

PicoCSS classless works on admin pages precisely because the template separation is clean: no game UI elements appear there, so there is nothing for Pico's defaults to conflict with.

PicoCSS is loaded only in `base_admin.html`. `base_admin.html` is created when the first admin template is built, not before.

PicoCSS is chosen with the awareness that HTMX will be added later (ADR 0003). The two are a natural pairing for admin surfaces.

## Consequences

- Game page CSS remains fully custom and under full control. No specificity surprises from a framework.
- Admin pages get accessible, responsive defaults for free. Future moderator UI requires no design work.
- The boundary is explicit in the template tree. A developer picking a base template makes a conscious choice about which CSS surface they are on.
- The Google Fonts CDN dependency introduced in `base.html` (Syne, DM Sans) is an open item not resolved by this ADR. Options are self-hosting via a `ServeDir` route or switching to a system font stack.
