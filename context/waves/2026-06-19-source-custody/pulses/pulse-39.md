# Pulse 39: Unsupported Queue Closure

## Intent

Close the non-promotable remainder of the 500-source candidate queue without
turning unsupported or relation-heavy candidates into accepted historical facts.

## Changes

- Mark 167 pending `contested_review` candidates with `query_readiness:
  unsupported` as rejected.
- Add `exclusion_reason: scope_deferred` to those rejected rows.
- Regenerate the candidate manifest report, TSV, TSV roundtrip manifest,
  duplicate URL report, rejected report, active manifest, archive manifest, and
  review shards.
- Leave the fourteen date-problem `title_identity_only` candidates pending for
  later source/date cleanup.

## Review Boundary

This pulse is a queue hygiene and source-custody decision only. It does not add
or remove accepted sources, facts, parentage, borders, holders, dynastic
continuity, control, or successor/predecessor relations.

## Validation

- `cargo run --bin duchy-import -- manifest data/staging/candidates-500.manifest`
- `cargo run --bin duchy-import -- rejected-report data/staging/candidates-500.manifest data/staging/candidates-500-rejected.md`
- `cargo run --bin duchy-import -- active-manifest data/staging/candidates-500.manifest data/staging/candidates-500-active.manifest`
- `cargo run --bin duchy-import -- archive-manifest data/staging/candidates-500.manifest data/staging/candidates-500-archive.manifest`
- `cargo run --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
- `cargo test --quiet`
- `cargo run --quiet`
- `cargo fmt --check`
- `git diff --check`
