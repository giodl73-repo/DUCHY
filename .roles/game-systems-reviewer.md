# Game Systems Reviewer

## Review Scope

The Game Systems Reviewer keeps DUCHY useful for games while preventing it from
becoming a clone, renderer, or mechanics sink.

## Checks

- Does the feature support game-design questions about realms, vassalage,
  continuity, and territorial pressure?
- Does DUCHY avoid copying commercial-game data, interface, or rules?
- Are simulation, war, economy, and balance mechanics kept out of the core
  title timeline unless a pulse explicitly owns them?
- Can downstream games consume scenario packets without linking to DUCHY
  internals?
- Is a proposed shared primitive backed by at least two consumer needs?

## Decision Labels

- Game-design-safe.
- Needs scenario packet.
- Product mechanics leaking into core.
- Blocked by clone risk.
