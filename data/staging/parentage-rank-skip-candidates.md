# DUCHY Parentage Rank Skip Candidate Report

sources: 438
facts: 1321
titles: 349
rank_skip_rows: 222

rows_with_any_candidate: 217
rows_with_bridge_candidate: 162

## Interpretation

- Candidate rows are accepted titles with the expected immediate rank and an existence span overlapping the rank-skip fact span.
- Bridge candidates are candidate titles that already have reviewed parentage to the same current parent during an overlapping span.
- This report suggests review targets only; it does not assert that a candidate is the correct intermediate parent.

## Review Rows

### fact-q1049854-parent-q171150 | Abaúj county -> Kingdom of Hungary

- child_id: title-q1049854
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q171150
- current_parent_rank: Kingdom
- span: 1201..1881
- candidate_count: 125
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Margraviate of Moravia (title-q2670751) | Duchy | 1182..1918 | 681 |  |
| Duchy of Schleswig (title-q26167) | Duchy | 1058..1866 | 666 |  |
| Duchy of Opava (title-q566639) | Duchy | 1269..1918 | 613 |  |
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 612 |  |
| Margraviate of Brandenburg (title-q148499) | Duchy | 1157..1806 | 606 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 605 |  |
| Duchy of Westphalia (title-q657241) | Duchy | 1180..1803 | 603 |  |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 603 |  |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 603 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 603 |  |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 603 |  |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 603 |  |

113 additional candidates omitted from this row.

### fact-q1048340-parent-q926295 | Albanian Kingdom -> Italian Empire

- child_id: title-q1048340
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q926295
- current_parent_rank: Empire
- span: 1939..1939
- candidate_count: 0
- bridge_candidate_count: 0

No accepted overlapping candidate titles found.

### fact-q686965-parent-q12548 | Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q686965
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1252..1806
- candidate_count: 56
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 555 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 555 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 555 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 555 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 555 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 555 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 555 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 555 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 555 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 555 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 555 |  |
| Sweden (title-q34) | Kingdom | 900.. | 555 |  |

44 additional candidates omitted from this row.

### fact-q686965-parent-q151624 | Anhalt-Bernburg -> German Confederation

- child_id: title-q686965
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1863
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 49 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 49 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 49 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 49 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 49 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 49 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 49 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 49 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 49 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 49 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 49 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 49 |  |

19 additional candidates omitted from this row.

### fact-q278874-parent-q12548 | Anhalt-Dessau -> Holy Roman Empire

- child_id: title-q278874
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1396..1806
- candidate_count: 51
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 411 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 411 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 411 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 411 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 411 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 411 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 411 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 411 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 411 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 411 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 411 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 411 |  |

39 additional candidates omitted from this row.

### fact-q278874-parent-q151624 | Anhalt-Dessau -> German Confederation

- child_id: title-q278874
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1853
- candidate_count: 30
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 39 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 39 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 39 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 39 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 39 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 39 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 39 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 39 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 39 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 39 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 39 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 39 |  |

18 additional candidates omitted from this row.

### fact-q264970-parent-q12548 | Anhalt-Kothen -> Holy Roman Empire

- child_id: title-q264970
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1396..1806
- candidate_count: 51
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 411 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 411 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 411 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 411 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 411 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 411 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 411 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 411 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 411 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 411 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 411 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 411 |  |

39 additional candidates omitted from this row.

### fact-q264970-parent-q151624 | Anhalt-Kothen -> German Confederation

- child_id: title-q264970
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1863
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 49 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 49 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 49 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 49 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 49 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 49 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 49 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 49 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 49 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 49 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 49 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 49 |  |

19 additional candidates omitted from this row.

### fact-q699964-parent-q12548 | Archduchy of Austria -> Holy Roman Empire

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1358..1803
- candidate_count: 46
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 446 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 446 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 446 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 446 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 446 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 446 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 446 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 446 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 446 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 446 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 446 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 446 |  |

34 additional candidates omitted from this row.

### fact-q699964-parent-q131964 | Archduchy of Austria -> Austrian Empire

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- candidate_count: 38
- bridge_candidate_count: 6

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 63 | fact-q42585-parent-q131964 |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 63 | fact-q2396442-parent-q131964 |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 63 | fact-q171150-parent-q131964 |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 63 | fact-q253094-parent-q131964 |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 | fact-q209857-parent-q131964 |
| Kingdom of Illyria (title-q1117051) | Kingdom | 1816..1849 | 34 | fact-q1117051-parent-q131964 |
| Denmark (title-q35) | Kingdom | 800.. | 63 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 63 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 63 |  |
| Sweden (title-q34) | Kingdom | 900.. | 63 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 61 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 61 |  |

26 additional candidates omitted from this row.

### fact-q699964-parent-q28513 | Archduchy of Austria -> Austria-Hungary

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 23
- bridge_candidate_count: 4

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 | fact-q42585-parent-q28513 |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 | fact-q2396442-parent-q28513 |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 52 | fact-q25395037-parent-q28513 |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 51 | fact-q533558-parent-q28513 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 52 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 |  |

11 additional candidates omitted from this row.

### fact-q552033-parent-q12548 | Bavaria-Munich -> Holy Roman Empire

- child_id: title-q552033
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1392..1505
- candidate_count: 36
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 114 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 114 | fact-q838931-parent-q12548 |
| Croatia in personal union with Hungary (title-q1789596) | Kingdom | 1102..1526 | 114 |  |
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 114 |  |
| Denmark (title-q35) | Kingdom | 800.. | 114 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 114 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 114 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 114 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 114 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 114 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 114 |  |
| Kingdom of Hungary (title-q16056854) | Kingdom | 1301..1526 | 114 |  |

24 additional candidates omitted from this row.

### fact-q556263-parent-q12548 | Brunswick-Luneburg -> Holy Roman Empire

- child_id: title-q556263
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1235..1806
- candidate_count: 56
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 572 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 572 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 572 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 572 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 572 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 572 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 572 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 572 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 572 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 572 |  |
| Sweden (title-q34) | Kingdom | 900.. | 572 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 561 |  |

44 additional candidates omitted from this row.

### fact-q830084-parent-q12548 | Brunswick-Wolfenbuttel -> Holy Roman Empire

- child_id: title-q830084
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1269..1806
- candidate_count: 54
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 538 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 538 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 538 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 538 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 538 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 538 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 538 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 538 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 538 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 538 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 538 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 538 |  |

42 additional candidates omitted from this row.

### fact-q568473-parent-q12548 | Burgraviate of Nuremberg -> Holy Roman Empire

- child_id: title-q568473
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1105..1440
- candidate_count: 75
- bridge_candidate_count: 50

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 336 | fact-q47261-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 336 | fact-q641138-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 336 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 336 | fact-q155019-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 336 | fact-q22880-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 336 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 336 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 336 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 336 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 336 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 336 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 336 | fact-q328001-parent-q12548 |

63 additional candidates omitted from this row.

### fact-q157109-parent-q7882199 | Burgundian Netherlands -> Burgundian State

- child_id: title-q157109
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q7882199
- current_parent_rank: Crown
- span: 1384..1482
- candidate_count: 35
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Croatia in personal union with Hungary (title-q1789596) | Kingdom | 1102..1526 | 99 |  |
| Denmark (title-q35) | Kingdom | 800.. | 99 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 99 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 99 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 99 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 99 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 99 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 99 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 99 |  |
| Kingdom of Hungary (title-q16056854) | Kingdom | 1301..1526 | 99 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 99 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 99 |  |

23 additional candidates omitted from this row.

### fact-q8273263-parent-q203493 | Cetatea-Albă County -> Kingdom of Romania

- child_id: title-q8273263
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q203493
- current_parent_rank: Kingdom
- span: 1925..1944
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Principality of Albania (title-q187035) | Duchy | 1914..1925 | 1 |  |

### fact-q1122980-parent-q170174 | Comtat Venaissin -> Papal States

- child_id: title-q1122980
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1274..1791
- candidate_count: 101
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Ferrara (title-q693570) | Duchy | 1264..1597 | 324 | fact-q693570-parent-q170174 |
| Duchy of Urbino (title-q649202) | Duchy | 1443..1631 | 189 | fact-q649202-parent-q170174 |
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 518 |  |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 518 |  |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 518 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 518 |  |
| Duchy of Brabant (title-q159856) | Duchy | 1183..1795 | 518 |  |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 518 |  |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 518 |  |
| Duchy of Opava (title-q566639) | Duchy | 1269..1918 | 518 |  |
| Duchy of Schleswig (title-q26167) | Duchy | 1058..1866 | 518 |  |
| Duchy of Westphalia (title-q657241) | Duchy | 1180..1803 | 518 |  |

89 additional candidates omitted from this row.

### fact-q1235737-parent-q3446210 | County of Aragon -> Kingdom of Pamplona

- child_id: title-q1235737
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q3446210
- current_parent_rank: Kingdom
- span: 824..1035
- candidate_count: 29
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 212 |  |
| Duchy of Saxony (title-q164092) | Duchy | 804..1296 | 212 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 212 |  |
| Duchy of Amalfi (title-q686312) | Duchy | 839..1137 | 197 |  |
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 195 |  |
| March of Tuscany (title-q1867844) | Duchy | 846..1197 | 190 |  |
| Duklja (title-q1252942) | Duchy | 854..1252 | 182 |  |
| Duchy of Bohemia (title-q2162698) | Duchy | 870..1198 | 166 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 156 |  |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 148 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 129 |  |
| Duchy of Normandy (title-q842091) | Duchy | 911..1259 | 125 |  |

17 additional candidates omitted from this row.

### fact-q1233672-parent-q204920 | County of Barcelona -> Crown of Aragon

- child_id: title-q1233672
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q204920
- current_parent_rank: Crown
- span: 1162..1164
- candidate_count: 34
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 3 |  |
| Duchy of Austria (title-q3624335) | Duchy | 1156..1453 | 3 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 3 |  |
| Duchy of Bohemia (title-q2162698) | Duchy | 870..1198 | 3 |  |
| Duchy of Brittany (title-q71747) | Duchy | 939..1547 | 3 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 3 |  |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 3 |  |
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 3 |  |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 3 |  |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 3 |  |
| Duchy of Normandy (title-q842091) | Duchy | 911..1259 | 3 |  |
| Duchy of Pomerania (title-q696640) | Duchy | 1121..1637 | 3 |  |

22 additional candidates omitted from this row.

### fact-q642314-parent-q12548 | County of Burgundy -> Holy Roman Empire

- child_id: title-q642314
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 982..1678
- candidate_count: 105
- bridge_candidate_count: 71

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 697 | fact-q47261-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 697 | fact-q155019-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 697 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 697 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 697 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 697 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 694 | fact-q158835-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 676 | fact-q836937-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 652 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 647 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 631 | fact-q328001-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 594 | fact-q22880-parent-q12548 |

93 additional candidates omitted from this row.

### fact-q1541699-parent-q204920 | County of Empuries -> Crown of Aragon

- child_id: title-q1541699
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q204920
- current_parent_rank: Crown
- span: 1341..1402
- candidate_count: 64
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 62 |  |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 62 |  |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 62 |  |
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 62 |  |
| Duchy of Athens (title-q334714) | Duchy | 1205..1458 | 62 |  |
| Duchy of Austria (title-q3624335) | Duchy | 1156..1453 | 62 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 62 |  |
| Duchy of Belz (title-q2183293) | Duchy | 1170..1462 | 62 |  |
| Duchy of Brabant (title-q159856) | Duchy | 1183..1795 | 62 |  |
| Duchy of Brittany (title-q71747) | Duchy | 939..1547 | 62 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 62 |  |
| Duchy of Bytom (title-q682001) | Duchy | 1281..1498 | 62 |  |

52 additional candidates omitted from this row.

### fact-q157070-parent-q12548 | County of Flanders -> Holy Roman Empire

- child_id: title-q157070
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1797
- candidate_count: 111
- bridge_candidate_count: 78

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 836 | fact-q47261-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 836 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 836 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 836 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 816 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 811 | fact-q158835-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 805 | fact-q155019-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 792 | fact-q836937-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 771 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 766 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 750 | fact-q328001-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 713 | fact-q22880-parent-q12548 |

