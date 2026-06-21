# County Scale 001 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-001.tsv`.
No fixtures, source code, queue files, or other shard result files were edited.
CK3 rows were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `already_accepted` | 1 |
| `rank_policy_blocked` | 3 |
| `source_resolution_blocked` | 35 |
| `reject_bad_lead` | 11 |

## Priority Rows

### 5 Abauj

Decision: `already_accepted`.

This row is a metrics seed only. The accepted local title is `title-q1049854` /
Abaúj county, rank `County`, exists `1201..1881`, with accepted parentage to
`title-q171150` / Kingdom of Hungary for `1201..1881`. The row is already
covered by fixtures and the pilot review; do not re-import it.

Source:

- https://www.wikidata.org/wiki/Q1049854

### 16 Ailech

Decision: `rank_policy_blocked`.

The local reviewed source-resolution record `src-wikidata-q15104874` already
accepts structured claims for Ailech only as a source-resolution record. It is a
historical-country entity with structured `600..780` existence, not a DUCHY
county rank. The pilot review also calls this out as a rank-semantics blocker.

Recommended action: define historical-country rank semantics or keep deferred.

Source:

- https://www.wikidata.org/wiki/Q15104874

### 11 Achaia

Decision: `rank_policy_blocked`.

The CK3 county seed resolves more plausibly to the medieval Principality of
Achaea than to the modern Achaea Regional Unit. The Byzantine Legacy describes
the Principality of Achaea as a Frankish Peloponnesian polity ruled by princes
from 1205 to 1430. That is useful source custody for identity, but
`Principality` is not currently a materialized DUCHY rank for county promotion.

Recommended action: define principality rank semantics or keep deferred.

Source:

- https://www.thebyzantinelegacy.com/achaia

## Useful Source Leads

### 15 Agenais

Decision: `source_resolution_blocked`.

The top CK3/Wikidata lead is modern Agen. A better lead exists: Britannica 1911
treats Agenais/Agenois as a former province that became a hereditary countship.
This pass did not establish a safe bounded `title_exists` span or parentage, so
it remains a source-resolution blocker rather than a packet.

Source:

- https://en.wikisource.org/wiki/1911_Encyclop%C3%A6dia_Britannica/Agenais

### 28 Alencon

Decision: `source_resolution_blocked`.

Britannica supports Alençon as capital of the county and duchy of Alençon and
notes its incorporation into Normandy in 911 and passage to the French crown in
1549. That confirms a useful historical title lead, but this pass did not
establish bounded title facts or parentage safe enough for promotion.

Source:

- https://www.britannica.com/place/Alencon

### 40 Angouleme

Decision: `source_resolution_blocked`.

Britannica supports Angoulême as the seat of counts from the 9th century.
Encyclopedia.com/Columbia also treats Angoumois as a county in the 9th century
and later crown land. The lead is useful, but bounded title dates and parentage
still need source custody.

Sources:

- https://www.britannica.com/place/Angouleme
- https://www.encyclopedia.com/reference/encyclopedias-almanacs-transcripts-and-maps/angoumois

### 42 Anjou

Decision: `source_resolution_blocked`.

The queue's top lead is the wrong modern commune. A better title candidate is
`title-q551476` / County of Anjou. Fordham's medieval sourcebook gives a
historical counts-of-Anjou source lead, and Wikidata has a bounded county
candidate (`861..1360`), but this pass did not establish independent parentage
evidence.

Source:

- https://sourcebooks.web.fordham.edu/source/Anjou.asp

### 47 Aosta

Decision: `source_resolution_blocked`.

Britannica supports Aosta city, bishopric, and later regional context. It does
not establish a bounded county/title packet for this workflow. Possible Aosta
Valley or comital material needs a sharper source before promotion.

Source:

- https://www.britannica.com/place/Aosta-Italy

## General Blockers

Rows were marked `reject_bad_lead` where the current lead is plainly wrong-type
or wrong-place: mollusc/genus, video game, family or given name, lake, political
organization, United States place, or encyclopedia-article item. Rows were
marked `source_resolution_blocked` where the current lead is a modern city,
municipality, region, province, or insufficiently described historical place and
where this pass did not find bounded title/date custody.

Ancient-city and historical-polity rows were blocked by rank policy where the
identity is plausible but the rank is not currently materialized for county
promotion.
