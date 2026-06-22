# DUCHY Parentage Rank Skip Bridge Cluster Report

source_tsv: data/staging/parentage-rank-skip-bridge-clusters.tsv
clusters: 20
clustered_children: 161
high_priority_rows: 150
medium_priority_rows: 9
low_priority_rows: 3

## Interpretation

- Clusters are packet-planning leads, not import-ready parentage claims.
- A large cluster usually means many skipped edges share one accepted same-parent bridge candidate.
- Reviewers must verify child-to-candidate parentage from sources before replacing direct skipped parentage.

## Cluster Summary

| Candidate Parent | Current Parent | Expected Rank | Children | High | Medium | Low | Span Range |
|---|---|---|---:|---:|---:|---:|---|
| Kingdom of Bohemia | Holy Roman Empire | Kingdom | 49 | 49 | 0 | 0 | 1200..1806 |
| Kingdom of Italy | Holy Roman Empire | Kingdom | 31 | 31 | 0 | 0 | 962..1806 |
| Kingdom of Bavaria | German Confederation | Kingdom | 27 | 27 | 0 | 0 | 1815..1866 |
| Duchy of Bavaria | Holy Roman Empire | Duchy | 12 | 12 | 0 | 0 | 962..1806 |
| Kingdom of Bavaria | German Empire | Kingdom | 8 | 8 | 0 | 0 | 1871..1918 |
| Kingdom of Prussia | North German Confederation | Kingdom | 6 | 6 | 0 | 0 | 1867..1870 |
| Kingdom of Bohemia | Austria-Hungary | Kingdom | 4 | 4 | 0 | 0 | 1867..1918 |
| Crown of the Kingdom of Poland | Polish-Lithuanian Commonwealth | Kingdom | 4 | 0 | 4 | 0 | 1569..1795 |
| Confederation of the Rhine | First French Empire | Crown | 3 | 0 | 0 | 3 | 1804..1814 |
| Kingdom of Westphalia | Confederation of the Rhine | Kingdom | 3 | 0 | 3 | 0 | 1806..1813 |
| Anhalt-Bernburg | Holy Roman Empire | Duchy | 2 | 3 | 0 | 0 | 1303..1743 |
| Kingdom of Bohemia | Austrian Empire | Kingdom | 2 | 2 | 0 | 0 | 1804..1866 |
| Kingdom of Italy | First French Empire | Kingdom | 2 | 2 | 0 | 0 | 1807..1815 |
| Duchy of Brittany | Kingdom of France | Duchy | 2 | 0 | 2 | 0 | 987..1527 |
| Archduchy of Austria | Austria-Hungary | Duchy | 1 | 1 | 0 | 0 | 1867..1918 |
| Archduchy of Austria | Austrian Empire | Duchy | 1 | 1 | 0 | 0 | 1804..1866 |
| Duchy of Ferrara | Papal States | Duchy | 1 | 1 | 0 | 0 | 1274..1791 |
| Eastern Hungarian Kingdom | Ottoman Empire | Kingdom | 1 | 1 | 0 | 0 | 1516..1696 |
| Kingdom of Imereti | Russian Empire | Kingdom | 1 | 1 | 0 | 0 | 1809..1917 |
| Margraviate of Brandenburg | Holy Roman Empire | Duchy | 1 | 1 | 0 | 0 | 1160..1806 |

## Review Packets

### Kingdom of Bohemia -> Holy Roman Empire

