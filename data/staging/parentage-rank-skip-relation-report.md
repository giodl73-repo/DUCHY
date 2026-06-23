# DUCHY Parentage Rank Skip Relation Report

sources: 532
facts: 1362
titles: 349
relation_facts: 32
rank_skip_rows: 223
relation_explained_rows: 32
unexplained_rows: 191

## Interpretation

- A row is relation-explained only when the child title has an overlapping source-backed relation fact to the current parent title.
- Relation facts preserve context such as imperial state, confederation member, split control, or vassalage without changing the canonical parentage tree.
- Unexplained rows still need an intermediate parent, a relation fact, or a source-custody blocker before import.

## Relation Kind Counts

| Relation Kind | Rows |
|---|---:|
| composite_crown_component | 1 |
| confederation_member | 5 |
| federal_state_member | 1 |
| imperial_state | 19 |
| split_fief_or_control | 3 |
| subdivision_or_appanage | 2 |
| vassalage_or_suzerainty | 1 |

## Review Rows

### fact-q1049854-parent-q171150 | Abaúj county -> Kingdom of Hungary

- child_id: title-q1049854
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q171150
- current_parent_rank: Kingdom
- span: 1201..1881
- relation_context: none

### fact-q1048340-parent-q926295 | Albanian Kingdom -> Italian Empire

- child_id: title-q1048340
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q926295
- current_parent_rank: Empire
- span: 1939..1939
- relation_context: none

### fact-q686965-parent-q12548 | Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q686965
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1252..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1252..1806 | 1252..1806 |

### fact-q686965-parent-q151624 | Anhalt-Bernburg -> German Confederation

- child_id: title-q686965
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1863

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| confederation_member | German Confederation (title-q151624) | 1815..1863 | 1815..1863 |

### fact-q278874-parent-q12548 | Anhalt-Dessau -> Holy Roman Empire

- child_id: title-q278874
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1396..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1396..1806 | 1396..1806 |

### fact-q278874-parent-q151624 | Anhalt-Dessau -> German Confederation

- child_id: title-q278874
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1853

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| confederation_member | German Confederation (title-q151624) | 1815..1853 | 1815..1853 |

### fact-q264970-parent-q12548 | Anhalt-Kothen -> Holy Roman Empire

- child_id: title-q264970
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1396..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1396..1806 | 1396..1806 |

### fact-q264970-parent-q151624 | Anhalt-Kothen -> German Confederation

- child_id: title-q264970
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1863

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| confederation_member | German Confederation (title-q151624) | 1815..1863 | 1815..1863 |

### fact-q699964-parent-q12548 | Archduchy of Austria -> Holy Roman Empire

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1358..1803

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1358..1803 | 1358..1803 |

### fact-q699964-parent-q131964 | Archduchy of Austria -> Austrian Empire

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- relation_context: none

### fact-q699964-parent-q28513 | Archduchy of Austria -> Austria-Hungary

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q552033-parent-q12548 | Bavaria-Munich -> Holy Roman Empire

- child_id: title-q552033
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1392..1505

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1392..1505 | 1392..1505 |

### fact-q556263-parent-q12548 | Brunswick-Luneburg -> Holy Roman Empire

- child_id: title-q556263
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1235..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1235..1806 | 1235..1806 |

### fact-q830084-parent-q12548 | Brunswick-Wolfenbuttel -> Holy Roman Empire

- child_id: title-q830084
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1269..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1269..1806 | 1269..1806 |

### fact-q568473-parent-q12548 | Burgraviate of Nuremberg -> Holy Roman Empire

- child_id: title-q568473
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1105..1440

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1105..1440 | 1105..1440 |

### fact-q157109-parent-q7882199 | Burgundian Netherlands -> Burgundian State

- child_id: title-q157109
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q7882199
- current_parent_rank: Crown
- span: 1384..1482
- relation_context: none

### fact-q8273263-parent-q203493 | Cetatea-Albă County -> Kingdom of Romania

- child_id: title-q8273263
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q203493
- current_parent_rank: Kingdom
- span: 1925..1944
- relation_context: none

### fact-q1122980-parent-q170174 | Comtat Venaissin -> Papal States

- child_id: title-q1122980
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1274..1791
- relation_context: none

### fact-q1235737-parent-q3446210 | County of Aragon -> Kingdom of Pamplona

- child_id: title-q1235737
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q3446210
- current_parent_rank: Kingdom
- span: 824..1035
- relation_context: none

### fact-q1233672-parent-q204920 | County of Barcelona -> Crown of Aragon

- child_id: title-q1233672
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q204920
- current_parent_rank: Crown
- span: 1162..1164

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| composite_crown_component | Crown of Aragon (title-q204920) | 1162..1164 | 1162..1164 |

