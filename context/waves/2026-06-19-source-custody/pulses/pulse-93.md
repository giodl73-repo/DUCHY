# Pulse 93: Duchy of Prussia Polish Crown Parentage

## Intent

Add a reviewed text-backed source for Duchy of Prussia parentage and close its
parentage gap under the Crown of the Kingdom of Poland.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Prussia parentage context.
- Promote Q153091 -> Q171348 for 1525-1618.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Teutonic Order secularization mechanics, Prussian Homage ceremony details,
Brandenburg-Prussia personal-union governance, post-1618 Polish suzerainty,
Treaty of Wehlau or Oliva sovereignty changes, estate politics, borders, later
East Prussia administration, or Kingdom of Prussia claims are imported in this
packet.

## Current State

- sources: 353
- facts: 1197
- titles: 333
- parentage facts: 198
- titles with parentage: 154
- titles without parentage: 179

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/prussia-duchy-polish-crown-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/prussia-duchy-polish-crown-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
