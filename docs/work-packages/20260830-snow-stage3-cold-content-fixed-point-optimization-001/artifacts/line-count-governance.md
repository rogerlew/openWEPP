# Line-count governance

Status: `WARN — compliant follow-up recorded`

Evidence mode: `Static + Ran`

Touching the pre-existing 3,252-line `open_snow.rs` required a mechanical
split. The unchanged
`execute_precomputed_terminal_accepted_endpoint` implementation moved into
`open_snow_terminal_accepted_endpoint.rs` through `include!`; its pre-move and
semantic method-body digest is
`97aec7cad748caac7a2b3c6fbf2c1023074495f6b4ce233c95893bb9bd10bdd5`
for both baseline lines 342-869 and include lines 1-528. The prior
`6da236...` value included a separator blank line and is not used as the
semantic-body identity.

Review-correction counts are 2,721 lines for `open_snow.rs` and 529 lines for the new
include file. Formatting, compilation, focused tests, integration contracts,
and the canonical one-day test pass after the split. The authority impact map
contains an exact critical SnowEnergy binding for the new file.

`open_snow.rs` remains above the 2,000-line WARN threshold but below the
3,000-line mandatory-split threshold. It still coordinates the coupled covered
carrier, fixed-point loop, physical ledgers, and test includes; splitting those
borrow-coupled responsibilities during this numerical correction would expand
semantic risk. Follow-up owner: openWEPP SnowEnergy maintainer. On the next
authorized touch, extract the covered fixed-point loop into a dedicated include
or module before adding new behavior. The 529-line terminal endpoint include
needs no further split.
