# DUCHY Parentage Rank Skip Bridge Review Report

source_tsv: data/staging/parentage-rank-skip-bridges.tsv
bridge_rows: 163

## Priority Counts

| Priority | Rows |
|---|---:|
| high_intermediate_parent_review | 151 |
| low_intermediate_parent_review | 3 |
| medium_intermediate_parent_review | 9 |

## Candidate Parent Counts

| Candidate Parent | Rows |
|---|---:|
| Kingdom of Bohemia | 55 |
| Kingdom of Bavaria | 35 |
| Kingdom of Italy | 31 |
| Duchy of Bavaria | 12 |
| Anhalt-Bernburg | 6 |
| Kingdom of Prussia | 6 |
| Crown of the Kingdom of Poland | 4 |
| Confederation of the Rhine | 3 |
| Kingdom of Westphalia | 3 |
| Archduchy of Austria | 2 |
| Duchy of Brittany | 2 |
| Duchy of Ferrara | 1 |
| Eastern Hungarian Kingdom | 1 |
| Kingdom of Imereti | 1 |
| Margraviate of Brandenburg | 1 |

## Current Parent Counts

| Current Parent | Rows |
|---|---:|
| Holy Roman Empire | 97 |
| German Confederation | 27 |
| German Empire | 8 |
| North German Confederation | 6 |
| Austria-Hungary | 5 |
| First French Empire | 5 |
| Polish-Lithuanian Commonwealth | 4 |
| Austrian Empire | 3 |
| Confederation of the Rhine | 3 |
| Kingdom of France | 2 |
| Ottoman Empire | 1 |
| Papal States | 1 |
| Russian Empire | 1 |

## Review Rows

### fact-q686965-parent-q12548 | Anhalt-Bernburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q686965
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1252..1806
- overlap_years: 555
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q686965-parent-q151624 | Anhalt-Bernburg -> Kingdom of Bavaria -> German Confederation

- child_id: title-q686965
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1863
- overlap_years: 49
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q278874-parent-q12548 | Anhalt-Dessau -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q278874
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1396..1806
- overlap_years: 411
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q278874-parent-q151624 | Anhalt-Dessau -> Kingdom of Bavaria -> German Confederation

- child_id: title-q278874
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1853
- overlap_years: 39
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q264970-parent-q12548 | Anhalt-Kothen -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q264970
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1396..1806
- overlap_years: 411
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q264970-parent-q151624 | Anhalt-Kothen -> Kingdom of Bavaria -> German Confederation

- child_id: title-q264970
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1863
- overlap_years: 49
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q699964-parent-q12548 | Archduchy of Austria -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1358..1803
- overlap_years: 446
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q699964-parent-q131964 | Archduchy of Austria -> Kingdom of Bohemia -> Austrian Empire

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- overlap_years: 63
- bridge_fact_id: fact-q42585-parent-q131964
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q699964-parent-q28513 | Archduchy of Austria -> Kingdom of Bohemia -> Austria-Hungary

- child_id: title-q699964
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- overlap_years: 52
- bridge_fact_id: fact-q42585-parent-q28513
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q552033-parent-q12548 | Bavaria-Munich -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q552033
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1392..1505
- overlap_years: 114
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q556263-parent-q12548 | Brunswick-Luneburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q556263
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1235..1806
- overlap_years: 572
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q830084-parent-q12548 | Brunswick-Wolfenbuttel -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q830084
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1269..1806
- overlap_years: 538
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q568473-parent-q12548 | Burgraviate of Nuremberg -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q568473
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1105..1440
- overlap_years: 336
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q1122980-parent-q170174 | Comtat Venaissin -> Duchy of Ferrara -> Papal States

