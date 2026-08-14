# Final Land-Surface Science And Numerics Review

Evidence class: `Static + Ran` independent exact-worktree review.

Verdict: **NO-GO / FAIL**.

## Exact reviewed bytes

- `SC-LANDSURFACEENERGY-001.md`:
  `7917d02a66c4ecefa70cf566b1057df9b990deae95a2daef512efa877855f5fc`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`;
- LSE definition:
  `51280eecaebd02fcde9675fc6bb48f2b3afa9e251be57246762be62cb92e484a`;
- V8 definition:
  `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- top-level calculator:
  `00197518134ea1d6ce351ac7ff42a3cb1e89888bf349c33de7ea672b103ce9ce`;
- joint canopy-ground core:
  `a5bbad2e80a75864ddd69ecb35b08a11c59949fe7de8a663c6abae9e5ce3a87c`;
- committed vectors:
  `b462d1710ebb991e19ac5936cdda543e1d0a5d8c39cc84afca85c22479c571b7`.

The six reviewed schema hashes are:

- configuration `6499b98cc1e25f1379bc0ad6052a7536e20c4bfbb9335f9ba5c8de191ae2f009`;
- coupled transaction `e9cea670e733cc97c84458ecc10b68d62aaba39caaf31535371ea45d66ddff2c`;
- diagnostics `df45462246d5d77b1151eb008c6262528fc27ebd58eb244047c97705bcf31853`;
- forcing `f1fb785e9e582ae9e20eac4b5f44fa2b5f0651f8535d0972520dbfff3d926b55`;
- state `91243e4087fa2c4775cb3629fe14c64379def4977d3c54a72348ac56d5fa4ee8`;
- water protocol `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07`.

Ran: the calculator was executed independently with the repository `.venv`
and wrote `/tmp/lse-review-session.json`. It exited zero after approximately
86 seconds, produced SHA-256 `b462d171...`, and compared byte-identically with
the committed vector file. The four vendorable source hashes and restricted
CLM5 hash match the acquisition ledger. Regeneration reproducibility and
reference custody therefore pass; they do not cure the scientific defects
below.

## Material findings

### `A3-CRITICAL-001` — Post-ingress surface enthalpy closes the liquid subset, not the admitted surface node

The canonical equation at `SC-LANDSURFACEENERGY-001.md:747` is
`U_s,1 = U_pre + sum(Q_retained_ingress)`. The oracle instead forms a liquid-
only mixture from pre-ingress liquid and all ingress, then assigns the dry body
that mixture temperature (`reference_calculator.py:498-512`). Its reported
energy residual at lines 527-528 excludes both beginning and ending dry-body
enthalpy.

The frozen fixture makes the defect non-roundoff and directly reproducible:

```text
U_pre                                      = 472699.0323082857 J m-2 tile
retained-ingress enthalpy                  =  46928.22645978135 J m-2 tile
canonical U_pre + retained ingress         = 519627.25876806705 J m-2 tile
oracle ending_surface_enthalpy             = 433927.02598390443 J m-2 tile
oracle minus canonical                     = -85700.23278416262 J m-2 tile
reported liquid-only energy residual       = -5.820766091346741e-11 J m-2 tile
```

Consequently the nominal complete owner vector can pass while creating or
destroying surface-node energy. The correction must apply the contract's
post-ingress operator to authoritative `U_pre`, preserve exact parcel
partition identity, and independently close the complete surface plus crossing
control volume. This is a continuing failure of `A2-CRITICAL-002` and the
energy portion of `A-CRITICAL-005`/`A-HIGH-009`.

### `A3-CRITICAL-002` — The mandatory joint vectors do not execute the admitted V7/V8 shortwave boundary problem

The contract requires the unchanged V7 two-stream column with surface-class
VIS/NIR albedo as the lower boundary (`SC-LANDSURFACEENERGY-001.md:458-461`).
The core instead imports already aggregated V3 canopy absorption, hard-codes
four terminal ground values (`reference_joint_canopy_core.py:86-104`), and
only multiplies those terminal values by ground absorptivity at lines 299-307.
The multirank builder further creates lower-rank canopy absorption by scalar
multiplication (`reference_joint_canopy_core.py:633-642`) rather than executing
the digest-bound two-stream column.

Ran: at the same complete joint trial, changing ground VIS/NIR albedo from
`0.12/0.24` to `0.62/0.74` changed absorbed ground shortwave from `107.52` to
`41.52 W m-2`, but all four canopy energy residuals remained exactly
byte-identical:

```text
[114.83123121035658, 40.37569081957115,
 146.893417392884, 48.42714627914787]
```

