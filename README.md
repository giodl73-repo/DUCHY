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
```

## Product Shape

DUCHY starts as a data/model repo, not a map renderer. The near-term product is
an inspectable timeline contract that can support historical strategy fixtures,
fictional realm generators, and design comparison packets.

The first vertical slice uses tiny hand-authored fixtures. Real historical data
requires source custody, citation policy, and rights review before import.

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
