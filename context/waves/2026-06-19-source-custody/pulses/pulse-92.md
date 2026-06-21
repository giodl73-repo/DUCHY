# Pulse 92: Francia Carolingian Empire Parentage

## Intent

Add a reviewed text-backed source for Francia parentage and close its parentage
gap under the Carolingian Empire.

## Changes

- Add one reviewed Wikimedia text source for Carolingian Empire parentage
  context.
- Promote Q146246 -> Q31929 for 800-843.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Carolingian holder sequence, imperial coronation theology, Treaty of Verdun
partition details, West/Middle/East Francia successor facts, borders, claims to
Holy Roman Empire continuity, or feudalism/legacy claims are imported in this
packet.

## Current State

- sources: 352
- facts: 1196
- titles: 333
- parentage facts: 197
- titles with parentage: 153
- titles without parentage: 180

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/francia-carolingian-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/francia-carolingian-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
