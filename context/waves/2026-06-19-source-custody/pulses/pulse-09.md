# Pulse 09: Second Reviewed Source Import

## Intent

Expand DUCHY from one real source-backed title to a two-title mini-catalog while
preserving the same custody boundary: source-reviewed name, rank, and existence
facts only.

## Source

- Source ID: `src-wikidata-q20135`
- URL: `https://www.wikidata.org/wiki/Q20135`
- Subject: Grand Duchy of Hesse
- Retrieval date: 2026-06-19
- Allowed use: structured claims only
- License posture: Wikidata structured data in the main namespace is CC0; page
  text is CC BY-SA and is not imported.

## Imported Facts

- `fact-q20135-name`: title name, `Grand Duchy of Hesse`
- `fact-q20135-rank`: normalized DUCHY rank, `Duchy`
- `fact-q20135-exists`: title existence span, 1806 through 1918

## Explicit Non-Imports

- No prose text.
- No geometry.
- No map boundary.
- No parentage or title hierarchy.
- No holder genealogy.
- No control timeline.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