That cannot demonstrate the lower-boundary reflection returning through the
overlying column once. The separately emitted inherited V3 fixture is not
joined to the same primitive surface albedo, topology, terminal flux, and
component absorption operands. The correction must execute one consistent
V7/V8 shortwave solve inside each mandatory coupled scenario and bind its
source-resolved outputs into the joint residual. This keeps
`A2-CRITICAL-001` and `A-CRITICAL-001` open.

### `A3-HIGH-003` — Multirank numerical acceptance and active-set differentiation are not the frozen algorithm

The multirank helper accepts on one untyped scalar `last_step <= 1e-8`
(`reference_joint_canopy_core.py:798-815`). The contract instead requires
simultaneous limits of `1e-8 K`, `1e-12 kg kg-1`, `1e-7 mm`, and `1e-10` for
beta (`SC-LANDSURFACEENERGY-001.md:792-797`). The result exposes only
`step_norm`, so conformance cannot be reconstructed by variable family.

In addition, finite-difference evaluations pass frozen root/ground branches to
the occupancy-local calls at lines 726-733, but the separately reconstructed
surface and soil block at lines 769-772 calls `_raw_residual` without
`frozen_branches`. Thus the ground cap branch can switch inside the centered
Jacobian even though the canonical generalized derivative freezes the accepted
active branch. The exact multirank potential/final vectors therefore do not
establish the admitted numerical method. This leaves `A-CRITICAL-007` and the
multirank part of `A2-CRITICAL-001` open.

### `A3-HIGH-004` — Physical poison and numerical-failure evidence is not semantic or diagnostically complete

`executed_component_poisons()` describes independent operand validation, but
its validator is only `digest(attempted) != expected[kind]`
(`reference_calculator.py:1149-1164`). Every changed record is rejected by
byte inequality, without reconstructing radiation, latent, ground-heat, or
advection physics. The poison result therefore cannot distinguish the named
wrong equation from any unrelated byte edit.

The natural singular, backtracking-limit, and iteration-limit branches are a
useful improvement, but their frozen records omit the contract-required model,
configuration, state, transaction, OFE, tile, occupancy, pass, and solve
identity. Their diagnostic payloads contain only residual/counter/pivot/cap
subsets, contrary to `SC-LANDSURFACEENERGY-001.md:811-815` and the normative
diagnostics schema. This keeps `A2-HIGH-003` and the diagnostics portion of
`A-CRITICAL-007` open.

## Initial and fresh finding reassessment

| Finding | Final-review assessment |
|---|---|
| `A-CRITICAL-001` | open: full coupled shortwave and owner transaction evidence is not exact |
| `A-CRITICAL-002` | corrected in canonical equations and accepted covered resistance calculations |
| `A-CRITICAL-003` | arbitrary-layer bare-soil/CN equations execute, but complete acceptance remains blocked by the findings above |
| `A-CRITICAL-004` | reciprocal source-resolved longwave equations and unequal component temperatures execute correctly |
| `A-CRITICAL-005` | open: post-ingress surface enthalpy violates the authoritative control volume |
| `A-CRITICAL-006` | beginning-store-only single authorization and fixed-cap rebuild are demonstrated |
| `A-CRITICAL-007` | open: multirank step/active-set rules and failure diagnostics are noncanonical |
| `A-HIGH-008` | corrected: constitutive and receipt signs are explicit |
| `A-HIGH-009` | open: liquid-only ingress closure omits dry-body enthalpy |
| `A-HIGH-010` | six positive instances validate against the checksum-bound schemas |
| `A-HIGH-011` | corrected: v3/V8 precedence is explicit |
| `A-MEDIUM-012` | corrected: R-157/R-158 metadata, locators, rights, and hashes are exact |
| `A2-CRITICAL-001` | open: manual terminal/canopy shortwave and noncanonical multirank solve remain |
| `A2-CRITICAL-002` | open: post-ingress candidates carry a false complete-node energy closure |
| `A2-HIGH-003` | open: hash inequality substitutes for semantic poison validation; failure payloads are incomplete |
| `A2-HIGH-004` | corrected: `C_dry=W=U_s=0`, algebraic current-trial CN endpoints, and alternate-warm-start invariance execute |
| `A2-HIGH-005` | corrected: all six positive DTO instances validate against their exact schemas |

No finding is rejected, deferred, or recommended for follow-up. I recommend
accepting all four new findings and correcting them inside this authority
package before another fresh review. The current bytes are reproducible but
are not eligible for `COMPLETE / snow-free land-surface-energy implementation
authority released`.
