# DUCHY Kingdom of Germany Parentage Packet Candidates

candidate_tsv: `data/staging/kingdom-germany-parentage-packet-candidates.tsv`
source_queue: active rank-skip targets after Kingdom of Italy replacement packet

## Purpose

Identify a next high-value replacement packet class without reopening the
closed bridge-cluster review queue. The likely missing layer for several
German duchy-to-HRE rank skips is the Kingdom of Germany, but accepted fixture
promotion requires child-level evidence for each replacement edge.

## Source Context

- `src-wikidata-q175211`: Kingdom of Germany (`Q175211`), not yet accepted in
  fixtures.
- `src-wikipedia-kingdom-germany`: textual context for the Kingdom of Germany
  as the German kingdom associated with the Holy Roman Empire.
- `src-wikipedia-duchy-bavaria`: textual context for the Duchy of Bavaria as a
  stem duchy of East Francia / Kingdom of Germany before 962 and as an HRE
  state from 962.

## Candidate Counts

| Candidate Status | Rows |
|---|---:|
| needs_child_level_source | 3 |
| held_for_scope_split | 1 |

## Replacement Candidates

| Child | Current Parent | Candidate Parent | Proposed Span | Status |
|---|---|---|---|---|
| Duchy of Bavaria | Holy Roman Empire | Kingdom of Germany | `962..1805` | needs child-level source |
| Bavaria-Munich | Holy Roman Empire | Kingdom of Germany | `1392..1505` | needs child-level source |
| Electorate of Bavaria | Holy Roman Empire | Kingdom of Germany | `1623..1805` | needs child-level source |

## Held Candidate

| Child | Current Parent | Candidate Parent | Proposed Span | Reason |
|---|---|---|---|---|
| Archduchy of Austria | Holy Roman Empire | Kingdom of Germany | `1358..1803` | needs a separate Austrian relation packet because existing accepted edges also cover Austrian Empire and Austria-Hungary replacement contexts |

## Boundary

This is not an accepted fixture packet. It does not add `title-q175211`, does
not supersede any direct HRE parentage, and does not import a Kingdom of
Germany hierarchy edge. Realm-level sources alone are not enough; each
replacement fact still needs child-level source custody.

## Next Review Step

Find child-level sources that explicitly tie the Bavarian title to the Kingdom
of Germany, not merely to the Holy Roman Empire. If source custody passes, the
accepted packet should:

- add `title-q175211` Kingdom of Germany title facts;
- add Kingdom of Germany parentage under the HRE for the accepted bounded span;
- add replacement parentage facts with `supersedes_fact_id` for each child;
- regenerate active rank-skip and graph reports and require zero temporal
  parent conflicts.
