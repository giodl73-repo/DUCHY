# Pulse 07: Fact Fixture Import

## Intent

Move the first real historical facts out of Rust literals and into a small
reviewable fixture file.

## Implementation

- Add `fixtures/first-real.facts`.
- Add `fact_records_from_text`.
- Add `first_real_fact_records_from_fixture`.
- Use the parsed facts to build the first real source-backed timeline in the
  CLI smoke path.

## File Format

The `.facts` format is line-oriented:

- `key: value` pairs
- `---` between records
- blank lines and `#` comments ignored
- comma-separated `source_ids`
- temporal spans as `start..end` or `start..present`

## Current Fixture Records

- `fact-q158445-name`
- `fact-q158445-rank`
- `fact-q158445-exists`

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet
git diff --check
```