- candidate_parent_id: title-q42585
- current_parent_id: title-q12548
- expected_parent_rank: Kingdom
- child_count: 49
- priority_counts: high 49, medium 0, low 0
- span_range: 1200..1806
- bridge_fact_ids: fact-q42585-parent-q12548
- skip_fact_ids: fact-q11024667-parent-q12548,fact-q14551680-parent-q12548,fact-q151095-parent-q12548,fact-q153529-parent-q12548,fact-q154849-parent-q12548,fact-q156199-parent-q12548,fact-q157013-parent-q12548,fact-q157710-parent-q12548,fact-q158151-parent-q12548,fact-q165040-parent-q12548,fact-q168651-parent-q12548,fact-q2172530-parent-q12548,fact-q2227570-parent-q12548,fact-q2252973-parent-q12548,fact-q2360973-parent-q12548,fact-q252580-parent-q12548,fact-q256961-parent-q12548,fact-q259511-parent-q12548,fact-q264970-parent-q12548,fact-q2719360-parent-q12548,fact-q278874-parent-q12548,fact-q281005-parent-q12548,fact-q310650-parent-q12548,fact-q313175-parent-q12548,fact-q426025-parent-q12548,fact-q454436-parent-q12548,fact-q477035-parent-q12548,fact-q552033-parent-q12548,fact-q552822-parent-q12548,fact-q556263-parent-q12548,fact-q58942549-parent-q12548,fact-q630163-parent-q12548,fact-q637238-parent-q12548,fact-q649192-parent-q12548,fact-q673837-parent-q12548,fact-q673865-parent-q12548,fact-q675085-parent-q12548,fact-q686965-parent-q12548,fact-q693551-parent-q12548,fact-q694594-parent-q12548,fact-q695322-parent-q12548,fact-q699964-parent-q12548,fact-q700663-parent-q12548,fact-q701614-parent-q12548,fact-q704288-parent-q12548,fact-q706018-parent-q12548,fact-q766501-parent-q12548,fact-q830084-parent-q12548,fact-q933592-parent-q12548
- child_ids: title-q11024667,title-q14551680,title-q151095,title-q153529,title-q154849,title-q156199,title-q157013,title-q157710,title-q158151,title-q165040,title-q168651,title-q2172530,title-q2227570,title-q2252973,title-q2360973,title-q252580,title-q256961,title-q259511,title-q264970,title-q2719360,title-q278874,title-q281005,title-q310650,title-q313175,title-q426025,title-q454436,title-q477035,title-q552033,title-q552822,title-q556263,title-q58942549,title-q630163,title-q637238,title-q649192,title-q673837,title-q673865,title-q675085,title-q686965,title-q693551,title-q694594,title-q695322,title-q699964,title-q700663,title-q701614,title-q704288,title-q706018,title-q766501,title-q830084,title-q933592
- child_names: Anhalt-Bernburg; Anhalt-Dessau; Anhalt-Kothen; Archduchy of Austria; Bavaria-Munich; Brunswick-Luneburg; Brunswick-Wolfenbuttel; Duchy of Berg; Duchy of Bremen and Verden; Duchy of Carniola; Duchy of Florence; Duchy of Holstein; Duchy of Luxembourg; Duchy of Mantua; Duchy of Massa and Carrara; Duchy of Mecklenburg-Schwerin; Duchy of Milan; Duchy of Modena and Reggio; Duchy of Parma and Piacenza; Duchy of Savoy; Duchy of Saxe-Meiningen; Duchy of Wurttemberg; Electorate of Baden; Electorate of Bavaria; Electorate of Hanover; Electorate of Saxony; Electorate of Wurttemberg; Grand Duchy of Tuscany; Hohenzollern-Hechingen; Hohenzollern-Sigmaringen; Landgraviate of Hesse; Landgraviate of Hesse-Darmstadt; Landgraviate of Hesse-Kassel; Landgraviate of Hesse-Marburg; Landgraviate of Lower Alsace; Mecklenburg-Gustrow; Palatinate-Sulzbach; Prince-Archbishopric of Salzburg; Prince-Bishopric of Freising; Prince-Bishopric of Osnabruck; Prince-Bishopric of Paderborn; Principality of Lippe; Saxe-Altenburg; Saxe-Coburg-Saalfeld; Saxe-Gotha-Altenburg; Saxe-Hildburghausen; Saxe-Lauenburg; Schaumburg-Lippe; Schwarzburg-Sondershausen

### Kingdom of Italy -> Holy Roman Empire

