# Pulse 133: Parentage Change Metrics Report

## Intent

Make the accepted fixture catalog answer the scaling question: how many times
titles, and especially county-ranked titles, change parentage.

## Changes

- Add `duchy-import parentage-change-report`.
- Generate `data/staging/parentage-change-report.md` from accepted source and
  fact fixtures.
- Add CLI helper tests for distinct-parent change counting and parent-rank
  aggregation.

## Boundary

No new historical sources, title facts, parentage facts, or CK3-derived facts
are imported. The report is a derived metric over already accepted fixtures.

## Current State

- sources: 419
- facts: 1271
- titles: 337
- parentage facts: 260
- parentage titles: 215
- titles with parent changes: 34
- parent changes: 44
- county parentage titles: 30
- county titles with parent changes: 2
- county parent changes: 2

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-change-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-change-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo fmt --check`
- `rg -n "Abaúj|Cetatea|Hordaland|Brycheiniog|Duklja|title-q1049854|title-q8273263|title-q50625|title-q954585|title-q1252942" src -S`
- `git diff --check`
- `cargo test --quiet -j 1`
