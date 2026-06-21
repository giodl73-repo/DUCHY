# County Agent Results Summary

Aggregates completed county-scale shard reviews. These are review artifacts only; accepted fixtures are unchanged.

## Completed Shards

- county-scale-001: 50 rows; already_accepted=1, rank_policy_blocked=3, reject_bad_lead=11, source_resolution_blocked=35
- county-scale-002: 50 rows; rank_policy_blocked=2, ready_title_packet=1, reject_bad_lead=6, source_resolution_blocked=41
- county-scale-003: 50 rows; rank_policy_blocked=4, ready_title_packet=4, reject_bad_lead=10, source_resolution_blocked=32
- county-scale-004: 50 rows; already_accepted=1, rank_policy_blocked=3, reject_bad_lead=15, source_resolution_blocked=31
- county-scale-005: 50 rows; already_accepted=1, rank_policy_blocked=6, ready_title_packet=3, reject_bad_lead=9, source_resolution_blocked=31
- county-scale-006: 50 rows; already_accepted=2, rank_policy_blocked=5, ready_title_packet=5, reject_bad_lead=6, source_resolution_blocked=32
- county-scale-007: 50 rows; already_accepted=4, rank_policy_blocked=8, ready_title_packet=6, reject_bad_lead=6, source_resolution_blocked=26
- county-scale-008: 50 rows; already_accepted=1, rank_policy_blocked=6, reject_bad_lead=7, source_resolution_blocked=36
- county-scale-009: 50 rows; already_accepted=1, rank_policy_blocked=5, reject_bad_lead=5, source_resolution_blocked=39
- county-scale-010: 50 rows; already_accepted=2, rank_policy_blocked=8, ready_title_packet=2, reject_bad_lead=12, source_resolution_blocked=26

## Aggregate Decisions

- already_accepted: 13
- rank_policy_blocked: 50
- ready_title_packet: 21
- reject_bad_lead: 87
- source_resolution_blocked: 329

## Ready Title Candidates

| Shard | CK ID | County | Recommended Title | Rank | Exists | Source | Next Action |
|---|---|---|---|---|---|---|---|
| county-scale-002 | c_barcelona | Barcelona | County of Barcelona | County | 801..1164 | https://www.britannica.com/place/Barcelona-historical-county-Spain | prepare_title_packet |
| county-scale-003 | c_benevento | Benevento | Duchy of Benevento | Duchy | 571..774 | https://www.historyfiles.co.uk/KingListsEurope/ItalyBenevento.htm | parentage_source_followup |
| county-scale-003 | c_brabant | Brabant | Duchy of Brabant | Duchy | 1183..1795 | https://www.wikidata.org/wiki/Q159856 | parentage_source_followup |
| county-scale-003 | c_brandenburg | Brandenburg | Margraviate of Brandenburg | Duchy | 1157..1806 | https://www.wikidata.org/wiki/Q148499 | parentage_source_followup |
| county-scale-003 | c_breifne | Breifne | Kingdom of Breifne | Kingdom | 700..1256 | https://www.wikidata.org/wiki/Q905131 | parentage_source_followup |
| county-scale-005 | c_cieszyn | Cieszyn | Duchy of Teschen | Duchy | 1290..1918 | https://www.britannica.com/place/Teschen | parentage_source_followup |
| county-scale-005 | c_denia | Denia | Taifa of Denia | Kingdom | 1010..1227 | https://en.wikipedia.org/wiki/Taifa_of_D%C3%A9nia | parentage_source_followup |
| county-scale-005 | c_desmond | Desmond | Kingdom of Desmond | Kingdom | 1118..1596 | https://en.wikipedia.org/wiki/Kingdom_of_Desmond | parentage_source_followup |
| county-scale-006 | c_dorohoi | Dorohoi | Dorohoi County | County | 1859..1950 | https://en.wikipedia.org/wiki/Dorohoi_County | parentage_source_followup |
| county-scale-006 | c_dortmund | Dortmund | Free imperial city of Dortmund | FreeCity | 1220..1806 | https://www.hanse.org/en/hanse/dortmund | parentage_source_followup |
| county-scale-006 | c_durham | Durham | County Palatine of Durham | TheocraticState | 883..1836 | https://en.wikipedia.org/wiki/County_Palatine_of_Durham | parentage_source_followup |
| county-scale-006 | c_essex | Essex | Kingdom of Essex | Kingdom | 527..825 | https://www.wikidata.org/wiki/Q110888 | parentage_source_followup |
| county-scale-006 | c_pembrokeshire | Dyfed | Kingdom of Dyfed | Kingdom | 410..920 | https://www.wikidata.org/wiki/Q956451 | parentage_source_followup |
| county-scale-007 | c_foix | Foix | County of Foix | County | 1012..1607 | https://en.wikipedia.org/wiki/County_of_Foix | parentage_source_followup |
| county-scale-007 | c_forcalquier | Forcalquier | County of Forcalquier | County | 1054..1481 | https://en.wikipedia.org/wiki/County_of_Forcalquier | parentage_source_followup |
| county-scale-007 | c_forez | Forez | County of Forez | County | 955..1531 | https://en.wikipedia.org/wiki/County_of_Forez | parentage_source_followup |
| county-scale-007 | c_fulda | Fulda | Princely Abbey of Fulda | TheocraticState | 1220..1802 | https://en.wikipedia.org/wiki/Princely_Abbey_of_Fulda | parentage_source_followup |
| county-scale-007 | c_furstenberg | Furstenberg | County of Furstenberg | County | 1250..1806 | https://en.wikipedia.org/wiki/County_of_F%C3%BCrstenberg | parentage_source_followup |
| county-scale-007 | c_girona | Girona | County of Girona | County | 785..897 | https://www.wikidata.org/wiki/Q2037817 | parentage_source_followup |
| county-scale-010 | c_komarom | Komárom | Komárom County | County | 11th century..1923 | https://en.wikipedia.org/wiki/Kom%C3%A1rom_County | parentage_source_followup |
| county-scale-010 | c_la_marche | La Marche | County of La Marche | County | c.958..1527 | https://en.wikipedia.org/wiki/County_of_La_Marche | parentage_source_followup |

## Blocking Themes

- Principality, melikdom, historical-country, ancient-city, Byzantine theme, region/zemlja, province, county-palatine, and theocratic or ecclesiastical leads need rank-policy decisions before safe promotion.
- Many rows have modern places, family names, natural regions, municipalities, wrong-place leads, islands, valleys, or article/disambiguation leads and need better independent historical source resolution.
- No completed shard has produced a ready parentage packet yet; the immediate harvest is title-ready candidates and explicit blockers.