99 additional candidates omitted from this row.

### fact-q675363-parent-q12548 | County of Geneva -> Holy Roman Empire

- child_id: title-q675363
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1032..1401
- candidate_count: 75
- bridge_candidate_count: 50

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 370 | fact-q47261-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 370 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 370 | fact-q155019-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 370 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 370 | fact-q284667-parent-q12548 |
| Margraviate of Meissen (title-q170180) | Duchy | 965..1423 | 370 | fact-q170180-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 370 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 370 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 370 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 370 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 370 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Utrecht (title-q707767) | Duchy | 1024..1528 | 370 | fact-q707767-parent-q12548 |

63 additional candidates omitted from this row.

### fact-q2037817-parent-q31929 | County of Girona -> Carolingian Empire

- child_id: title-q2037817
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q31929
- current_parent_rank: Empire
- span: 800..887
- candidate_count: 11
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 88 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 88 |  |
| Duchy of Saxony (title-q164092) | Duchy | 804..1296 | 84 |  |
| Duchy of Amalfi (title-q686312) | Duchy | 839..1137 | 49 |  |
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 47 |  |
| March of Tuscany (title-q1867844) | Duchy | 846..1197 | 42 |  |
| Duklja (title-q1252942) | Duchy | 854..1252 | 34 |  |
| Lotharingia (title-q6673921) | Duchy | 855..959 | 33 |  |
| Duchy of Bohemia (title-q2162698) | Duchy | 870..1198 | 18 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 8 |  |
| Duchy of Alsace (title-q95950649) | Duchy | 700..800 | 1 |  |

### fact-q762943-parent-q12548 | County of Holland -> Holy Roman Empire

- child_id: title-q762943
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1795
- candidate_count: 111
- bridge_candidate_count: 78

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 834 | fact-q47261-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 834 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 834 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 834 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 814 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 811 | fact-q158835-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 805 | fact-q155019-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 792 | fact-q836937-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 769 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 764 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 748 | fact-q328001-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 711 | fact-q22880-parent-q12548 |

99 additional candidates omitted from this row.

### fact-q921473-parent-q70972 | County of La Marche -> Kingdom of France

- child_id: title-q921473
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q70972
- current_parent_rank: Kingdom
- span: 1527..1527
- candidate_count: 61
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Brittany (title-q71747) | Duchy | 939..1547 | 1 | fact-q71747-parent-q70972 |
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 1 |  |
| Anhalt-Dessau (title-q278874) | Duchy | 1396..1853 | 1 |  |
| Anhalt-Kothen (title-q264970) | Duchy | 1396..1863 | 1 |  |
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 1 |  |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 1 |  |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 1 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 1 |  |
| Duchy of Berg (title-q151095) | Duchy | 1380..1806 | 1 |  |
| Duchy of Brabant (title-q159856) | Duchy | 1183..1795 | 1 |  |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 1 |  |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 1 |  |

49 additional candidates omitted from this row.

### fact-q5177890-parent-q12548 | County of Luxembourg -> Holy Roman Empire

- child_id: title-q5177890
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 963..1354
- candidate_count: 65
- bridge_candidate_count: 42

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 392 | fact-q47261-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 392 | fact-q155019-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 392 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 392 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 392 | fact-q173863-parent-q12548 |
| Margraviate of Meissen (title-q170180) | Duchy | 965..1423 | 390 | fact-q170180-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 373 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 370 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Verdun (title-q17015016) | Duchy | 997..1552 | 358 | fact-q17015016-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 352 | fact-q836937-parent-q12548 |
| Duchy of Swabia (title-q693980) | Duchy | 917..1313 | 351 | fact-q693980-parent-q12548 |
| Duchy of Saxony (title-q164092) | Duchy | 804..1296 | 334 | fact-q164092-parent-q12548 |

53 additional candidates omitted from this row.

### fact-q700198-parent-q27306 | County of Mark -> Kingdom of Prussia

- child_id: title-q700198
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q27306
- current_parent_rank: Kingdom
- span: 1701..1806
- candidate_count: 77
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 106 |  |
| Anhalt-Dessau (title-q278874) | Duchy | 1396..1853 | 106 |  |
| Anhalt-Kothen (title-q264970) | Duchy | 1396..1863 | 106 |  |
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 106 |  |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 106 |  |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 106 |  |
| Duchy of Berg (title-q151095) | Duchy | 1380..1806 | 106 |  |
| Duchy of Bremen and Verden (title-q694594) | Duchy | 1648..1823 | 106 |  |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 106 |  |
| Duchy of Holstein (title-q704288) | Duchy | 1474..1867 | 106 |  |
| Duchy of Massa and Carrara (title-q933592) | Duchy | 1473..1829 | 106 |  |
| Duchy of Mecklenburg-Schwerin (title-q11024667) | Duchy | 1701..1815 | 106 |  |

65 additional candidates omitted from this row.

### fact-q589251-parent-q12548 | County of Montbeliard -> Holy Roman Empire

- child_id: title-q589251
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1042..1793
- candidate_count: 110
- bridge_candidate_count: 78

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 752 | fact-q47261-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 752 | fact-q836937-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 752 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 752 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 752 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 752 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 752 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 752 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 752 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 746 | fact-q328001-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 725 | fact-q155019-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 709 | fact-q22880-parent-q12548 |

98 additional candidates omitted from this row.

### fact-q599613-parent-q12548 | County of Namur -> Holy Roman Empire

- child_id: title-q599613
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 981..1795
- candidate_count: 111
- bridge_candidate_count: 78

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 815 | fact-q47261-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 815 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 815 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 815 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 814 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 811 | fact-q158835-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 792 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 786 | fact-q155019-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 769 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 764 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 748 | fact-q328001-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 711 | fact-q22880-parent-q12548 |

99 additional candidates omitted from this row.

### fact-q12817455-parent-q12548 | County of Nassau -> Holy Roman Empire

- child_id: title-q12817455
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1160..1806
- candidate_count: 114
- bridge_candidate_count: 78

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Margraviate of Brandenburg (title-q148499) | Duchy | 1157..1806 | 647 | fact-q148499-parent-q12548 |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 646 | fact-q47261-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 644 | fact-q22880-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 644 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 644 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 644 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 644 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 644 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 644 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 642 | fact-q328001-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 636 | fact-q641138-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 636 | fact-q158835-parent-q12548 |

102 additional candidates omitted from this row.

### fact-q706553-parent-q2577303 | County of Nice -> Kingdom of Sardinia

- child_id: title-q706553
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q2577303
- current_parent_rank: Kingdom
- span: 1814..1818
- candidate_count: 46
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 5 |  |
| Anhalt-Dessau (title-q278874) | Duchy | 1396..1853 | 5 |  |
| Anhalt-Kothen (title-q264970) | Duchy | 1396..1863 | 5 |  |
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 5 |  |
| Duchy of Bremen and Verden (title-q694594) | Duchy | 1648..1823 | 5 |  |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 5 |  |
| Duchy of Holstein (title-q704288) | Duchy | 1474..1867 | 5 |  |
| Duchy of Massa and Carrara (title-q933592) | Duchy | 1473..1829 | 5 |  |
| Duchy of Modena and Reggio (title-q252580) | Duchy | 1452..1859 | 5 |  |
| Duchy of Nassau (title-q836680) | Duchy | 1806..1866 | 5 |  |
| Duchy of Nysa (title-q570702) | Duchy | 1290..1850 | 5 |  |
| Duchy of Opava (title-q566639) | Duchy | 1269..1918 | 5 |  |

34 additional candidates omitted from this row.

### fact-q1139807-parent-q231392 | County of Portugal -> Kingdom of Asturias

- child_id: title-q1139807
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q231392
- current_parent_rank: Kingdom
- span: 868..909
- candidate_count: 12
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Amalfi (title-q686312) | Duchy | 839..1137 | 42 |  |
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 42 |  |
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 42 |  |
| Duchy of Saxony (title-q164092) | Duchy | 804..1296 | 42 |  |
| Duklja (title-q1252942) | Duchy | 854..1252 | 42 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 42 |  |
| Lotharingia (title-q6673921) | Duchy | 855..959 | 42 |  |
| March of Tuscany (title-q1867844) | Duchy | 846..1197 | 42 |  |
| Duchy of Bohemia (title-q2162698) | Duchy | 870..1198 | 40 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 30 |  |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 22 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 3 |  |

### fact-q1139807-parent-q303421 | County of Portugal -> Kingdom of Galicia

- child_id: title-q1139807
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q303421
- current_parent_rank: Kingdom
- span: 910..1139
- candidate_count: 37
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 230 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 230 |  |
| Duchy of Bohemia (title-q2162698) | Duchy | 870..1198 | 230 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 230 |  |
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 230 |  |
| Duchy of Saxony (title-q164092) | Duchy | 804..1296 | 230 |  |
| Duklja (title-q1252942) | Duchy | 854..1252 | 230 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 230 |  |
| March of Tuscany (title-q1867844) | Duchy | 846..1197 | 230 |  |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 230 |  |
| Duchy of Normandy (title-q842091) | Duchy | 911..1259 | 229 |  |
| Duchy of Amalfi (title-q686312) | Duchy | 839..1137 | 228 |  |

25 additional candidates omitted from this row.

### fact-q2991382-parent-q70972 | County of Provence -> Kingdom of France

- child_id: title-q2991382
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q70972
- current_parent_rank: Kingdom
- span: 987..1487
- candidate_count: 82
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Brittany (title-q71747) | Duchy | 939..1547 | 501 | fact-q71747-parent-q70972 |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 491 | fact-q4712-parent-q70972 |
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 467 | fact-q2295939-parent-q70972 |
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 463 | fact-q7703611-parent-q70972 |
| Duchy of Normandy (title-q842091) | Duchy | 911..1259 | 273 | fact-q842091-parent-q70972 |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 501 |  |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 501 |  |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 501 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 501 |  |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 501 |  |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 501 |  |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 501 |  |

70 additional candidates omitted from this row.

### fact-q573290-parent-q12548 | County of Ravensberg -> Holy Roman Empire

- child_id: title-q573290
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1140..1806
- candidate_count: 115
- bridge_candidate_count: 79

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 666 | fact-q47261-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 664 | fact-q22880-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 664 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 664 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 664 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 664 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 664 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 664 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 662 | fact-q328001-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 656 | fact-q641138-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 656 | fact-q158835-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 655 | fact-q836937-parent-q12548 |

103 additional candidates omitted from this row.

### fact-q1297894-parent-q199442 | County of Ribagorza -> Kingdom of Aragon

- child_id: title-q1297894
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q199442
- current_parent_rank: Kingdom
- span: 1035..1598
- candidate_count: 97
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 564 |  |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 564 |  |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 564 |  |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 564 |  |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 564 |  |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 564 |  |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 564 |  |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 564 |  |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 564 |  |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 564 |  |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 551 |  |
| Duchy of Schleswig (title-q26167) | Duchy | 1058..1866 | 541 |  |

85 additional candidates omitted from this row.

### fact-q1232887-parent-q12548 | County of Savoy -> Holy Roman Empire

- child_id: title-q1232887
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1003..1416
- candidate_count: 77
- bridge_candidate_count: 51

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 414 | fact-q47261-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 414 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 414 | fact-q155019-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 414 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 414 | fact-q284667-parent-q12548 |
| Margraviate of Meissen (title-q170180) | Duchy | 965..1423 | 414 | fact-q170180-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 414 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 414 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 414 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Verdun (title-q17015016) | Duchy | 997..1552 | 414 | fact-q17015016-parent-q12548 |
| Prince-Bishopric of Utrecht (title-q707767) | Duchy | 1024..1528 | 393 | fact-q707767-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 390 | fact-q1231403-parent-q12548 |

65 additional candidates omitted from this row.

### fact-q1917014-parent-q188586 | County of Sicily -> Kingdom of Sicily

