# County Agent Results Summary

Aggregates completed county-scale shard reviews. These are review artifacts only; accepted fixtures are unchanged.

## Completed Shards

- county-scale-001: 50 rows; already_accepted=1, rank_policy_blocked=3, reject_bad_lead=11, source_resolution_blocked=35
- county-scale-002: 50 rows; rank_policy_blocked=2, ready_title_packet=1, reject_bad_lead=6, source_resolution_blocked=41
- county-scale-004: 50 rows; already_accepted=1, rank_policy_blocked=3, reject_bad_lead=15, source_resolution_blocked=31
- county-scale-006: 50 rows; already_accepted=2, rank_policy_blocked=5, ready_title_packet=5, reject_bad_lead=6, source_resolution_blocked=32
- county-scale-009: 50 rows; already_accepted=1, rank_policy_blocked=5, reject_bad_lead=5, source_resolution_blocked=39

## Aggregate Decisions

- already_accepted: 5
- rank_policy_blocked: 18
- ready_title_packet: 6
- reject_bad_lead: 43
- source_resolution_blocked: 178

## Ready Title Candidates

| Shard | CK ID | County | Recommended Title | Rank | Exists | Source | Next Action |
|---|---|---|---|---|---|---|---|
| county-scale-002 | c_barcelona | Barcelona | County of Barcelona | County | 801..1164 | https://www.britannica.com/place/Barcelona-historical-county-Spain | prepare_title_packet |
| county-scale-006 | c_dorohoi | Dorohoi | Dorohoi County | County | 1859..1950 | https://en.wikipedia.org/wiki/Dorohoi_County | parentage_source_followup |
| county-scale-006 | c_dortmund | Dortmund | Free imperial city of Dortmund | FreeCity | 1220..1806 | https://www.hanse.org/en/hanse/dortmund | parentage_source_followup |
| county-scale-006 | c_durham | Durham | County Palatine of Durham | TheocraticState | 883..1836 | https://en.wikipedia.org/wiki/County_Palatine_of_Durham | parentage_source_followup |
| county-scale-006 | c_pembrokeshire | Dyfed | Kingdom of Dyfed | Kingdom | 410..920 | https://www.wikidata.org/wiki/Q956451 | parentage_source_followup |
| county-scale-006 | c_essex | Essex | Kingdom of Essex | Kingdom | 527..825 | https://www.wikidata.org/wiki/Q110888 | parentage_source_followup |

## Blocking Themes

- Principality, melikdom, historical-country, ancient-city, and region/zemlja leads need rank-policy decisions before safe promotion.
- Many rows have modern places, family names, natural regions, municipalities, or wrong-place leads and need better independent historical source resolution.
- No completed shard has produced a ready parentage packet yet; the immediate harvest is title-ready candidates and explicit blockers.
