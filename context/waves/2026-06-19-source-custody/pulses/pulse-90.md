# Pulse 90: Kingdom of Imereti Russian Empire Parentage

## Intent

Add a reviewed text-backed source for Kingdom of Imereti parentage and close
its parentage gap under the Russian Empire.

## Changes

- Add one reviewed Wikimedia text source for Georgia within the Russian Empire.
- Promote Q1069959 -> Q34266 for 1804-1810.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Georgian dynastic sequence, Mingrelia revolt mechanics, Ottoman or Persian
claims, Russian military campaign details, Kutaisi occupation details,
annexation administration, later Imereti province facts, borders, or successor
claims are imported in this packet.

## Current State

- sources: 349
- facts: 1191
- titles: 332
- parentage facts: 195
- titles with parentage: 151
- titles without parentage: 181

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/imereti-russian-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/imereti-russian-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