- child_id: title-q1917014
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q188586
- current_parent_rank: Kingdom
- span: 1130..1130
- candidate_count: 33
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Amalfi (title-q686312) | Duchy | 839..1137 | 1 |  |
| Duchy of Aquitaine (title-q7703611) | Duchy | 841..1449 | 1 |  |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 1 |  |
| Duchy of Bohemia (title-q2162698) | Duchy | 870..1198 | 1 |  |
| Duchy of Brittany (title-q71747) | Duchy | 939..1547 | 1 |  |
| Duchy of Burgundy (title-q4712) | Duchy | 880..1477 | 1 |  |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 1 |  |
| Duchy of Gascony (title-q2295939) | Duchy | 602..1453 | 1 |  |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 1 |  |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 1 |  |
| Duchy of Normandy (title-q842091) | Duchy | 911..1259 | 1 |  |
| Duchy of Pomerania (title-q696640) | Duchy | 1121..1637 | 1 |  |

21 additional candidates omitted from this row.

### fact-q2991474-parent-q12548 | County of Wurttemberg -> Holy Roman Empire

- child_id: title-q2991474
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1083..1495
- candidate_count: 82
- bridge_candidate_count: 56

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 413 | fact-q47261-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 413 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 413 | fact-q155019-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 413 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 413 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 413 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 413 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 413 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 413 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 413 | fact-q328001-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 413 | fact-q1231403-parent-q12548 |
| Prince-Bishopric of Utrecht (title-q707767) | Duchy | 1024..1528 | 413 | fact-q707767-parent-q12548 |

70 additional candidates omitted from this row.

### fact-q5298169-parent-q203493 | Dorohoi County -> Kingdom of Romania

- child_id: title-q5298169
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q203493
- current_parent_rank: Kingdom
- span: 1881..1947
- candidate_count: 27
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Schwarzburg-Sondershausen (title-q630163) | Duchy | 1599..1920 | 40 |  |
| Schwarzburg-Rudolstadt (title-q695316) | Duchy | 1599..1919 | 39 |  |
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 38 |  |
| Duchy of Anhalt (title-q16550783) | Duchy | 1863..1918 | 38 |  |
| Duchy of Brunswick (title-q326029) | Duchy | 1815..1918 | 38 |  |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 38 |  |
| Duchy of Opava (title-q566639) | Duchy | 1269..1918 | 38 |  |
| Duchy of Salzburg (title-q661340) | Duchy | 1849..1918 | 38 |  |
| Duchy of Saxe-Meiningen (title-q157710) | Duchy | 1675..1918 | 38 |  |
| Duchy of Teschen (title-q671899) | Duchy | 1281..1918 | 38 |  |
| Grand Duchy of Baden (title-q186320) | Duchy | 1806..1918 | 38 |  |
| Grand Duchy of Hesse (title-q20135) | Duchy | 1806..1918 | 38 |  |

15 additional candidates omitted from this row.

### fact-q686312-parent-q12544 | Duchy of Amalfi -> Byzantine Empire

- child_id: title-q686312
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12544
- current_parent_rank: Empire
- span: 839..958
- candidate_count: 29
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Denmark (title-q35) | Kingdom | 800.. | 120 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 120 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 120 |  |
| Kingdom of Brycheiniog (title-q954585) | Kingdom | 450..1045 | 120 |  |
| Kingdom of Dublin (title-q436994) | Kingdom | 839..1171 | 120 |  |
| Kingdom of Glywysing (title-q2253783) | Kingdom | 500..1063 | 120 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 120 |  |
| Kingdom of Pamplona (title-q3446210) | Kingdom | 824..1162 | 120 |  |
| Kingdom of Powys (title-q769782) | Kingdom | 500..1160 | 120 |  |
| Neustria (title-q106577) | Kingdom | 511..977 | 120 |  |
| East Francia (title-q153080) | Kingdom | 843..962 | 116 |  |
| Kingdom of Northumbria (title-q107299) | Kingdom | 653..954 | 116 |  |

17 additional candidates omitted from this row.

### fact-q16550783-parent-q150981 | Duchy of Anhalt -> North German Confederation

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 15
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q150981 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q150981 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 4 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 |  |

3 additional candidates omitted from this row.

### fact-q16550783-parent-q151624 | Duchy of Anhalt -> German Confederation

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1863..1866
- candidate_count: 15
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 4 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |

3 additional candidates omitted from this row.

### fact-q16550783-parent-q43287 | Duchy of Anhalt -> German Empire

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q3624335-parent-q12548 | Duchy of Austria -> Holy Roman Empire

- child_id: title-q3624335
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1156..1453
- candidate_count: 45
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 298 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 256 | fact-q42585-parent-q12548 |
| Croatia in personal union with Hungary (title-q1789596) | Kingdom | 1102..1526 | 298 |  |
| Denmark (title-q35) | Kingdom | 800.. | 298 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 298 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 298 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 298 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 298 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 298 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 298 |  |
| Kingdom of Georgia (title-q154667) | Kingdom | 1008..1466 | 298 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 298 |  |

33 additional candidates omitted from this row.

### fact-q47261-parent-q12548 | Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q47261
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1805
- candidate_count: 65
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 844 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 608 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 844 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 844 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 844 |  |
| Sweden (title-q34) | Kingdom | 900.. | 844 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 806 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 805 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 746 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 746 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 721 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 676 |  |

53 additional candidates omitted from this row.

### fact-q151095-parent-q12548 | Duchy of Berg -> Holy Roman Empire

- child_id: title-q151095
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1380..1806
- candidate_count: 51
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 427 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 427 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 427 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 427 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 427 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 427 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 427 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 427 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 427 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 427 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 427 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 427 |  |

39 additional candidates omitted from this row.

### fact-q2162698-parent-q12548 | Duchy of Bohemia -> Holy Roman Empire

- child_id: title-q2162698
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1002..1198
- candidate_count: 34
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 197 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 1 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 197 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 197 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 197 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 197 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 197 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 197 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 197 |  |
| Kingdom of Hungary (1000-1301) (title-q1470101) | Kingdom | 1000..1301 | 197 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 197 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 197 |  |

22 additional candidates omitted from this row.

### fact-q159856-parent-q12548 | Duchy of Brabant -> Holy Roman Empire

- child_id: title-q159856
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1183..1795
- candidate_count: 54
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 613 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 598 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 613 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 613 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 613 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 613 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 613 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 613 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 613 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 613 |  |
| Sweden (title-q34) | Kingdom | 900.. | 613 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 609 |  |

42 additional candidates omitted from this row.

### fact-q694594-parent-q12548 | Duchy of Bremen and Verden -> Holy Roman Empire

- child_id: title-q694594
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1648..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 159 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 159 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 159 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 159 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 159 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 159 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 159 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 159 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 159 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 159 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 159 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 159 |  |

28 additional candidates omitted from this row.

### fact-q326029-parent-q150981 | Duchy of Brunswick -> North German Confederation

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 15
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q150981 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q150981 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 4 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 |  |

3 additional candidates omitted from this row.

### fact-q326029-parent-q151624 | Duchy of Brunswick -> German Confederation

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q326029-parent-q43287 | Duchy of Brunswick -> German Empire

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q2360973-parent-q12548 | Duchy of Carniola -> Holy Roman Empire

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1364..1803
- candidate_count: 46
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 440 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 440 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 440 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 440 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 440 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 440 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 440 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 440 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 440 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 440 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 440 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 440 |  |

34 additional candidates omitted from this row.

### fact-q2360973-parent-q131964 | Duchy of Carniola -> Austrian Empire

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- candidate_count: 38
- bridge_candidate_count: 6

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 63 | fact-q42585-parent-q131964 |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 63 | fact-q2396442-parent-q131964 |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 63 | fact-q171150-parent-q131964 |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 63 | fact-q253094-parent-q131964 |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 | fact-q209857-parent-q131964 |
| Kingdom of Illyria (title-q1117051) | Kingdom | 1816..1849 | 34 | fact-q1117051-parent-q131964 |
| Denmark (title-q35) | Kingdom | 800.. | 63 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 63 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 63 |  |
| Sweden (title-q34) | Kingdom | 900.. | 63 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 61 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 61 |  |

26 additional candidates omitted from this row.

### fact-q2360973-parent-q28513 | Duchy of Carniola -> Austria-Hungary

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 23
- bridge_candidate_count: 4

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 | fact-q42585-parent-q28513 |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 | fact-q2396442-parent-q28513 |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 52 | fact-q25395037-parent-q28513 |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 51 | fact-q533558-parent-q28513 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 52 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 |  |

11 additional candidates omitted from this row.

### fact-q641138-parent-q12548 | Duchy of Cleves -> Holy Roman Empire

- child_id: title-q641138
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1092..1795
- candidate_count: 58
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 704 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 598 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 704 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 704 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 704 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 704 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 704 |  |
| Sweden (title-q34) | Kingdom | 900.. | 704 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 700 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 666 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 657 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 634 |  |

46 additional candidates omitted from this row.

### fact-q156038-parent-q172107 | Duchy of Courland and Semigallia -> Polish-Lithuanian Commonwealth

- child_id: title-q156038
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1795
- candidate_count: 39
- bridge_candidate_count: 3

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 227 | fact-q171348-parent-q172107 |
| Kingdom of Poland (title-q1649871) | Kingdom | 1386..1569 | 1 | fact-q1649871-parent-q172107 |
| Kingdom of Poland (title-q8890160) | Kingdom | 1025..1569 | 1 | fact-q8890160-parent-q172107 |
| Denmark (title-q35) | Kingdom | 800.. | 227 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 227 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 227 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 227 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 227 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 227 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 227 |  |
| Kingdom of Ireland (title-q215530) | Kingdom | 1542..1801 | 227 |  |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 227 |  |

27 additional candidates omitted from this row.

### fact-q1991540-parent-q43287 | Duchy of Courland and Semigallia -> German Empire

- child_id: title-q1991540
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1918..1918
- candidate_count: 21
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 1 | fact-q154195-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 1 | fact-q696908-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 1 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 1 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 1 | fact-q159631-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 1 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 1 |  |
| Kingdom of Bulgaria (title-q147909) | Kingdom | 1908..1946 | 1 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 1 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 1 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 1 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 1 |  |

9 additional candidates omitted from this row.

### fact-q693570-parent-q170174 | Duchy of Ferrara -> Papal States

- child_id: title-q693570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1597..1597
- candidate_count: 29
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 1 |  |
| Denmark (title-q35) | Kingdom | 800.. | 1 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 1 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 1 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 1 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 1 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 1 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 1 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 1 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 1 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 1 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 1 |  |

17 additional candidates omitted from this row.

### fact-q2252973-parent-q12548 | Duchy of Florence -> Holy Roman Empire

- child_id: title-q2252973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1532..1569
- candidate_count: 33
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 38 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 38 | fact-q838931-parent-q12548 |
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 38 |  |
| Denmark (title-q35) | Kingdom | 800.. | 38 |  |
| Eastern Hungarian Kingdom (title-q625380) | Kingdom | 1526..1570 | 38 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 38 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 38 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 38 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 38 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 38 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 38 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 38 |  |

21 additional candidates omitted from this row.

### fact-q152420-parent-q12548 | Duchy of Guelders -> Holy Roman Empire

- child_id: title-q152420
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1096..1795
- candidate_count: 109
- bridge_candidate_count: 77

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 700 | fact-q47261-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 700 | fact-q641138-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 700 | fact-q22880-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 700 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 700 | fact-q284667-parent-q12548 |
| Prince-Bishopric of Augsburg (title-q173863) | Duchy | 888..1803 | 700 | fact-q173863-parent-q12548 |
| Prince-Bishopric of Basel (title-q319586) | Duchy | 1032..1803 | 700 | fact-q319586-parent-q12548 |
| Prince-Bishopric of Liege (title-q158835) | Duchy | 985..1795 | 700 | fact-q158835-parent-q12548 |
| Prince-Bishopric of Strasbourg (title-q771332) | Duchy | 982..1803 | 700 | fact-q771332-parent-q12548 |
| Prince-Bishopric of Toul (title-q328001) | Duchy | 1048..1801 | 700 | fact-q328001-parent-q12548 |
| Prince-Bishopric of Trent (title-q1231403) | Duchy | 1027..1803 | 700 | fact-q1231403-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 699 | fact-q836937-parent-q12548 |

