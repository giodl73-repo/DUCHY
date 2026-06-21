# Pulse 98: Grand Principality of Moscow Golden Horde Parentage

## Intent

Promote Golden Horde as a reviewed parent title and close Grand Principality of
Moscow's parentage gap under it.

## Changes

- Add one reviewed Wikidata source for Golden Horde title identity.
- Add one reviewed Wikimedia text source for Grand Principality of Moscow
  parentage context.
- Promote Golden Horde name, Empire rank, and 1242-1502 existence facts.
- Promote Q170770 -> Q141472 for 1263-1478.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No tribute mechanics, grand-princely patent competition, Horde internal
succession, Moscow-Tver rivalry, Novgorod annexation, Russian centralized-state
formation, 1480 military campaign details, later Great Horde claims, or Tsardom
of Russia claims are imported in this packet.

## Current State

- sources: 359
- facts: 1205
- titles: 334
- parentage facts: 203
- titles with parentage: 159
- titles without parentage: 175

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/moscow-golden-horde-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/moscow-golden-horde-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
