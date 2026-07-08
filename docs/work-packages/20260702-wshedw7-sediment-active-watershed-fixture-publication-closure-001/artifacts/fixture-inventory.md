# Fixture Inventory

Status: `passed`

Evidence mode: `Static:` fixture inventory plus `Ran:` probes.

| Fixture | Full fixture | Sediment result | Disposition |
| --- | --- | --- | --- |
| `tests/fixtures/watershed/carnivorous-adobo/` | yes, 32 hillslopes | Historical W7 probe stayed zero-only | Not W7R acceptance fixture. |
| `tests/fixtures/watershed/onshore-xenophobia/` | yes, 1305 hillslopes | Generated all HBP files, then failed WS10 channel dispatch with `WKERNEL-WS10-CHANNEL-E-003`; H102 in this fixture is one-OFE and not the W7DC01 p102 producer | Not W7R acceptance fixture. |
| `tests/fixtures/erosion_multi_ofe_p102/` | hillslope only | Current-main release producer proof passed with nonzero `tdet`, `tdep`, and all `sedcon_*` | Used as source substrate for generated W7R watershed fixture. |
| `tests/fixtures/watershed/p102-sediment-active/` | yes, one hillslope and one channel | Release `--jobs 1`/`--jobs 4` watershed runs passed with nonzero public `tdet`, `tdep`, and `sed_del` | W7R acceptance fixture. |

The accepted fixture is complete for its selected watershed topology. It is not
a subset or representative slice of a larger watershed, and it does not contain
generated HBP/pass outputs.