97 additional candidates omitted from this row.

### fact-q704288-parent-q12548 | Duchy of Holstein -> Holy Roman Empire

- child_id: title-q704288
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1474..1806
- candidate_count: 47
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 333 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 333 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 333 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 333 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 333 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 333 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 333 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 333 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 333 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 333 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 333 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 333 |  |

35 additional candidates omitted from this row.

### fact-q704288-parent-q151624 | Duchy of Holstein -> German Confederation

- child_id: title-q704288
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q836937-parent-q12548 | Duchy of Julich -> Holy Roman Empire

- child_id: title-q836937
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1003..1794
- candidate_count: 61
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 792 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 597 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 792 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 792 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 792 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 792 |  |
| Sweden (title-q34) | Kingdom | 900.. | 792 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 789 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 710 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 705 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 705 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 673 |  |

49 additional candidates omitted from this row.

### fact-q1352878-parent-q172107 | Duchy of Livonia -> Polish-Lithuanian Commonwealth

- child_id: title-q1352878
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1621
- candidate_count: 34
- bridge_candidate_count: 3

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 53 | fact-q171348-parent-q172107 |
| Kingdom of Poland (title-q1649871) | Kingdom | 1386..1569 | 1 | fact-q1649871-parent-q172107 |
| Kingdom of Poland (title-q8890160) | Kingdom | 1025..1569 | 1 | fact-q8890160-parent-q172107 |
| Denmark (title-q35) | Kingdom | 800.. | 53 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 53 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 53 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 53 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 53 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 53 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 53 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 53 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 53 |  |

22 additional candidates omitted from this row.

### fact-q155019-parent-q12548 | Duchy of Lorraine -> Holy Roman Empire

- child_id: title-q155019
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1766
- candidate_count: 62
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 805 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 569 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 805 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 805 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 805 |  |
| Sweden (title-q34) | Kingdom | 900.. | 805 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 780 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 767 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 746 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 746 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 682 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 673 |  |

50 additional candidates omitted from this row.

### fact-q2719360-parent-q12548 | Duchy of Luxembourg -> Holy Roman Empire

- child_id: title-q2719360
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1353..1795
- candidate_count: 45
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 443 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 443 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 443 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 443 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 443 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 443 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 443 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 443 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 443 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 443 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 443 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 443 |  |

33 additional candidates omitted from this row.

### fact-q766501-parent-q12548 | Duchy of Mantua -> Holy Roman Empire

- child_id: title-q766501
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1530..1797
- candidate_count: 39
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 268 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 268 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 268 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 268 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 268 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 268 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 268 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 268 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 268 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 268 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 268 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 268 |  |

27 additional candidates omitted from this row.

### fact-q933592-parent-q12548 | Duchy of Massa and Carrara -> Holy Roman Empire

- child_id: title-q933592
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1473..1806
- candidate_count: 47
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 334 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 334 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 334 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 334 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 334 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 334 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 334 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 334 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 334 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 334 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 334 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 334 |  |

35 additional candidates omitted from this row.

### fact-q11024667-parent-q12548 | Duchy of Mecklenburg-Schwerin -> Holy Roman Empire

- child_id: title-q11024667
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1701..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 106 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 106 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 106 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 106 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 106 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 106 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 106 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 106 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 106 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 106 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 106 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 106 |  |

28 additional candidates omitted from this row.

### fact-q153529-parent-q12548 | Duchy of Milan -> Holy Roman Empire

- child_id: title-q153529
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1395..1797
- candidate_count: 45
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 403 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 403 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 403 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 403 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 403 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 403 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 403 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 403 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 403 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 403 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 403 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 403 |  |

33 additional candidates omitted from this row.

### fact-q1615455-parent-q12548 | Duchy of Mirandola -> Holy Roman Empire

- child_id: title-q1615455
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1310..1710
- candidate_count: 99
- bridge_candidate_count: 69

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 401 | fact-q686965-parent-q12548 |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 401 | fact-q556263-parent-q12548 |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 401 | fact-q830084-parent-q12548 |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 401 | fact-q47261-parent-q12548 |
| Duchy of Brabant (title-q159856) | Duchy | 1183..1795 | 401 | fact-q159856-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 401 | fact-q641138-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 401 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 401 | fact-q155019-parent-q12548 |
| Duchy of Westphalia (title-q657241) | Duchy | 1180..1803 | 401 | fact-q657241-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 401 | fact-q22880-parent-q12548 |
| Electorate of Cologne (title-q7904317) | Duchy | 953..1803 | 401 | fact-q7904317-parent-q12548 |
| Electorate of Mainz (title-q284667) | Duchy | 780..1803 | 401 | fact-q284667-parent-q12548 |

87 additional candidates omitted from this row.

### fact-q252580-parent-q12548 | Duchy of Modena and Reggio -> Holy Roman Empire

- child_id: title-q252580
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1452..1796
- candidate_count: 43
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 345 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 345 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 345 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 345 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 345 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 345 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 345 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 345 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 345 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 345 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 345 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 345 |  |

31 additional candidates omitted from this row.

### fact-q836680-parent-q151624 | Duchy of Nassau -> German Confederation

- child_id: title-q836680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q165040-parent-q12548 | Duchy of Parma and Piacenza -> Holy Roman Empire

- child_id: title-q165040
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1748..1801
- candidate_count: 28
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 54 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 54 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 54 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 54 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 54 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 54 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 54 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 54 |  |
| Kingdom of Ireland (title-q215530) | Kingdom | 1542..1801 | 54 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 54 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 54 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 54 |  |

16 additional candidates omitted from this row.

### fact-q696640-parent-q12548 | Duchy of Pomerania -> Holy Roman Empire

- child_id: title-q696640
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1121..1637
- candidate_count: 52
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 517 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 440 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 517 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 517 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 517 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 517 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 517 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 517 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 517 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 517 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 517 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 517 |  |

40 additional candidates omitted from this row.

### fact-q661340-parent-q28513 | Duchy of Salzburg -> Austria-Hungary

- child_id: title-q661340
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 23
- bridge_candidate_count: 4

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 | fact-q42585-parent-q28513 |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 | fact-q2396442-parent-q28513 |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 52 | fact-q25395037-parent-q28513 |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 51 | fact-q533558-parent-q28513 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 52 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 |  |

11 additional candidates omitted from this row.

### fact-q426025-parent-q12548 | Duchy of Savoy -> Holy Roman Empire

- child_id: title-q426025
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1416..1806
- candidate_count: 50
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 391 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 391 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 391 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 391 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 391 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 391 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 391 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 391 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 391 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 391 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 391 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 391 |  |

38 additional candidates omitted from this row.

### fact-q157710-parent-q12548 | Duchy of Saxe-Meiningen -> Holy Roman Empire

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1675..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 132 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 132 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 132 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 132 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 132 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 132 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 132 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 132 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 132 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 132 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 132 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 132 |  |

28 additional candidates omitted from this row.

### fact-q157710-parent-q151624 | Duchy of Saxe-Meiningen -> German Confederation

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q157710-parent-q43287 | Duchy of Saxe-Meiningen -> German Empire

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q164092-parent-q12548 | Duchy of Saxony -> Holy Roman Empire

- child_id: title-q164092
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1296
- candidate_count: 46
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 335 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 99 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 335 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 335 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 335 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 335 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 335 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 335 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 335 |  |
| Sweden (title-q34) | Kingdom | 900.. | 335 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 310 |  |
| Kingdom of Hungary (1000-1301) (title-q1470101) | Kingdom | 1000..1301 | 297 |  |

34 additional candidates omitted from this row.

### fact-q693980-parent-q12548 | Duchy of Swabia -> Holy Roman Empire

- child_id: title-q693980
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1313
- candidate_count: 47
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 352 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 116 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 352 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 352 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 352 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 352 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 352 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 352 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 352 |  |
| Sweden (title-q34) | Kingdom | 900.. | 352 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 327 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 314 |  |

35 additional candidates omitted from this row.

### fact-q649202-parent-q170174 | Duchy of Urbino -> Papal States

- child_id: title-q649202
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1443..1631
- candidate_count: 38
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 189 |  |
| Denmark (title-q35) | Kingdom | 800.. | 189 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 189 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 189 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 189 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 189 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 189 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 189 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 189 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 189 |  |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 189 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 189 |  |

26 additional candidates omitted from this row.

### fact-q152115-parent-q71084 | Duchy of Warsaw -> First French Empire

- child_id: title-q152115
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1807..1815
- candidate_count: 32
- bridge_candidate_count: 3

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q223936) | Kingdom | 1805..1814 | 8 | fact-q223936-parent-q71084 |
| Kingdom of Holland (title-q212278) | Kingdom | 1806..1810 | 4 | fact-q212278-parent-q71084 |
| Kingdom of Etruria (title-q223793) | Kingdom | 1801..1807 | 1 | fact-q223793-parent-q71084 |
| Denmark (title-q35) | Kingdom | 800.. | 9 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 9 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 9 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 9 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 9 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 9 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 9 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 9 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 9 |  |

20 additional candidates omitted from this row.

### fact-q657241-parent-q12548 | Duchy of Westphalia -> Holy Roman Empire

- child_id: title-q657241
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1803
- candidate_count: 55
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 624 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 624 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 624 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 624 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 624 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 624 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 624 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 624 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 624 |  |
| Sweden (title-q34) | Kingdom | 900.. | 624 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 612 |  |

43 additional candidates omitted from this row.

### fact-q2227570-parent-q12548 | Duchy of Wurttemberg -> Holy Roman Empire

- child_id: title-q2227570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1495..1803
- candidate_count: 42
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 309 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 309 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 309 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 309 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 309 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 309 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 309 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 309 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 309 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 309 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 309 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 309 |  |

30 additional candidates omitted from this row.

### fact-q1252942-parent-q12544 | Duklja -> Byzantine Empire

- child_id: title-q1252942
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12544
- current_parent_rank: Empire
- span: 854..1252
- candidate_count: 50
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Denmark (title-q35) | Kingdom | 800.. | 399 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 399 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 399 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 399 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 381 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 363 |  |
| Sweden (title-q34) | Kingdom | 900.. | 353 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 343 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 343 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 326 |  |
| Kingdom of Dublin (title-q436994) | Kingdom | 839..1171 | 318 |  |
| Kingdom of Pamplona (title-q3446210) | Kingdom | 824..1162 | 309 |  |

38 additional candidates omitted from this row.

### fact-q153080-parent-q31929 | East Francia -> Carolingian Empire

- child_id: title-q153080
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q31929
- current_parent_rank: Empire
- span: 843..887
- candidate_count: 0
- bridge_candidate_count: 0

No accepted overlapping candidate titles found.

### fact-q625380-parent-q12560 | Eastern Hungarian Kingdom -> Ottoman Empire

- child_id: title-q625380
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1529..1570
- candidate_count: 4
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 42 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 42 |  |
| Burgundian State (title-q7882199) | Crown | 1363..1559 | 31 |  |
| Polish-Lithuanian Commonwealth (title-q172107) | Crown | 1569..1795 | 2 |  |

### fact-q22880-parent-q12548 | Electoral Palatinate -> Holy Roman Empire

- child_id: title-q22880
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1085..1803
- candidate_count: 59
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 719 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 719 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 719 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 719 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 719 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Sweden (title-q34) | Kingdom | 900.. | 719 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 707 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 665 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 642 |  |

47 additional candidates omitted from this row.

### fact-q637238-parent-q12548 | Electorate of Baden -> Holy Roman Empire

- child_id: title-q637238
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1803..1806
- candidate_count: 28
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 4 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Etruria (title-q223793) | Kingdom | 1801..1807 | 4 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 4 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 4 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 4 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 4 |  |

16 additional candidates omitted from this row.

### fact-q256961-parent-q12548 | Electorate of Bavaria -> Holy Roman Empire

- child_id: title-q256961
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1623..1805
- candidate_count: 36
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 183 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 183 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 183 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 183 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 183 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 183 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 183 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 183 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 183 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 183 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 183 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 183 |  |

24 additional candidates omitted from this row.

