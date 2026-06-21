# Pulse 97: Kingdom of Naples Crown of Aragon Parentage

## Intent

Add a reviewed text-backed source for Kingdom of Naples parentage and close its
parentage gap under the Crown of Aragon.

## Changes

- Add one reviewed Wikimedia text source for Kingdom of Naples parentage
  context.
- Promote Q173065 -> Q204920 for 1442-1458.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No Angevin claim, Aragonese conquest campaign mechanics, French personal union,
Spanish viceroyalty, Habsburg monarchy, Bourbon restoration, Napoleonic rule,
territorial borders, title-holder sequence, or Two Sicilies merger claims are
imported in this packet.

## Current State

- sources: 357
- facts: 1201
- titles: 333
- parentage facts: 202
- titles with parentage: 158
- titles without parentage: 175

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/naples-crown-aragon-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/naples-crown-aragon-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