### fact-q642314-parent-q12548 | County of Burgundy -> Holy Roman Empire

- child_id: title-q642314
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 982..1678

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 982..1678 | 982..1678 |

### fact-q1541699-parent-q204920 | County of Empuries -> Crown of Aragon

- child_id: title-q1541699
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q204920
- current_parent_rank: Crown
- span: 1341..1402

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| subdivision_or_appanage | Crown of Aragon (title-q204920) | 1341..1402 | 1341..1402 |

### fact-q157070-parent-q12548 | County of Flanders -> Holy Roman Empire

- child_id: title-q157070
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1383
- relation_context: none

### fact-q157070-parent-q12548 | County of Flanders -> Holy Roman Empire

- child_id: title-q157070
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1483..1797

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| split_fief_or_control | Holy Roman Empire (title-q12548) | 1483..1797 | 1483..1797 |

### fact-q675363-parent-q12548 | County of Geneva -> Holy Roman Empire

- child_id: title-q675363
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1032..1401

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1032..1401 | 1032..1401 |

### fact-q2037817-parent-q31929 | County of Girona -> Carolingian Empire

- child_id: title-q2037817
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q31929
- current_parent_rank: Empire
- span: 800..887

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| subdivision_or_appanage | Carolingian Empire (title-q31929) | 800..887 | 800..887 |

### fact-q762943-parent-q12548 | County of Holland -> Holy Roman Empire

- child_id: title-q762943
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1432
- relation_context: none

### fact-q762943-parent-q12548 | County of Holland -> Holy Roman Empire

- child_id: title-q762943
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1483..1795

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| split_fief_or_control | Holy Roman Empire (title-q12548) | 1483..1795 | 1483..1795 |

### fact-q921473-parent-q70972 | County of La Marche -> Kingdom of France

- child_id: title-q921473
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q70972
- current_parent_rank: Kingdom
- span: 1527..1527
- relation_context: none

### fact-q5177890-parent-q12548 | County of Luxembourg -> Holy Roman Empire

- child_id: title-q5177890
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 963..1354
- relation_context: none

### fact-q700198-parent-q27306 | County of Mark -> Kingdom of Prussia

- child_id: title-q700198
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q27306
- current_parent_rank: Kingdom
- span: 1701..1806
- relation_context: none

### fact-q589251-parent-q12548 | County of Montbeliard -> Holy Roman Empire

- child_id: title-q589251
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1042..1793

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1042..1793 | 1042..1793 |

### fact-q599613-parent-q12548 | County of Namur -> Holy Roman Empire

- child_id: title-q599613
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 981..1420
- relation_context: none

### fact-q599613-parent-q12548 | County of Namur -> Holy Roman Empire

- child_id: title-q599613
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1483..1795

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| split_fief_or_control | Holy Roman Empire (title-q12548) | 1483..1795 | 1483..1795 |

### fact-q12817455-parent-q12548 | County of Nassau -> Holy Roman Empire

- child_id: title-q12817455
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1160..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1160..1806 | 1160..1806 |

### fact-q706553-parent-q2577303 | County of Nice -> Kingdom of Sardinia

- child_id: title-q706553
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q2577303
- current_parent_rank: Kingdom
- span: 1814..1818
- relation_context: none

### fact-q1139807-parent-q231392 | County of Portugal -> Kingdom of Asturias

- child_id: title-q1139807
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q231392
- current_parent_rank: Kingdom
- span: 868..909
- relation_context: none

### fact-q1139807-parent-q303421 | County of Portugal -> Kingdom of Galicia

- child_id: title-q1139807
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q303421
- current_parent_rank: Kingdom
- span: 910..1139
- relation_context: none

### fact-q2991382-parent-q70972 | County of Provence -> Kingdom of France

- child_id: title-q2991382
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q70972
- current_parent_rank: Kingdom
- span: 987..1487
- relation_context: none

### fact-q573290-parent-q12548 | County of Ravensberg -> Holy Roman Empire

- child_id: title-q573290
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1140..1806
- relation_context: none

### fact-q1297894-parent-q199442 | County of Ribagorza -> Kingdom of Aragon

- child_id: title-q1297894
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q199442
- current_parent_rank: Kingdom
- span: 1035..1598
- relation_context: none

### fact-q1232887-parent-q12548 | County of Savoy -> Holy Roman Empire

- child_id: title-q1232887
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1003..1416

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1003..1416 | 1003..1416 |

### fact-q1917014-parent-q188586 | County of Sicily -> Kingdom of Sicily

- child_id: title-q1917014
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q188586
- current_parent_rank: Kingdom
- span: 1130..1130
- relation_context: none

