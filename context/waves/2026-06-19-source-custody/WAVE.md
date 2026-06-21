# Wave: Source Custody Intake

## Goal

Move DUCHY from seed-only fixtures toward source-aware historical data without
importing real title facts before custody review.

## Thesis

DUCHY can only answer real European historical questions responsibly if every
source-backed claim carries source metadata, allowed-use posture, confidence,
and review evidence. The next safe implementation layer is metadata-only source
records and validation.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Metadata-only source records | complete | Add source records, allowed-use posture, review decisions, and validation. |
| 02 | Metadata source file format | complete | Define and parse a fixture file shape for source records without importing historical facts. |
| 03 | Source-backed fact gate | complete | Validate fact records against accepted source records and confidence labels. |
| 04 | First real source-backed facts | complete | Import only reviewed name, rank, and existence facts for Wikidata Q158445. |
| 05 | Source-backed title materialization | complete | Convert reviewed fact sets into a real `Title` record without parentage/control claims. |
| 06 | Source-backed title query | complete | Answer a source-backed title-path query for the first real title. |
| 07 | Fact fixture import | complete | Parse first real source-backed facts from `fixtures/first-real.facts`. |
| 08 | Contested-history review packet | complete | Represent conflicting source-backed claims without overwriting them. |
| 09 | Second reviewed source import | complete | Add Q20135 Grand Duchy of Hesse name/rank/existence facts. |
| 10 | Source-backed parentage review | complete | Materialize reviewed parentage facts while keeping real fixture hierarchy-free. |
| 11 | Reviewed source fixture import | complete | Parse reviewed real source records from `fixtures/first-real.sources`. |
| 12 | First real parentage source import | complete | Import Q20135 -> Q43287 and Q158445 -> Q43287 for 1871-1918 after source review. |
| 13 | Fixture-canonical import path | complete | Remove reviewed historical data literals from Rust; make fixtures the import source of truth. |
| 14 | Reviewed Prussia import packet | complete | Import Q27306 title facts and Q27306 -> Q43287 for 1871-1918 after source review. |
| 15 | Reviewed Saxony import packet | complete | Import Q153015 title facts and Q153015 -> Q43287 for 1871-1918 after source review. |
| 16 | Batch import staging gate | complete | Add CLI status/dry-run promotion and duplicate/conflict validation for staged batches. |
| 17 | Apply-mode promotion | complete | Let reviewed staging batches rewrite accepted fixture files after full validation. |
| 18 | Promotion review reports | complete | Emit promotion reports listing candidate titles, parentage, fact IDs, and merged counts. |
| 19 | Candidate manifest queue | complete | Parse and validate staging manifests before fact extraction. |
| 20 | Source stub generation | complete | Generate blocked source stubs from reviewed manifest candidates. |
| 21 | Rejected candidate audit | complete | Generate rejected-candidate reports before queue cleanup. |
| 22 | Active manifest cleanup | complete | Generate pending/reviewed-only manifests after audit/archive. |
| 23 | Archive manifest cleanup | complete | Generate promoted/rejected-only manifests for queue audit. |
| 24 | Manifest sharding | complete | Split large candidate manifests into fixed-size review batches. |
| 25 | Shard index | complete | Write per-shard status counts for large review queues. |
| 26 | Manifest review report | complete | List every candidate grouped by queue status for inspection. |
| 27 | Duplicate URL report | complete | Report repeated candidate source URLs before source review. |
| 28 | Manifest TSV export | complete | Write fixed-column candidate queue exports for batch tooling. |
| 29 | Manifest TSV import | complete | Convert fixed-column TSV candidate queues into validated manifests. |
| 30 | German bridge import | complete | Import reviewed 1815-1866, 1867-1870, and 1871-1918 German parentage spans. |
| 31 | Baden and Hanover import | complete | Import reviewed Baden and Hanover title facts and parentage spans. |
| 32 | Oldenburg and Brunswick import | complete | Import reviewed Oldenburg and Brunswick title facts and parentage spans. |
| 33 | Mass title source scale-up | complete | Import reviewed title facts until the accepted source catalog reaches 50 sources. |
| 34 | Scale metadata manifest gate | complete | Require 500-source readiness metadata for reviewed/promoted manifest rows. |
| 35 | 500-source candidate queue | complete | Stage 450 additional candidate sources so accepted plus staged sources reach 500 under review. |
| 36 | Batch 001 title promotion | complete | Promote 18 reviewed title-identity sources from the 500-source candidate queue. |
| 37 | Batch 002 title promotion | complete | Promote 26 reviewed title-identity sources from the 500-source candidate queue. |
| 38 | Remaining title queue promotion | complete | Promote all remaining clean title-identity sources from the 500-source candidate queue. |
| 39 | Unsupported queue closure | complete | Archive unsupported and relation-heavy candidates as scope-deferred rejections. |
| 40 | Date-problem queue closure | complete | Reject final date-problem candidates and close the 500-source queue. |
| 41 | Additional German parentage packet | complete | Import parentage-only facts that cite already accepted source records. |
| 42 | Austrian parentage packet | complete | Import Austrian Empire and Austria-Hungary parentage facts using accepted source records. |
| 43 | Holy Roman Empire parentage packet | complete | Import Holy Roman Empire parentage facts using accepted source records. |
| 44 | Kingdom of France parentage packet | complete | Import France parentage facts using accepted source records and non-overlapping spans. |
| 45 | Residual German Confederation parentage packet | complete | Import remaining clean German Confederation parentage facts using accepted source records. |
| 46 | CK3 Europe county discovery queue | complete | Stage 500 pending Europe-bucket CK3 county candidates as search leads, with no fact promotion. |
| 47 | CK3 batch 001 source-resolution leads | complete | Query Wikidata for the first 50 CK3 county candidates and preserve exact/fuzzy/no-result leads for manual review. |
| 48 | CK3 batch 001 reviewed source-resolution packet | complete | Promote 7 reviewed Wikidata source records from the first CK3 research shard, with no fact promotion. |
| 49 | CK3 batch 001 first title fact packet | complete | Promote Abaúj county name, rank, and existence facts from an accepted source-resolution record. |
| 50 | CK3 batch 001 structured screen and Ailech source record | complete | Screen first-shard leads for bounded structured dates and promote Ailech as source-only with rank facts deferred. |
| 51 | CK3 batch 002 source-resolution packet | complete | Query and screen the second CK3 shard, then promote Arbanon and Béarn as source-only records with title facts deferred. |
| 52 | CK3 batch 003 research screen | complete | Query and screen the third CK3 shard; no top leads have bounded date pairs for promotion. |
| 53 | CK3 batch 004 reviewed packet | complete | Query and screen the fourth CK3 shard, then promote 3 sources and 6 title facts while deferring Byzantium title facts. |
| 54 | CK3 batch 005 research screen | complete | Query and screen the fifth CK3 shard; no top leads have bounded date pairs for promotion. |
| 55 | CK3 batch 006 source-resolution packet | complete | Query and screen the sixth CK3 shard, then promote Donji Kraji as source-only while skipping already accepted Duklja. |
| 56 | CK3 batch 007 research screen | complete | Query and screen the seventh CK3 shard; no top leads have bounded date pairs for promotion. |
| 57 | CK3 queue closure | complete | Query and screen final shards 008-010, promote Hordaland title facts, and close all 500 CK3 discovery rows as scope-deferred. |
| 58 | Accepted relation bridges parentage packet | complete | Import 4 parentage-only facts using already accepted source records and structured relation review. |
| 59 | Second accepted relation bridges parentage packet | complete | Import 11 non-overlapping parentage-only facts using accepted source records and structured relation review. |
| 60 | Parentage coverage report | complete | Add a repeatable report for hierarchy coverage by rank, unparented titles, and multi-parentage review targets. |
| 61 | Parentage gap TSV queue | complete | Export the 211 unparented accepted titles as a machine-readable review queue with rank-based priority. |
| 62 | Parentage gap sharding and reports | complete | Split the 211-row gap queue into 9 review shards and render Markdown reports for each shard. |
| 63 | Composite crown parentage packet | complete | Add a composite `Crown` rank and import the reviewed Kingdom of Murcia -> Crown of Castile bridge from parentage-gap shard 001. |
| 64 | Crown of Aragon parentage packet | complete | Import the reviewed County of Barcelona -> Crown of Aragon bridge from parentage-gap shard 001. |
| 65 | Crown bridge parentage packet | complete | Import 10 parentage-only kingdom -> crown bridges using already accepted source records. |
| 66 | Commonwealth rank correction and Livonia parentage | complete | Correct Polish-Lithuanian Commonwealth to Crown rank and import Duchy of Livonia -> Commonwealth parentage. |
| 67 | Commonwealth children parentage packet | complete | Import Crown of the Kingdom of Poland and Grand Duchy of Lithuania under Polish-Lithuanian Commonwealth. |
| 68 | Commonwealth boundary parentage packet | complete | Import the year-granular Kingdom of Poland -> Polish-Lithuanian Commonwealth transition at 1569. |
| 69 | Theocratic state parentage packet | complete | Add `TheocraticState` rank support, promote Papal States, and import Comtat Venaissin -> Papal States parentage. |
| 70 | Kingdom of Poland German Empire parentage | complete | Import the reviewed 1916-1918 Kingdom of Poland -> German Empire relation. |
| 71 | Russian Empire Finland parentage | complete | Promote Russian Empire and import Grand Duchy of Finland -> Russian Empire parentage. |
| 72 | Gorizia and Gradisca Austrian parentage | complete | Correct Gorizia and Gradisca to County rank and import Austrian Empire/Austria-Hungary parentage. |
| 73 | Duchy of Urbino Papal States parentage | complete | Import Duchy of Urbino -> Papal States parentage from an accepted structured relation. |
| 74 | Illyrian Provinces French Empire parentage | complete | Add `Province` rank support, correct Illyrian Provinces rank, and import First French Empire parentage. |
| 75 | Electoral Palatinate HRE parentage | complete | Correct Electoral Palatinate rank to Duchy and import Holy Roman Empire parentage. |
| 76 | Free Imperial City of Aachen HRE parentage | complete | Add `FreeCity` rank support, correct Free Imperial City of Aachen rank, and import Holy Roman Empire parentage. |
| 77 | Burgraviate of Nuremberg HRE parentage | complete | Correct Burgraviate of Nuremberg rank to County and import Holy Roman Empire parentage. |
| 78 | Courland Commonwealth parentage | complete | Import Duchy of Courland and Semigallia parentage under Polish-Lithuanian Commonwealth. |
| 79 | Estonia Sweden parentage | complete | Promote Sweden and import Duchy of Estonia parentage under Sweden. |
| 80 | Schleswig Denmark parentage | complete | Promote Denmark and import Duchy of Schleswig parentage under Denmark. |
| 81 | Burgundian Netherlands State parentage | complete | Promote Burgundian State and import Burgundian Netherlands parentage under it. |
| 82 | Hispanic Monarchy crown parentage | complete | Promote Hispanic Monarchy and import Crown of Aragon and Crown of Castile parentage under it. |
| 83 | Kalmar Union Sweden parentage | complete | Correct Kalmar Union to Crown rank and import Sweden parentage under it. |
| 84 | Prince-Bishopric of Trent HRE parentage | complete | Add a reviewed Wikimedia text source and import Trent parentage under the Holy Roman Empire. |
| 85 | County of Savoy HRE parentage | complete | Add a reviewed Wikimedia text source and import County of Savoy parentage under the Holy Roman Empire. |
| 86 | Duchy of Warsaw French Empire parentage | complete | Add a reviewed Wikimedia text source and import Duchy of Warsaw parentage under the First French Empire. |
| 87 | Confederation of the Rhine Westphalia parentage | complete | Promote Confederation of the Rhine and import Kingdom of Westphalia parentage under it. |
| 88 | Confederation of the Rhine French Empire parentage | complete | Add a reviewed Wikimedia text source and import Confederation of the Rhine parentage under the First French Empire. |
| 89 | Prince-Bishopric of Augsburg HRE parentage | complete | Add a reviewed Wikimedia text source and import Prince-Bishopric of Augsburg parentage under the Holy Roman Empire. |
| 90 | Kingdom of Imereti Russian Empire parentage | complete | Add a reviewed Wikimedia text source and import Kingdom of Imereti parentage under the Russian Empire. |
| 91 | Italian protectorate of Albania Italian Empire parentage | complete | Promote Italian Empire and import Italian protectorate of Albania parentage under it. |
| 92 | Francia Carolingian Empire parentage | complete | Add a reviewed Wikimedia text source and import Francia parentage under the Carolingian Empire. |
| 93 | Duchy of Prussia Polish Crown parentage | complete | Add a reviewed Wikimedia text source and import Duchy of Prussia parentage under the Crown of the Kingdom of Poland. |
| 94 | County of Empuries Crown of Aragon parentage | complete | Add a reviewed Wikimedia text source and import County of Empuries parentage under the Crown of Aragon. |
| 95 | Grand Duchy of Tuscany HRE parentage | complete | Add a reviewed Wikimedia text source and import Grand Duchy of Tuscany parentage under the Holy Roman Empire. |
| 96 | Duchy of Parma and Piacenza Empire parentage | complete | Add a reviewed Wikimedia text source and import Duchy of Parma and Piacenza parentage under the Holy Roman Empire. |

## Success Criteria

- Source records validate required metadata.
- Metadata-only records cannot masquerade as accepted historical facts.
- Review decisions reference existing source records.
- Validation commands pass.
- Real title facts enter only through reviewed source records and fact gate validation.
- Reviewed historical IDs, names, spans, and relations live in fixtures rather
  than Rust literals.
- Candidate batches validate in staging before accepted fixture promotion.
