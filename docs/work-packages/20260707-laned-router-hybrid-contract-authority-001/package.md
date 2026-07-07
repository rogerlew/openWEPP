# LANED-HYB-SC — Authoritative specification for the hybrid stepping subsystem

Status: **REVIEWED-FIXED-PENDING-VERIFICATION** (2026-07-07, operator-directed:
"do we have an authoritative specification for the hybrid solver? maybe that
is warranted now so we can continue with the hold lift and optimization").
`SC-OFEROUTE-002` rev 1 is authored at the canonical path (`status: draft`),
`SC-OFEROUTE-001` rev 32 re-points its hybrid rows, the index row is
registered, and all doc/SC lints pass (`artifacts/gate-results.md`). The
dual-agent review EXECUTED (A: NO-GO, 1 High + 1 Medium; B:
GO-WITH-AMENDMENTS, 3 Medium + 2 Low) and ALL findings are fixed —
`artifacts/science-contracts/SC-OFEROUTE-002/disposition.md` (A-H1 warm-seed
finite+positive restored; A-M1 lever provenance relabeled; B-M1 C-L1
exception threaded; B-M2 transactionality wording narrowed; B-M3 maturity
vocabulary; B-L1 real test names; B-L2 registry date). Post-fix lints green.
Open gate: the verification pass (`verification_agent_{a,b}.md`) confirming
the fixes, at which point `draft` lifts to `approved` (the disposition's
recorded act).

## Problem

The hybrid implicit-explicit stepping subsystem has no single normative
authority. Its rules are fragmented across four `SC-OFEROUTE-001` changelog
entries (revs 28-31), one Branch-and-Guard row, one BEI row, and the
package-local `i0-scheme-design.md` (evidence, not authority). The forthcoming
work — the GAP hold-lift (wave-quiet switching predicate; Case-4 hybrid
ladder fails `22.8/15.5/10.2 %` vs the 5 % rung tolerance) and continued
solve-cost optimization — would otherwise amend changelogs indefinitely and
force every future worker (and every Codex delegation) to reverse-engineer
the scheme from history.

## Objective

1. Author `SC-OFEROUTE-002` — the Hybrid Implicit-Explicit Kinematic-Wave
   Stepping contract — as a CONSOLIDATION of the existing authority
   (SC-OFEROUTE-001 revs 28-31 + the T3 design/evidence chain), per
   `docs/specifications/science-contract-spec.md` and the kernel-process
   contract profile. Maturity: `experimental` (matching the selector's
   posture); status: `draft` until dual review.
2. Amend `SC-OFEROUTE-001` (rev 32): re-point its hybrid rows (Branch/Guard,
   Test-Vector, BEI) to the new contract as thin pointers; revs 28-31 remain
   the historical record.
3. Register the contract in the canonical index.
4. Consolidation rule (review surface): NO new normative content beyond
   (a) stable invariant/gap IDs, (b) the recorded Case-4 hold and its two
   recorded design levers. Anything else new is a review finding.

## Acceptance

- `SC-OFEROUTE-002` present at the canonical path with every required
  section (spec §"Required Section Order" + kernel profile schema).
- `SC-OFEROUTE-001` rev 32 pointer amendment; no dangling references.
- Index row registered.
- Gates: `markdown-doc lint` on touched docs; `tools/check_sc_binding_exposure.py`
  on BOTH contracts; `tools/release/check_sc_unit_compliance.sh` on both.
  No code changes — cargo gates not in scope (nothing touched).
- Dual-agent contract review per the authoring procedure is the NAMED next
  step (artifact set scaffolded; review prompt authored for dispatch);
  `status: draft` until it passes.

## Included / Excluded

Included: the two contract files, index registration, package artifacts,
review prompt. Excluded: any code change, the hold-lift design itself (the
wave-quiet predicate is a GAP in the new contract, not this package's scope),
tolerance ratification, Tier-1/Tier-2.
