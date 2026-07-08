# Sediment Fixture Inventory

Status: `passed`

Evidence mode: `Static:` fixture inventory plus `Ran:` current-main probes.

W7R supersedes the original W7 zero-sediment hold evidence.

Accepted fixture:

- `tests/fixtures/watershed/p102-sediment-active/`
- complete one-channel watershed fixture wrapping the real W7DC01 p102
  two-OFE sediment producer
- no generated HBP/pass outputs committed
- input manifest validates with `18` entries `OK`

Current-main producer proof:

- Source: `tests/fixtures/erosion_multi_ofe_p102/`
- `sum(tdet)=41531.85795763501`
- `sum(tdep)=29195.4647928195`
- all five `sedcon_*` sums nonzero

Rejected candidates retained for history:

- `tests/fixtures/watershed/carnivorous-adobo/`: zero-only sediment in the
  original W7 probe.
- `tests/fixtures/watershed/onshore-xenophobia/`: generated all HBP files under
  W7R but failed WS10 channel dispatch with `WKERNEL-WS10-CHANNEL-E-003`; not
  used as acceptance evidence.

Detailed current inventory is in `fixture-inventory.md`.