### fact-q2991474-parent-q12548 | County of Wurttemberg -> Holy Roman Empire

- child_id: title-q2991474
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1083..1495

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1083..1495 | 1083..1495 |

### fact-q5298169-parent-q203493 | Dorohoi County -> Kingdom of Romania

- child_id: title-q5298169
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q203493
- current_parent_rank: Kingdom
- span: 1881..1947
- relation_context: none

### fact-q686312-parent-q12544 | Duchy of Amalfi -> Byzantine Empire

- child_id: title-q686312
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12544
- current_parent_rank: Empire
- span: 839..958

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| vassalage_or_suzerainty | Byzantine Empire (title-q12544) | 839..958 | 839..958 |

### fact-q16550783-parent-q150981 | Duchy of Anhalt -> North German Confederation

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| confederation_member | North German Confederation (title-q150981) | 1867..1870 | 1867..1870 |

### fact-q16550783-parent-q151624 | Duchy of Anhalt -> German Confederation

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1863..1866

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| confederation_member | German Confederation (title-q151624) | 1863..1866 | 1863..1866 |

### fact-q16550783-parent-q43287 | Duchy of Anhalt -> German Empire

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| federal_state_member | German Empire (title-q43287) | 1871..1918 | 1871..1918 |

### fact-q3624335-parent-q12548 | Duchy of Austria -> Holy Roman Empire

- child_id: title-q3624335
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1156..1453

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1156..1453 | 1156..1453 |

### fact-q47261-parent-q12548 | Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q47261
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1805

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 962..1805 | 962..1805 |

### fact-q151095-parent-q12548 | Duchy of Berg -> Holy Roman Empire

- child_id: title-q151095
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1380..1806

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1380..1806 | 1380..1806 |

### fact-q2162698-parent-q12548 | Duchy of Bohemia -> Holy Roman Empire

- child_id: title-q2162698
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1002..1198

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1002..1198 | 1002..1198 |

### fact-q159856-parent-q12548 | Duchy of Brabant -> Holy Roman Empire

- child_id: title-q159856
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1183..1795
- relation_context: none

### fact-q694594-parent-q12548 | Duchy of Bremen and Verden -> Holy Roman Empire

- child_id: title-q694594
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1648..1806
- relation_context: none

### fact-q326029-parent-q150981 | Duchy of Brunswick -> North German Confederation

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q326029-parent-q151624 | Duchy of Brunswick -> German Confederation

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q326029-parent-q43287 | Duchy of Brunswick -> German Empire

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q2360973-parent-q12548 | Duchy of Carniola -> Holy Roman Empire

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1364..1803
- relation_context: none

### fact-q2360973-parent-q131964 | Duchy of Carniola -> Austrian Empire

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- relation_context: none

### fact-q2360973-parent-q28513 | Duchy of Carniola -> Austria-Hungary

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q641138-parent-q12548 | Duchy of Cleves -> Holy Roman Empire

- child_id: title-q641138
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1092..1795
- relation_context: none

### fact-q156038-parent-q172107 | Duchy of Courland and Semigallia -> Polish-Lithuanian Commonwealth

- child_id: title-q156038
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1795
- relation_context: none

### fact-q1991540-parent-q43287 | Duchy of Courland and Semigallia -> German Empire

- child_id: title-q1991540
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1918..1918
- relation_context: none

### fact-q693570-parent-q170174 | Duchy of Ferrara -> Papal States

- child_id: title-q693570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1597..1597
- relation_context: none

### fact-q2252973-parent-q12548 | Duchy of Florence -> Holy Roman Empire

- child_id: title-q2252973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1532..1569
- relation_context: none

### fact-q152420-parent-q12548 | Duchy of Guelders -> Holy Roman Empire

- child_id: title-q152420
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1096..1795
- relation_context: none

### fact-q704288-parent-q12548 | Duchy of Holstein -> Holy Roman Empire

- child_id: title-q704288
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1474..1806
- relation_context: none

### fact-q704288-parent-q151624 | Duchy of Holstein -> German Confederation

- child_id: title-q704288
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q836937-parent-q12548 | Duchy of Julich -> Holy Roman Empire

- child_id: title-q836937
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1003..1794
- relation_context: none

### fact-q1352878-parent-q172107 | Duchy of Livonia -> Polish-Lithuanian Commonwealth

- child_id: title-q1352878
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1621
- relation_context: none

### fact-q155019-parent-q12548 | Duchy of Lorraine -> Holy Roman Empire

- child_id: title-q155019
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1766
- relation_context: none

### fact-q2719360-parent-q12548 | Duchy of Luxembourg -> Holy Roman Empire

