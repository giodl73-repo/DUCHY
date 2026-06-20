# Pulse 60: Parentage Coverage Report

## Intent

Stop treating parentage scale-up as an open-ended manual loop. Add a repeatable
coverage report so the next imports can target explicit hierarchy gaps and
multi-parentage review cases.

## Changes

- Add `duchy-import parentage-coverage-report sources-file facts-file output.md`.
- Generate `data/staging/parentage-coverage-report.md` from the accepted
  fixture catalog.
- Report total title, fact, source, and parentage counts.
- Summarize title coverage by rank.
- List titles without parentage and titles with multiple parentage facts.

## Current Coverage

- sources: 334
- facts: 1124
- titles: 323
- parentage facts: 155
- titles with parentage: 112
- titles without parentage: 211

## Review Boundary

This pulse adds reporting only. It does not import source records or historical
facts.

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo test --quiet -j 1`
- `cargo fmt --check`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
