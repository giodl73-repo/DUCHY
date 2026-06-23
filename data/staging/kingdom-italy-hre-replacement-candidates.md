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
| accepted_replacement_imported | 2 |
| held_for_relation_model | 1 |

## Replacement Candidates

| Child | Current Parent | Candidate Parent | Span | Decision |
|---|---|---|---|---|
| March of Turin | Holy Roman Empire | Kingdom of Italy | 964..1091 | imported as replacement parentage; older direct HRE fact retained for audit |
| March of Tuscany | Holy Roman Empire | Kingdom of Italy | 962..1197 | imported as replacement parentage; older direct HRE fact retained for audit |

## Held Lead

| Child | Current Parent | Candidate Parent | Span | Decision |
|---|---|---|---|---|
| Prince-Bishopric of Trent | Holy Roman Empire | Kingdom of Italy | 1027..1803 | held; current source supports HRE estate/state, not accepted child parentage under Kingdom of Italy |

## Engineering Need

DUCHY now has partial parentage replacement support. March of Turin and March
of Tuscany have moved from candidate rows to accepted replacement facts. Trent
remains held because its current source basis supports HRE estate/state
semantics, not accepted child parentage under the Kingdom of Italy.

## Sources Checked

- `https://en.wikipedia.org/wiki/March_of_Turin`
- `https://en.wikipedia.org/wiki/March_of_Tuscany`
- `https://en.wikipedia.org/wiki/Kingdom_of_Italy_(Holy_Roman_Empire)`
- `https://en.wikipedia.org/wiki/Prince-Bishopric_of_Trent`