- child_id: title-q1122980
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q693570
- candidate_exists: 1264..1597
- current_parent_id: title-q170174
- current_parent_rank: TheocraticState
- span: 1274..1791
- overlap_years: 324
- bridge_fact_id: fact-q693570-parent-q170174
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q642314-parent-q12548 | County of Burgundy -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q642314
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 982..1678
- overlap_years: 697
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157070-parent-q12548 | County of Flanders -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q157070
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1383
- overlap_years: 422
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157070-parent-q12548 | County of Flanders -> Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q157070
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q686965
- candidate_exists: 1252..1863
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1483..1797
- overlap_years: 315
- bridge_fact_id: fact-q686965-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q675363-parent-q12548 | County of Geneva -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q675363
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1032..1401
- overlap_years: 370
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q762943-parent-q12548 | County of Holland -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q762943
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1432
- overlap_years: 471
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q762943-parent-q12548 | County of Holland -> Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q762943
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q686965
- candidate_exists: 1252..1863
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1483..1795
- overlap_years: 313
- bridge_fact_id: fact-q686965-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q921473-parent-q70972 | County of La Marche -> Duchy of Brittany -> Kingdom of France

- child_id: title-q921473
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q71747
- candidate_exists: 939..1547
- current_parent_id: title-q70972
- current_parent_rank: Kingdom
- span: 1527..1527
- overlap_years: 1
- bridge_fact_id: fact-q71747-parent-q70972
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q5177890-parent-q12548 | County of Luxembourg -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q5177890
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 963..1354
- overlap_years: 392
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q589251-parent-q12548 | County of Montbeliard -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q589251
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1042..1793
- overlap_years: 752
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q599613-parent-q12548 | County of Namur -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q599613
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 981..1420
- overlap_years: 440
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q599613-parent-q12548 | County of Namur -> Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q599613
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q686965
- candidate_exists: 1252..1863
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1483..1795
- overlap_years: 313
- bridge_fact_id: fact-q686965-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q12817455-parent-q12548 | County of Nassau -> Margraviate of Brandenburg -> Holy Roman Empire

- child_id: title-q12817455
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q148499
- candidate_exists: 1157..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1160..1806
- overlap_years: 647
- bridge_fact_id: fact-q148499-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2991382-parent-q70972 | County of Provence -> Duchy of Brittany -> Kingdom of France

- child_id: title-q2991382
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q71747
- candidate_exists: 939..1547
- current_parent_id: title-q70972
- current_parent_rank: Kingdom
- span: 987..1487
- overlap_years: 501
- bridge_fact_id: fact-q71747-parent-q70972
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q573290-parent-q12548 | County of Ravensberg -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q573290
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1140..1806
- overlap_years: 666
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q1232887-parent-q12548 | County of Savoy -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q1232887
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1003..1416
- overlap_years: 414
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2991474-parent-q12548 | County of Wurttemberg -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q2991474
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1083..1495
- overlap_years: 413
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q16550783-parent-q150981 | Duchy of Anhalt -> Kingdom of Prussia -> North German Confederation

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q27306
- candidate_exists: 1701..1918
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- overlap_years: 4
- bridge_fact_id: fact-q27306-parent-q150981
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q16550783-parent-q151624 | Duchy of Anhalt -> Kingdom of Bavaria -> German Confederation

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1863..1866
- overlap_years: 4
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q16550783-parent-q43287 | Duchy of Anhalt -> Kingdom of Bavaria -> German Empire

- child_id: title-q16550783
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q3624335-parent-q12548 | Duchy of Austria -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q3624335
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1156..1453
- overlap_years: 298
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q47261-parent-q12548 | Duchy of Bavaria -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q47261
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1805
- overlap_years: 844
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q151095-parent-q12548 | Duchy of Berg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q151095
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1380..1806
- overlap_years: 427
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2162698-parent-q12548 | Duchy of Bohemia -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q2162698
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1002..1198
- overlap_years: 197
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q159856-parent-q12548 | Duchy of Brabant -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q159856
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1183..1795
- overlap_years: 613
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q694594-parent-q12548 | Duchy of Bremen and Verden -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q694594
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1648..1806
- overlap_years: 159
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q326029-parent-q150981 | Duchy of Brunswick -> Kingdom of Prussia -> North German Confederation

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q27306
- candidate_exists: 1701..1918
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- overlap_years: 4
- bridge_fact_id: fact-q27306-parent-q150981
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q326029-parent-q151624 | Duchy of Brunswick -> Kingdom of Bavaria -> German Confederation

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q326029-parent-q43287 | Duchy of Brunswick -> Kingdom of Bavaria -> German Empire

