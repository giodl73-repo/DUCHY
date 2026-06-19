# Pulse 04: First Real Source-Backed Facts

## Intent

Start historical import with the smallest source-reviewed slice possible: one
source record and two structured facts that can pass custody validation.

## Source

- Source ID: `src-wikidata-q158445`
- URL: `https://www.wikidata.org/wiki/Q158445`
- Subject: Grand Duchy of Mecklenburg-Schwerin
- Retrieval date: 2026-06-19
- Allowed use: structured claims only
- License posture: Wikidata structured data in the main namespace is CC0; page
  text is CC BY-SA and is not imported.

## Imported Facts

- `fact-q158445-name`: title name, `Grand Duchy of Mecklenburg-Schwerin`
- `fact-q158445-exists`: title existence span, 1815 through 1918

## Explicit Non-Imports

- No prose text.
- No geometry.
- No map boundary.
- No holder genealogy.
- No full parentage or control timeline.
- No commercial-game data or mechanics.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
