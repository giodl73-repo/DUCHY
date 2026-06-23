# DUCHY Relation Model Work Package Review

source_inputs:

- `data/staging/county-rank-skip-review-batch-01.tsv`
- `data/staging/county-rank-skip-review-batch-02.tsv`
- `data/staging/parentage-rank-skip-review-batch-03.tsv`
- `data/staging/parentage-rank-skip-review-batch-04.tsv`

## Boundary

- This is not an import packet.
- This is not a schema migration.
- This review turns repeated rank-skip blockers into a VTRACE work package for
  relation facts that are not strict parentage.

## Finding

The high-priority rank-skip queue is no longer primarily blocked by missing
facts. It is blocked by relation semantics that DUCHY currently flattens into
parentage or rejects from parentage replacement.

Across the first 40 reviewed high-priority rank-skip keys:

| Class | Rows |
|---|---:|
| importable replacement facts | 0 |
| relation-model blockers | 27 |
| split-control relation-model blockers | 4 |
| child-level source blockers | 6 |
| source-custody tightening rows | 2 |
| rank-identity blockers | 1 |

## Relation Types Needed

| Relation type | Why parentage is insufficient | Examples from reviewed blockers |
|---|---|---|
| `imperial_state` | HRE state/estate/principality status is membership in an imperial polity, not proof of a missing kingdom parent. | Anhalt, Brunswick, Bavaria, Austria, Bohemia, Savoy, Nassau |
| `confederation_member` | German Confederation and North German Confederation membership should not imply Bavaria or Prussia parentage. | Anhalt, Brunswick |
| `federal_state_member` | German Empire member-state status should not be rerouted through another kingdom unless a child-level source proves it. | Duchy of Anhalt |
| `composite_crown_component` | Composite monarchies and crowns can contain titles without behaving like CK-style immediate parentage. | Crown of Aragon, Kingdom of Great Britain, Polish-Lithuanian Commonwealth |
| `split_fief_or_control` | Some titles were simultaneously or sequentially French, Imperial, Burgundian, Habsburg, or other fiefs/control domains. | Flanders, Holland, Namur |
| `vassalage_or_suzerainty` | Vassalage can explain dependency without a normal title-tree parent. | Amalfi, Serbia, United Principalities, Montenegro, Vladimir |
| `subdivision_or_appanage` | Subdivision, apanage, and partition relationships are not always parentage edges. | Brunswick-Wolfenbuttel, Empuries, Anhalt partitions |
| `rank_transition` | Elevation from county/duchy/kingdom is an event, not necessarily a parentage replacement. | Bohemia, Wurttemberg, Guelders, Poland |

## Proposed Work Package

Create `WP-007: Non-parentage relation model and query integration`.

Exit should require:

- a relation fact kind distinct from `parentage`,
- typed relation variants for at least the relation types above,
- source-backed fixture parsing and validation for relation facts,
- query trace fields that can show relation facts beside parentage without
  replacing the parentage path,
- rank-skip reports that can suppress or reclassify rows explained by accepted
  relation facts,
- tests proving that relation facts do not create parentage conflicts or
  title-path cycles.

## Next Step

Implement the VTRACE package before continuing large-scale rank-skip imports.
After relation facts exist, replay the reviewed blockers as relation packets and
rerun the parentage graph/rank-skip reports to measure which skips are explained
rather than missing.