- child_id: title-q2719360
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1353..1795
- relation_context: none

### fact-q766501-parent-q12548 | Duchy of Mantua -> Holy Roman Empire

- child_id: title-q766501
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1530..1797
- relation_context: none

### fact-q933592-parent-q12548 | Duchy of Massa and Carrara -> Holy Roman Empire

- child_id: title-q933592
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1473..1806
- relation_context: none

### fact-q11024667-parent-q12548 | Duchy of Mecklenburg-Schwerin -> Holy Roman Empire

- child_id: title-q11024667
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1701..1806
- relation_context: none

### fact-q153529-parent-q12548 | Duchy of Milan -> Holy Roman Empire

- child_id: title-q153529
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1395..1797
- relation_context: none

### fact-q1615455-parent-q12548 | Duchy of Mirandola -> Holy Roman Empire

- child_id: title-q1615455
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1310..1710
- relation_context: none

### fact-q252580-parent-q12548 | Duchy of Modena and Reggio -> Holy Roman Empire

- child_id: title-q252580
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1452..1796
- relation_context: none

### fact-q836680-parent-q151624 | Duchy of Nassau -> German Confederation

- child_id: title-q836680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q165040-parent-q12548 | Duchy of Parma and Piacenza -> Holy Roman Empire

- child_id: title-q165040
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1748..1801
- relation_context: none

### fact-q696640-parent-q12548 | Duchy of Pomerania -> Holy Roman Empire

- child_id: title-q696640
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1121..1637
- relation_context: none

### fact-q661340-parent-q28513 | Duchy of Salzburg -> Austria-Hungary

- child_id: title-q661340
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q426025-parent-q12548 | Duchy of Savoy -> Holy Roman Empire

- child_id: title-q426025
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1416..1806
- relation_context: none

### fact-q157710-parent-q12548 | Duchy of Saxe-Meiningen -> Holy Roman Empire

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1675..1806
- relation_context: none

### fact-q157710-parent-q151624 | Duchy of Saxe-Meiningen -> German Confederation

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q157710-parent-q43287 | Duchy of Saxe-Meiningen -> German Empire

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q164092-parent-q12548 | Duchy of Saxony -> Holy Roman Empire

- child_id: title-q164092
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1296
- relation_context: none

### fact-q693980-parent-q12548 | Duchy of Swabia -> Holy Roman Empire

- child_id: title-q693980
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1313
- relation_context: none

### fact-q649202-parent-q170174 | Duchy of Urbino -> Papal States

- child_id: title-q649202
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1443..1631
- relation_context: none

### fact-q152115-parent-q71084 | Duchy of Warsaw -> First French Empire

- child_id: title-q152115
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1807..1815
- relation_context: none

### fact-q657241-parent-q12548 | Duchy of Westphalia -> Holy Roman Empire

- child_id: title-q657241
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1803
- relation_context: none

### fact-q2227570-parent-q12548 | Duchy of Wurttemberg -> Holy Roman Empire

- child_id: title-q2227570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1495..1803
- relation_context: none

### fact-q1252942-parent-q12544 | Duklja -> Byzantine Empire

- child_id: title-q1252942
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12544
- current_parent_rank: Empire
- span: 854..1252
- relation_context: none

### fact-q153080-parent-q31929 | East Francia -> Carolingian Empire

- child_id: title-q153080
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q31929
- current_parent_rank: Empire
- span: 843..887
- relation_context: none

### fact-q625380-parent-q12560 | Eastern Hungarian Kingdom -> Ottoman Empire

- child_id: title-q625380
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1529..1570
- relation_context: none

### fact-q22880-parent-q12548 | Electoral Palatinate -> Holy Roman Empire

- child_id: title-q22880
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1085..1803
- relation_context: none

### fact-q637238-parent-q12548 | Electorate of Baden -> Holy Roman Empire

- child_id: title-q637238
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1803..1806
- relation_context: none

### fact-q256961-parent-q12548 | Electorate of Bavaria -> Holy Roman Empire

- child_id: title-q256961
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1623..1805
- relation_context: none

### fact-q7904317-parent-q12548 | Electorate of Cologne -> Holy Roman Empire

- child_id: title-q7904317
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- relation_context: none

### fact-q706018-parent-q12548 | Electorate of Hanover -> Holy Roman Empire

- child_id: title-q706018
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1692..1806
- relation_context: none

### fact-q529605-parent-q151624 | Electorate of Hesse -> German Confederation

- child_id: title-q529605
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q284667-parent-q12548 | Electorate of Mainz -> Holy Roman Empire

- child_id: title-q284667
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- relation_context: none

### fact-q156199-parent-q12548 | Electorate of Saxony -> Holy Roman Empire