- candidate_parent_id: title-q838931
- current_parent_id: title-q12548
- expected_parent_rank: Kingdom
- child_count: 31
- priority_counts: high 31, medium 0, low 0
- span_range: 962..1806
- bridge_fact_ids: fact-q838931-parent-q12548
- skip_fact_ids: fact-q1231403-parent-q12548,fact-q148499-parent-q12548,fact-q155019-parent-q12548,fact-q158835-parent-q12548,fact-q159856-parent-q12548,fact-q164092-parent-q12548,fact-q17015016-parent-q12548,fact-q170180-parent-q12548,fact-q173863-parent-q12548,fact-q1867844-parent-q12548,fact-q2162698-parent-q12548,fact-q22880-parent-q12548,fact-q283627-parent-q12548,fact-q284667-parent-q12548,fact-q319586-parent-q12548,fact-q328001-parent-q12548,fact-q3624335-parent-q12548,fact-q426488-parent-q12548,fact-q47261-parent-q12548,fact-q641138-parent-q12548,fact-q650645-parent-q12548,fact-q657241-parent-q12548,fact-q660393-parent-q12548,fact-q693980-parent-q12548,fact-q696640-parent-q12548,fact-q697254-parent-q12548,fact-q707767-parent-q12548,fact-q751868-parent-q12548,fact-q771332-parent-q12548,fact-q7904317-parent-q12548,fact-q836937-parent-q12548
- child_ids: title-q1231403,title-q148499,title-q155019,title-q158835,title-q159856,title-q164092,title-q17015016,title-q170180,title-q173863,title-q1867844,title-q2162698,title-q22880,title-q283627,title-q284667,title-q319586,title-q328001,title-q3624335,title-q426488,title-q47261,title-q641138,title-q650645,title-q657241,title-q660393,title-q693980,title-q696640,title-q697254,title-q707767,title-q751868,title-q771332,title-q7904317,title-q836937
- child_names: Duchy of Austria; Duchy of Bavaria; Duchy of Bohemia; Duchy of Brabant; Duchy of Cleves; Duchy of Julich; Duchy of Lorraine; Duchy of Pomerania; Duchy of Saxony; Duchy of Swabia; Duchy of Westphalia; Electoral Palatinate; Electorate of Cologne; Electorate of Mainz; Landgraviate of Brabant; Lower Lotharingia; March of Turin; March of Tuscany; Margraviate of Austria; Margraviate of Brandenburg; Margraviate of Meissen; Prince-Bishopric of Augsburg; Prince-Bishopric of Basel; Prince-Bishopric of Liege; Prince-Bishopric of Minden; Prince-Bishopric of Munster; Prince-Bishopric of Strasbourg; Prince-Bishopric of Toul; Prince-Bishopric of Trent; Prince-Bishopric of Utrecht; Prince-Bishopric of Verdun

### Kingdom of Bavaria -> German Confederation

- candidate_parent_id: title-q154195
- current_parent_id: title-q151624
- expected_parent_rank: Kingdom
- child_count: 27
- priority_counts: high 27, medium 0, low 0
- span_range: 1815..1866
- bridge_fact_ids: fact-q154195-parent-q151624
- skip_fact_ids: fact-q14551680-parent-q151624,fact-q155570-parent-q151624,fact-q157013-parent-q151624,fact-q157710-parent-q151624,fact-q158151-parent-q151624,fact-q158445-parent-q151624,fact-q161215-parent-q151624,fact-q16550783-parent-q151624,fact-q186320-parent-q151624,fact-q20135-parent-q151624,fact-q264970-parent-q151624,fact-q278874-parent-q151624,fact-q281005-parent-q151624,fact-q310650-parent-q151624,fact-q313175-parent-q151624,fact-q326029-parent-q151624,fact-q3462133-parent-q151624,fact-q529605-parent-q151624,fact-q630163-parent-q151624,fact-q673865-parent-q151624,fact-q675085-parent-q151624,fact-q686965-parent-q151624,fact-q693669-parent-q151624,fact-q695316-parent-q151624,fact-q700663-parent-q151624,fact-q704288-parent-q151624,fact-q836680-parent-q151624
- child_ids: title-q14551680,title-q155570,title-q157013,title-q157710,title-q158151,title-q158445,title-q161215,title-q16550783,title-q186320,title-q20135,title-q264970,title-q278874,title-q281005,title-q310650,title-q313175,title-q326029,title-q3462133,title-q529605,title-q630163,title-q673865,title-q675085,title-q686965,title-q693669,title-q695316,title-q700663,title-q704288,title-q836680
- child_names: Anhalt-Bernburg; Anhalt-Dessau; Anhalt-Kothen; Duchy of Anhalt; Duchy of Brunswick; Duchy of Holstein; Duchy of Nassau; Duchy of Saxe-Meiningen; Electorate of Hesse; Grand Duchy of Baden; Grand Duchy of Hesse; Grand Duchy of Mecklenburg-Schwerin; Grand Duchy of Mecklenburg-Strelitz; Grand Duchy of Oldenburg; Hohenzollern-Hechingen; Hohenzollern-Sigmaringen; Principality of Lippe; Saxe-Altenburg; Saxe-Coburg and Gotha; Saxe-Coburg-Saalfeld; Saxe-Gotha-Altenburg; Saxe-Hildburghausen; Saxe-Lauenburg; Saxe-Weimar-Eisenach; Schaumburg-Lippe; Schwarzburg-Rudolstadt; Schwarzburg-Sondershausen