- child_id: title-q326029
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2360973-parent-q12548 | Duchy of Carniola -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1364..1803
- overlap_years: 440
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2360973-parent-q131964 | Duchy of Carniola -> Kingdom of Bohemia -> Austrian Empire

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- overlap_years: 63
- bridge_fact_id: fact-q42585-parent-q131964
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2360973-parent-q28513 | Duchy of Carniola -> Kingdom of Bohemia -> Austria-Hungary

- child_id: title-q2360973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- overlap_years: 52
- bridge_fact_id: fact-q42585-parent-q28513
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q641138-parent-q12548 | Duchy of Cleves -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q641138
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1092..1795
- overlap_years: 704
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q156038-parent-q172107 | Duchy of Courland and Semigallia -> Crown of the Kingdom of Poland -> Polish-Lithuanian Commonwealth

- child_id: title-q156038
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q171348
- candidate_exists: 1386..1795
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1795
- overlap_years: 227
- bridge_fact_id: fact-q171348-parent-q172107
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q1991540-parent-q43287 | Duchy of Courland and Semigallia -> Kingdom of Bavaria -> German Empire

- child_id: title-q1991540
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1918..1918
- overlap_years: 1
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2252973-parent-q12548 | Duchy of Florence -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q2252973
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1532..1569
- overlap_years: 38
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q152420-parent-q12548 | Duchy of Guelders -> Duchy of Bavaria -> Holy Roman Empire

- child_id: title-q152420
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q47261
- candidate_exists: 907..1805
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1096..1795
- overlap_years: 700
- bridge_fact_id: fact-q47261-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q704288-parent-q12548 | Duchy of Holstein -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q704288
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1474..1806
- overlap_years: 333
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q704288-parent-q151624 | Duchy of Holstein -> Kingdom of Bavaria -> German Confederation

- child_id: title-q704288
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q836937-parent-q12548 | Duchy of Julich -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q836937
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1003..1794
- overlap_years: 792
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q1352878-parent-q172107 | Duchy of Livonia -> Crown of the Kingdom of Poland -> Polish-Lithuanian Commonwealth

- child_id: title-q1352878
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q171348
- candidate_exists: 1386..1795
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1621
- overlap_years: 53
- bridge_fact_id: fact-q171348-parent-q172107
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q155019-parent-q12548 | Duchy of Lorraine -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q155019
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1766
- overlap_years: 805
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2719360-parent-q12548 | Duchy of Luxembourg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q2719360
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1353..1795
- overlap_years: 443
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q766501-parent-q12548 | Duchy of Mantua -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q766501
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1530..1797
- overlap_years: 268
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q933592-parent-q12548 | Duchy of Massa and Carrara -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q933592
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1473..1806
- overlap_years: 334
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q11024667-parent-q12548 | Duchy of Mecklenburg-Schwerin -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q11024667
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1701..1806
- overlap_years: 106
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q153529-parent-q12548 | Duchy of Milan -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q153529
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1395..1797
- overlap_years: 403
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q1615455-parent-q12548 | Duchy of Mirandola -> Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q1615455
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q686965
- candidate_exists: 1252..1863
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1310..1710
- overlap_years: 401
- bridge_fact_id: fact-q686965-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q252580-parent-q12548 | Duchy of Modena and Reggio -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q252580
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1452..1796
- overlap_years: 345
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q836680-parent-q151624 | Duchy of Nassau -> Kingdom of Bavaria -> German Confederation

