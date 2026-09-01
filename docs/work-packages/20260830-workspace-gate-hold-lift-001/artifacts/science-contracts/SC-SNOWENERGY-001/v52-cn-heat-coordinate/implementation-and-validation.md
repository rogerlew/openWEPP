# V52 CN heat coordinate implementation and validation

Evidence mode: `Static + Ran`

Status: `IMPLEMENTED; DUAL REVIEW APPROVED; CANONICAL QUALIFICATION PENDING`

## Implemented map

- Each covered lane now contributes ordered coordinates `(W, H, rho, Q)`,
  followed by ordered soil `(E, T)` coordinates.
- `PrivateTrial` consumes coordinate `Q` exactly once through the existing
  unpublished CN operand. Stage 3 receives snow `+Q`; the paired soil credit is
  `-Q`.
- Receipt-stabilization probes and replay continue to consume the supplied
  sealed receipt heat unchanged. Coordinate `Q` neither overwrites nor reseals
  authentic input.
- Every charged posture reconstructs physical endpoint `Q_out` from the same
  Stage 3 and soil candidate and evaluates `R_Q = Q - Q_out` with the unchanged
  lane energy tolerance.
- Residual shape, finite checks, merit, branch-coordinate indexing, exact
  replay comparison, and V46 complete-step budget preflight cover the enlarged
  `4L + 2S` system. The shared maximum remains 96.

## Validation run

Ran:

- `nix develop -c cargo check -p openwepp-hillslope-orchestrator --all-targets`
  — PASS.
- `nix develop -c cargo test -p openwepp-hillslope-orchestrator v52_cn_heat_coordinate --no-fail-fast -- --nocapture`
  — PASS, 5/5.
- `nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v52_ --no-capture`
  — PASS, 2/2, run `c67381fb-e296-4574-8092-f6e177a9e598`.
- retained `v35_` unit selection — PASS, 6/6.
- retained `v45_` unit selection — PASS, 10/10.
- retained `v46_` unit selection — PASS, 8/8.
- retained `v51_` unit selection — PASS, 5/5.

Independent terminal reruns:

- retained V35/V45/V46/V51/V52 — PASS, 34/34, run
  `0923c7c4-8609-43ef-acb3-eea155a95a62`;
- V52 contract/source — PASS, 2/2, run
  `3bd323e6-64a6-4cb4-9b42-72cd0b4b43cb`.

The mandatory Rust correctness reviewer returned `APPROVE` with no remaining
findings. The mandatory Rust QA reviewer returned `APPROVE` with no blocking
findings after independently verifying the focused, retained, source,
all-target, formatting, diff-hygiene, line-count, and diagnostic gates.

The contract-first expected-red source run was
`39e19040-d521-42e7-a2fe-508fbca80378`; it failed only for the then-absent
production Q seams and five required behavior names.

## Behavior evidence

The V52 vectors cover the retained r134 heat pair, exact zero and nonzero
`R_Q`, two-lane coordinate/residual order and reorder/cardinality poisons,
omission, nonfinite, sign, signed-zero, support-duration double-application,
and foreign static-receipt geometry poisons. They also cover exact `4L+2S`
dimensions 6 and 10 at complete-step exact-fit and one-short budget boundaries,
exact shared-budget probe/replay at used 96, one-ULP endpoint/receipt-digest
replay substitution, unchanged authentic receipt bytes, rollback,
no-publication, and absence from the persisted-restart schema.

## Line count and diagnostics

- `open_snow.rs`: 2886 lines (`WARN`, remains below the 3000-line hard limit;
  existing split intent remains active).
- `phase_consistent_coupled_solve.rs`: 2523 lines (`WARN`, below 3000;
  follow-on solver decomposition remains appropriate after active numerical
  closure).
- `open_snow_convergence_tests.rs`: 2978 lines, with V52 behavior in the
  dedicated 418-line `open_snow_convergence_v52_tests.rs` split.
- No `DFF_V52`, R134, or persistent microstepping diagnostic seam remains.

Canonical one-day qualification and its accepted/rejected counts, widths,
runtime, limiting reasons, and ledger closure are not claimed here and remain
pending the independent review disposition.
