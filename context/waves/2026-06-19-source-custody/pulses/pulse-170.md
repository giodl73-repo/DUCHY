# Pulse 170: Kingdom of Italy HRE Replacement Candidates

## Intent

Carry the Kingdom of Italy / Holy Roman Empire child split through one more
review step without creating overlapping parentage facts. The child-level
sources identify two hierarchy refinement candidates, but both would replace
existing direct HRE parentage rather than add a simultaneous parent.

## Changes

- Add `data/staging/kingdom-italy-hre-replacement-candidates.tsv`.
- Add `data/staging/kingdom-italy-hre-replacement-candidates.md`.
- Add text-source stubs for March of Turin and March of Tuscany replacement
  review.

## Candidates

- March of Turin: source-backed replacement candidate from direct Holy Roman
  Empire parentage to Kingdom of Italy parentage for `964..1091`.
- March of Tuscany: source-backed replacement candidate from direct Holy Roman
  Empire parentage to Kingdom of Italy parentage for `962..1197`.
- Prince-Bishopric of Trent: held for relation modeling; the current source
  supports HRE estate/state semantics, not accepted child parentage under the
  Kingdom of Italy.

## Boundary

This pulse does not promote sources into accepted fixtures and does not replace
any accepted parentage facts. Importing these rows additively would create
temporal parent conflicts because the current direct HRE facts cover the same
spans. Accepted fixture changes are blocked until DUCHY has explicit
parentage replacement/deprecation support.

## Current State

- bridge-cluster review rows: 20
- pending bridge-cluster rows: 0
- reviewed false-positive rows: 19
- mixed child-split rows: 1
- child split rows: 31
- replacement candidates: 2
- held relation-model leads: 1
- ready_for_packet rows: 0
- packet stubs: 0
- accepted source/fact/title counts: unchanged

## Validation

- `Import-Csv -Path data/staging/kingdom-italy-hre-child-split-review.tsv -Delimiter "`t"`: 31 rows, 3 leads, 28 false positives.
- `Import-Csv -Path data/staging/kingdom-italy-hre-replacement-candidates.tsv -Delimiter "`t"`: 3 rows, 2 replacement candidates, 1 held lead.
- `cargo test --quiet parentage_rank_skip_bridge_cluster`
- `git diff --check`