- child_id: title-q836680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q165040-parent-q12548 | Duchy of Parma and Piacenza -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q165040
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1748..1801
- overlap_years: 54
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q696640-parent-q12548 | Duchy of Pomerania -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q696640
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1121..1637
- overlap_years: 517
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q661340-parent-q28513 | Duchy of Salzburg -> Kingdom of Bohemia -> Austria-Hungary

- child_id: title-q661340
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- overlap_years: 52
- bridge_fact_id: fact-q42585-parent-q28513
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q426025-parent-q12548 | Duchy of Savoy -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q426025
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1416..1806
- overlap_years: 391
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157710-parent-q12548 | Duchy of Saxe-Meiningen -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1675..1806
- overlap_years: 132
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157710-parent-q151624 | Duchy of Saxe-Meiningen -> Kingdom of Bavaria -> German Confederation

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157710-parent-q43287 | Duchy of Saxe-Meiningen -> Kingdom of Bavaria -> German Empire

- child_id: title-q157710
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q164092-parent-q12548 | Duchy of Saxony -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q164092
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1296
- overlap_years: 335
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q693980-parent-q12548 | Duchy of Swabia -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q693980
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1313
- overlap_years: 352
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q152115-parent-q71084 | Duchy of Warsaw -> Kingdom of Italy -> First French Empire

- child_id: title-q152115
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q223936
- candidate_exists: 1805..1814
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1807..1815
- overlap_years: 8
- bridge_fact_id: fact-q223936-parent-q71084
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q657241-parent-q12548 | Duchy of Westphalia -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q657241
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1803
- overlap_years: 624
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2227570-parent-q12548 | Duchy of Wurttemberg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q2227570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1495..1803
- overlap_years: 309
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q22880-parent-q12548 | Electoral Palatinate -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q22880
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1085..1803
- overlap_years: 719
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q637238-parent-q12548 | Electorate of Baden -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q637238
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1803..1806
- overlap_years: 4
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q256961-parent-q12548 | Electorate of Bavaria -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q256961
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1623..1805
- overlap_years: 183
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q7904317-parent-q12548 | Electorate of Cologne -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q7904317
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- overlap_years: 842
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q706018-parent-q12548 | Electorate of Hanover -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q706018
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1692..1806
- overlap_years: 115
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q529605-parent-q151624 | Electorate of Hesse -> Kingdom of Bavaria -> German Confederation

- child_id: title-q529605
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q284667-parent-q12548 | Electorate of Mainz -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q284667
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- overlap_years: 842
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q156199-parent-q12548 | Electorate of Saxony -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q156199
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1356..1806
- overlap_years: 451
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2172530-parent-q12548 | Electorate of Wurttemberg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q2172530
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1803..1806
- overlap_years: 4
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q692946-parent-q131964 | Gorizia and Gradisca -> Archduchy of Austria -> Austrian Empire

- child_id: title-q692946
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q699964
- candidate_exists: 1358..1918
- current_parent_id: title-q131964
- current_parent_rank: Empire
- span: 1804..1866
- overlap_years: 63
- bridge_fact_id: fact-q699964-parent-q131964
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q692946-parent-q28513 | Gorizia and Gradisca -> Archduchy of Austria -> Austria-Hungary

- child_id: title-q692946
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q699964
- candidate_exists: 1358..1918
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- overlap_years: 52
- bridge_fact_id: fact-q699964-parent-q28513
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q186320-parent-q151624 | Grand Duchy of Baden -> Kingdom of Bavaria -> German Confederation

- child_id: title-q186320
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q186320-parent-q43287 | Grand Duchy of Baden -> Kingdom of Bavaria -> German Empire

- child_id: title-q186320
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q249428-parent-q154741 | Grand Duchy of Berg -> Kingdom of Westphalia -> Confederation of the Rhine

