# One-Child Bridge Cluster Review Batch 01

## Intent

Scale the bridge-cluster review process by clearing the remaining one-child
clusters that can be resolved as same-parent sibling false positives.

## Decisions

All five rows reviewed in this batch are marked
`same_parent_sibling_false_positive`. None are marked `ready_for_packet`.

| Candidate Parent | Current Parent | Child | Decision |
|---|---|---|---|
| Archduchy of Austria | Austria-Hungary | Gorizia and Gradisca | false positive |
| Archduchy of Austria | Austrian Empire | Gorizia and Gradisca | false positive |
| Duchy of Ferrara | Papal States | Comtat Venaissin | false positive |
| Eastern Hungarian Kingdom | Ottoman Empire | Prince-Bishopric of Montenegro | false positive |
| Kingdom of Imereti | Russian Empire | Grand Duchy of Finland | false positive |

## Evidence

- Gorizia and Gradisca is described as a Habsburg crown land in the Austrian
  Littoral, a crown land of the Austrian Empire until 1867, and a
  Cisleithanian crown land of Austria-Hungary; this supports a sibling
  crown-land/littoral reading rather than parentage under the Archduchy of
  Austria.
  <https://en.wikipedia.org/wiki/Princely_County_of_Gorizia_and_Gradisca>
- Austrian Littoral is described as comprising Trieste, Istria, and Gorizia and
  Gradisca as an Austria-Hungary subdivision.
  <https://en.wikipedia.org/wiki/Austrian_Littoral>
- Comtat Venaissin is described as part of the Papal States and a papal enclave
  in France; Papal States sources separately describe Ferrara being reclaimed
  into direct papal rule. That supports separate papal-territory siblings, not
  Comtat under Ferrara.
  <https://en.wikipedia.org/wiki/Comtat_Venaissin>
  <https://www.britannica.com/place/Comtat-Venaissin>
  <https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/papacy-and-papal-states>
- Prince-Bishopric of Montenegro is described as an Ottoman vassal state from
  1516 to 1696 and then independent, not as a dependency of the Eastern
  Hungarian Kingdom.
  <https://en.wikipedia.org/wiki/Prince-Bishopric_of_Montenegro>
- Finland is described in nineteenth-century context as a separate autonomous
  Grand Duchy within or joined to the Russian Empire; Kingdom of Imereti is a
  separate Caucasian title already directly parented to the Russian Empire in
  DUCHY. This supports same-current-parent false positive, not Finnish
  parentage under Imereti.
  <https://scholarlypublishingcollective.org/uip/jfs/article/25/2/143/351680/Introduction-Finland-in-Imperial-Context>
  <https://www.swedishfinnhistoricalsociety.org/2020/05/24/grand-duchy-of-finland-1809-1917/>

## Boundary

This batch changes review metadata only. It does not promote sources, facts,
titles, parentage spans, de facto relations, successor semantics, or
territorial geometry.
