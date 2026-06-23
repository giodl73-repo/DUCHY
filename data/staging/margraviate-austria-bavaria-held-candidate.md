# Margraviate of Austria Bavaria Held Candidate

## Candidate

- child: Margraviate of Austria (`title-q283627`)
- candidate parent: Duchy of Bavaria (`title-q47261`)
- current active parent: Holy Roman Empire (`title-q12548`)
- candidate span: `976..1156`
- source lead: https://en.wikipedia.org/wiki/Margraviate_of_Austria

## Source Reading

The Margraviate of Austria article identifies the margraviate as within the
Duchy of Bavaria and the Holy Roman Empire, and describes Austria becoming an
independent duchy in 1156.

## Disposition

Held for rank semantics or relation modeling.

DUCHY currently materializes Margraviate of Austria as `Duchy`. Replacing the
direct HRE parentage with Duchy of Bavaria would make a `Duchy`-rank title
parent another `Duchy`-rank title, which fails temporal parent-rank validation.

## Next Step

Resolve whether margraviates should receive a lower immediate-rank mapping in
the DUCHY model, or whether "within Duchy of Bavaria" should be captured as a
separate relation rather than active parentage.
