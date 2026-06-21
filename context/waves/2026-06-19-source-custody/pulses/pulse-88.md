# Pulse 88: Confederation of the Rhine French Empire Parentage

## Intent

Add a reviewed text-backed source for Confederation of the Rhine parentage and
close its parentage gap under the First French Empire.

## Changes

- Add one reviewed Wikimedia text source for Confederation of the Rhine.
- Promote Q154741 -> Q71084 for 1806-1813.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Confederation member inventory, Napoleonic protectorate mechanics, military
obligations, borders, constitutional acts, dissolution details, or successor
claims are imported in this packet.

## Current State

- sources: 347
- facts: 1189
- titles: 332
- parentage facts: 193
- titles with parentage: 149
- titles without parentage: 183

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/rhine-confederation-french-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/rhine-confederation-french-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
