# Conservation Reconstruction

Status: `PARTIAL-PRE-FIX-BINDING`

Evidence class: **Ran + Static** arithmetic test binding; incomplete campaign
closure.

Independent readers in the named tests reconstruct:

- p61: pass-Parquet `sedcon_1..5 * runvol` against `tdet`, and HBP hourly
  runoff/sediment sums against pass `runvol` and `tdet-tdep`;
- p102: HBP `sum(S_h)` against chain detachment minus deposition, `sum(V_h)`
  against outlet `runvol`, and changed OFE-2 texture against exit composition;
- W7R: HBP hourly sediment against export, public `tdet/tdep` against HBP, and
  routed public `sed_del` against EBE while rejecting the raw-export alias;
- W11B: serialized external runoff against terminal EBE volume plus residual
  channel storage, zero published channel balance, and terminal sediment yield
  against external hourly sediment for two distinct timing shapes;
- snow forcing: row-wise forcing precipitation and snow fraction against audit
  totals, with separate WAT extraction of SWE and physical snow depth.

Commands 06, 07, 10, 11, and 13 passed the listed p61, p102, W7R, and W11B
identities; command 09 passed the broad frost profile. The run did not archive
complete independent H2637 groundwater or selected snow/frost numeric operand
tables and output hashes. This is therefore partial pre-fix evidence, not a
terminal conservation claim, and must not carry into the required restart.