### Duchy of Bavaria -> Holy Roman Empire

- candidate_parent_id: title-q47261
- current_parent_id: title-q12548
- expected_parent_rank: Duchy
- child_count: 12
- priority_counts: high 12, medium 0, low 0
- span_range: 962..1806
- bridge_fact_ids: fact-q47261-parent-q12548
- skip_fact_ids: fact-q1232887-parent-q12548,fact-q152420-parent-q12548,fact-q157070-parent-q12548,fact-q2991474-parent-q12548,fact-q5177890-parent-q12548,fact-q568473-parent-q12548,fact-q573290-parent-q12548,fact-q589251-parent-q12548,fact-q599613-parent-q12548,fact-q642314-parent-q12548,fact-q675363-parent-q12548,fact-q762943-parent-q12548
- child_ids: title-q1232887,title-q152420,title-q157070,title-q2991474,title-q5177890,title-q568473,title-q573290,title-q589251,title-q599613,title-q642314,title-q675363,title-q762943
- child_names: Burgraviate of Nuremberg; County of Burgundy; County of Flanders; County of Geneva; County of Holland; County of Luxembourg; County of Montbeliard; County of Namur; County of Ravensberg; County of Savoy; County of Wurttemberg; Duchy of Guelders

### Kingdom of Bavaria -> German Empire

- candidate_parent_id: title-q154195
- current_parent_id: title-q43287
- expected_parent_rank: Kingdom
- child_count: 8
- priority_counts: high 8, medium 0, low 0
- span_range: 1871..1918
- bridge_fact_ids: fact-q154195-parent-q43287
- skip_fact_ids: fact-q157710-parent-q43287,fact-q158445-parent-q43287,fact-q16550783-parent-q43287,fact-q186320-parent-q43287,fact-q1991540-parent-q43287,fact-q20135-parent-q43287,fact-q326029-parent-q43287,fact-q693669-parent-q43287
- child_ids: title-q157710,title-q158445,title-q16550783,title-q186320,title-q1991540,title-q20135,title-q326029,title-q693669
- child_names: Duchy of Anhalt; Duchy of Brunswick; Duchy of Courland and Semigallia; Duchy of Saxe-Meiningen; Grand Duchy of Baden; Grand Duchy of Hesse; Grand Duchy of Mecklenburg-Schwerin; Grand Duchy of Oldenburg

### Kingdom of Prussia -> North German Confederation

- candidate_parent_id: title-q27306
- current_parent_id: title-q150981
- expected_parent_rank: Kingdom
- child_count: 6
- priority_counts: high 6, medium 0, low 0
- span_range: 1867..1870
- bridge_fact_ids: fact-q27306-parent-q150981
- skip_fact_ids: fact-q155570-parent-q150981,fact-q158445-parent-q150981,fact-q161215-parent-q150981,fact-q16550783-parent-q150981,fact-q326029-parent-q150981,fact-q693669-parent-q150981
- child_ids: title-q155570,title-q158445,title-q161215,title-q16550783,title-q326029,title-q693669
- child_names: Duchy of Anhalt; Duchy of Brunswick; Grand Duchy of Mecklenburg-Schwerin; Grand Duchy of Mecklenburg-Strelitz; Grand Duchy of Oldenburg; Saxe-Weimar-Eisenach

