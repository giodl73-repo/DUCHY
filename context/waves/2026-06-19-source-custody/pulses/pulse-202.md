# Pulse 202: Swabia Wurttemberg Ecclesiastical Source Custody

## Intent

Add reviewed text support for more existing Holy Roman Empire parentage facts
after the 500-source milestone.

## Changes

- Add `src-wikipedia-duchy-swabia` and attach it to Duchy of Swabia -> Holy
  Roman Empire for `962..1313`.
- Add `src-wikipedia-duchy-wurttemberg` and attach it to Duchy of Wurttemberg
  -> Holy Roman Empire for `1495..1803`.
- Add `src-wikipedia-mecklenburg-gustrow` and attach it to Mecklenburg-Gustrow
  -> Holy Roman Empire for `1520..1695`.
- Add `src-wikipedia-prince-archbishopric-salzburg` and attach it to Salzburg
  -> Holy Roman Empire for `1278..1803`.
- Add `src-wikipedia-prince-bishopric-liege` and attach it to Liege -> Holy
  Roman Empire for `985..1795`.
- Add `src-wikipedia-prince-bishopric-minden` and attach it to Minden -> Holy
  Roman Empire for `1180..1648`.
- Add `src-wikipedia-prince-bishopric-munster` and attach it to Munster -> Holy
  Roman Empire for `1180..1802`.
- Add `src-wikipedia-prince-bishopric-osnabruck` and attach it to Osnabruck ->
  Holy Roman Empire for `1225..1803`.
- Add `src-wikipedia-prince-bishopric-paderborn` and attach it to Paderborn ->
  Holy Roman Empire for `1281..1802`.
- Add `src-wikipedia-prince-bishopric-strasbourg` and attach it to Strasbourg
  -> Holy Roman Empire for `982..1803`.
- Add `src-wikipedia-prince-bishopric-utrecht` and attach it to Utrecht ->
  Holy Roman Empire for `1024..1528`.
- Add `src-wikipedia-prince-bishopric-verdun` and attach it to Verdun -> Holy
  Roman Empire for `997..1552`.
- Add `src-wikipedia-free-imperial-city-aachen` and attach it to Aachen -> Holy
  Roman Empire for `1306..1801`.
- Add `src-wikipedia-burgraviate-nuremberg` and attach it to Burgraviate of
  Nuremberg -> Holy Roman Empire for `1105..1440`.

## Boundary

This pulse adds source support only. It does not change spans, add parentage
facts, or import diocesan jurisdictions, ecclesiastical personal unions,
imperial-circle membership, secularization destinations, French/Prussian/
Hanoverian/Baden successor claims, Swabian Circle successor geography,
Wurttemberg electorate/kingdom claims, Mecklenburg partition succession, or
Nuremberg Hohenzollern succession.

Several source pages carry citation or quality warnings. Those records are
accepted only as bounded text support for already reviewed parentage facts, not
as authority for new graph edges.

## Current State

- reviewed sources: 514
- reviewed facts: 1327
- active parentage facts: 278
- superseded parentage facts: 2
- rank-skip rows: 220
- bridge rows: 160
- temporal parent conflicts: 0
- snapshot cycle years: 0

## Validation

- `cargo test --quiet`
- `cargo run --quiet --bin duchy-import -- status fixtures/first-real.sources fixtures/first-real.facts`