- child_id: title-q249428
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q153943
- candidate_exists: 1807..1813
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1806..1813
- overlap_years: 7
- bridge_fact_id: fact-q153943-parent-q154741
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q62633-parent-q34266 | Grand Duchy of Finland -> Kingdom of Imereti -> Russian Empire

- child_id: title-q62633
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q1069959
- candidate_exists: 1260..1810
- current_parent_id: title-q34266
- current_parent_rank: Empire
- span: 1809..1917
- overlap_years: 2
- bridge_fact_id: fact-q1069959-parent-q34266
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q704312-parent-q154741 | Grand Duchy of Frankfurt -> Kingdom of Westphalia -> Confederation of the Rhine

- child_id: title-q704312
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q153943
- candidate_exists: 1807..1813
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1810..1813
- overlap_years: 4
- bridge_fact_id: fact-q153943-parent-q154741
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q20135-parent-q151624 | Grand Duchy of Hesse -> Kingdom of Bavaria -> German Confederation

- child_id: title-q20135
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q20135-parent-q43287 | Grand Duchy of Hesse -> Kingdom of Bavaria -> German Empire

- child_id: title-q20135
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q49683-parent-q172107 | Grand Duchy of Lithuania -> Crown of the Kingdom of Poland -> Polish-Lithuanian Commonwealth

- child_id: title-q49683
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q171348
- candidate_exists: 1386..1795
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1795
- overlap_years: 227
- bridge_fact_id: fact-q171348-parent-q172107
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q158445-parent-q150981 | Grand Duchy of Mecklenburg-Schwerin -> Kingdom of Prussia -> North German Confederation

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q27306
- candidate_exists: 1701..1918
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- overlap_years: 4
- bridge_fact_id: fact-q27306-parent-q150981
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q158445-parent-q151624 | Grand Duchy of Mecklenburg-Schwerin -> Kingdom of Bavaria -> German Confederation

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q158445-parent-q43287 | Grand Duchy of Mecklenburg-Schwerin -> Kingdom of Bavaria -> German Empire

- child_id: title-q158445
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q161215-parent-q150981 | Grand Duchy of Mecklenburg-Strelitz -> Kingdom of Prussia -> North German Confederation

- child_id: title-q161215
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q27306
- candidate_exists: 1701..1918
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- overlap_years: 4
- bridge_fact_id: fact-q27306-parent-q150981
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q161215-parent-q151624 | Grand Duchy of Mecklenburg-Strelitz -> Kingdom of Bavaria -> German Confederation

- child_id: title-q161215
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q693669-parent-q150981 | Grand Duchy of Oldenburg -> Kingdom of Prussia -> North German Confederation

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q27306
- candidate_exists: 1701..1918
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- overlap_years: 4
- bridge_fact_id: fact-q27306-parent-q150981
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q693669-parent-q151624 | Grand Duchy of Oldenburg -> Kingdom of Bavaria -> German Confederation

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1816..1866
- overlap_years: 51
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q693669-parent-q43287 | Grand Duchy of Oldenburg -> Kingdom of Bavaria -> German Empire

- child_id: title-q693669
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q43287
- current_parent_rank: Empire
- span: 1871..1918
- overlap_years: 48
- bridge_fact_id: fact-q154195-parent-q43287
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q154849-parent-q12548 | Grand Duchy of Tuscany -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q154849
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1575..1801
- overlap_years: 227
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q698089-parent-q154741 | Grand Duchy of Wurzburg -> Kingdom of Westphalia -> Confederation of the Rhine

- child_id: title-q698089
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q153943
- candidate_exists: 1807..1813
- current_parent_id: title-q154741
- current_parent_rank: Crown
- span: 1806..1813
- overlap_years: 7
- bridge_fact_id: fact-q153943-parent-q154741
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q673865-parent-q12548 | Hohenzollern-Hechingen -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q673865
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1576..1806
- overlap_years: 231
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q673865-parent-q151624 | Hohenzollern-Hechingen -> Kingdom of Bavaria -> German Confederation