### Kingdom of Bohemia -> Austria-Hungary

- candidate_parent_id: title-q42585
- current_parent_id: title-q28513
- expected_parent_rank: Kingdom
- child_count: 4
- priority_counts: high 4, medium 0, low 0
- span_range: 1867..1918
- bridge_fact_ids: fact-q42585-parent-q28513
- skip_fact_ids: fact-q2360973-parent-q28513,fact-q2670751-parent-q28513,fact-q661340-parent-q28513,fact-q699964-parent-q28513
- child_ids: title-q2360973,title-q2670751,title-q661340,title-q699964
- child_names: Archduchy of Austria; Duchy of Carniola; Duchy of Salzburg; Margraviate of Moravia

### Crown of the Kingdom of Poland -> Polish-Lithuanian Commonwealth

- candidate_parent_id: title-q171348
- current_parent_id: title-q172107
- expected_parent_rank: Kingdom
- child_count: 4
- priority_counts: high 0, medium 4, low 0
- span_range: 1569..1795
- bridge_fact_ids: fact-q171348-parent-q172107
- skip_fact_ids: fact-q1352878-parent-q172107,fact-q156038-parent-q172107,fact-q49683-parent-q172107,fact-q825902-parent-q172107
- child_ids: title-q1352878,title-q156038,title-q49683,title-q825902
- child_names: Duchy of Courland and Semigallia; Duchy of Livonia; Grand Duchy of Lithuania; Polish-Lithuanian union

### Confederation of the Rhine -> First French Empire

- candidate_parent_id: title-q154741
- current_parent_id: title-q71084
- expected_parent_rank: Crown
- child_count: 3
- priority_counts: high 0, medium 0, low 3
- span_range: 1804..1814
- bridge_fact_ids: fact-q154741-parent-q71084
- skip_fact_ids: fact-q212278-parent-q71084,fact-q223793-parent-q71084,fact-q223936-parent-q71084
- child_ids: title-q212278,title-q223793,title-q223936
- child_names: Kingdom of Etruria; Kingdom of Holland; Kingdom of Italy

### Kingdom of Westphalia -> Confederation of the Rhine

- candidate_parent_id: title-q153943
- current_parent_id: title-q154741
- expected_parent_rank: Kingdom
- child_count: 3
- priority_counts: high 0, medium 3, low 0
- span_range: 1806..1813
- bridge_fact_ids: fact-q153943-parent-q154741
- skip_fact_ids: fact-q249428-parent-q154741,fact-q698089-parent-q154741,fact-q704312-parent-q154741
- child_ids: title-q249428,title-q698089,title-q704312
- child_names: Grand Duchy of Berg; Grand Duchy of Frankfurt; Grand Duchy of Wurzburg

### Anhalt-Bernburg -> Holy Roman Empire

- candidate_parent_id: title-q686965
- current_parent_id: title-q12548
- expected_parent_rank: Duchy
- child_count: 2
- priority_counts: high 3, medium 0, low 0
- span_range: 1303..1743
- bridge_fact_ids: fact-q686965-parent-q12548
- skip_fact_ids: fact-q1615455-parent-q12548,fact-q736029-parent-q12548-1303,fact-q736029-parent-q12548-1606
- child_ids: title-q1615455,title-q736029
- child_names: Duchy of Mirandola; Nassau-Siegen

### Kingdom of Bohemia -> Austrian Empire

- candidate_parent_id: title-q42585
- current_parent_id: title-q131964
- expected_parent_rank: Kingdom
- child_count: 2
- priority_counts: high 2, medium 0, low 0
- span_range: 1804..1866
- bridge_fact_ids: fact-q42585-parent-q131964
- skip_fact_ids: fact-q2360973-parent-q131964,fact-q699964-parent-q131964
- child_ids: title-q2360973,title-q699964
- child_names: Archduchy of Austria; Duchy of Carniola

