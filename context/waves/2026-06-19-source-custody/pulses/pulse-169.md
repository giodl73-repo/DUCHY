# Pulse 169: Kingdom of Italy HRE Child Split Review

## Intent

Resolve the remaining mixed bridge-cluster row without promoting row-level
parentage. The Kingdom of Italy / Holy Roman Empire cluster combines possible
Imperial Italy child leads with many obvious same-parent HRE peer states.

## Changes

- Add `data/staging/kingdom-italy-hre-child-split-review.tsv`.
- Add `data/staging/kingdom-italy-hre-child-split-review.md`.
- Classify 31 children from the mixed cluster into:
  - 3 `candidate_child_review_lead` rows.
  - 28 `same_parent_sibling_false_positive` rows.

## Candidate Leads

- March of Turin.
- March of Tuscany.
- Prince-Bishopric of Trent.

## Boundary

This pulse changes staging review metadata only. It does not promote sources,
facts, titles, parentage spans, Imperial Italy membership, de facto control,
successor semantics, or geometry. Each lead still needs source-backed
child-level parentage review with a bounded span before any packet can be
created.

## Current State

- bridge-cluster review rows: 20
- pending bridge-cluster rows: 0
- reviewed false-positive rows: 19
- mixed child-split rows: 1
- child split rows: 31
- child split candidate leads: 3
- child split false positives: 28
- ready_for_packet rows: 0
- packet stubs: 0
- accepted source/fact/title counts: unchanged

## Validation

- `Import-Csv -Path data/staging/kingdom-italy-hre-child-split-review.tsv -Delimiter "`t"`: 31 rows, 3 leads, 28 false positives.
- `cargo test --quiet parentage_rank_skip_bridge_cluster`
- `git diff --check`