- child_id: title-q673865
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1850
- overlap_years: 36
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157013-parent-q12548 | Hohenzollern-Sigmaringen -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q157013
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1576..1806
- overlap_years: 231
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q157013-parent-q151624 | Hohenzollern-Sigmaringen -> Kingdom of Bavaria -> German Confederation

- child_id: title-q157013
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1850
- overlap_years: 36
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q699923-parent-q71084 | Illyrian Provinces -> Kingdom of Italy -> First French Empire

- child_id: title-q699923
- child_rank: Province
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q223936
- candidate_exists: 1805..1814
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1809..1815
- overlap_years: 6
- bridge_fact_id: fact-q223936-parent-q71084
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q223793-parent-q71084 | Kingdom of Etruria -> Confederation of the Rhine -> First French Empire

- child_id: title-q223793
- child_rank: Kingdom
- expected_parent_rank: Crown
- candidate_parent_id: title-q154741
- candidate_exists: 1806..1813
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1804..1807
- overlap_years: 2
- bridge_fact_id: fact-q154741-parent-q71084
- review_priority: low_intermediate_parent_review
- notes: Review whether this rank skip is acceptable for query behavior or needs an intermediate parent packet.

### fact-q212278-parent-q71084 | Kingdom of Holland -> Confederation of the Rhine -> First French Empire

- child_id: title-q212278
- child_rank: Kingdom
- expected_parent_rank: Crown
- candidate_parent_id: title-q154741
- candidate_exists: 1806..1813
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1806..1810
- overlap_years: 5
- bridge_fact_id: fact-q154741-parent-q71084
- review_priority: low_intermediate_parent_review
- notes: Review whether this rank skip is acceptable for query behavior or needs an intermediate parent packet.

### fact-q223936-parent-q71084 | Kingdom of Italy -> Confederation of the Rhine -> First French Empire

- child_id: title-q223936
- child_rank: Kingdom
- expected_parent_rank: Crown
- candidate_parent_id: title-q154741
- candidate_exists: 1806..1813
- current_parent_id: title-q71084
- current_parent_rank: Empire
- span: 1805..1814
- overlap_years: 8
- bridge_fact_id: fact-q154741-parent-q71084
- review_priority: low_intermediate_parent_review
- notes: Review whether this rank skip is acceptable for query behavior or needs an intermediate parent packet.

### fact-q751868-parent-q12548 | Landgraviate of Brabant -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q751868
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1085..1183
- overlap_years: 99
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q695322-parent-q12548 | Landgraviate of Hesse -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q695322
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1264..1567
- overlap_years: 304
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q693551-parent-q12548 | Landgraviate of Hesse-Darmstadt -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q693551
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1567..1806
- overlap_years: 240
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q168651-parent-q12548 | Landgraviate of Hesse-Kassel -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q168651
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1567..1803
- overlap_years: 237
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q673837-parent-q12548 | Landgraviate of Hesse-Marburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q673837
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1458..1604
- overlap_years: 147
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q58942549-parent-q12548 | Landgraviate of Lower Alsace -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q58942549
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1200..1700
- overlap_years: 501
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q660393-parent-q12548 | Lower Lotharingia -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q660393
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1190
- overlap_years: 229
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q283627-parent-q12548 | Margraviate of Austria -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q283627
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 976..1156
- overlap_years: 181
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q148499-parent-q12548 | Margraviate of Brandenburg -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q148499
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1157..1806
- overlap_years: 650
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q170180-parent-q12548 | Margraviate of Meissen -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q170180
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 965..1423
- overlap_years: 459
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q2670751-parent-q28513 | Margraviate of Moravia -> Kingdom of Bohemia -> Austria-Hungary