### fact-q7904317-parent-q12548 | Electorate of Cologne -> Holy Roman Empire

- child_id: title-q7904317
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- candidate_count: 64
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 842 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 842 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 842 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 842 |  |
| Sweden (title-q34) | Kingdom | 900.. | 842 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 805 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 804 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 746 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 746 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |

52 additional candidates omitted from this row.

### fact-q706018-parent-q12548 | Electorate of Hanover -> Holy Roman Empire

- child_id: title-q706018
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1692..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 115 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 115 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 115 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 115 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 115 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 115 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 115 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 115 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 115 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 115 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 115 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 115 |  |

28 additional candidates omitted from this row.

### fact-q529605-parent-q151624 | Electorate of Hesse -> German Confederation

- child_id: title-q529605
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q284667-parent-q12548 | Electorate of Mainz -> Holy Roman Empire

- child_id: title-q284667
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- candidate_count: 64
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 842 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 842 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 842 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 842 |  |
| Sweden (title-q34) | Kingdom | 900.. | 842 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 805 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 804 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 746 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 746 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |

52 additional candidates omitted from this row.

### fact-q156199-parent-q12548 | Electorate of Saxony -> Holy Roman Empire

- child_id: title-q156199
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1356..1806
- candidate_count: 51
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 451 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 451 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 451 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 451 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 451 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 451 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 451 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 451 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 451 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 451 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 451 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 451 |  |

39 additional candidates omitted from this row.

### fact-q2172530-parent-q12548 | Electorate of Wurttemberg -> Holy Roman Empire

- child_id: title-q2172530
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1803..1806
- candidate_count: 28
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 4 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Etruria (title-q223793) | Kingdom | 1801..1807 | 4 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 4 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 4 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 4 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 4 |  |

16 additional candidates omitted from this row.

### fact-q146246-parent-q31929 | Francia -> Carolingian Empire

- child_id: title-q146246
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q31929
- current_parent_rank: Empire
- span: 800..843
- candidate_count: 0
- bridge_candidate_count: 0

No accepted overlapping candidate titles found.

### fact-q692946-parent-q131964 | Gorizia and Gradisca -> Austrian Empire

- child_id: title-q692946
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- candidate_count: 63
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 63 | fact-q699964-parent-q131964 |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 63 | fact-q2360973-parent-q131964 |
| Duchy of Holstein (title-q704288) | Duchy | 1474..1867 | 63 |  |
| Duchy of Opava (title-q566639) | Duchy | 1269..1918 | 63 |  |
| Duchy of Saxe-Meiningen (title-q157710) | Duchy | 1675..1918 | 63 |  |
| Duchy of Schleswig (title-q26167) | Duchy | 1058..1866 | 63 |  |
| Duchy of Teschen (title-q671899) | Duchy | 1281..1918 | 63 |  |
| Margraviate of Moravia (title-q2670751) | Duchy | 1182..1918 | 63 |  |
| Principality of Lippe (title-q14551680) | Duchy | 1789..1918 | 63 |  |
| Saxe-Altenburg (title-q158151) | Duchy | 1602..1918 | 63 |  |
| Saxe-Lauenburg (title-q313175) | Duchy | 1296..1876 | 63 |  |
| Schaumburg-Lippe (title-q310650) | Duchy | 1643..1918 | 63 |  |

51 additional candidates omitted from this row.

### fact-q692946-parent-q28513 | Gorizia and Gradisca -> Austria-Hungary

- child_id: title-q692946
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 29
- bridge_candidate_count: 4

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 52 | fact-q699964-parent-q28513 |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 52 | fact-q2360973-parent-q28513 |
| Duchy of Salzburg (title-q661340) | Duchy | 1849..1918 | 52 | fact-q661340-parent-q28513 |
| Margraviate of Moravia (title-q2670751) | Duchy | 1182..1918 | 52 | fact-q2670751-parent-q28513 |
| Duchy of Anhalt (title-q16550783) | Duchy | 1863..1918 | 52 |  |
| Duchy of Brunswick (title-q326029) | Duchy | 1815..1918 | 52 |  |
| Duchy of Opava (title-q566639) | Duchy | 1269..1918 | 52 |  |
| Duchy of Saxe-Meiningen (title-q157710) | Duchy | 1675..1918 | 52 |  |
| Duchy of Teschen (title-q671899) | Duchy | 1281..1918 | 52 |  |
| Grand Duchy of Baden (title-q186320) | Duchy | 1806..1918 | 52 |  |
| Grand Duchy of Hesse (title-q20135) | Duchy | 1806..1918 | 52 |  |
| Grand Duchy of Mecklenburg-Schwerin (title-q158445) | Duchy | 1815..1918 | 52 |  |

17 additional candidates omitted from this row.

### fact-q186320-parent-q151624 | Grand Duchy of Baden -> German Confederation

- child_id: title-q186320
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q186320-parent-q43287 | Grand Duchy of Baden -> German Empire

- child_id: title-q186320
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q249428-parent-q154741 | Grand Duchy of Berg -> Confederation of the Rhine

- child_id: title-q249428
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1806..1813
- candidate_count: 29
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Westphalia (title-q153943) | Kingdom | 1807..1813 | 7 | fact-q153943-parent-q154741 |
| Denmark (title-q35) | Kingdom | 800.. | 8 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 8 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 8 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 8 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 8 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 8 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 8 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 8 |  |
| Kingdom of Italy (title-q223936) | Kingdom | 1805..1814 | 8 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 8 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 8 |  |

17 additional candidates omitted from this row.

### fact-q62633-parent-q34266 | Grand Duchy of Finland -> Russian Empire

- child_id: title-q62633
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q34266
- current_parent_rank: Empire
- span: 1809..1917
- candidate_count: 44
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 2 | fact-q1069959-parent-q34266 |
| Denmark (title-q35) | Kingdom | 800.. | 109 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 109 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 109 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 109 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 109 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 109 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 109 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 109 |  |
| Sweden (title-q34) | Kingdom | 900.. | 109 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 102 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 86 |  |

32 additional candidates omitted from this row.

### fact-q704312-parent-q154741 | Grand Duchy of Frankfurt -> Confederation of the Rhine

- child_id: title-q704312
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1810..1813
- candidate_count: 27
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Westphalia (title-q153943) | Kingdom | 1807..1813 | 4 | fact-q153943-parent-q154741 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 4 |  |
| Kingdom of Italy (title-q223936) | Kingdom | 1805..1814 | 4 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 4 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 4 |  |

15 additional candidates omitted from this row.

### fact-q20135-parent-q151624 | Grand Duchy of Hesse -> German Confederation

- child_id: title-q20135
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q20135-parent-q43287 | Grand Duchy of Hesse -> German Empire

- child_id: title-q20135
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q49683-parent-q172107 | Grand Duchy of Lithuania -> Polish-Lithuanian Commonwealth

- child_id: title-q49683
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1795
- candidate_count: 39
- bridge_candidate_count: 3

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 227 | fact-q171348-parent-q172107 |
| Kingdom of Poland (title-q1649871) | Kingdom | 1386..1569 | 1 | fact-q1649871-parent-q172107 |
| Kingdom of Poland (title-q8890160) | Kingdom | 1025..1569 | 1 | fact-q8890160-parent-q172107 |
| Denmark (title-q35) | Kingdom | 800.. | 227 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 227 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 227 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 227 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 227 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 227 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 227 |  |
| Kingdom of Ireland (title-q215530) | Kingdom | 1542..1801 | 227 |  |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 227 |  |

27 additional candidates omitted from this row.

### fact-q158445-parent-q150981 | Grand Duchy of Mecklenburg-Schwerin -> North German Confederation

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 15
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q150981 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q150981 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 4 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 |  |

3 additional candidates omitted from this row.

### fact-q158445-parent-q151624 | Grand Duchy of Mecklenburg-Schwerin -> German Confederation

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q158445-parent-q43287 | Grand Duchy of Mecklenburg-Schwerin -> German Empire

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q161215-parent-q150981 | Grand Duchy of Mecklenburg-Strelitz -> North German Confederation

- child_id: title-q161215
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 15
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q150981 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q150981 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 4 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 |  |

3 additional candidates omitted from this row.

### fact-q161215-parent-q151624 | Grand Duchy of Mecklenburg-Strelitz -> German Confederation

- child_id: title-q161215
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q693669-parent-q150981 | Grand Duchy of Oldenburg -> North German Confederation

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 15
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q150981 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q150981 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 4 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 |  |

3 additional candidates omitted from this row.

### fact-q693669-parent-q151624 | Grand Duchy of Oldenburg -> German Confederation

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1816..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 51 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 51 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 51 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 51 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 51 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 51 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 51 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 51 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 51 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 51 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 51 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 51 |  |

19 additional candidates omitted from this row.

### fact-q693669-parent-q43287 | Grand Duchy of Oldenburg -> German Empire

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 22
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 48 | fact-q154195-parent-q43287 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 48 | fact-q27306-parent-q43287 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 48 | fact-q153015-parent-q43287 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 48 | fact-q159631-parent-q43287 |
| Kingdom of Poland (title-q696908) | Kingdom | 1916..1918 | 3 | fact-q696908-parent-q43287 |
| Denmark (title-q35) | Kingdom | 800.. | 48 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 48 |  |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 48 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 48 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 48 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 48 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 48 |  |

10 additional candidates omitted from this row.

### fact-q154849-parent-q12548 | Grand Duchy of Tuscany -> Holy Roman Empire

- child_id: title-q154849
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1575..1801
- candidate_count: 37
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 227 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 227 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 227 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 227 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 227 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 227 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 227 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 227 |  |
| Kingdom of Ireland (title-q215530) | Kingdom | 1542..1801 | 227 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 227 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 227 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 227 |  |

25 additional candidates omitted from this row.

### fact-q698089-parent-q154741 | Grand Duchy of Wurzburg -> Confederation of the Rhine

- child_id: title-q698089
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1806..1813
- candidate_count: 29
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Westphalia (title-q153943) | Kingdom | 1807..1813 | 7 | fact-q153943-parent-q154741 |
| Denmark (title-q35) | Kingdom | 800.. | 8 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 8 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 8 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 8 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 8 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 8 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 8 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 8 |  |
| Kingdom of Italy (title-q223936) | Kingdom | 1805..1814 | 8 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 8 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 8 |  |

17 additional candidates omitted from this row.

### fact-q170770-parent-q141472 | Grand Principality of Moscow -> Golden Horde

- child_id: title-q170770
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q141472
- current_parent_rank: Empire
- span: 1263..1478
- candidate_count: 39
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Croatia in personal union with Hungary (title-q1789596) | Kingdom | 1102..1526 | 216 |  |
| Denmark (title-q35) | Kingdom | 800.. | 216 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 216 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 216 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 216 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 216 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 216 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 216 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 216 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 216 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 216 |  |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 216 |  |

27 additional candidates omitted from this row.

### fact-q83546-parent-q141472 | Grand Principality of Vladimir -> Golden Horde

- child_id: title-q83546
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q141472
- current_parent_rank: Empire
- span: 1259..1389
- candidate_count: 37
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Croatia in personal union with Hungary (title-q1789596) | Kingdom | 1102..1526 | 131 |  |
| Denmark (title-q35) | Kingdom | 800.. | 131 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 131 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 131 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 131 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 131 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 131 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 131 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 131 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 131 |  |
| Kingdom of Georgia (title-q154667) | Kingdom | 1008..1466 | 131 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 131 |  |

25 additional candidates omitted from this row.

### fact-q673865-parent-q12548 | Hohenzollern-Hechingen -> Holy Roman Empire

- child_id: title-q673865
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1576..1806
- candidate_count: 42
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 231 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 231 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 231 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 231 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 231 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 231 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 231 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 231 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 231 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 231 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 231 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 231 |  |

30 additional candidates omitted from this row.

### fact-q673865-parent-q151624 | Hohenzollern-Hechingen -> German Confederation

- child_id: title-q673865
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1850
- candidate_count: 30
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 36 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 36 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 36 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 36 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 36 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 36 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 36 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 36 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 36 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 36 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 36 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 36 |  |

18 additional candidates omitted from this row.

