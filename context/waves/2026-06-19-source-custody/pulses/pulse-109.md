# Pulse 109: Duchy of Austria HRE Parentage

## Intent

Close Duchy of Austria's parentage gap under the Holy Roman Empire for its
reviewed medieval duchy span.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Austria parentage context.
- Promote Q3624335 -> Q12548 for 1156-1453.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Margraviate of Austria continuation detail, Bavarian detachment politics,
Babenberg succession dispute, Habsburg dynastic claims, Privilegium Maius
detail, Archduchy of Austria continuation, Habsburg monarchy, Austrian Empire,
or later Austrian claims are imported in this packet.

## Current State

- sources: 371
- facts: 1219
- titles: 335
- parentage facts: 214
- titles with parentage: 170
- titles without parentage: 165

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/austria-duchy-hre-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/austria-duchy-hre-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
