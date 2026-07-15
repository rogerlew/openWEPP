# ASSURE-04A Review A

Status: PASS after two remediation cycles

Evidence class: Static + Ran

Reviewer: independent coding-agent reviewer A (Hilbert)

## Initial Finding Set

1. **High — accepted claim meanings were collapsed.** The source declared
   only `GW-P03`, `GW-P05`, and `GW-P06` while the manuscript asserted all
   nine accepted ASSURE-02 claims. In particular, prior-day timing (`GW-P02`)
   and active-router exclusion (`GW-P07`) had been absorbed into other IDs.
   Stable claims must retain their accepted meaning and all nine bindings.
2. **High — the article citation was materially wrong.** DOI
   `10.13031/2013.42691` identifies Srivastava, Dobre, Wu, Elliot, Bruner, Dun,
   Brooks, and Miller (2013), “Modifying WEPP to improve streamflow simulation
   in a Pacific Northwest watershed,” *Transactions of the ASABE* 56(2),
   603–611. The initial source named a different article.
3. **High — accountable authorship and agent provenance were absent.** The
   fixture needed typed authorship, human report-lead and scientific-approver
   accountability, agent-assistance provenance, conclusions, and reader-facing
   source metadata. Unassigned human roles and incomplete historical agent
   configuration must block review entry; coding-agent review must not be
   represented as external scientific peer review.
4. **Medium — JSON Schema companions were too permissive.** IDs, semantic
   versions, cardinality, paths, uniqueness, and kind/lifecycle conditionals
   were not meaningfully executable. The package needed either real Draft
   2020-12 execution or an explicit structural-only limitation.

## Initial Evidence

- Focused assurance tests: 22/22 passed on the initial tree.
- Scoped documentation checks: passed.
- Protected ASSURE-03 byte identities: passed.
- Disposition: **HOLD** pending all four remediations.

## First Re-review

Reviewer A reran focused Nextest (24/24, run
`e6e3d15f-cb68-4d71-ab36-d583069b4dca`), named/all CLI validation,
documentation checks, protected hashes, and roadmap-boundary inspection.
Citation correctness and accountable authorship/agent disclosure were closed,
and all nine claim meanings were distinct. Two blockers remained:

1. **High — exact `GW-P06` and `GW-P07` evidence lineage was absent.** The
   source did not bind the accepted positive consumer-path proof
   (`708a5d…`) or active-router negative proof (`fed22f…`) declared by the
   ASSURE-02 claim matrix.
2. **Medium — schema/Rust lexical parity was incomplete.** Rust admitted
   leading-zero semantic-version components and leading-punctuation IDs that
   the executable companions rejected.

Disposition: **HOLD** pending exact proof dependencies, claim/method/supplement
bindings, aligned lexical admission, regression tests, and another review.

## Terminal Re-review

Disposition: **PASS**. Reviewer A confirmed both exact proof identities and
their complete dependency/claim/method/supplement/agent/fixture routes, as well
as Rust/schema lexical parity and the two loader regression vectors. All nine
accepted claim meanings and limits, the corrected citation, accountable
authorship, incomplete-agent-provenance blocker, DRAFT lifecycle, and
zero-public boundary remained intact.

Focused Nextest passed 24/24 (run
`5012d3f8-7d58-4ea5-9703-4f7e18a81db0`); real all-source validation,
documentation checks, `git diff --check`, and protected hashes passed. No new
actionable finding was reported.
