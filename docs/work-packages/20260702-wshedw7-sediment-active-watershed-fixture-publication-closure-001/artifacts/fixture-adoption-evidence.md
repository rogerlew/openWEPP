# Fixture Adoption Evidence

Status: `executed-hold`

Evidence mode: `Ran:` fixture/source probes.

No W7 acceptance fixture was adopted.

Reason: all current committed full watershed fixtures and inspected local
sediment-active source substrates fail the W7 acceptance requirement for
production-generated nonzero sediment response. Adopting a zero-only fixture
would repeat the W6 residual-risk gap, and manually editing HBP/pass sediment
values is explicitly prohibited.

The closest source substrate,
`/wc1/runs/in/insensible-aliquot/wepp`, is not directly a full watershed fixture
because it lacks the public watershed topology inputs required by
`openwepp-cli-watershed`. Constructing a new watershed wrapper around it before
fixing hillslope sediment production would still publish zero sediment from the
current openWEPP passes.

Hold-lift requirement: first close the production hillslope sediment emission
blocker, then rerun W7 fixture adoption from committed inputs.
