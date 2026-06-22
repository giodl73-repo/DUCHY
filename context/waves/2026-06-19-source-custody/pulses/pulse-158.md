# Pulse 158: Rank Skip Candidate Targeting

## Intent

Turn rank-skip review batches into targeted intermediate-parent searches by
cross-checking each skipped edge against accepted titles at the expected
immediate rank.

## Changes

- Add `duchy-import parentage-rank-skip-candidates`.
- Generate `data/staging/parentage-rank-skip-candidates.md`.
- For each rank-skip row, list accepted overlapping titles at the expected
  immediate rank.
- Mark stronger bridge candidates when an accepted expected-rank title already
  has reviewed parentage to the same current parent during an overlapping span.
- Document the command and current candidate counts in `README.md`.
- Add VTRACE evidence for the candidate targeting baseline.

## Boundary

This pulse suggests review targets only. It does not assert that any candidate
is the correct immediate parent, and it does not promote sources, facts, titles,
parentage spans, de facto relations, successor semantics, or territorial
geometry.

## Current State

- rank-skip rows: 222
- rows with accepted overlapping expected-rank candidates: 217
- rows with same-current-parent bridge candidates: 162
- rows without accepted overlapping expected-rank candidates: 5

## Validation

- `cargo run --quiet --bin duchy-import -- parentage-rank-skip-candidates fixtures/first-real.sources fixtures/first-real.facts data/staging/parentage-rank-skip-candidates.md`