- child_id: title-q2670751
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q28513
- current_parent_rank: Empire
- span: 1867..1918
- overlap_years: 52
- bridge_fact_id: fact-q42585-parent-q28513
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q552822-parent-q12548 | Mecklenburg-Gustrow -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q552822
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1520..1695
- overlap_years: 176
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q736029-parent-q12548-1303 | Nassau-Siegen -> Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q736029
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q686965
- candidate_exists: 1252..1863
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1303..1328
- overlap_years: 26
- bridge_fact_id: fact-q686965-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q736029-parent-q12548-1606 | Nassau-Siegen -> Anhalt-Bernburg -> Holy Roman Empire

- child_id: title-q736029
- child_rank: County
- expected_parent_rank: Duchy
- candidate_parent_id: title-q686965
- candidate_exists: 1252..1863
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1606..1743
- overlap_years: 138
- bridge_fact_id: fact-q686965-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q454436-parent-q12548 | Palatinate-Sulzbach -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q454436
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1557..1806
- overlap_years: 250
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q825902-parent-q172107 | Polish-Lithuanian union -> Crown of the Kingdom of Poland -> Polish-Lithuanian Commonwealth

- child_id: title-q825902
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q171348
- candidate_exists: 1386..1795
- current_parent_id: title-q172107
- current_parent_rank: Crown
- span: 1569..1569
- overlap_years: 1
- bridge_fact_id: fact-q171348-parent-q172107
- review_priority: medium_intermediate_parent_review
- notes: Review for a missing immediate parent layer, or document why this direct parentage should remain rank-skipped.

### fact-q701614-parent-q12548 | Prince-Archbishopric of Salzburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q701614
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1278..1803
- overlap_years: 526
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q173863-parent-q12548 | Prince-Bishopric of Augsburg -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q173863
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 962..1803
- overlap_years: 842
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q319586-parent-q12548 | Prince-Bishopric of Basel -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q319586
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1032..1803
- overlap_years: 772
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q259511-parent-q12548 | Prince-Bishopric of Freising -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q259511
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1294..1802
- overlap_years: 509
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q158835-parent-q12548 | Prince-Bishopric of Liege -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q158835
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 985..1795
- overlap_years: 811
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q650645-parent-q12548 | Prince-Bishopric of Minden -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q650645
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1648
- overlap_years: 469
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q3324486-parent-q12560 | Prince-Bishopric of Montenegro -> Eastern Hungarian Kingdom -> Ottoman Empire

- child_id: title-q3324486
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q625380
- candidate_exists: 1526..1570
- current_parent_id: title-q12560
- current_parent_rank: Empire
- span: 1516..1696
- overlap_years: 45
- bridge_fact_id: fact-q625380-parent-q12560
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q697254-parent-q12548 | Prince-Bishopric of Munster -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q697254
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1180..1802
- overlap_years: 623
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q477035-parent-q12548 | Prince-Bishopric of Osnabruck -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q477035
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1225..1803
- overlap_years: 579
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q649192-parent-q12548 | Prince-Bishopric of Paderborn -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q649192
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1281..1802
- overlap_years: 522
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q771332-parent-q12548 | Prince-Bishopric of Strasbourg -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q771332
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 982..1803
- overlap_years: 822
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q328001-parent-q12548 | Prince-Bishopric of Toul -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q328001
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1048..1648
- overlap_years: 601
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q1231403-parent-q12548 | Prince-Bishopric of Trent -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q1231403
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1027..1803
- overlap_years: 777
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q707767-parent-q12548 | Prince-Bishopric of Utrecht -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q707767
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1024..1528
- overlap_years: 505
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q17015016-parent-q12548 | Prince-Bishopric of Verdun -> Kingdom of Italy -> Holy Roman Empire