### fact-q157013-parent-q12548 | Hohenzollern-Sigmaringen -> Holy Roman Empire

- child_id: title-q157013
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1576..1806
- candidate_count: 42
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 231 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 231 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 231 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 231 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 231 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 231 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 231 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 231 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 231 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 231 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 231 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 231 |  |

30 additional candidates omitted from this row.

### fact-q157013-parent-q151624 | Hohenzollern-Sigmaringen -> German Confederation

- child_id: title-q157013
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1850
- candidate_count: 30
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 36 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 36 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 36 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 36 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 36 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 36 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 36 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 36 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 36 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 36 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 36 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 36 |  |

18 additional candidates omitted from this row.

### fact-q50625-parent-q20 | Hordaland -> Kingdom of Norway

- child_id: title-q50625
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q20
- current_parent_rank: Kingdom
- span: 1919..2019
- candidate_count: 3
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Principality of Albania (title-q187035) | Duchy | 1914..1925 | 7 |  |
| Schwarzburg-Sondershausen (title-q630163) | Duchy | 1599..1920 | 2 |  |
| Schwarzburg-Rudolstadt (title-q695316) | Duchy | 1599..1919 | 1 |  |

### fact-q699923-parent-q71084 | Illyrian Provinces -> First French Empire

- child_id: title-q699923
- child_rank: Province
- expected_parent_rank: Kingdom
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1809..1815
- candidate_count: 31
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q223936) | Kingdom | 1805..1814 | 6 | fact-q223936-parent-q71084 |
| Kingdom of Holland (title-q212278) | Kingdom | 1806..1810 | 2 | fact-q212278-parent-q71084 |
| Denmark (title-q35) | Kingdom | 800.. | 7 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 7 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 7 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 7 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 7 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 7 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 7 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 7 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 7 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 7 |  |

19 additional candidates omitted from this row.

### fact-q1448131-parent-q926295 | Italian protectorate of Albania -> Italian Empire

- child_id: title-q1448131
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q926295
- current_parent_rank: Empire
- span: 1939..1943
- candidate_count: 0
- bridge_candidate_count: 0

No accepted overlapping candidate titles found.

### fact-q154195-parent-q151624 | Kingdom of Bavaria -> German Confederation

- child_id: title-q154195
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q154195-parent-q43287 | Kingdom of Bavaria -> German Empire

- child_id: title-q154195
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 48 |  |

### fact-q42585-parent-q12548 | Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q42585
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1198..1803
- candidate_count: 7
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 518 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 486 |  |
| Polish-Lithuanian Commonwealth (title-q172107) | Crown | 1569..1795 | 227 |  |
| Burgundian State (title-q7882199) | Crown | 1363..1559 | 197 |  |
| Kalmar Union (title-q62623) | Crown | 1397..1523 | 127 |  |
| Kingdom of Great Britain (title-q161885) | Crown | 1707..1801 | 95 |  |
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 3 |  |

### fact-q42585-parent-q131964 | Kingdom of Bohemia -> Austrian Empire

- child_id: title-q42585
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- candidate_count: 2
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 63 |  |
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 8 |  |

### fact-q42585-parent-q28513 | Kingdom of Bohemia -> Austria-Hungary

- child_id: title-q42585
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q2980623-parent-q12560 | Kingdom of Bosnia -> Ottoman Empire

- child_id: title-q2980623
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1463..1463
- candidate_count: 4
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Burgundian State (title-q7882199) | Crown | 1363..1559 | 1 |  |
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 1 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 1 |  |
| Kalmar Union (title-q62623) | Crown | 1397..1523 | 1 |  |

### fact-q533558-parent-q28513 | Kingdom of Croatia-Slavonia -> Austria-Hungary

- child_id: title-q533558
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1868..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 51 |  |

### fact-q223793-parent-q71084 | Kingdom of Etruria -> First French Empire

- child_id: title-q223793
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1804..1807
- candidate_count: 2
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 2 | fact-q154741-parent-q71084 |
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 4 |  |

### fact-q2396442-parent-q131964 | Kingdom of Galicia and Lodomeria -> Austrian Empire

- child_id: title-q2396442
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- candidate_count: 2
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 63 |  |
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 8 |  |

### fact-q2396442-parent-q28513 | Kingdom of Galicia and Lodomeria -> Austria-Hungary

- child_id: title-q2396442
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q164079-parent-q151624 | Kingdom of Hanover -> German Confederation

- child_id: title-q164079
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q212278-parent-q71084 | Kingdom of Holland -> First French Empire

- child_id: title-q212278
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1806..1810
- candidate_count: 2
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 5 | fact-q154741-parent-q71084 |
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 5 |  |

### fact-q171150-parent-q131964 | Kingdom of Hungary -> Austrian Empire

- child_id: title-q171150
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1867
- candidate_count: 2
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 64 |  |
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 8 |  |

### fact-q253094-parent-q131964 | Kingdom of Hungary -> Austrian Empire

- child_id: title-q253094
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1867
- candidate_count: 2
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 64 |  |
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 8 |  |

### fact-q25395037-parent-q28513 | Kingdom of Hungary -> Austria-Hungary

- child_id: title-q25395037
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q1117051-parent-q131964 | Kingdom of Illyria -> Austrian Empire

- child_id: title-q1117051
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1816..1849
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 34 |  |

### fact-q1069959-parent-q34266 | Kingdom of Imereti -> Russian Empire

- child_id: title-q1069959
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q34266
- current_parent_rank: Empire
- span: 1804..1810
- candidate_count: 2
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 7 |  |
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 5 |  |

### fact-q223936-parent-q71084 | Kingdom of Italy -> First French Empire

- child_id: title-q223936
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1805..1814
- candidate_count: 2
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 8 | fact-q154741-parent-q71084 |
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 10 |  |

### fact-q838931-parent-q12548 | Kingdom of Italy -> Holy Roman Empire

- child_id: title-q838931
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1806
- candidate_count: 8
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 554 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 486 |  |
| Polish-Lithuanian Commonwealth (title-q172107) | Crown | 1569..1795 | 227 |  |
| Burgundian State (title-q7882199) | Crown | 1363..1559 | 197 |  |
| Kalmar Union (title-q62623) | Crown | 1397..1523 | 127 |  |
| Kingdom of Great Britain (title-q161885) | Crown | 1707..1801 | 95 |  |
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 6 |  |
| Confederation of the Rhine (title-q154741) | Crown | 1806..1813 | 1 |  |

### fact-q2346056-parent-q186096 | Kingdom of Livonia -> Tsardom of Russia

- child_id: title-q2346056
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q186096
- current_parent_rank: Empire
- span: 1570..1578
- candidate_count: 3
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 9 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 9 |  |
| Polish-Lithuanian Commonwealth (title-q172107) | Crown | 1569..1795 | 9 |  |

### fact-q209857-parent-q131964 | Kingdom of Lombardy-Venetia -> Austrian Empire

- child_id: title-q209857
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q696908-parent-q43287 | Kingdom of Poland -> German Empire

- child_id: title-q696908
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1916..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 3 |  |

### fact-q45670-parent-q377350 | Kingdom of Portugal -> Iberian Union

- child_id: title-q45670
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q377350
- current_parent_rank: Empire
- span: 1580..1640
- candidate_count: 3
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 61 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 61 |  |
| Polish-Lithuanian Commonwealth (title-q172107) | Crown | 1569..1795 | 61 |  |

### fact-q27306-parent-q150981 | Kingdom of Prussia -> North German Confederation

- child_id: title-q27306
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 4 |  |

### fact-q27306-parent-q151624 | Kingdom of Prussia -> German Confederation

- child_id: title-q27306
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q27306-parent-q43287 | Kingdom of Prussia -> German Empire

- child_id: title-q27306
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 48 |  |

### fact-q153015-parent-q150981 | Kingdom of Saxony -> North German Confederation

- child_id: title-q153015
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 4 |  |

### fact-q153015-parent-q151624 | Kingdom of Saxony -> German Confederation

- child_id: title-q153015
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q153015-parent-q43287 | Kingdom of Saxony -> German Empire

- child_id: title-q153015
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 48 |  |

### fact-q2415003-parent-q1406298 | Kingdom of Serbia -> Serbian Empire

- child_id: title-q2415003
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q1406298
- current_parent_rank: Empire
- span: 1346..1346
- candidate_count: 2
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 1 |  |
| Crown of Castile (title-q217196) | Crown | 1230..1715 | 1 |  |

### fact-q325461-parent-q178897 | Kingdom of Thessalonica -> Latin Empire

- child_id: title-q325461
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q178897
- current_parent_rank: Empire
- span: 1204..1224
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of Aragon (title-q204920) | Crown | 1162..1715 | 21 |  |

### fact-q159631-parent-q151624 | Kingdom of Wurttemberg -> German Confederation

- child_id: title-q159631
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 52 |  |

### fact-q159631-parent-q43287 | Kingdom of Wurttemberg -> German Empire

- child_id: title-q159631
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- candidate_count: 1
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| United Kingdom of Great Britain and Ireland (title-q174193) | Crown | 1801..1922 | 48 |  |

### fact-q13590051-parent-q42834 | Kingdom of the Burgundians -> Western Roman Empire

- child_id: title-q13590051
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q42834
- current_parent_rank: Empire
- span: 443..476
- candidate_count: 0
- bridge_candidate_count: 0

No accepted overlapping candidate titles found.

### fact-q751868-parent-q12548 | Landgraviate of Brabant -> Holy Roman Empire

- child_id: title-q751868
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1085..1183
- candidate_count: 30
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 99 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 99 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 99 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 99 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 99 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 99 |  |
| Kingdom of Deheubarth (title-q837136) | Kingdom | 920..1197 | 99 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 99 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 99 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 99 |  |
| Kingdom of Georgia (title-q154667) | Kingdom | 1008..1466 | 99 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 99 |  |

18 additional candidates omitted from this row.

### fact-q695322-parent-q12548 | Landgraviate of Hesse -> Holy Roman Empire

- child_id: title-q695322
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1264..1567
- candidate_count: 42
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 304 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 304 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 304 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 304 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 304 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 304 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 304 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 304 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 304 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 304 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 304 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 304 |  |

30 additional candidates omitted from this row.

### fact-q693551-parent-q12548 | Landgraviate of Hesse-Darmstadt -> Holy Roman Empire

- child_id: title-q693551
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1567..1806
- candidate_count: 45
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 240 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 240 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 240 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 240 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 240 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 240 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 240 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 240 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 240 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 240 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 240 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 240 |  |

33 additional candidates omitted from this row.

### fact-q168651-parent-q12548 | Landgraviate of Hesse-Kassel -> Holy Roman Empire

- child_id: title-q168651
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1567..1803
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 237 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 237 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 237 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 237 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 237 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 237 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 237 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 237 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 237 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 237 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 237 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 237 |  |

28 additional candidates omitted from this row.

### fact-q673837-parent-q12548 | Landgraviate of Hesse-Marburg -> Holy Roman Empire

- child_id: title-q673837
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1458..1604
- candidate_count: 38
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 147 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 147 | fact-q838931-parent-q12548 |
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 147 |  |
| Denmark (title-q35) | Kingdom | 800.. | 147 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 147 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 147 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 147 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 147 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 147 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 147 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 147 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 147 |  |

26 additional candidates omitted from this row.

### fact-q58942549-parent-q12548 | Landgraviate of Lower Alsace -> Holy Roman Empire

- child_id: title-q58942549
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1200..1700
- candidate_count: 48
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 501 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 501 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 501 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 501 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 501 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 501 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 501 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 501 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 501 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 501 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 501 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 501 |  |

36 additional candidates omitted from this row.

### fact-q660393-parent-q12548 | Lower Lotharingia -> Holy Roman Empire

- child_id: title-q660393
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1190
- candidate_count: 35
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 229 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 229 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 229 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 229 |  |
| Kingdom of Deheubarth (title-q837136) | Kingdom | 920..1197 | 229 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 229 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 229 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 229 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 229 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 229 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 229 |  |
| Sweden (title-q34) | Kingdom | 900.. | 229 |  |

23 additional candidates omitted from this row.

