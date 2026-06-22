# Ireland UKGBI Parentage 01 Review

This packet promotes the United Kingdom of Great Britain and Ireland as a
bounded composite crown title and adds endpoint parentage from the Kingdom of
Ireland.

## Accepted Title

| Title | Rank | Span | Sources |
|---|---|---|---|
| United Kingdom of Great Britain and Ireland | Crown | 1801..1922 | src-wikidata-q174193, src-britannica-act-of-union-1801 |

## Accepted Parentage

| Child | Parent | Span | Sources |
|---|---|---|---|
| Kingdom of Ireland | United Kingdom of Great Britain and Ireland | 1801..1801 | src-wikidata-q215530, src-wikidata-q174193, src-britannica-act-of-union-1801 |

## Evidence

- The accepted Kingdom of Ireland title facts end in `1801`.
- The new UKGBI title is modeled as a composite `Crown` because the reviewed
  source describes a legislative union of Great Britain and Ireland under the
  United Kingdom of Great Britain and Ireland name.
- Britannica identifies the Act of Union as taking effect on January 1, 1801 and
  uniting Great Britain and Ireland under the name United Kingdom of Great
  Britain and Ireland.
- Wikidata Q174193 provides the bounded historical-state identity used for the
  title span.

## Boundary

- Great Britain is not imported as a child in this packet because DUCHY currently
  models Great Britain as a `Crown`, and accepted parentage validation rejects
  same-rank `Crown -> Crown` facts. That relation needs successor/replacement
  semantics rather than hierarchy parentage.
- No parliamentary mechanics, Catholic emancipation, representation, churches,
  army, flag, monarchy, empire, dominions, Irish Free State, Northern Ireland,
  modern United Kingdom identity, or post-1922/1927 continuity claims are
  promoted.