- child_id: title-q156199
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1356..1806
- relation_context: none

### fact-q2172530-parent-q12548 | Electorate of Wurttemberg -> Holy Roman Empire

- child_id: title-q2172530
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1803..1806
- relation_context: none

### fact-q146246-parent-q31929 | Francia -> Carolingian Empire

- child_id: title-q146246
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q31929
- current_parent_rank: Empire
- span: 800..843
- relation_context: none

### fact-q692946-parent-q131964 | Gorizia and Gradisca -> Austrian Empire

- child_id: title-q692946
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- relation_context: none

### fact-q692946-parent-q28513 | Gorizia and Gradisca -> Austria-Hungary

- child_id: title-q692946
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q186320-parent-q151624 | Grand Duchy of Baden -> German Confederation

- child_id: title-q186320
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q186320-parent-q43287 | Grand Duchy of Baden -> German Empire

- child_id: title-q186320
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q249428-parent-q154741 | Grand Duchy of Berg -> Confederation of the Rhine

- child_id: title-q249428
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1806..1813
- relation_context: none

### fact-q62633-parent-q34266 | Grand Duchy of Finland -> Russian Empire

- child_id: title-q62633
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q34266
- current_parent_rank: Empire
- span: 1809..1917
- relation_context: none

### fact-q704312-parent-q154741 | Grand Duchy of Frankfurt -> Confederation of the Rhine

- child_id: title-q704312
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1810..1813
- relation_context: none

### fact-q20135-parent-q151624 | Grand Duchy of Hesse -> German Confederation

- child_id: title-q20135
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q20135-parent-q43287 | Grand Duchy of Hesse -> German Empire

- child_id: title-q20135
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q49683-parent-q172107 | Grand Duchy of Lithuania -> Polish-Lithuanian Commonwealth

- child_id: title-q49683
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1795
- relation_context: none

### fact-q158445-parent-q150981 | Grand Duchy of Mecklenburg-Schwerin -> North German Confederation

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q158445-parent-q151624 | Grand Duchy of Mecklenburg-Schwerin -> German Confederation

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q158445-parent-q43287 | Grand Duchy of Mecklenburg-Schwerin -> German Empire

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q161215-parent-q150981 | Grand Duchy of Mecklenburg-Strelitz -> North German Confederation

- child_id: title-q161215
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q161215-parent-q151624 | Grand Duchy of Mecklenburg-Strelitz -> German Confederation

- child_id: title-q161215
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q693669-parent-q150981 | Grand Duchy of Oldenburg -> North German Confederation

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q693669-parent-q151624 | Grand Duchy of Oldenburg -> German Confederation

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1816..1866
- relation_context: none

### fact-q693669-parent-q43287 | Grand Duchy of Oldenburg -> German Empire

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q154849-parent-q12548 | Grand Duchy of Tuscany -> Holy Roman Empire

- child_id: title-q154849
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1575..1801
- relation_context: none

### fact-q698089-parent-q154741 | Grand Duchy of Wurzburg -> Confederation of the Rhine

- child_id: title-q698089
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1806..1813
- relation_context: none

### fact-q170770-parent-q141472 | Grand Principality of Moscow -> Golden Horde

- child_id: title-q170770
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q141472
- current_parent_rank: Empire
- span: 1263..1478
- relation_context: none

### fact-q83546-parent-q141472 | Grand Principality of Vladimir -> Golden Horde

- child_id: title-q83546
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q141472
- current_parent_rank: Empire
- span: 1259..1389
- relation_context: none

### fact-q673865-parent-q12548 | Hohenzollern-Hechingen -> Holy Roman Empire

- child_id: title-q673865
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1576..1806
- relation_context: none

### fact-q673865-parent-q151624 | Hohenzollern-Hechingen -> German Confederation

- child_id: title-q673865
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1850
- relation_context: none

### fact-q157013-parent-q12548 | Hohenzollern-Sigmaringen -> Holy Roman Empire

- child_id: title-q157013
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1576..1806
- relation_context: none

### fact-q157013-parent-q151624 | Hohenzollern-Sigmaringen -> German Confederation

- child_id: title-q157013
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1850
- relation_context: none

### fact-q50625-parent-q20 | Hordaland -> Kingdom of Norway

- child_id: title-q50625
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q20
- current_parent_rank: Kingdom
- span: 1919..2019
- relation_context: none

### fact-q699923-parent-q71084 | Illyrian Provinces -> First French Empire

- child_id: title-q699923
- child_rank: Province
- expected_parent_rank: Kingdom
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1809..1815
- relation_context: none

### fact-q1448131-parent-q926295 | Italian protectorate of Albania -> Italian Empire

