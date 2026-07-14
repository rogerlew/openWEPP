# Terminal Review A

Verdict: `PASS`

Evidence class: **Ran + Static**. Review target:
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

This is Review A's independent disposition of the frozen source, canonical
contract amendments, contract-derived regressions, required-suite bindings,
and terminal release/reconstruction evidence. It is not a substitute for the
separately required Review B or terminal verifiers.

## Findings

| Severity | Finding | Disposition |
| --- | --- | --- |
| Critical | None. | `accepted: no correction needed` |
| High | None. | `accepted: no correction needed` |
| Medium | None. | `accepted: no correction needed` |
| Low | None. | `accepted: no correction needed` |
| Note | Three touched production files exceed the 2,000-line warning threshold; none reaches the 3,000-line closure block. | `accepted: nonblocking governance disposition below` |

No current-scope finding is deferred or left for follow-up as a condition of
this review PASS.

## Contract And Correction Audit

- `SC-SED-001` revisions 54/55 precede and narrowly authorize the two erosion
  corrections: pinned `profil.for` terminal-station normalization, and a
  dimensionless `1e-15` class floor followed by renormalization to authoritative
  `ldbot` before caps. The latter is an explicit bounded correction of a
  trace-load baseline defect, not surrogate erosion physics; total routed mass,
  the no-floor path, typed nonnegative publication guards, and downstream
  closures remain intact.
- `SC-PLANT-001` revision 20 restores pinned `grow.for` cap-before-increment
  ordering. Exact-zero `rtmmax` selects the saturated branch and cannot reach
  division; negative/non-finite inputs remain typed failures.
- `SC-PERC-001` revision 30 and `SC-INFILE-SOIL-001` revision 0.1.12 bind exact
  zero restrictive conductivity as an active impermeable boundary. The source
  preserves restriction and returns zero effective conductivity; it does not
  disable the layer or leak through a fallback. Existing `INV-PERC-017` already
  requires every positive hourly ingress, so replacing an epsilon no-op with
  exact zero removes an unauthorized loss without relaxing closure guards.
- `SC-SNOWFREEZE-001` revision 117 binds pinned `frostn`/`frwatc(0)` egress and
  single-owner thaw projection. The one-condition production correction retains
  the original layer basis until R4W applies the authoritative projection and
  liquid scalar together; the nonmaterial stale-clear path and its typed
  rebalancing failures remain present.
- `SC-GWBASEFLOW-001` revision 0.1.2 adds observability only. The run summary
  retains existing `S0`, `SN`, `QbN`, and `QsN` recurrence operands, the runner
  serializes them as optional provenance, and disabled output remains absent.
  Daily recurrence, HBP/pass schema, routing, and export behavior are unchanged.
- The package's preimplementation gate contains the contract-first red evidence
  for each production correction. Static inspection found no provisional,
  proxy, empirical stand-in, wrapper, compatibility detour, shadow cutover,
  silent default, production `unwrap`/`expect`, new `unsafe`, or broad erased
  production error introduced by the reviewed diffs.

## Authority, Evasion, And Security

**Ran:** `bash tools/release/check_authority_suite_antievasion.sh` passed during
this review. **Ran:**
`cargo nextest run --test auth11_required_suite_obligation_guards_contract`
passed 3/3 with zero skipped.

Static inspection confirms that all five restored active required targets are
explicitly registered in `Cargo.toml`; AUTH11 generically scans every active,
required, hard-fail registry row rather than naming only the restored cases.
The provenance-only fixture correction changes metadata, not fixture bytes,
and AUTH06 checks the exact Git object plus fixture/lock/provenance hashes. No
tolerance, cohort CSV, watchlist, fixture payload, required-lane status, or
failure classification was loosened. The release script's workspace gate was
strengthened from shared-process `cargo test` to full-profile nextest isolation.

The terminal full run reports three configured ignored tests, but the reviewed
diff adds no `#[ignore]` marker and uses no skip/exclude flag. The material
ignored H2637 consumer was run explicitly and passed 1/1 at the frozen source;
there is therefore no skip-based closure claim.

No secret, shell-interpolation production path, serialization relaxation, or
fail-open error conversion appears in the reviewed source changes.

## Red/Green And Terminal Evidence

The focused evidence is mechanism-specific and changes from expected red to
green: EROD16 `100 -> 0`; growth/ingress/restrictive `100 -> 0`; thaw-clear
`100 -> 0`; enrichment floor `100 -> 0`; and real H2637 missing recurrence
operand `100 -> 0`. The final H2637 reconstruction closes both groundwater
identities within `4.26e-11 m3`, while the independently decoded P61/P102,
snow/frost, serial/parallel publication, and W11B real-consumer evidence closes
without producer-only or alias-based acceptance.

The exact frozen-source release exits zero: full workspace 1,960/1,960; deny,
fixture provenance, all required authority lanes, release binary/sidecar/lint;
main stability 1,166/1,166; watchlist 19/19. The independent restarted domain
lanes also pass, including erosion 368/368, frost 320/320, runner 214/214, and
watershed 129/129. Earlier candidate and first-restart results are not used for
the terminal claim.

## Line-Count Governance

- `erosion_continuity.rs`: 2,573 lines (`WARN`). The reviewed geometry change
  is 16 narrow lines in the existing Wave-1 owner; decomposition during the
  frozen terminal correction would add validation risk. Split intent: extract
  slope/profile derivation and its tests from the continuity owner in a
  behavior-preserving maintenance package.
- `subsurface.rs`: 2,094 lines (`WARN`). The reviewed change is 11 lines at the
  percolation validation/ingress/lower-boundary seam. Split intent: extract
  percolation validation and restrictive-boundary helpers without changing
  equations or guard semantics.
- `laned_active.rs`: 2,022 lines (`WARN`). The added fields preserve one run
  summary owner and its recorder. Split intent: extract run-summary/provenance
  accounting and its tests after release closure.

All other touched Rust files are below 2,000 lines; no touched file is 3,000+
or needs a generated/fixture exemption. These warnings are explicitly
dispositioned and do not block the frozen correction.

## Review Disposition

`PASS`: the frozen source is contract-first, fail-closed, independently
reconstructable, bound to the real consumers and required authority suites,
and free of a current-scope correction finding.
