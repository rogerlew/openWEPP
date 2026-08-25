# Coordinated successor numerical/DAE review

Evidence mode: `Static`

Independence: reviewer verified all six frozen manifest hashes, read no other
review output, ran no tests and made no edits.

Recommendation: `HOLD`.

| ID | Severity | Finding | Required correction |
|---|---:|---|---|
| `TBTV20-NUM-001` | Critical | Raw BE/CN separation is dominated by the lower-order BE defect and does not by itself establish or bound the installed CN local error. No estimator scaling/bound, acceptance inequality, controller/safety rule or stiff/DAE order justification exists. | Define and prove a reliable/effective installed-CN estimator across the admitted domain, including acceptance/controller semantics and independent evidence. |
| `TBTV20-NUM-002` | Critical | The promised complete seven-owner residual is incomplete. Snow/soil storage is named, but vegetation storage, hydrology and BGC use opaque deterministic transitions without component equations or BE/CN order semantics; `P_deterministic` is unreconstructable. | Define every participating typed storage/residual/projection and show both arms have consistent order without differentiating owner bytes. |
| `TBTV20-NUM-003` | Critical | Required evidence is absent. The 1.875-second component receipts remain unavailable and the new tests prove text presence only, not order, estimator effectivity, conservation, floor behavior or real-carrier convergence. | Produce the prescribed analytical, manufactured, conservation, floor and real-fixture evidence before implementation authorization. |
| `TBTV20-NUM-004` | Major | Lexicographic root ordering does not define exhaustive root discovery, equivalence of close roots, iteration map, step norm, isolation completeness or iteration bound. | Specify a reproducible nonlinear/root algorithm and typed completeness limits. |
| `TBTV20-NUM-005` | Major | CN order/consistency across algebraic and complementarity transitions is asserted but not established; event-containing support semantics remain incomplete. | Define DAE regularity/index and active-set/event consistency or retain typed unsupported cases. |

Positive findings retained: prescribed interval totals are separated from
endpoint rates; anti-requadrature is explicit; the `600 ms` floor, opaque owner
bytes, typed unsupported posture and rollback remain intact.
