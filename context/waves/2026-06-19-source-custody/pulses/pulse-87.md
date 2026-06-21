# Pulse 87: Confederation of the Rhine Westphalia Parentage

## Intent

Promote Confederation of the Rhine as a reviewed composite parent title and
close the Kingdom of Westphalia parentage gap with a reviewed membership
relation.

## Changes

- Promote Q154741 Confederation of the Rhine as a `Crown` composite parent
  title.
- Promote Q153943 -> Q154741 for 1807-1813.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Confederation member inventories beyond Westphalia, Napoleonic protectorate
mechanics, military obligations, borders, constitutional acts, Confederation
dissolution details, or successor claims are imported in this packet.

## Current State

- sources: 346
- facts: 1188
- titles: 332
- parentage facts: 192
- titles with parentage: 148
- titles without parentage: 184

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/rhine-confederation-westphalia-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/rhine-confederation-westphalia-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
