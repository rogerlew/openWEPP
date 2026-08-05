# Review Finding Disposition

Status: complete / dual review pass

Evidence mode: Static + Ran

| Finding | Disposition | Remediation |
|---|---|---|
| Solid ledger omitted refrozen credit and full precip/vapor boundaries. | accepted | Added full solid identity with solid precipitation, deposition, sublimation, melt, and refreeze. |
| Energy ledger hid refreeze latent heat. | accepted | Added exact `Q_refreeze=L_f*m_refrozen` and the normative energy identity. |
| Liquid ledger double-counted initial retained store. | accepted | Defined external liquid separately and retained change as end minus start. |
| Vapor/melt mutation order was inconsistent. | accepted | Bound pinned precipitation -> melt/refreeze -> vapor -> compaction -> runoff order across contract and artifacts. |
| Melt and sublimation could overdraw the same ice. | accepted | Added exact signed vapor split, sublimation reservation, joint availability equation, and branch/test/verifier guards. |
| Available-ice saturation left energy fate undefined. | accepted | Added `Q_unallocated_after_exhaustion`, zero requirement, and terminal physical-recipient/next-state hard hold. |
| Supersession inventory was too narrow. | accepted | Made `INV-SNOWFREEZE-093` globally prospective only as to CoE target ownership and classified older active clauses/addenda. |
| Target input/output, alias, unit, and branch surfaces were incomplete. | accepted | Added explicit state, output, symbol, unit, branch, obligation, gap, and vector coverage. |
| Static test/verifier did not bind exact equations and holds. | accepted | Added exact equation, supersession, terminal-hold, joint-bound, and decision-hold checks. |
| Fusion constant was `333500` instead of pinned `333600 J kg^-1`. | accepted self-review | Corrected constant and source anchors; static test binds `333600`. |

No finding was rejected or deferred. Focused revalidation passes 47/47
authority checks, 2/2 verifier tests, and 11/11 contract tests.
