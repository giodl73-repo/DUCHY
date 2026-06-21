# Pulse 94: County of Empuries Crown of Aragon Parentage

## Intent

Add a reviewed text-backed source for County of Empuries parentage and close its
parentage gap under the Crown of Aragon.

## Changes

- Add one reviewed Wikimedia text source for County of Empuries parentage
  context.
- Promote Q1541699 -> Q204920 for 1341-1402.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No earlier Frankish, Girona, Barcelona, Peralada, Prades exchange, Catalan
institutional, Principality of Catalonia, border, title-holder, or later
administrative claims are imported in this packet.

## Current State

- sources: 354
- facts: 1198
- titles: 333
- parentage facts: 199
- titles with parentage: 155
- titles without parentage: 178

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/empuries-crown-aragon-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/empuries-crown-aragon-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
