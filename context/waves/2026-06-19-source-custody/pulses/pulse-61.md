# Pulse 61: Parentage Gap TSV Queue

## Intent

Make the parentage coverage gap actionable for batch review. The Markdown
coverage report is useful for inspection, but the next scale-up needs a stable
machine-readable queue.

## Changes

- Add `duchy-import parentage-gap-tsv sources-file facts-file output.tsv`.
- Generate `data/staging/parentage-gap-targets.tsv`.
- Export every accepted materialized title with no parentage fact.
- Include title id, name, rank, existence span, parentage count, review
  priority, and reviewer notes.

## Current Queue

- titles: 323
- parentage facts: 155
- gap rows: 211

## Review Boundary

This pulse adds queue generation only. It does not import source records or
historical facts.

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo test --quiet -j 1`
- `cargo fmt --check`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
