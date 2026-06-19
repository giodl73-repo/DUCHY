# Pulse 11: Reviewed Source Fixture Import

## Intent

Move reviewed real source metadata out of Rust literals and into a small
reviewable fixture file that pairs with `fixtures/first-real.facts`.

## Implementation

- Add `fixtures/first-real.sources`.
- Add `first_real_source_catalog_from_fixture`.
- Add `first_real_titles_from_fixture`.
- Make `first_real_timeline_from_fixture` use parsed source metadata and parsed
  fact records together.
- Validate every real fact in the CLI against the parsed source fixture.

## Current Source Records

- `src-wikidata-q158445`
- `src-wikidata-q20135`

## Boundary

The source fixture records only authorize minimal structured claims already
listed in `fixtures/first-real.facts`. They do not authorize prose, geometry,
map boundaries, holder genealogy, control timelines, or parentage relations.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