### Kingdom of Italy -> First French Empire

- candidate_parent_id: title-q223936
- current_parent_id: title-q71084
- expected_parent_rank: Kingdom
- child_count: 2
- priority_counts: high 2, medium 0, low 0
- span_range: 1807..1815
- bridge_fact_ids: fact-q223936-parent-q71084
- skip_fact_ids: fact-q152115-parent-q71084,fact-q699923-parent-q71084
- child_ids: title-q152115,title-q699923
- child_names: Duchy of Warsaw; Illyrian Provinces

### Duchy of Brittany -> Kingdom of France

- candidate_parent_id: title-q71747
- current_parent_id: title-q70972
- expected_parent_rank: Duchy
- child_count: 2
- priority_counts: high 0, medium 2, low 0
- span_range: 987..1527
- bridge_fact_ids: fact-q71747-parent-q70972
- skip_fact_ids: fact-q2991382-parent-q70972,fact-q921473-parent-q70972
- child_ids: title-q2991382,title-q921473
- child_names: County of La Marche; County of Provence

### Archduchy of Austria -> Austria-Hungary

- candidate_parent_id: title-q699964
- current_parent_id: title-q28513
- expected_parent_rank: Duchy
- child_count: 1
- priority_counts: high 1, medium 0, low 0
- span_range: 1867..1918
- bridge_fact_ids: fact-q699964-parent-q28513
- skip_fact_ids: fact-q692946-parent-q28513
- child_ids: title-q692946
- child_names: Gorizia and Gradisca

### Archduchy of Austria -> Austrian Empire

- candidate_parent_id: title-q699964
- current_parent_id: title-q131964
- expected_parent_rank: Duchy
- child_count: 1
- priority_counts: high 1, medium 0, low 0
- span_range: 1804..1866
- bridge_fact_ids: fact-q699964-parent-q131964
- skip_fact_ids: fact-q692946-parent-q131964
- child_ids: title-q692946
- child_names: Gorizia and Gradisca

### Duchy of Ferrara -> Papal States

- candidate_parent_id: title-q693570
- current_parent_id: title-q170174
- expected_parent_rank: Duchy
- child_count: 1
- priority_counts: high 1, medium 0, low 0
- span_range: 1274..1791
- bridge_fact_ids: fact-q693570-parent-q170174
- skip_fact_ids: fact-q1122980-parent-q170174
- child_ids: title-q1122980
- child_names: Comtat Venaissin

### Eastern Hungarian Kingdom -> Ottoman Empire

- candidate_parent_id: title-q625380
- current_parent_id: title-q12560
- expected_parent_rank: Kingdom
- child_count: 1
- priority_counts: high 1, medium 0, low 0
- span_range: 1516..1696
- bridge_fact_ids: fact-q625380-parent-q12560
- skip_fact_ids: fact-q3324486-parent-q12560
- child_ids: title-q3324486
- child_names: Prince-Bishopric of Montenegro

### Kingdom of Imereti -> Russian Empire

- candidate_parent_id: title-q1069959
- current_parent_id: title-q34266
- expected_parent_rank: Kingdom
- child_count: 1
- priority_counts: high 1, medium 0, low 0
- span_range: 1809..1917
- bridge_fact_ids: fact-q1069959-parent-q34266
- skip_fact_ids: fact-q62633-parent-q34266
- child_ids: title-q62633
- child_names: Grand Duchy of Finland

### Margraviate of Brandenburg -> Holy Roman Empire

- candidate_parent_id: title-q148499
- current_parent_id: title-q12548
- expected_parent_rank: Duchy
- child_count: 1
- priority_counts: high 1, medium 0, low 0
- span_range: 1160..1806
- bridge_fact_ids: fact-q148499-parent-q12548
- skip_fact_ids: fact-q12817455-parent-q12548
- child_ids: title-q12817455
- child_names: County of Nassau

