# Pulse 40: Date-Problem Queue Closure

## Intent

Close the final active rows in the 500-source candidate queue without importing
title spans that failed the structured source gate.

## Changes

- Mark the remaining fourteen pending `title_identity_only` candidates as
  rejected.
- Use `exclusion_reason: quality_blocked` for thirteen candidates with missing
  structured inception or dissolution years.
- Use `exclusion_reason: date_conflict` for the Kingdom of Germany candidate
  whose structured dates produced a reversed span.
- Regenerate the candidate manifest report, TSV, TSV roundtrip manifest,
  duplicate URL report, rejected report, archive manifest, and review shards.
- Remove the active manifest artifact because the queue has no pending or
  reviewed candidates left.

## Review Boundary

This pulse is a queue closure decision only. It does not add or remove accepted
sources, facts, parentage, borders, holders, dynastic continuity, control, or
successor/predecessor relations.

## Validation

- `cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest`
- `cargo run --bin duchy-import -- rejected-report data/staging/candidates-500.manifest data/staging/candidates-500-rejected.md`
- `cargo run --bin duchy-import -- archive-manifest data/staging/candidates-500.manifest data/staging/candidates-500-archive.manifest`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
