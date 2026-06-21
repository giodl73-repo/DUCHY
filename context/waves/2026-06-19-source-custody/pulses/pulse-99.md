# Pulse 99: County of Astarac Duchy of Gascony Parentage

## Intent

Promote Duchy of Gascony as a reviewed parent title and close County of
Astarac's parentage gap under it.

## Changes

- Add one reviewed Wikidata source for Duchy of Gascony title identity.
- Add one reviewed Wikimedia text source for County of Astarac parentage
  context.
- Promote Duchy of Gascony name, Duchy rank, and 602-1453 existence facts.
- Promote Q1541613 -> Q2295939 for 920-1300.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No later province, House of Astarac genealogy, Armagnac relationship, Gascon
stronghold mapping, Gascony-Aquitaine succession, English Gascony, borders,
title-holder sequence, or post-medieval administrative claims are imported in
this packet.

## Current State

- sources: 361
- facts: 1209
- titles: 335
- parentage facts: 204
- titles with parentage: 160
- titles without parentage: 175

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/astarac-gascony-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/astarac-gascony-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
