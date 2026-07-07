# Disposition — SC-OFEROUTE-002 dual-agent review

Status: **EXECUTED-COMPLETE** (2026-07-07). Lane verdicts: Agent A **NO-GO**
(1 High, 1 Medium), Agent B **GO-WITH-AMENDMENTS** (3 Medium, 2 Low). Every
finding is ACCEPTED, FIXED, and VERIFIED in this disposition; the contract is
now `status: approved` / `maturity: active`. Evidence mode: **Ran** for all
re-gates.

## Agent A

**A-H1 (High) — warm-seed acceptance weakened (finite + positive dropped).
ACCEPTED, FIXED.** A genuine consolidation error: compressing the rev-31
rule to "branch-side only" lost two of the three acceptance conditions
(side-of-`Q_c` alone admits non-finite/zero/negative same-side candidates).
Restored "FINITE, POSITIVE, and on the evaluated branch's side, else cold
fallback" on all three surfaces the reviewer anchored: §Algorithm 3, the
Warm-seed validity Branch/Guard row, and INV-OFEHYB-003. Agent B's positive
checks confirm the CODE already enforces all three — the fix realigns the
contract with both provenance and implementation.

**A-M1 (Medium) — assessment-class levers overclaimed as "recorded".
ACCEPTED, FIXED (relabel, the reviewer's second option).** GAP-OFEHYB-001
now carries exactly ONE recorded design lever (the I0 §2 explicit
cool-down, with its provenance cite) and separately labels the spatial
wave-quiet predicate + the q-departure observation as NON-BINDING
ASSESSMENT CANDIDATES with their true provenance (this WP's authoring
session) and an explicit "NOT authority until a contract-first design
increment adopts one" marker. The revision-history "two recorded design
levers" phrasing is corrected to match. Rationale for keeping (not
trimming) the candidates: the gap register is the right place to hand the
hold-lift designer the known solution space, provided the authority class
is honest — which was the reviewer's actual objection.

## Agent B

**B-M1 (Medium) — exact-total claims lack the C-L1 exception. ACCEPTED,
FIXED.** The approved bounded all-dry/insufficient-gross drop is now
threaded through every binding exact-total surface the reviewer anchored:
required outputs (§State Surfaces), §Algorithm 5.5, INV-OFEHYB-006,
OBL-OFEHYB-P-001, and OBL-OFEHYB-C-001 (with the consumer-facing bound:
zero on any series with material gross). The contract is no longer stricter
than the retained implementation.

**B-M2 (Medium) — residual guard not transactional as worded. ACCEPTED,
FIXED (narrow the wording, the reviewer's second option).** §Algorithm 3
now states precisely: no exit path returns `Ok` with an unvalidated pair;
the low-level API mutates working buffers DURING the march; on typed
failure the buffers are UNDEFINED and must not be consumed — the production
path fails the routing window closed without publishing them (matching
Agent B's own no-publication-leak audit). OBL-OFEHYB-P-002 amended to the
same semantics. Staging the commit behind the guard is recorded in the
contract as a non-blocking hardening candidate (a code change belongs to a
future code package, not this docs package).

**B-M3 (Medium) — `maturity: experimental` outside vocabulary. ACCEPTED,
FIXED.** Front matter, body header, and registry row now use the sanctioned
`draft`; the EXPERIMENTAL subsystem posture is expressed where it belongs —
the body header note, INV-OFEHYB-008, and the BEI — not the lifecycle
field. (At approval the lifecycle becomes `approved`/`active` while the
subsystem posture stays experimental until INV-OFEHYB-008 lifts.)

**B-L1 (Low) — guard-map family labels. ACCEPTED, FIXED.** All guard-map
rows now cite the actual retained test function names the reviewer
enumerated (`implicit_step_ledger_is_exact_and_positive`,
`implicit_step_books_upstream_inflow_exactly`,
`steady_state_is_a_fixed_point_of_the_implicit_step`,
`dust_scale_steps_do_not_accumulate_a_material_leak`,
`low_jump_recovers_high_branch_root_and_never_commits_filippov`,
`branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work`,
`branch_warm_seed_acceptance_is_basin_locked`,
`hybrid_is_bit_identical_on_all_explicit_windows`,
`hybrid_rejects_non_integral_windows`,
`hybrid_rejects_cadence_that_does_not_partition_the_seam_hour`,
`absorb_deficit_exact_total_and_non_negative`,
`dispose_terminal_carry_material_deficit_fails_closed`,
`dispose_terminal_carry_subnoise_absorbs_backward_exactly`,
`dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`,
`bin_recorder_returns_material_terminal_deficit_exactly`, plus the
already-named Case-4 held vector).

**B-L2 (Low) — stale registry `last_reviewed` for SC-OFEROUTE-001.
ACCEPTED, FIXED.** Index row updated to `2026-07-07`, matching the rev-32
front matter.

## Post-fix re-gates (Ran)

- `markdown-doc lint` (both contracts + index + this package):
  `8 files validated, 0 errors, 0 warnings`.
- `tools/check_sc_binding_exposure.py` on SC-OFEROUTE-002: `PASS-DEFERRED`
  (4 rows, 4 follow-on — expected).
- `tools/release/check_sc_unit_compliance.sh` on SC-OFEROUTE-002: `PASS`.

## Verification closure

Per the authoring procedure, `verification_agent_a.md` /
`verification_agent_b.md` checked each fix against this disposition.
Agent A returned GO. Agent B returned NO-GO on one Low residue in B-L1: the
`INV-OFEHYB-006` guard-map row still used the
`rev30_deficit_carry_tests` shorthand. The row was amended to name the five
retained deficit-carry tests directly, and
`verification_agent_b_followup.md` returned GO. The recorded disposition act
is now executed: `SC-OFEROUTE-002` is lifted to `status: approved` /
`maturity: active` (front matter + body + registry row) and rev 2 is recorded
in the contract changelog.
