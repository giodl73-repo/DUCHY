# DUCHY Kingdom of Italy / HRE Replacement Candidates

source_split: `data/staging/kingdom-italy-hre-child-split-review.tsv`
candidate_tsv: `data/staging/kingdom-italy-hre-replacement-candidates.tsv`

## Boundary

- This is not an accepted fact packet.
- These rows are replacement candidates for existing direct HRE parentage, not
  additive parentage.
- Importing these as additional parentage facts would create temporal parent
  conflicts because the current direct HRE facts cover the same spans.

## Candidate Counts

| Status | Rows |
|---|---:|
| source_backed_replacement_candidate | 2 |
| held_for_relation_model | 1 |

## Replacement Candidates

| Child | Current Parent | Candidate Parent | Span | Decision |
|---|---|---|---|---|
| March of Turin | Holy Roman Empire | Kingdom of Italy | 964..1091 | source-backed replacement candidate; blocked until replacement/deprecation support exists |
| March of Tuscany | Holy Roman Empire | Kingdom of Italy | 962..1197 | source-backed replacement candidate; blocked until replacement/deprecation support exists |

## Held Lead

| Child | Current Parent | Candidate Parent | Span | Decision |
|---|---|---|---|---|
| Prince-Bishopric of Trent | Holy Roman Empire | Kingdom of Italy | 1027..1803 | held; current source supports HRE estate/state, not accepted child parentage under Kingdom of Italy |

## Engineering Need

DUCHY needs an explicit parentage replacement/deprecation mechanism before
accepted fixtures can safely replace a broad direct parent edge with an
intermediate parent. The mechanism should:

- identify the current fact being superseded;
- preserve source custody for the original and replacement facts;
- prevent overlapping active parents for the same child span;
- regenerate graph health reports with zero temporal parent conflicts;
- keep a review note explaining why the replacement is a hierarchy refinement,
  not a new simultaneous parent.

## Sources Checked

- `https://en.wikipedia.org/wiki/March_of_Turin`
- `https://en.wikipedia.org/wiki/March_of_Tuscany`
- `https://en.wikipedia.org/wiki/Kingdom_of_Italy_(Holy_Roman_Empire)`
- `https://en.wikipedia.org/wiki/Prince-Bishopric_of_Trent`
