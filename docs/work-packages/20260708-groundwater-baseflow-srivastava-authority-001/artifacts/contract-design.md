# Contract Design

Status: executed.

## Placement Decision

Execution created `SC-GWBASEFLOW-001` instead of amending an existing contract.
Rationale:

- `SC-INFILE-GWCOEFF-001` owns `gwcoeff.txt` parsing and malformed/absent
  sidecar behavior, not groundwater-reservoir process physics.
- `SC-SUBHYD-001` owns lateral subsurface export such as `latqcc`, which must
  remain separate from generated groundwater-reservoir baseflow.
- `SC-ROUTE-001`/`SC-INFILE-CHANINP-001` own channel routing and `chan.inp`
  `cbase`; those are consumers or separate branches, not the reservoir process.
- `SC-OFEROUTE-001` owns the Lane D active surface-router boundary. The new
  contract binds the non-surface export obligations without turning baseflow
  into a surface-router source.

## Accepted Contract Shape

`SC-GWBASEFLOW-001` defines:

- Srivastava linear-reservoir state and recurrence:
  `S_i = S_{i-1} + D_i - Qb_{i-1} - Qs_{i-1}`,
  `Qb_i = bfcoeff * S_i`, and `Qs_i = dscoeff * S_i`.
- Variables and units for `lr_bf`, `igwstrd`, `bfcoeff`, `dscoeff`,
  `bftharea`, `D_i`/`gwstrv2`, `S_i`/`gwstrv3`, `Qb_i`/`gwbfv`,
  `Qs_i`/`gwdsv`, `tmpgwbfv`, and `tmpgwdsv`.
- Daily timestep volume semantics for `D_i`, `Qb_i`, and `Qs_i`; channel
  flow-rate conversion is a downstream consumer operation.
- Explicit namespace separation from `cbase` and `latqcc`.
- Branch/guard behavior for disabled, enabled, malformed, out-of-domain, mixed
  Lane D authority, pass/HBP consumer, and publication anti-alias surfaces.
- Invariants `INV-GWBASEFLOW-001` through `INV-GWBASEFLOW-008`.
- Producer obligations `OBL-GWBASEFLOW-P-001` through `P-004` and consumer
  obligations `OBL-GWBASEFLOW-C-001` through `C-003`.
- Test-vector obligations `TV-GWBASEFLOW-001` through `TV-GWBASEFLOW-008`.
- Binding Exposure Index row `GWBASEFLOW-MT2A-AUTHORITY`.

## Unit And Registry Decision

Runtime boundary registry entries for groundwater storage/recharge/baseflow and
deep-seepage symbols do not exist yet. The contract records those as registry
gaps rather than pretending that unregistered symbols are already governed.
M-T2B must add registry entries or carry an explicit implementation hold before
claiming runtime/publication closure.

## Publication Decision

The legacy water-balance surface can print `Baseflow` as zero under `lr_bf=1`
because generated groundwater baseflow is carried through runoff/streamflow
surfaces. The contract therefore requires M-T2B/M-T3 to expose metadata that
distinguishes true zero, disabled groundwater process, missing authority, and
generated-but-carried legacy publication behavior.

## M-T2B Entry Conditions

M-T2B starts from canonical `SC-GWBASEFLOW-001` authority and must not reopen
the linear-vs-nonlinear authority decision unless the user explicitly asks for a
new nonlinear amendment. The first implementation work should be contract-derived
tests for disabled branch behavior, one-hillslope recurrence, coefficient domain
guards, pass/HBP export, namespace separation, and Lane D active-ledger closure.

Coefficient tests must match parser/baseline authority: finite non-negative
values are parser-domain valid, while recurrence guards fail closed on non-finite
state or outflow-over-storage behavior rather than imposing an uncited parser
upper bound.