### fact-q426488-parent-q12548 | March of Turin -> Holy Roman Empire

- child_id: title-q426488
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 964..1091
- candidate_count: 29
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 128 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 128 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 128 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 128 |  |
| Kingdom of Croatia (title-q858841) | Kingdom | 925..1102 | 128 |  |
| Kingdom of Deheubarth (title-q837136) | Kingdom | 920..1197 | 128 |  |
| Kingdom of Dublin (title-q436994) | Kingdom | 839..1171 | 128 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 128 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 128 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 128 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 128 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 128 |  |

17 additional candidates omitted from this row.

### fact-q1867844-parent-q12548 | March of Tuscany -> Holy Roman Empire

- child_id: title-q1867844
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1197
- candidate_count: 35
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 236 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 236 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 236 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 236 |  |
| Kingdom of Deheubarth (title-q837136) | Kingdom | 920..1197 | 236 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 236 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 236 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 236 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 236 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 236 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 236 |  |
| Sweden (title-q34) | Kingdom | 900.. | 236 |  |

23 additional candidates omitted from this row.

### fact-q283627-parent-q12548 | Margraviate of Austria -> Holy Roman Empire

- child_id: title-q283627
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 976..1156
- candidate_count: 33
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 181 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 181 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 181 |  |
| Kingdom of Breifne (title-q905131) | Kingdom | 700..1256 | 181 |  |
| Kingdom of Deheubarth (title-q837136) | Kingdom | 920..1197 | 181 |  |
| Kingdom of Dublin (title-q436994) | Kingdom | 839..1171 | 181 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 181 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 181 |  |
| Kingdom of Gwynedd (title-q816814) | Kingdom | 401..1216 | 181 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 181 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 181 |  |
| Kingdom of Pamplona (title-q3446210) | Kingdom | 824..1162 | 181 |  |

21 additional candidates omitted from this row.

### fact-q148499-parent-q12548 | Margraviate of Brandenburg -> Holy Roman Empire

- child_id: title-q148499
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1157..1806
- candidate_count: 63
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 650 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 609 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 650 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 650 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 650 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 650 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 650 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 650 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 650 |  |
| Sweden (title-q34) | Kingdom | 900.. | 650 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 645 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 635 |  |

51 additional candidates omitted from this row.

### fact-q170180-parent-q12548 | Margraviate of Meissen -> Holy Roman Empire

- child_id: title-q170180
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 965..1423
- candidate_count: 50
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 459 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 226 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 459 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 459 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 459 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 459 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 459 |  |
| Sweden (title-q34) | Kingdom | 900.. | 459 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 456 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 437 |  |
| Kingdom of Norway (title-q2196956) | Kingdom | 872..1397 | 433 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 424 |  |

38 additional candidates omitted from this row.

### fact-q2670751-parent-q28513 | Margraviate of Moravia -> Austria-Hungary

- child_id: title-q2670751
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- candidate_count: 23
- bridge_candidate_count: 4

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 | fact-q42585-parent-q28513 |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 | fact-q2396442-parent-q28513 |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 52 | fact-q25395037-parent-q28513 |
| Kingdom of Croatia-Slavonia (title-q533558) | Kingdom | 1868..1918 | 51 | fact-q533558-parent-q28513 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 52 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 |  |

11 additional candidates omitted from this row.

### fact-q552822-parent-q12548 | Mecklenburg-Gustrow -> Holy Roman Empire

- child_id: title-q552822
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1520..1695
- candidate_count: 36
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 176 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 176 | fact-q838931-parent-q12548 |
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 176 |  |
| Denmark (title-q35) | Kingdom | 800.. | 176 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 176 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 176 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 176 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 176 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 176 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 176 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 176 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 176 |  |

24 additional candidates omitted from this row.

### fact-q2273304-parent-q12560 | Moravian Serbia -> Ottoman Empire

- child_id: title-q2273304
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1390..1402
- candidate_count: 33
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Croatia in personal union with Hungary (title-q1789596) | Kingdom | 1102..1526 | 13 |  |
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 13 |  |
| Denmark (title-q35) | Kingdom | 800.. | 13 |  |
| Judicate of Arborea (title-q1241847) | Kingdom | 800..1420 | 13 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 13 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 13 |  |
| Kingdom of Bosnia (title-q2980623) | Kingdom | 1377..1463 | 13 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 13 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 13 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 13 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 13 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 13 |  |

21 additional candidates omitted from this row.

### fact-q736029-parent-q12548-1303 | Nassau-Siegen -> Holy Roman Empire

- child_id: title-q736029
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1303..1328
- candidate_count: 53
- bridge_candidate_count: 34

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 26 | fact-q686965-parent-q12548 |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 26 | fact-q556263-parent-q12548 |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 26 | fact-q830084-parent-q12548 |
| Duchy of Austria (title-q3624335) | Duchy | 1156..1453 | 26 | fact-q3624335-parent-q12548 |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 26 | fact-q47261-parent-q12548 |
| Duchy of Brabant (title-q159856) | Duchy | 1183..1795 | 26 | fact-q159856-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 26 | fact-q641138-parent-q12548 |
| Duchy of Julich (title-q836937) | Duchy | 1003..1794 | 26 | fact-q836937-parent-q12548 |
| Duchy of Lorraine (title-q155019) | Duchy | 959..1766 | 26 | fact-q155019-parent-q12548 |
| Duchy of Pomerania (title-q696640) | Duchy | 1121..1637 | 26 | fact-q696640-parent-q12548 |
| Duchy of Westphalia (title-q657241) | Duchy | 1180..1803 | 26 | fact-q657241-parent-q12548 |
| Electoral Palatinate (title-q22880) | Duchy | 1085..1803 | 26 | fact-q22880-parent-q12548 |

41 additional candidates omitted from this row.

### fact-q736029-parent-q12548-1606 | Nassau-Siegen -> Holy Roman Empire

- child_id: title-q736029
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1606..1743
- candidate_count: 76
- bridge_candidate_count: 60

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Anhalt-Bernburg (title-q686965) | Duchy | 1252..1863 | 138 | fact-q686965-parent-q12548 |
| Anhalt-Dessau (title-q278874) | Duchy | 1396..1853 | 138 | fact-q278874-parent-q12548 |
| Anhalt-Kothen (title-q264970) | Duchy | 1396..1863 | 138 | fact-q264970-parent-q12548 |
| Archduchy of Austria (title-q699964) | Duchy | 1358..1918 | 138 | fact-q699964-parent-q12548 |
| Brunswick-Luneburg (title-q556263) | Duchy | 1235..1806 | 138 | fact-q556263-parent-q12548 |
| Brunswick-Wolfenbuttel (title-q830084) | Duchy | 1269..1815 | 138 | fact-q830084-parent-q12548 |
| Duchy of Bavaria (title-q47261) | Duchy | 907..1805 | 138 | fact-q47261-parent-q12548 |
| Duchy of Berg (title-q151095) | Duchy | 1380..1806 | 138 | fact-q151095-parent-q12548 |
| Duchy of Brabant (title-q159856) | Duchy | 1183..1795 | 138 | fact-q159856-parent-q12548 |
| Duchy of Carniola (title-q2360973) | Duchy | 1364..1918 | 138 | fact-q2360973-parent-q12548 |
| Duchy of Cleves (title-q641138) | Duchy | 1092..1795 | 138 | fact-q641138-parent-q12548 |
| Duchy of Holstein (title-q704288) | Duchy | 1474..1867 | 138 | fact-q704288-parent-q12548 |

64 additional candidates omitted from this row.

### fact-q454436-parent-q12548 | Palatinate-Sulzbach -> Holy Roman Empire

- child_id: title-q454436
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1557..1806
- candidate_count: 45
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 250 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 250 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 250 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 250 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 250 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 250 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 250 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 250 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 250 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 250 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 250 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 250 |  |

33 additional candidates omitted from this row.

### fact-q825902-parent-q172107 | Polish-Lithuanian union -> Polish-Lithuanian Commonwealth

- child_id: title-q825902
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1569
- candidate_count: 33
- bridge_candidate_count: 3

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 1 | fact-q171348-parent-q172107 |
| Kingdom of Poland (title-q1649871) | Kingdom | 1386..1569 | 1 | fact-q1649871-parent-q172107 |
| Kingdom of Poland (title-q8890160) | Kingdom | 1025..1569 | 1 | fact-q8890160-parent-q172107 |
| Denmark (title-q35) | Kingdom | 800.. | 1 |  |
| Eastern Hungarian Kingdom (title-q625380) | Kingdom | 1526..1570 | 1 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 1 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 1 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 1 |  |
| Kingdom of Desmond (title-q904346) | Kingdom | 1118..1596 | 1 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 1 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 1 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 1 |  |

21 additional candidates omitted from this row.

### fact-q701614-parent-q12548 | Prince-Archbishopric of Salzburg -> Holy Roman Empire

- child_id: title-q701614
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1278..1803
- candidate_count: 49
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 526 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 526 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 526 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 526 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 526 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 526 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 526 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 526 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 526 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 526 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 526 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 526 |  |

37 additional candidates omitted from this row.

### fact-q173863-parent-q12548 | Prince-Bishopric of Augsburg -> Holy Roman Empire

- child_id: title-q173863
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- candidate_count: 64
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 842 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 842 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 842 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 842 |  |
| Sweden (title-q34) | Kingdom | 900.. | 842 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 805 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 804 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 746 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 746 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |

52 additional candidates omitted from this row.

### fact-q319586-parent-q12548 | Prince-Bishopric of Basel -> Holy Roman Empire

- child_id: title-q319586
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1032..1803
- candidate_count: 61
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 772 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 772 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 772 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 772 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 772 |  |
| Sweden (title-q34) | Kingdom | 900.. | 772 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 760 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 676 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 676 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |

49 additional candidates omitted from this row.

### fact-q259511-parent-q12548 | Prince-Bishopric of Freising -> Holy Roman Empire

- child_id: title-q259511
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1294..1802
- candidate_count: 49
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 509 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 509 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 509 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 509 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 509 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 509 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 509 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 509 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 509 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 509 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 509 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 509 |  |

37 additional candidates omitted from this row.

### fact-q158835-parent-q12548 | Prince-Bishopric of Liege -> Holy Roman Empire

- child_id: title-q158835
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 985..1795
- candidate_count: 61
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 811 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 598 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 811 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 811 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 811 |  |
| Sweden (title-q34) | Kingdom | 900.. | 811 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 805 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 796 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 723 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 723 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 711 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 673 |  |

49 additional candidates omitted from this row.

### fact-q650645-parent-q12548 | Prince-Bishopric of Minden -> Holy Roman Empire

- child_id: title-q650645
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1648
- candidate_count: 49
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 469 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 451 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 469 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 469 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 469 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 469 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 469 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 469 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 469 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 469 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 469 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 469 |  |

37 additional candidates omitted from this row.

### fact-q3324486-parent-q12560 | Prince-Bishopric of Montenegro -> Ottoman Empire

- child_id: title-q3324486
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1516..1696
- candidate_count: 36
- bridge_candidate_count: 1

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Eastern Hungarian Kingdom (title-q625380) | Kingdom | 1526..1570 | 45 | fact-q625380-parent-q12560 |
| Crown of the Kingdom of Poland (title-q171348) | Kingdom | 1386..1795 | 181 |  |
| Denmark (title-q35) | Kingdom | 800.. | 181 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 181 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 181 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 181 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 181 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 181 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 181 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 181 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 181 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 181 |  |

24 additional candidates omitted from this row.

### fact-q697254-parent-q12548 | Prince-Bishopric of Munster -> Holy Roman Empire

- child_id: title-q697254
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1802
- candidate_count: 55
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 623 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 605 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 623 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 623 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 623 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 623 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 623 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 623 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 623 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 623 |  |
| Sweden (title-q34) | Kingdom | 900.. | 623 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 612 |  |

43 additional candidates omitted from this row.

### fact-q477035-parent-q12548 | Prince-Bishopric of Osnabruck -> Holy Roman Empire

- child_id: title-q477035
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1225..1803
- candidate_count: 52
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 579 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 579 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 579 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 579 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 579 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 579 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 579 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 579 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 579 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 579 |  |
| Sweden (title-q34) | Kingdom | 900.. | 579 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 567 |  |

