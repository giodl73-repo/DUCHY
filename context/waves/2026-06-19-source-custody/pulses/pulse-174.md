# Pulse 174: Early Bavaria East Francia Parentage

## Intent

Import one source-backed parentage span from the Kingdom of Germany candidate
review without overreaching into unsupported replacement facts. The reviewed
Duchy of Bavaria source supports early stem-duchy parentage under East Francia
/ Kingdom of Germany before the HRE edge begins.

## Changes

- Add `fact-q47261-parent-q153080`.
- Parent Duchy of Bavaria under East Francia for `907..961`.
- Keep the existing Duchy of Bavaria -> Holy Roman Empire fact active from
  `962..1805`; no supersession is needed because the spans do not overlap.
- Regenerate active graph, coverage, gap, rank-skip, candidate, and change
  reports.

## Boundary

This pulse does not promote Kingdom of Germany (`Q175211`) into accepted
fixtures and does not replace the later Duchy of Bavaria direct HRE parentage.
The broader Kingdom of Germany replacement packet remains blocked on
child-level source custody for the post-962 span.

## Current State

- reviewed sources: 440
- reviewed facts: 1324
- materialized titles: 349
- active parentage facts: 275
- superseded parentage facts: 2
- active rank-skip rows: 220
- weighted span coverage: 59.33%
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-graph-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-graph-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