- child_id: title-q17015016
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q838931
- candidate_exists: 961..1806
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 997..1552
- overlap_years: 556
- bridge_fact_id: fact-q838931-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q14551680-parent-q12548 | Principality of Lippe -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q14551680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1789..1806
- overlap_years: 18
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q14551680-parent-q151624 | Principality of Lippe -> Kingdom of Bavaria -> German Confederation

- child_id: title-q14551680
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q158151-parent-q12548 | Saxe-Altenburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q158151
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1602..1806
- overlap_years: 205
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q158151-parent-q151624 | Saxe-Altenburg -> Kingdom of Bavaria -> German Confederation

- child_id: title-q158151
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q3462133-parent-q151624 | Saxe-Coburg and Gotha -> Kingdom of Bavaria -> German Confederation

- child_id: title-q3462133
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1826..1866
- overlap_years: 41
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q700663-parent-q12548 | Saxe-Coburg-Saalfeld -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q700663
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1699..1806
- overlap_years: 108
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q700663-parent-q151624 | Saxe-Coburg-Saalfeld -> Kingdom of Bavaria -> German Confederation

- child_id: title-q700663
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1825
- overlap_years: 11
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q675085-parent-q12548 | Saxe-Gotha-Altenburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q675085
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1680..1806
- overlap_years: 127
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q675085-parent-q151624 | Saxe-Gotha-Altenburg -> Kingdom of Bavaria -> German Confederation

- child_id: title-q675085
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1826
- overlap_years: 12
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q281005-parent-q12548 | Saxe-Hildburghausen -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q281005
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1680..1806
- overlap_years: 127
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q281005-parent-q151624 | Saxe-Hildburghausen -> Kingdom of Bavaria -> German Confederation

- child_id: title-q281005
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1826
- overlap_years: 12
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q313175-parent-q12548 | Saxe-Lauenburg -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q313175
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1296..1806
- overlap_years: 511
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q313175-parent-q151624 | Saxe-Lauenburg -> Kingdom of Bavaria -> German Confederation

- child_id: title-q313175
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q155570-parent-q150981 | Saxe-Weimar-Eisenach -> Kingdom of Prussia -> North German Confederation

- child_id: title-q155570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q27306
- candidate_exists: 1701..1918
- current_parent_id: title-q150981
- current_parent_rank: Empire
- span: 1867..1870
- overlap_years: 4
- bridge_fact_id: fact-q27306-parent-q150981
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q155570-parent-q151624 | Saxe-Weimar-Eisenach -> Kingdom of Bavaria -> German Confederation

- child_id: title-q155570
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q310650-parent-q12548 | Schaumburg-Lippe -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q310650
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1643..1806
- overlap_years: 164
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q310650-parent-q151624 | Schaumburg-Lippe -> Kingdom of Bavaria -> German Confederation

- child_id: title-q310650
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q695316-parent-q151624 | Schwarzburg-Rudolstadt -> Kingdom of Bavaria -> German Confederation

- child_id: title-q695316
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q630163-parent-q12548 | Schwarzburg-Sondershausen -> Kingdom of Bohemia -> Holy Roman Empire

- child_id: title-q630163
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q42585
- candidate_exists: 1198..1918
- current_parent_id: title-q12548
- current_parent_rank: Empire
- span: 1599..1806
- overlap_years: 208
- bridge_fact_id: fact-q42585-parent-q12548
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

### fact-q630163-parent-q151624 | Schwarzburg-Sondershausen -> Kingdom of Bavaria -> German Confederation

- child_id: title-q630163
- child_rank: Duchy
- expected_parent_rank: Kingdom
- candidate_parent_id: title-q154195
- candidate_exists: 1806..1918
- current_parent_id: title-q151624
- current_parent_rank: Empire
- span: 1815..1866
- overlap_years: 52
- bridge_fact_id: fact-q154195-parent-q151624
- review_priority: high_intermediate_parent_review
- notes: Find reviewed immediate parent layer for this span before treating the hierarchy as a complete tree.

