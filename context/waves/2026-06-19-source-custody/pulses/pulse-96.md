# Pulse 96: Duchy of Parma and Piacenza Empire Parentage

## Intent

Add a reviewed text-backed source for Duchy of Parma and Piacenza parentage and
close its parentage gap under the Holy Roman Empire.

## Changes

- Add one reviewed Wikimedia text source for Duchy of Parma and Piacenza
  parentage context.
- Promote Q165040 -> Q12548 for 1748-1801.
- Preserve the source packet, relation screen, dry-run report, and apply report
  in staging.
- Regenerate the parentage coverage report, gap TSV, shards, and shard reports.

## Boundary

No papal creation, Farnese succession, Austrian administration, Bourbon-Parma
succession, French protectorate, Napoleonic department, Restoration after 1814,
territorial borders, title-holder, Guastalla, or Italian unification claims are
imported in this packet.

## Current State

- sources: 356
- facts: 1200
- titles: 333
- parentage facts: 201
- titles with parentage: 157
- titles without parentage: 176

## Validation

- `cargo run --quiet --bin duchy-promote -- --dry-run --report data/staging/parma-piacenza-empire-parentage-01-report.md`
- `cargo run --quiet --bin duchy-promote -- --apply --report data/staging/parma-piacenza-empire-parentage-01-apply-report.md`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo run --quiet --bin duchy-import -- parentage-coverage-report fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-coverage-report.md`
- `cargo run --quiet --bin duchy-import -- parentage-gap-tsv fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-gap-targets.tsv`
- `cargo run --quiet --bin duchy-import -- parentage-gap-shard data/staging/parentage-gap-targets.tsv data/staging/parentage-gap-shards 25`
