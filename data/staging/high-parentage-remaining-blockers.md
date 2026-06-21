# Remaining High-Priority Parentage Blockers

Reviewed on 2026-06-21 after high-priority parentage batches 01-09.

## Summary

- High-priority parentage gaps remaining: 3.
- These rows are not promoted because the currently accepted fixture catalog
  lacks a reviewed higher-rank parent that overlaps the title span, or the
  source-backed relation is not a de jure parentage claim under the current
  model.

## Blockers

| Title | Span | Blocker |
|---|---:|---|
| Principality of Albania (Q187035) | 1914-1925 | The accepted Italian protectorate/Italian Empire Albania parentage context begins outside this title's accepted span; World War I occupation/protectorate sources are partial and do not provide a whole-title parentage claim under an accepted higher-rank parent. |
| Hordaland (Q50625) | 1919-2019 | The accepted Norway fixture records do not include an overlapping modern Kingdom of Norway parent spanning 1919-2019; existing Norway title records end in 1397 or 1814. |
| Principality of Montenegro (Q779011) | 1852-1910 | Sources support de facto independence and 1878 recognition rather than a clean Ottoman parentage span; importing Ottoman parentage would overstate the relation without contested/de facto status modeling. |

## Next Work

- Add reviewed modern Norway parent title coverage before importing Hordaland.
- Add contested/de facto/de jure relation semantics before importing Montenegro
  or partial Albania occupation/protectorate claims.
- Keep these rows in the generated gap queue until those modeling gaps are
  resolved.
