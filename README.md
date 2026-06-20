# DUCHY

DUCHY is a historical-title timeline lab for game design. It tracks counties,
duchies, kingdoms, and empires across years so strategy and world-building
projects can ask CK-style questions about title rank, continuity, control, and
legitimacy without copying a commercial game's data or rules.

The first foundation is a Rust model for title snapshots:

- title ranks from county through empire,
- year-bounded existence and control spans,
- de jure parentage separate from de facto holder/control,
- continuity events such as creation, conquest, inheritance, partition, and
  extinction,
- validation that title hierarchies and spans are internally coherent.

## First Command

```powershell
cargo test --quiet
cargo run --quiet
cargo run --bin duchy-import -- status
cargo run --bin duchy-import -- manifest data/staging/example.manifest
cargo run --bin duchy-import -- source-stubs data/staging/example.manifest data/staging/generated.sources
cargo run --bin duchy-import -- rejected-report data/staging/example.manifest data/staging/rejected.md
cargo run --bin duchy-import -- active-manifest data/staging/example.manifest data/staging/active.manifest
cargo run --bin duchy-import -- archive-manifest data/staging/example.manifest data/staging/archive.manifest
cargo run --bin duchy-import -- manifest-report data/staging/example.manifest data/staging/manifest-report.md
cargo run --bin duchy-import -- duplicate-url-report data/staging/example.manifest data/staging/duplicate-urls.md
cargo run --bin duchy-import -- manifest-tsv data/staging/example.manifest data/staging/manifest.tsv
cargo run --bin duchy-import -- manifest-from-tsv data/staging/manifest.tsv data/staging/from-tsv.manifest
cargo run --bin duchy-import -- shard-manifest data/staging/example.manifest data/staging/shards 2
```

## Product Shape

DUCHY starts as a data/model repo, not a map renderer. The near-term product is
an inspectable timeline contract that can support historical strategy fixtures,
fictional realm generators, and design comparison packets.

The first vertical slice uses tiny hand-authored fixtures. Real historical data
requires source custody, citation policy, and rights review before import.

## Governance And VTRACE

- `.roles/` defines review authority for timeline semantics, territorial
  lineage, source custody, game-system boundaries, and query interfaces.
- `docs/vtrace/` defines the lineage-query mission, CONOPS, requirements,
  interfaces, trace matrix, verification, validation, and work packages.
- `docs/vtrace/source-custody/` defines the gate for real historical sources.
  The accepted fixture catalog now contains 319 reviewed Wikidata structured
  claim sources.

The first accepted capability target is the ability to ask which higher title
contained an area in a year, how an area moved between duchies over a date
range, and which continuity events explain a title lineage.

The current source-custody slice validates metadata-only source records, review
decisions, the source-backed fact gate, the first minimal real title facts, and
materialization of those facts into a `Title`. It can also answer a traced
source-backed title-path query for the reviewed mini-catalog, including bounded
Q20135 -> Q43287, Q158445 -> Q43287, Q27306 -> Q43287, and Q153015 -> Q43287
parentage paths for 1871-1918. The first real facts now live in
`fixtures/first-real.facts`, and contested fact groups are blocked from normal
materialization until reviewed. Reviewed real source metadata lives in
`fixtures/first-real.sources`. The German Confederation bridge packet adds
Q151624, Q150981, Q154195, and Q159631, giving the reviewed mini-catalog
1815-1866, 1867-1870, and 1871-1918 parentage spans for selected German
kingdoms and grand duchies. The Rust crate treats those fixtures as canonical
for reviewed historical imports; source-backed historical IDs, names, and spans
must not be duplicated as Rust literals.
The remaining clean title-identity queue promotion extends the accepted fixture
set to 319 reviewed sources. After follow-on German, Austrian, and Holy Roman
Empire parentage packets, the accepted fact catalog contains 1089 reviewed
facts. The title promotion adds title identity, rank, and existence facts only;
parentage remains a separate review step.
The 500-source candidate queue is now closed for unsupported rows: 167
relation-heavy, non-title, free-city, administrative-region, and otherwise
unsupported candidates are archived as `scope_deferred` rejections, leaving only
the fourteen date-problem title candidates active.
Those fourteen date-problem title candidates are also closed: thirteen are
`quality_blocked` because the structured Wikidata date gate could not provide a
complete span, and one is `date_conflict` because the extracted span was
reversed. The 500-source candidate queue now has no active pending rows.
The second German parentage packet adds 23 reviewed parentage spans for
additional accepted German Confederation, North German Confederation, and German
Empire-era titles. The first Austrian parentage packet adds 15 reviewed spans
for Austrian Empire and Austria-Hungary-era titles. The first Holy Roman Empire
parentage packet adds 70 reviewed pre-1807 spans, raising reviewed real
parentage coverage to 132 facts.

Batch candidate imports go through `data/staging/` and must pass dry-run
promotion before accepted fixture rows are appended:

```powershell
cargo run --bin duchy-promote -- --dry-run fixtures/first-real.sources fixtures/first-real.facts data/staging/example.sources data/staging/example.facts
```

Reviewed batches can then be appended with `duchy-promote --apply`, which
rewrites accepted fixture files only after the same validation path passes.
Add `--report path.md` after the mode to emit a promotion review report.

## Relationship To Games Design

- BANISH can consume DUCHY-style political timelines when settlement pressure
  needs feudal control, law, tribute, or war context.
- QUEST can use title continuity for dynastic tabletop campaign scaffolds.
- TIGRIS can use title maps as board-state scenarios.
- COURT/RACKET/MUDDLE are future experience surfaces only after DUCHY has a
  stable timeline contract.

## Non-Goals

- No commercial-game data, mechanics, or clone behavior.
- No claim that the seed fixtures are authoritative history.
- No renderer or grand-strategy engine in the foundation wave.
- No raw source corpus import before source custody and citation policy exist.
- No shared framework changes until at least two repos need the same primitive.

## License

[MIT](LICENSE) - copyright 2026 Gio Della-Libera.
