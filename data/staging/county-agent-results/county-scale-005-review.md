# County Scale 005 Review

## Scope

Reviewed `data/staging/county-parentage-scale-shards/county-scale-005.tsv`.
No fixtures, source code, existing queue files, or other shard result files were
edited. CK3 rows were treated only as search drivers.

## Decision Counts

| Decision | Rows |
|---|---:|
| `already_accepted` | 1 |
| `ready_title_packet` | 3 |
| `rank_policy_blocked` | 6 |
| `source_resolution_blocked` | 31 |
| `reject_bad_lead` | 9 |
| `ready_parentage_packet` | 0 |

## Notable Rows

### 219 Cleves

Decision: `already_accepted`.

Local fixtures already contain `title-q641138` / Duchy of Cleves, rank `Duchy`,
exists `1092..1795`, and accepted parentage to `title-q12548` / Holy Roman
Empire for `1092..1795`. The shard row is therefore a metrics seed only and
should not be re-imported.

Sources:

- https://www.wikidata.org/wiki/Q641138
- https://en.wikipedia.org/wiki/Duchy_of_Cleves

### 215 Cieszyn

Decision: `ready_title_packet`.

The modern town lead should not be promoted directly, but the historical title
lead is useful: Duchy of Teschen/Cieszyn, centered on Cieszyn, with a plausible
`1290..1918` existence span. Parentage was not promoted because the Bohemian
Crown, Austrian Silesia, and later Habsburg context needs its own source custody
and spans.

Source:

- https://www.britannica.com/place/Teschen

### 248 Denia

Decision: `ready_title_packet`.

The better historical lead is the Taifa of Denia, an Islamic kingdom centered
on Denia with a usable `1010..1227` span. This is title-only; parentage and
successor relations require direct review before any fact packet.

Source:

- https://en.wikipedia.org/wiki/Taifa_of_D%C3%A9nia

### 250 Desmond

Decision: `ready_title_packet`.

The top Wikidata lead is only a family name, but the CK3 seed resolves to the
Kingdom of Desmond. The source lead supports a Gaelic kingdom in southwest
Ireland with `1118..1596` as a candidate existence span. No parentage packet is
ready from this shard review.

Source:

- https://en.wikipedia.org/wiki/Kingdom_of_Desmond

### 203 Charsianon and 224 Colonea

Decision: `rank_policy_blocked`.

Both resolve to Byzantine themes: Theme of Charsianon and Theme of Koloneia.
The source leads provide useful bounded identity evidence, but `Theme` is a
military-civilian province category and is not currently a materialized DUCHY
rank.

Sources:

- https://en.wikipedia.org/wiki/Charsianon
- https://en.wikipedia.org/wiki/Koloneia_(theme)

### 207 Chernigov

Decision: `rank_policy_blocked`.

Independent evidence resolves the row to the Chernihiv/Chernigov principality,
with rule beginning in the 11th century and Riurykide governance continuing
until the 14th century. This remains blocked because principality rank semantics
are not yet materialized for county promotion.

Source:

- https://www.encyclopediaofukraine.com/display.asp?linkpath=pages%5CC%5CH%5CChernihivprincipality.htm

### 208 Cherven

Decision: `rank_policy_blocked`.

The better lead is the Cherven towns: a frontier group of towns/region tied to
Volhynia and later Galicia-Volhynia. That supports identity resolution but not a
current title rank.

Source:

- https://www.encyclopediaofukraine.com/display.asp?linkpath=pages%5CC%5CH%5CCherventowns.htm

### 205 Chartres, 221 Coimbra, and 225 Comminges

Decision: `source_resolution_blocked`.

Each has a plausible historical title lead, but not enough bounded title/date
and parentage custody for promotion in this pass:

- Chartres: medieval County of Chartres.
- Coimbra: County of Coimbra context after reconquest.
- Comminges: medieval County of Comminges.

Sources:

- https://fmg.ac/Projects/MedLands/cfrachacha.htm
- https://www.britannica.com/place/Coimbra-Portugal
- https://fmg.ac/Projects/MedLands/toulcofo.htm

### 236 Csanad and 237 Csongrad

Decision: `source_resolution_blocked`.

Both resolve to historical counties of the Kingdom of Hungary, but the available
leads expose interrupted Ottoman, Habsburg, and modern administrative spans.
They need a dedicated span-custody pass before any title packet or parentage
packet.

Sources:

- https://en.wikipedia.org/wiki/Csan%C3%A1d_County
- https://en.wikipedia.org/wiki/Csongr%C3%A1d_County_(former)

## General Blockers

Rows were marked `reject_bad_lead` where the current top lead is plainly
wrong-type or wrong-place: French commune for Cherso, bird genus for Chilia,
modern ship for Cilicia, family-name rows for Cosenza, Cuellar, and Dannenberg,
generic valley for Dal, beetle genus for Demetrias, and Derby, Connecticut for
the English Derby seed.

Rows were marked `source_resolution_blocked` where the surfaced lead is a
modern city, town, municipality, island, historic county without bounded spans,
or region where a plausible historical title may exist but independent bounded
title and parentage custody is still missing. No row in this shard produced a
ready parentage packet.