- child_id: title-q1448131
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q926295
- current_parent_rank: Empire
- span: 1939..1943
- relation_context: none

### fact-q154195-parent-q151624 | Kingdom of Bavaria -> German Confederation

- child_id: title-q154195
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q154195-parent-q43287 | Kingdom of Bavaria -> German Empire

- child_id: title-q154195
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q42585-parent-q12548 | Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q42585
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1198..1803
- relation_context: none

### fact-q42585-parent-q131964 | Kingdom of Bohemia -> Austrian Empire

- child_id: title-q42585
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- relation_context: none

### fact-q42585-parent-q28513 | Kingdom of Bohemia -> Austria-Hungary

- child_id: title-q42585
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q2980623-parent-q12560 | Kingdom of Bosnia -> Ottoman Empire

- child_id: title-q2980623
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1463..1463
- relation_context: none

### fact-q533558-parent-q28513 | Kingdom of Croatia-Slavonia -> Austria-Hungary

- child_id: title-q533558
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1868..1918
- relation_context: none

### fact-q223793-parent-q71084 | Kingdom of Etruria -> First French Empire

- child_id: title-q223793
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1804..1807
- relation_context: none

### fact-q2396442-parent-q131964 | Kingdom of Galicia and Lodomeria -> Austrian Empire

- child_id: title-q2396442
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- relation_context: none

### fact-q2396442-parent-q28513 | Kingdom of Galicia and Lodomeria -> Austria-Hungary

- child_id: title-q2396442
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q164079-parent-q151624 | Kingdom of Hanover -> German Confederation

- child_id: title-q164079
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q212278-parent-q71084 | Kingdom of Holland -> First French Empire

- child_id: title-q212278
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1806..1810
- relation_context: none

### fact-q171150-parent-q131964 | Kingdom of Hungary -> Austrian Empire

- child_id: title-q171150
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1867
- relation_context: none

### fact-q253094-parent-q131964 | Kingdom of Hungary -> Austrian Empire

- child_id: title-q253094
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1867
- relation_context: none

### fact-q25395037-parent-q28513 | Kingdom of Hungary -> Austria-Hungary

- child_id: title-q25395037
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q1117051-parent-q131964 | Kingdom of Illyria -> Austrian Empire

- child_id: title-q1117051
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1816..1849
- relation_context: none

### fact-q1069959-parent-q34266 | Kingdom of Imereti -> Russian Empire

- child_id: title-q1069959
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q34266
- current_parent_rank: Empire
- span: 1804..1810
- relation_context: none

### fact-q223936-parent-q71084 | Kingdom of Italy -> First French Empire

- child_id: title-q223936
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1805..1814
- relation_context: none

### fact-q838931-parent-q12548 | Kingdom of Italy -> Holy Roman Empire

- child_id: title-q838931
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1806
- relation_context: none

### fact-q2346056-parent-q186096 | Kingdom of Livonia -> Tsardom of Russia

- child_id: title-q2346056
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q186096
- current_parent_rank: Empire
- span: 1570..1578
- relation_context: none

### fact-q209857-parent-q131964 | Kingdom of Lombardy-Venetia -> Austrian Empire

- child_id: title-q209857
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q696908-parent-q43287 | Kingdom of Poland -> German Empire

- child_id: title-q696908
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1916..1918
- relation_context: none

### fact-q45670-parent-q377350 | Kingdom of Portugal -> Iberian Union

- child_id: title-q45670
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q377350
- current_parent_rank: Empire
- span: 1580..1640
- relation_context: none

### fact-q27306-parent-q150981 | Kingdom of Prussia -> North German Confederation

- child_id: title-q27306
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q27306-parent-q151624 | Kingdom of Prussia -> German Confederation

- child_id: title-q27306
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q27306-parent-q43287 | Kingdom of Prussia -> German Empire

- child_id: title-q27306
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q153015-parent-q150981 | Kingdom of Saxony -> North German Confederation

- child_id: title-q153015
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q153015-parent-q151624 | Kingdom of Saxony -> German Confederation

- child_id: title-q153015
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q153015-parent-q43287 | Kingdom of Saxony -> German Empire

- child_id: title-q153015
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q2415003-parent-q1406298 | Kingdom of Serbia -> Serbian Empire

- child_id: title-q2415003
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q1406298
- current_parent_rank: Empire
- span: 1346..1346
- relation_context: none

### fact-q325461-parent-q178897 | Kingdom of Thessalonica -> Latin Empire

- child_id: title-q325461
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q178897
- current_parent_rank: Empire
- span: 1204..1224
- relation_context: none

### fact-q159631-parent-q151624 | Kingdom of Wurttemberg -> German Confederation

