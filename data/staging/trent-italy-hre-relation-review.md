# DUCHY Trent / Kingdom of Italy Relation Review

source_split: `data/staging/kingdom-italy-hre-child-split-review.tsv`
review_tsv: `data/staging/trent-italy-hre-relation-review.tsv`

## Boundary

- This is not an import packet.
- The reviewed question is whether Prince-Bishopric of Trent can replace its
  direct Holy Roman Empire parentage with a bounded Kingdom of Italy parentage
  span.
- The current evidence does not support that replacement.

## Decision

`blocked_relation_model`

Keep Prince-Bishopric of Trent parented directly under the Holy Roman Empire for
`1027..1803`. Do not import a Trent -> Kingdom of Italy replacement fact from
the current evidence.

## Evidence Read

- Accepted Trent source custody identifies the Prince-Bishopric of Trent as an
  Imperial estate/state of the Holy Roman Empire for `1027..1803`.
- The Kingdom of Italy/HRE source identifies the Kingdom of Italy as a
  constituent kingdom of the Holy Roman Empire.
- The same Kingdom of Italy/HRE context treats Trent as an Italian imperial
  state represented through imperial institutions, but does not establish a
  clean child-parent relation from Trent to the Kingdom of Italy.

## Resulting Queue State

| Status | Rows |
|---|---:|
| active Italy/HRE split leads | 1 |
| importable Trent replacement facts | 0 |
| blocked relation-model leads | 1 |

## Next Step

Leave the Italy/HRE cluster blocked. Reopen only if a child-level source
explicitly supports Trent under the Kingdom of Italy with a bounded span, or if
DUCHY adds a relation model for imperial fiefs/circles that is distinct from
parentage.
