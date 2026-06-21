# Pulse 91: Italian Protectorate of Albania Italian Empire Parentage

## Intent

Promote Italian Empire as a reviewed parent title and close the Italian
protectorate of Albania parentage gap under it.

## Changes

- Add one reviewed Wikidata source for Italian Empire.
- Add one reviewed Wikimedia text source for Albania's Italian Empire parentage.
- Promote Italian Empire title identity facts for Q926295.
- Promote Q1448131 -> Q926295 for 1939-1943.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Italian occupation operations, personal-union mechanics, governors, Greater
Albania claims, Albanian Fascist Party details, military organization, borders,
German occupation, or Communist Albania successor claims are imported in this
packet.

## Current State

- sources: 351
- facts: 1195
- titles: 333
- parentage facts: 196
- titles with parentage: 152
- titles without parentage: 181

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/albania-italian-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/albania-italian-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
