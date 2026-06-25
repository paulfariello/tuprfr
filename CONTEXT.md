# tuprfr — Domain Glossary

## Question

A "Tu préfères ?" prompt composed of exactly two **Options**. Has a **Status**.

## Option

One of the two mutually exclusive choices in a **Question** (Option A / Option B).

## Vote

A **Session**'s recorded choice of one **Option** in a **Question**. One Vote per Session per Question.

## Session

A browser-scoped identity tracked by cookie. Stores: seen **Question** IDs (for **Game Loop** filtering), and recent **Submission** timestamps (for rate limiting: max 3 submissions per hour). Not tied to an account. Designed to become nullable when **User** accounts are introduced.

## Status

The lifecycle state of a **Question**: `pending` | `published` | `hidden`.

- `pending` — submitted, awaiting moderator approval
- `published` — live, visible to all
- `hidden` — removed from public view by a moderator

## Submission Mode

A server-level config that controls whether new **Question** submissions are auto-published (`published`) or queued for moderator review (`pending`). Modes: `open` (auto-publish) | `moderated` (requires approval).

## Game Loop

The core play sequence: a random **Question** is shown → the player either **Votes** or **Skips** → the result distribution is revealed → a "next" button loads another random Question. A Question that has been Voted on or Skipped is not shown again in the same **Session**. When all published Questions have been seen, the session reaches **Pool Exhaustion**.

## Pool Exhaustion

The terminal state when a **Session** has Voted on or Skipped every published **Question**. Shown as a congratulations screen with the **Submission** form displayed directly, turning the dead end into a contribution prompt.

## Skip

A **Session** action that reveals the result distribution of a **Question** without recording a **Vote**. The Question is marked as seen in the Session so it is not shown again. No skip count is stored server-side.

## Submission

The act of creating a new **Question**. Requires two **Option** texts. The submitter's identity is always stored internally (`author_session_id`, later `author_user_id`) but may be hidden from public display via the `is_anonymous` flag. The anonymity toggle is not shown in the UI until **User** accounts exist, since **Sessions** carry no display name.

## User (future)

An authenticated identity with a moderator role. Created via CLI (`tuprfr create-moderator`), not public registration. Authenticated with username + password (argon2 hashing). **Sessions** will become linkable to a User for authorship attribution.
