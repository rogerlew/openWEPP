# Review Disposition

Status: `EXECUTED-ALL-ACCEPTED-AND-FIXED`

Evidence mode: `Static + Ran`

Review A returned `HOLD` with five High findings and one Medium finding.
Review B returned `PASS` with no formal findings and one live anti-alias
suggestion. The owner accepts every item; none is deferred.

| Finding | Disposition | Correction and evidence |
|---|---|---|
| `A-H1` KW storage used MC boundary mean | Accepted, fixed | `SC-ROUTE-001` v56; KW retains terminal spatial state and averages independently reconstructed Manning area over all `nseg+1` nodes. The 101-segment vector rejects the boundary mean by more than `1 m3`. |
| `A-H2` only `ntchr-1` updates | Accepted, fixed | time-zero state separated; loop executes `0..ntchr` projected slots as pinned `it=1..ntchr`; cross-day first-terminal test fails the seed alias, while final-slot-only KW vectors require terminal `ntchr` response and zero earlier terminals at both 3,600/600 seconds. |
| `A-H3` tautological water acceptance | Accepted, fixed | independent rectangular-Manning bisection reconstructs the 101-segment spatial storage, checks two-sided magnitude/ratio, and rejects boundary-mean/flux candidates; a second real channel vector separately reconstructs fresh `sinit`, terminal `sfnl`, `chvol`, public inflow, and storage. |
| `A-H4` no admitted production MC vector | Accepted, fixed | matched parser/frame-to-channel static and dynamic 60-second routes each publish 1,440 finite slots, convex unit-sum coefficients and passive peaks; coefficients and hydrographs diverge under dynamic refresh. |
| `A-H5` channel/impoundment/channel double count | Accepted, fixed | `SC-SYSTEM-001` v90 narrows supported sediment ancestry and selector traversal; explicit red/green vector changes outlet set `{1,2}` to `{2}` and excludes pre-impoundment sediment. |
| `A-M1` `chan.inp` matrix/metadata contradiction | Accepted, fixed | `SC-INFILE-CHANINP-001` v0.1.4 splits positive four-record and zero-count three-record cases and refreshes metadata; parser behavior is unchanged. |
| Review B live CLI-consumer anti-alias suggestion | Accepted, fixed | zero-count 600-second CLI output matches an otherwise identical positive-count 600-second control and differs from the 60-second default candidate. |

The full-profile p102 regression was also accepted. Its historical incidental
600-second MC configuration is contract-inadmissible; the committed wrapper now
selects the valid KW interval lane, with the exact line change and unchanged
p102 hillslope/HBP substrate documented in its README and checksum manifest.
The protected test still runs the documented committed fixture directly;
owner and delegated focused runs pass 1/1 without weakening the MC guard.

The same reviewers independently reverified the stabilized
`c7e0d2ab4b688356fe269acc279f3aa4cd0e62a03b494b3e8f890b43d7debbf6`
implementation/test/contract fingerprint. Both verification artifacts recommend
`PASS`; every accepted finding is closed, with no new High, Medium, or Low
finding and no deferred or follow-up review item.
