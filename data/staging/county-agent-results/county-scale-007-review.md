# County Scale 007 Review

## Scope

Reviewed only `county-scale-007` research rows. CK3 rows were treated as search
drivers, not source-backed facts. No fixtures, source files, queue files, source
code, or other shard artifacts were edited.

## Decision Counts

| Decision | Count |
|---|---:|
| source_resolution_blocked | 26 |
| rank_policy_blocked | 8 |
| ready_title_packet | 6 |
| reject_bad_lead | 6 |
| already_accepted | 4 |
| ready_parentage_packet | 0 |

## Notable Rows

- `c_finland`, `c_geneva`, `c_gorz`, and `c_granada` already have accepted
  fixture parentage and are recorded as metrics seeds only.
- `c_foix`, `c_forcalquier`, `c_forez`, `c_fulda`, `c_furstenberg`, and
  `c_girona` have plausible title packets, but parentage is not ready without
  separate direct source custody.
- `c_finnveden`, `c_frisia`, `c_galatia`, `c_galatia_salutaris`,
  `c_allenstein`, `c_galloway`, `c_gallura`, and `c_grisons` are useful rank
  policy blockers: their better leads are regions, ancient provinces,
  lordships, judicates, or league/republic polities rather than currently
  materialized county-style ranks.
- `c_fraga`, `c_frankfurt`, `c_freistadt`, `c_gacka`, `c_gleichenstein`, and
  `c_gowrie` have bad or wrong-place top leads and need different search terms
  before promotion work.

## Source Follow-Up

Rows with better leads but no safe parentage packet include `c_friuli`,
`c_fogaras`, `c_gemer`, `c_genoa`, `c_gastrikland`, `c_gevaudan`,
`c_gloucestershire`, `c_gottingen`, and `c_gutland`. These should be revisited
with direct title-history sources before any fixture import.

The Old Norse fylki-style rows `c_firdafylki` and `c_gauldala` remain unresolved
source-resolution work; no independent bounded title source was found in this
pass.