- child_id: title-q159631
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q159631-parent-q43287 | Kingdom of Wurttemberg -> German Empire

- child_id: title-q159631
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- relation_context: none

### fact-q13590051-parent-q42834 | Kingdom of the Burgundians -> Western Roman Empire

- child_id: title-q13590051
- child_rank: Kingdom
- expected_parent_rank: Crown
- current_parent_id: title-q42834
- current_parent_rank: Empire
- span: 443..476
- relation_context: none

### fact-q751868-parent-q12548 | Landgraviate of Brabant -> Holy Roman Empire

- child_id: title-q751868
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1085..1183
- relation_context: none

### fact-q695322-parent-q12548 | Landgraviate of Hesse -> Holy Roman Empire

- child_id: title-q695322
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1264..1567
- relation_context: none

### fact-q693551-parent-q12548 | Landgraviate of Hesse-Darmstadt -> Holy Roman Empire

- child_id: title-q693551
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1567..1806
- relation_context: none

### fact-q168651-parent-q12548 | Landgraviate of Hesse-Kassel -> Holy Roman Empire

- child_id: title-q168651
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1567..1803
- relation_context: none

### fact-q673837-parent-q12548 | Landgraviate of Hesse-Marburg -> Holy Roman Empire

- child_id: title-q673837
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1458..1604
- relation_context: none

### fact-q58942549-parent-q12548 | Landgraviate of Lower Alsace -> Holy Roman Empire

- child_id: title-q58942549
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1200..1700
- relation_context: none

### fact-q660393-parent-q12548 | Lower Lotharingia -> Holy Roman Empire

- child_id: title-q660393
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1190
- relation_context: none

### fact-q283627-parent-q12548 | Margraviate of Austria -> Holy Roman Empire

- child_id: title-q283627
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 976..1156
- relation_context: none

### fact-q148499-parent-q12548 | Margraviate of Brandenburg -> Holy Roman Empire

- child_id: title-q148499
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1157..1806
- relation_context: none

### fact-q170180-parent-q12548 | Margraviate of Meissen -> Holy Roman Empire

- child_id: title-q170180
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 965..1423
- relation_context: none

### fact-q2670751-parent-q28513 | Margraviate of Moravia -> Austria-Hungary

- child_id: title-q2670751
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- relation_context: none

### fact-q552822-parent-q12548 | Mecklenburg-Gustrow -> Holy Roman Empire

- child_id: title-q552822
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1520..1695
- relation_context: none

### fact-q2273304-parent-q12560 | Moravian Serbia -> Ottoman Empire

- child_id: title-q2273304
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1390..1402
- relation_context: none

### fact-q736029-parent-q12548-1303 | Nassau-Siegen -> Holy Roman Empire

- child_id: title-q736029
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1303..1328
- relation_context: none

### fact-q736029-parent-q12548-1606 | Nassau-Siegen -> Holy Roman Empire

- child_id: title-q736029
- child_rank: County
- expected_parent_rank: Duchy
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1606..1743
- relation_context: none

### fact-q454436-parent-q12548 | Palatinate-Sulzbach -> Holy Roman Empire

- child_id: title-q454436
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1557..1806
- relation_context: none

### fact-q825902-parent-q172107 | Polish-Lithuanian union -> Polish-Lithuanian Commonwealth

- child_id: title-q825902
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1569
- relation_context: none

### fact-q701614-parent-q12548 | Prince-Archbishopric of Salzburg -> Holy Roman Empire

- child_id: title-q701614
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1278..1803
- relation_context: none

### fact-q173863-parent-q12548 | Prince-Bishopric of Augsburg -> Holy Roman Empire

- child_id: title-q173863
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- relation_context: none

### fact-q319586-parent-q12548 | Prince-Bishopric of Basel -> Holy Roman Empire

- child_id: title-q319586
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1032..1803
- relation_context: none

### fact-q259511-parent-q12548 | Prince-Bishopric of Freising -> Holy Roman Empire

- child_id: title-q259511
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1294..1802
- relation_context: none

### fact-q158835-parent-q12548 | Prince-Bishopric of Liege -> Holy Roman Empire

- child_id: title-q158835
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 985..1795
- relation_context: none

### fact-q650645-parent-q12548 | Prince-Bishopric of Minden -> Holy Roman Empire

- child_id: title-q650645
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1648
- relation_context: none

### fact-q3324486-parent-q12560 | Prince-Bishopric of Montenegro -> Ottoman Empire

- child_id: title-q3324486
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1516..1696
- relation_context: none

### fact-q697254-parent-q12548 | Prince-Bishopric of Munster -> Holy Roman Empire

- child_id: title-q697254
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1802
- relation_context: none

