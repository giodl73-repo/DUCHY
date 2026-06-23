# Pulse 221 - Low Countries Successor Sources

Date: 2026-06-23

## Scope

- Begin the next source-custody target from the relation-model remaining
  blocker audit.
- Add source records for Flanders, Holland, and Namur split-control and
  successor-state review without promoting facts yet.
- Improve promotion reports so source-only packets are auditable.

## Results

- Added 6 accepted source records:
  - `src-wikipedia-habsburg-netherlands`
  - `src-wikipedia-seventeen-provinces`
  - `src-wikipedia-spanish-netherlands`
  - `src-wikipedia-austrian-netherlands`
  - `src-wikipedia-dutch-republic`
  - `src-wikipedia-batavian-republic`
- Promoted no facts in this packet.
- Added `SourceCatalog::records()` and promotion report source listing for
  source-only packets.

## Measurements

- Sources: 526.
- Facts: 1358.
- Parentage facts: 278.
- Relation facts: 28.
- Timeline: valid.

## Validation

```powershell
cargo fmt --check
cargo test --quiet
cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/low-countries-successor-sources-01-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-successor-sources-01.sources data/staging/low-countries-successor-sources-01.facts
cargo run --quiet --bin duchy-promote -- --apply --report data/staging/low-countries-successor-sources-01-apply-report.md fixtures/first-real.sources fixtures/first-real.facts data/staging/low-countries-successor-sources-01.sources data/staging/low-countries-successor-sources-01.facts
cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts
```

## Notes

- This packet intentionally changes source custody only.
- Next step is a narrow Low Countries relation review that decides which, if
  any, split-control or successor-state relation facts can be promoted for
  Flanders, Holland, and Namur.
