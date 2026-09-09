# Independent verification A

Static: authorization, common/verifier instructions, assurance template, testing
strategy sections 9–10/18, package/protocol/source map, corrected report, final
source reviews, finding/gate records, manifest and collector/analyzer inspected.
Ran: independent bounded checks below. No simulation series, builds, source
changes or external actions performed by this verifier.

Role/session: `/root/terminal_verifier_a`; configured effort medium; effective
runtime setting UNOBSERVED. Assigned write scope is this artifact only.

## Exact identity and independently executed checks

P root `/tmp/openwepp-compact-cost-P-vKI6gb`; baseline A common cut2b, not J.
Reviewed patch SHA256
`d9d3203b635f352ddb8a3f3ed39c0e8c5b13ba7b58ad8606473d78d01ee1f720`;
runner `c2276fa901e5c0c77a574f390f1c399915122b6c6c87085c18e138b3d0d0c858`;
terminal bundle manifest
`4106705cde42448aa89c5971d39d76861f43c7003a95f98501cb92c2ce0bab7a`.

All commands below exited 0; read-only Python used primary `.venv/bin/python`
with `PYTHONDONTWRITEBYTECODE=1`. Outputs are retained in this verifier's session
tool records and summarized here; no extra artifact/log files were written.

- Independently hashed all ten current P source files against the manifest,
  plus the patch, runner, adjacent sidecar, LSE test executable and bundle
  manifest. All match. LSE test SHA256 is
  `f57e0205b3722f1c1b05fc97b1975ccae22ea154b02b3bb5eaacb6336adfe724`.
- Ran `.venv/bin/python tools/agents/evidence_bundle.py verify
  /tmp/openwepp-compact-cost-P-vKI6gb/final-custody-bundle`: PASS. This verifies
  selected blob custody, not an independent full A restoration or Nix rebuild.
- Ran `patch --dry-run --fuzz=0 -p1 -i <absolute combined-instrumentation.patch>`
  with cwd retained A: PASS. Separately parsed unified hunks in memory, required
  exact old-line matches against A and reconstructed each output. All ten
  reconstructed P SHA256 values exactly match the manifest/current files.
  No patch was applied to a filesystem tree.
- Ran `RUST_MIN_STACK=67108864 <final LSE test executable>
  compact_cost::tests --nocapture` with cwd P: **3 passed, 0 failed, 156 filtered**.
  These execute accounting/off neutrality, live/stale/non-LIFO/overflow handling,
  clock reversal and preserved synthetic errors. They do not execute observer-on
  rejected physical solves; that stronger claim is not made.
- Independently opened all 16 numbered final series-03 receipts and raw logs.
  Every child is valid/exit zero, every log hash matches, and each sole emitted
  mechanism record equals its receipt. Order is exactly four declared warmups
  and six balanced fresh off/on pairs. All argv equal the frozen CPU0 command;
  protocol retains no retry and the exact measured runner hash.
- Re-executed collector accounting/denominator/family checks on every record.
  Normalized protected identity and carrier/LSE counts match the independently
  retained A admission, not merely the P off arm. No F frame exception applies.
- Reconstructed final analysis using the retained analyzer and independently
  checked all output fields against `raw/analysis-02.json`: exact equality.
  Negative elapsed, zero denominator and wrong signed-difference mutations all
  reject. Separately computed the disjoint Assembly+Linear+InactiveLeafProbe
  union: median **2.579401252%**, range **2.572182289–2.584827895%**.

## Claim legitimacy and corrections

COST-A01/A02 and QA-COST-DOC-01 corrections are accepted: the 31.85% envelope
is mixed construction/physics with reusable cost UNMEASURED, and 1.268% bounds
only assembly/linear, not every possible coordinate-compaction effect. The
three-bucket union remains below the owner's 5% priority rule. No cubic saving,
global overhead subtraction, additive inclusive-parent double count, or claimed
same-state duplicate physics follows from these profiles. Each on run accounts
exactly; a sum of displayed medians is not called a reconciled run.

COST-A03 labels are now truthful. Inspected retained source-real rejection and
open Potential/Final nonmutation test logs, each 1/1 PASS; compact observation
is inactive in those tests. Actual accepted Covered/GenericV3 off/on equivalence
is supported by the 16 real-run records checked above. Synthetic audit events
are not recast as authentic solver execution.

The 49.24% unresolved remainder and denied single sampling attempt are legitimate
characterization limits, not measurement-integrity failure. Ranking is qualified;
no optimization automatically follows. Prior J rejection and full-workspace
failures remain unchanged. Memory is not an admission gate in this authorization.
No production/science/external-authority suite mutation is included, so inherited
full-campaign reruns, derivative oracles and multi-OFE cases are not current
requirements. This is applicability, not retrospective deferral.

## Focused final reconciliation and verdict

VER-A01 CLOSED: reread the added runner/orchestrator lint disposition and both
retained `runner-clippy*.log` records. Initial warnings-denied runner lint failed
on the overlength function; its build also reports the inherited dead field.
The bounded rerun completed through orchestrator and runner in 32.63 s with
`-D warnings` and exactly the declared `-A dead_code -A clippy::too_many_lines`.
This is scoped lint evidence, not an unqualified warnings-free workspace claim.
The first failure is retained; no source allow attribute, numerical change or
silently expanded historical-cleanup obligation was introduced. Inspected the
corresponding A function/source context when adjudicating the bounded exception.

The report now separately states measured full-process wall 4.982/4.983 s and
CPU 4.960/4.967 s off/on. These correctly round this verifier's independent raw
medians 4.9821775165/4.9834499585 and 4.9604565/4.967157 s, respectively.
No measurement or model threshold changed for the reporting correction.

Reread the populated final disposition: complete characterization is distinct
from production qualification. It retains exact-accounting evidence, the
qualified ranking, external recovery limits, no optimization and no push.
Final publication synchronization, the other independent verification and
scoped commit remain the executor's distinct terminal steps, not evidence
fabricated by this artifact.

Verdict: **PASS for verifier A's bounded source/custody, gate, 16-run integrity,
arithmetic and characterization-completeness verification.** No unresolved
finding remains in this assigned scope. Production remains HOLD; this is not
approval to implement the next experiment or to waive inherited failures.