### fact-q477035-parent-q12548 | Prince-Bishopric of Osnabruck -> Holy Roman Empire

- child_id: title-q477035
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1225..1803
- relation_context: none

### fact-q649192-parent-q12548 | Prince-Bishopric of Paderborn -> Holy Roman Empire

- child_id: title-q649192
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1281..1802
- relation_context: none

### fact-q771332-parent-q12548 | Prince-Bishopric of Strasbourg -> Holy Roman Empire

- child_id: title-q771332
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 982..1803
- relation_context: none

### fact-q328001-parent-q12548 | Prince-Bishopric of Toul -> Holy Roman Empire

- child_id: title-q328001
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1048..1648
- relation_context: none

### fact-q1231403-parent-q12548 | Prince-Bishopric of Trent -> Holy Roman Empire

- child_id: title-q1231403
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1027..1803

| Relation Kind | Related Title | Relation Span | Overlap |
|---|---|---|---|
| imperial_state | Holy Roman Empire (title-q12548) | 1027..1803 | 1027..1803 |

### fact-q707767-parent-q12548 | Prince-Bishopric of Utrecht -> Holy Roman Empire

- child_id: title-q707767
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1024..1528
- relation_context: none

### fact-q17015016-parent-q12548 | Prince-Bishopric of Verdun -> Holy Roman Empire

- child_id: title-q17015016
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 997..1552
- relation_context: none

### fact-q14551680-parent-q12548 | Principality of Lippe -> Holy Roman Empire

- child_id: title-q14551680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1789..1806
- relation_context: none

### fact-q14551680-parent-q151624 | Principality of Lippe -> German Confederation

- child_id: title-q14551680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q684030-parent-q12560 | Principality of Serbia -> Ottoman Empire

- child_id: title-q684030
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1815..1867
- relation_context: none

### fact-q158151-parent-q12548 | Saxe-Altenburg -> Holy Roman Empire

- child_id: title-q158151
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1602..1806
- relation_context: none

### fact-q158151-parent-q151624 | Saxe-Altenburg -> German Confederation

- child_id: title-q158151
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q3462133-parent-q151624 | Saxe-Coburg and Gotha -> German Confederation

- child_id: title-q3462133
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1826..1866
- relation_context: none

### fact-q700663-parent-q12548 | Saxe-Coburg-Saalfeld -> Holy Roman Empire

- child_id: title-q700663
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1699..1806
- relation_context: none

### fact-q700663-parent-q151624 | Saxe-Coburg-Saalfeld -> German Confederation

- child_id: title-q700663
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1825
- relation_context: none

### fact-q675085-parent-q12548 | Saxe-Gotha-Altenburg -> Holy Roman Empire

- child_id: title-q675085
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1680..1806
- relation_context: none

### fact-q675085-parent-q151624 | Saxe-Gotha-Altenburg -> German Confederation

- child_id: title-q675085
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1826
- relation_context: none

### fact-q281005-parent-q12548 | Saxe-Hildburghausen -> Holy Roman Empire

- child_id: title-q281005
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1680..1806
- relation_context: none

### fact-q281005-parent-q151624 | Saxe-Hildburghausen -> German Confederation

- child_id: title-q281005
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1826
- relation_context: none

### fact-q313175-parent-q12548 | Saxe-Lauenburg -> Holy Roman Empire

- child_id: title-q313175
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1296..1806
- relation_context: none

### fact-q313175-parent-q151624 | Saxe-Lauenburg -> German Confederation

- child_id: title-q313175
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q155570-parent-q150981 | Saxe-Weimar-Eisenach -> North German Confederation

- child_id: title-q155570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- relation_context: none

### fact-q155570-parent-q151624 | Saxe-Weimar-Eisenach -> German Confederation

- child_id: title-q155570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q310650-parent-q12548 | Schaumburg-Lippe -> Holy Roman Empire

- child_id: title-q310650
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1643..1806
- relation_context: none

### fact-q310650-parent-q151624 | Schaumburg-Lippe -> German Confederation

- child_id: title-q310650
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q695316-parent-q151624 | Schwarzburg-Rudolstadt -> German Confederation

- child_id: title-q695316
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q630163-parent-q12548 | Schwarzburg-Sondershausen -> Holy Roman Empire

- child_id: title-q630163
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1599..1806
- relation_context: none

### fact-q630163-parent-q151624 | Schwarzburg-Sondershausen -> German Confederation

- child_id: title-q630163
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- relation_context: none

### fact-q958291-parent-q12560 | United Principalities of Moldavia and Wallachia -> Ottoman Empire

- child_id: title-q958291
- child_rank: Duchy
- expected_parent_rank: Kingdom
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1859..1877
- relation_context: none

