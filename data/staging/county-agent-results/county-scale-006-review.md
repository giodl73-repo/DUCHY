# County Scale 006 Review

## Scope

Reviewed only `county-scale-006` research rows. CK3 rows were treated as search
drivers, not source-backed facts. No fixtures, source files, queue files, or
other shard artifacts were edited.

## Decision Counts

| Decision | Count |
|---|---:|
| source_resolution_blocked | 32 |
| ready_title_packet | 5 |
| rank_policy_blocked | 5 |
| reject_bad_lead | 6 |
| already_accepted | 2 |
| ready_parentage_packet | 0 |

## Notable Rows

- `c_duklja` is already accepted from local fixtures and pilot review:
  Duklja -> Byzantine Empire for `854..1252`. It is recorded as a metrics seed
  only.
- `c_ferrara` has accepted endpoint parentage in fixtures:
  Duchy of Ferrara -> Papal States at `1597..1597`. It is recorded as already
  accepted, not a new packet.
- `c_donjikraji` remains a rank-policy blocker. Local source-only custody
  identifies Donji Kraji as a medieval Bosnian `zemlja` or historical region
  with `1230..1463`, but that rank is not materialized.
- `c_dyrrachion`, `c_epeiros`, `c_ditmarschen`, and `c_euchaita` also block on
  rank semantics: theme, despotate, peasant republic/region, and ancient site
  are not safe county-title promotions under current policy.
- `c_dorohoi`, `c_dortmund`, `c_pembrokeshire`, `c_durham`, and `c_essex` have
  plausible title packets or already accepted title facts, but not enough direct
  parentage custody for promotion.

## Source Follow-Up

Several rows have better historical leads but still need bounded title or
parentage custody before any promotion:

- `c_eu`: County of Eu creation is supported, but a safe end date and parent
  span were not established.
- `c_east_riding`, `c_devon`, `c_dorset`, and `c_fife`: historic county context
  is supportable, but bounded `title_exists` spans and parentage remain open.
- `c_dublin`, `c_eichstadt`, `c_falkenstein`, `c_fermo`, and `c_euboea`: likely
  alternate historical title material exists, but the top shard leads are modern
  places or geographic entities and should not be promoted directly.

## Bad Leads

Rejected lead rows are explicit so they can be re-searched with better terms:

- `c_dobrzynska`, `c_dunbar`, `c_durne`, `c_erbach`, and `c_feher` resolve to
  family-name items.
- `c_dvin` resolves to Daugavpils, a wrong-place lead for the Armenian Dvin
  seed.