40 additional candidates omitted from this row.

### fact-q649192-parent-q12548 | Prince-Bishopric of Paderborn -> Holy Roman Empire

- child_id: title-q649192
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1281..1802
- candidate_count: 49
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 522 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 522 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 522 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 522 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 522 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 522 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 522 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 522 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 522 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 522 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 522 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 522 |  |

37 additional candidates omitted from this row.

### fact-q771332-parent-q12548 | Prince-Bishopric of Strasbourg -> Holy Roman Empire

- child_id: title-q771332
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 982..1803
- candidate_count: 62
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 822 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 822 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 822 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 822 |  |
| Sweden (title-q34) | Kingdom | 900.. | 822 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 805 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 804 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 726 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 726 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |

50 additional candidates omitted from this row.

### fact-q328001-parent-q12548 | Prince-Bishopric of Toul -> Holy Roman Empire

- child_id: title-q328001
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1048..1648
- candidate_count: 54
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 601 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 451 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 601 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 601 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 601 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 601 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 601 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 601 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 601 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 601 |  |
| Sweden (title-q34) | Kingdom | 900.. | 601 |  |
| Kingdom of Castile (title-q179293) | Kingdom | 1065..1715 | 584 |  |

42 additional candidates omitted from this row.

### fact-q1231403-parent-q12548 | Prince-Bishopric of Trent -> Holy Roman Empire

- child_id: title-q1231403
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1027..1803
- candidate_count: 61
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 777 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 606 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 777 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 777 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 777 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 777 |  |
| Sweden (title-q34) | Kingdom | 900.. | 777 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 765 |  |
| Kingdom of Toledo (title-q2301372) | Kingdom | 1085..1833 | 719 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 681 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 681 |  |
| Kingdom of Sicily (title-q188586) | Kingdom | 1130..1816 | 674 |  |

49 additional candidates omitted from this row.

### fact-q707767-parent-q12548 | Prince-Bishopric of Utrecht -> Holy Roman Empire

- child_id: title-q707767
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1024..1528
- candidate_count: 53
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 505 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 331 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 505 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 505 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 505 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 505 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 505 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 505 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 505 |  |
| Sweden (title-q34) | Kingdom | 900.. | 505 |  |
| Kingdom of Poland (title-q8890160) | Kingdom | 1025..1569 | 504 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 494 |  |

41 additional candidates omitted from this row.

### fact-q17015016-parent-q12548 | Prince-Bishopric of Verdun -> Holy Roman Empire

- child_id: title-q17015016
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 997..1552
- candidate_count: 55
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 556 | fact-q838931-parent-q12548 |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 355 | fact-q42585-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 556 |  |
| Kingdom of England (title-q179876) | Kingdom | 927..1707 | 556 |  |
| Kingdom of France (title-q70972) | Kingdom | 987..1791 | 556 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 556 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 556 |  |
| Kingdom of Scotland (title-q230791) | Kingdom | 843..1707 | 556 |  |
| Sweden (title-q34) | Kingdom | 900.. | 556 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 553 |  |
| Kingdom of Poland (title-q8890160) | Kingdom | 1025..1569 | 528 |  |
| Kingdom of Aragon (title-q199442) | Kingdom | 1035..1707 | 518 |  |

43 additional candidates omitted from this row.

### fact-q14551680-parent-q12548 | Principality of Lippe -> Holy Roman Empire

- child_id: title-q14551680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1789..1806
- candidate_count: 31
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 18 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 18 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 18 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 18 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 18 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 18 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 18 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 18 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 18 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 18 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 18 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 18 |  |

19 additional candidates omitted from this row.

### fact-q14551680-parent-q151624 | Principality of Lippe -> German Confederation

- child_id: title-q14551680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q684030-parent-q12560 | Principality of Serbia -> Ottoman Empire

- child_id: title-q684030
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1815..1867
- candidate_count: 32
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Denmark (title-q35) | Kingdom | 800.. | 53 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 53 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 53 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 53 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 53 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 53 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 53 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 53 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 53 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 53 |  |
| Sweden (title-q34) | Kingdom | 900.. | 53 |  |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 |  |

20 additional candidates omitted from this row.

### fact-q158151-parent-q12548 | Saxe-Altenburg -> Holy Roman Empire

- child_id: title-q158151
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1602..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 205 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 205 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 205 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 205 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 205 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 205 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 205 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 205 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 205 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 205 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 205 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 205 |  |

28 additional candidates omitted from this row.

### fact-q158151-parent-q151624 | Saxe-Altenburg -> German Confederation

- child_id: title-q158151
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q3462133-parent-q151624 | Saxe-Coburg and Gotha -> German Confederation

- child_id: title-q3462133
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1826..1866
- candidate_count: 29
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 41 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 41 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 41 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 41 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 41 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 41 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 41 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 41 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 41 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 41 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 41 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 41 |  |

17 additional candidates omitted from this row.

### fact-q700663-parent-q12548 | Saxe-Coburg-Saalfeld -> Holy Roman Empire

- child_id: title-q700663
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1699..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 108 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 108 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 108 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 108 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 108 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 108 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 108 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 108 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 108 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 108 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 108 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 108 |  |

28 additional candidates omitted from this row.

### fact-q700663-parent-q151624 | Saxe-Coburg-Saalfeld -> German Confederation

- child_id: title-q700663
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1825
- candidate_count: 27
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 11 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 11 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 11 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 11 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 11 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 11 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 11 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 11 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 11 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 11 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 11 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 11 |  |

15 additional candidates omitted from this row.

### fact-q675085-parent-q12548 | Saxe-Gotha-Altenburg -> Holy Roman Empire

- child_id: title-q675085
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1680..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 127 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 127 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 127 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 127 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 127 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 127 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 127 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 127 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 127 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 127 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 127 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 127 |  |

28 additional candidates omitted from this row.

### fact-q675085-parent-q151624 | Saxe-Gotha-Altenburg -> German Confederation

- child_id: title-q675085
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1826
- candidate_count: 27
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 12 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 12 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 12 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 12 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 12 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 12 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 12 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 12 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 12 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 12 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 12 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 12 |  |

15 additional candidates omitted from this row.

### fact-q281005-parent-q12548 | Saxe-Hildburghausen -> Holy Roman Empire

- child_id: title-q281005
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1680..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 127 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 127 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 127 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 127 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 127 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 127 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 127 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 127 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 127 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 127 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 127 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 127 |  |

28 additional candidates omitted from this row.

### fact-q281005-parent-q151624 | Saxe-Hildburghausen -> German Confederation

- child_id: title-q281005
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1826
- candidate_count: 27
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 12 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 12 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 12 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 12 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 12 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 12 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 12 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 12 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 12 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 12 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 12 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 12 |  |

15 additional candidates omitted from this row.

### fact-q313175-parent-q12548 | Saxe-Lauenburg -> Holy Roman Empire

- child_id: title-q313175
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1296..1806
- candidate_count: 54
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 511 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 511 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 511 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 511 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 511 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 511 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 511 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 511 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 511 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 511 |  |
| Kingdom of Navarre (title-q200262) | Kingdom | 1162..1841 | 511 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 511 |  |

42 additional candidates omitted from this row.

### fact-q313175-parent-q151624 | Saxe-Lauenburg -> German Confederation

- child_id: title-q313175
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q155570-parent-q150981 | Saxe-Weimar-Eisenach -> North German Confederation

- child_id: title-q155570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- candidate_count: 15
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 4 | fact-q27306-parent-q150981 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 4 | fact-q153015-parent-q150981 |
| Denmark (title-q35) | Kingdom | 800.. | 4 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 4 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 4 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 4 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 4 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 4 |  |
| Kingdom of Hungary (title-q25395037) | Kingdom | 1867..1918 | 4 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 4 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 4 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 4 |  |

3 additional candidates omitted from this row.

### fact-q155570-parent-q151624 | Saxe-Weimar-Eisenach -> German Confederation

- child_id: title-q155570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q310650-parent-q12548 | Schaumburg-Lippe -> Holy Roman Empire

- child_id: title-q310650
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1643..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 164 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 164 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 164 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 164 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 164 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 164 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 164 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 164 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 164 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 164 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 164 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 164 |  |

28 additional candidates omitted from this row.

### fact-q310650-parent-q151624 | Schaumburg-Lippe -> German Confederation

- child_id: title-q310650
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q695316-parent-q151624 | Schwarzburg-Rudolstadt -> German Confederation

- child_id: title-q695316
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q630163-parent-q12548 | Schwarzburg-Sondershausen -> Holy Roman Empire

- child_id: title-q630163
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1599..1806
- candidate_count: 40
- bridge_candidate_count: 2

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 208 | fact-q42585-parent-q12548 |
| Kingdom of Italy (title-q838931) | Kingdom | 961..1806 | 208 | fact-q838931-parent-q12548 |
| Denmark (title-q35) | Kingdom | 800.. | 208 |  |
| Kingdom of Galicia (title-q303421) | Kingdom | 910..1833 | 208 |  |
| Kingdom of Granada (title-q1796202) | Kingdom | 1492..1833 | 208 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 208 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 208 |  |
| Kingdom of Imereti (title-q1069959) | Kingdom | 1260..1810 | 208 |  |
| Kingdom of Jaen (title-q1617495) | Kingdom | 1246..1833 | 208 |  |
| Kingdom of Leon (title-q175276) | Kingdom | 910..1833 | 208 |  |
| Kingdom of Murcia (title-q1164500) | Kingdom | 1258..1833 | 208 |  |
| Kingdom of Naples (title-q173065) | Kingdom | 1282..1816 | 208 |  |

28 additional candidates omitted from this row.

### fact-q630163-parent-q151624 | Schwarzburg-Sondershausen -> German Confederation

- child_id: title-q630163
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- candidate_count: 31
- bridge_candidate_count: 5

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 52 | fact-q154195-parent-q151624 |
| Kingdom of Hanover (title-q164079) | Kingdom | 1814..1866 | 52 | fact-q164079-parent-q151624 |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 52 | fact-q27306-parent-q151624 |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 52 | fact-q153015-parent-q151624 |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 52 | fact-q159631-parent-q151624 |
| Denmark (title-q35) | Kingdom | 800.. | 52 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 52 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 52 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 52 |  |
| Kingdom of Hungary (title-q253094) | Kingdom | 1526..1867 | 52 |  |
| Kingdom of Lombardy-Venetia (title-q209857) | Kingdom | 1815..1866 | 52 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 52 |  |

19 additional candidates omitted from this row.

### fact-q958291-parent-q12560 | United Principalities of Moldavia and Wallachia -> Ottoman Empire

- child_id: title-q958291
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1859..1877
- candidate_count: 20
- bridge_candidate_count: 0

| Candidate | Candidate Rank | Exists | Overlap Years | Bridge Fact |
|---|---|---|---:|---|
| Denmark (title-q35) | Kingdom | 800.. | 19 |  |
| Kingdom of Bavaria (title-q154195) | Kingdom | 1806..1918 | 19 |  |
| Kingdom of Bohemia (title-q42585) | Kingdom | 1198..1918 | 19 |  |
| Kingdom of Galicia and Lodomeria (title-q2396442) | Kingdom | 1772..1918 | 19 |  |
| Kingdom of Greece (title-q209065) | Kingdom | 1832..1973 | 19 |  |
| Kingdom of Hungary (title-q171150) | Kingdom | 1000..1946 | 19 |  |
| Kingdom of Portugal (title-q45670) | Kingdom | 1139..1910 | 19 |  |
| Kingdom of Prussia (title-q27306) | Kingdom | 1701..1918 | 19 |  |
| Kingdom of Saxony (title-q153015) | Kingdom | 1806..1918 | 19 |  |
| Kingdom of Wurttemberg (title-q159631) | Kingdom | 1806..1918 | 19 |  |
| Sweden (title-q34) | Kingdom | 900.. | 19 |  |
| Kingdom of Italy (title-q172579) | Kingdom | 1861..1946 | 17 |  |

8 additional candidates omitted from this row.

