# County Scale 008 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-008.tsv`.
No fixtures, source code, existing queue files, or other shard result files were
edited. CK3 rows were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `already_accepted` | 1 |
| `rank_policy_blocked` | 6 |
| `source_resolution_blocked` | 36 |
| `reject_bad_lead` | 7 |

## Priority Rows

### 355 Guelders

Decision: `source_resolution_blocked`.

The better lead is the locally materialized `title-q152420` / Duchy of
Guelders. Britannica's Gelre lead supports the countship/duchy identity, and
fixtures already contain title facts, but this shard has no accepted parentage
span. Treat as parentage source review, not a new title import.

Source:

- https://www.britannica.com/place/Gelre

### 386 Holland

Decision: `source_resolution_blocked`.

The top lead was the modern Netherlands, but the useful historical candidate is
the locally materialized `title-q762943` / County of Holland. Fixtures already
contain title facts. No direct parentage custody was established in this shard,
so the row remains a parentage follow-up blocker.

Source:

- https://www.wikidata.org/wiki/Q762943

### 389 Hordaland

Decision: `already_accepted`.

`title-q50625` / Hordaland already has accepted title facts and parentage to
`title-q20` / Kingdom of Norway for `1919..2019`. Use it only as a metrics
seed; do not re-import it.

Source:

- https://www.wikidata.org/wiki/Q50625

### 362 Hainaut

Decision: `source_resolution_blocked`.

The better lead is County of Hainaut. The source lead supports a medieval county
identity and a plausible `1071..1797` span, but no direct parentage span is
ready from this pass. It should continue through parentage source review.

Source:

- https://en.wikipedia.org/wiki/County_of_Hainaut

### 358 Gwent

Decision: `source_resolution_blocked`.

The top lead was the modern preserved county, while the useful historical lead
is Kingdom of Gwent. The kingdom identity is plausible, but bounded existence
and parentage were not established enough for a packet.

Source:

- https://www.wikidata.org/wiki/Q1570649

## Rank Policy Blockers

Six rows resolved to historically meaningful entities whose rank is not
currently materialized for county parentage:

- `c_guria` -> Principality of Guria, principality rank.
- `c_hadrianeia` -> Hadrianeia, ancient city rank.
- `c_halland` and `c_halsingland` -> Swedish/Danish provincial material.
- `c_helenopontus` and `c_honorias` -> Roman province material.

These rows need rank policy before title or parentage promotion.

## Bad Leads

Seven rows had top leads that should not be promoted:

- `c_grunningen` -> protected area in Norway.
- `c_hayk` -> particle physicist.
- `c_hedmork` -> Icelandic conservation area instead of Norwegian Hedmark.
- `c_hohenau` -> Paraguay city/district.
- `c_hont` -> family name.
- `c_hotin` -> 2021 album.
- `c_hunyad` -> Hunyadi family name.

## Other Notable Rows

`c_hvosno` has a promising medieval Serbian county lead, and `c_guines` has a
promising County of Guines lead, but neither has bounded title existence and
parentage custody ready.

English historic-county rows such as Hampshire, Herefordshire, Hertfordshire,
and Huntingdonshire remain blocked on historic-county rank/date treatment.

Several rows are likely real historical regions or kingdoms, including Hereti,
Hy Many, Gudbrandsdalen, Harjedalen, and Halych/Galicia material, but each needs
better bounded source custody before packet work.
