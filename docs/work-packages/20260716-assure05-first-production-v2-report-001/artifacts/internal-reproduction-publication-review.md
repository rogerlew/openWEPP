# ASSURE-05 Internal Reproduction And Publication Review

Status: **HOLD — reproduction/publication findings require remediation before
review-root freeze**

Reviewed: `2026-07-16 UTC`

Evidence class: Static + Ran

This is an internal coding-agent review of reproducibility, evidence
provenance, deterministic assembly, research-object usability, accessibility,
and the publication boundary. It is not scientific peer review, independent
human reproduction approval, publication approval, assurance-steward approval,
or release-owner authorization. H2637 was not rerun and no report source,
result, test, public surface, or approval record was edited.

## Verdict

The current quantitative objects are internally coherent and reproduced from
the still-present accepted scratch acquisition. The source validates, seeded
staging is deterministic, rendered report surfaces are usable, stale-run
evidence is truthfully excluded, artifact hashes are current, and publication
is correctly held at `DRAFT` pending accountable humans.

The review nevertheless returns HOLD because the staged H2637 reproduction
route is not self-contained: it requires raw files that the staged research
objects do not retain, while the report-specific test reconstructs only from
the already-normalized result. Two additional portability/provenance defects
should be corrected before the subject root is frozen.

## Findings

### F1 — High: staged H2637 reproduction cannot execute from retained objects

Evidence:

- `reproduce_groundwater_report.py h2637` requires a produced `manifest.json`,
  HBP, and pass-Parquet file. It verifies the HBP and Parquet hashes against the
  manifest before reading manifest operands.
- `assure05-production-evidence.json` explicitly records
  `raw_binary_outputs_committed: false`. It retains filenames, hashes, normalized
  manifest operands, executable identities, and run IDs, but not the three
  inputs required by the procedure.
- The deterministic staged tree contains the normalized evidence JSON,
  `h2637-ledger.json`, and the procedure, but no manifest, HBP, or pass-Parquet
  object. The supplement nevertheless tells a reader to run H2637 mode with
  “retained” paths.
- The package execution record names `/tmp/laned_shadow_h2637_active_on_473038`.
  Those files are present now, but `/tmp` is transient and is neither a staged
  research object nor durable release evidence.
- `retained_h2637_values_close_both_groundwater_and_surface_ledgers` reads all
  operands and expected reconstructions from `h2637-ledger.json`. It checks
  arithmetic self-consistency but does not execute the independent procedure
  against produced evidence or prove the normalized operands came from the
  hashed manifest.
- This review executed the procedure against the present accepted scratch
  manifest/HBP/Parquet. The emitted JSON was semantically identical to
  `h2637-ledger.json`, and the three raw hashes matched the recorded
  `756e324e…`, `378a8c1d…`, and `915f3b99…` identities. The current values are
  therefore supported, but the reproduction path will disappear when scratch
  is removed.

Impact: a future reader can recheck arithmetic from normalized operands but
cannot replay the report's declared produced-file authentication or regenerate
the H2637 result from the published research-object set. This violates the
package requirement that material values reconstruct from retained objects and
that the report-specific contract execute the analysis procedure against those
objects.

Recommendation: retain the public-safe accepted manifest, HBP, and pass-Parquet
as digest-bound research objects and make the report test execute H2637 mode
against them, comparing its JSON semantically or byte-for-byte under an
explicit canonicalization contract. If raw publication is intentionally
prohibited, provide a strict, independently generated normalized-manifest
object that the procedure can consume, test the normalization from raw inputs
before discard, and narrow the prose so it does not claim raw checksum replay
from the public package.

### F2 — Medium: documented deterministic-build command omits a required input

Evidence:

- The package and supplement present the assurance `build` command with an
  unrelated absolute staging root, but do not state that the root must first
  contain `usersum/hillslope-hydrology-and-sediment-physics.md`.
- An independent build into a new empty root failed with
  `rendered local link is unresolved` for that narrative.
- `prepared_stage` in `assurance_v2_assembly_contract.rs` silently creates the
  directory and copies the narrative before every successful build.
- After this review applied the same prerequisite to two unrelated scratch
  roots, build/check passed and both complete staging trees compared
  byte-for-byte equal.

Impact: a reader following the documented command cannot reproduce the staged
report without inspecting test helper source. The builder itself remains
deterministic once its complete consumer root is supplied.

Recommendation: add the exact narrative-seeding command and source digest to
the supplement and package evidence, or provide a bounded helper that prepares
the complete staging consumer root. Keep the narrative as an explicit input;
do not weaken the resolved-link gate.

### F3 — Medium: the staged agent-assistance packet points to an unavailable prompt

Evidence:

- `agent-assistance-packet.json` names
  `prompts/archived/20260716-codex-execute-assure05_prompt.md` with SHA-256
  `5a740e7f…`.
- That archived file does not currently exist. The byte-identical prompt is
  still correctly active at `prompts/active/` with the stated hash.
- The staged research-object set includes the packet but not the prompt, so a
  report reader cannot inspect the “exact prompt” from the staged report even
  after the repository path is eventually archived.
- The packet appropriately declares `provenance_complete: false`; the defect is
  the inaccessible path/object, not a hidden bitwise prose-regeneration claim.

Impact: the source validates because the packet bytes are digest-bound, but the
packet's principal audit target is neither current at its stated path nor
portable with the report.

Recommendation: after the required byte-preserving prompt archival, regenerate
and rebind the packet. Either declare the prompt itself as a public-safe staged
research object or state explicitly that prompt inspection is repository-only
and provide a current resolvable repository path. Renew manuscript,
supplement, packet, descriptor, and catalog hashes in the required dependency
order.

## Positive Evidence

### Reproduction and identities

- The standard-library analytical procedure reproduced
  `two-day-recurrence.json` with exact semantic JSON equality.
- H2637 replay from the accepted scratch files reproduced
  `h2637-ledger.json` with exact semantic JSON equality.
- Every manuscript, supplement, descriptor dependency, result, procedure,
  protocol, freeze, production-evidence, and agent-packet digest checked in
  this review matches its current descriptor/catalog binding.
- The rejected H2637 acquisition is clearly identified by run ID, marked
  `accepted_for_claims: false`, and excluded because its executable sidecar did
  not bind the frozen commit. The accepted run records an isolated target,
  source-bound sidecar, matching executable digest, produced-object hashes,
  and the frozen realization. No rejected-run value is silently substituted.

### Deterministic consumer and accessibility

- Assurance source validation passed with one selected production-domain
  report, zero public reports, lifecycle `DRAFT`, and `fixture_only=false`.
- With the required narrative input present, two unrelated builds and one
  exact check passed; complete staged trees were byte-identical.
- The staged tree contains the manuscript, supplement, two accessible SVG
  figures, build manifest, science contract, analysis procedure, inputs,
  results, protocol/freeze objects, and normalized evidence objects.
- The actual WEPPcloud `cmarkgfm` renderer processed staged `index.md` and
  `supplement.md` without unresolved directives or raw script content. Assembly
  contracts require SVG `role="img"`, title, description, pattern distinction,
  portable links, and exact research-object copies.
- The report-specific integration target passed 3/3 in nextest run
  `a63359ce-bd82-4d08-ad57-d46a8512597f`. Its H2637 test is supporting
  arithmetic evidence only for F1's lineage question.

### Human-approval boundary

The publication hold is correct and must remain in force:

- `report.yaml` is production-domain source but remains `lifecycle: DRAFT`;
- authorship names Codex only as an agent draft author, with human report lead
  and scientific approver null;
- the principal registry contains only `codex-agent-assure05`, kind `agent`,
  role `draft_author`;
- review is `DRAFT/not_started`, with null subject/finding/approval roots and
  no approvals;
- publication is `DRAFT`, release identity and transfer are null, and export
  and vendoring are false; and
- tracked `usersum/assurance` remains the one-file zero-report surface. Its
  README and the three protected catalog/template/export hashes equal the
  ASSURE-05 intake baseline.

Internal remediation of F1-F3 can make the draft review-ready. It cannot create
human competence, independence, approval, release transfer, or publication
authority. Unless those records are supplied by accountable people for the
later exact root, the truthful package disposition remains
`HOLD-HUMAN-APPROVAL`.

## Commands Renewed By This Review

| Command or check | Result |
| --- | --- |
| Analytical procedure against retained two-day input | PASS — semantic equality with retained result |
| H2637 procedure against present accepted scratch manifest/HBP/Parquet | PASS — semantic equality and all three recorded hashes matched |
| `cargo run --quiet -p openwepp-assurance -- validate --report linear-groundwater-reservoir-recurrence` | PASS |
| Build into a new empty staging root | EXPECTED FINDING — failed on missing model narrative input |
| Build/check and repeated build into two correctly seeded unrelated roots | PASS — complete byte equality |
| `cargo nextest run --profile quick --test assurance_v2_groundwater_report_contract` | PASS — 3/3 |
| Actual `cmarkgfm` render of staged report and supplement | PASS |
| Protected public diff, inventory, and four intake hashes | PASS — unchanged zero-report state |

## Required Disposition

F1 is closure-blocking for the package's reproduction/publication acceptance.
F2 and F3 are accepted-quality findings that should be resolved before the
subject root and review charge are frozen. After remediation, renew source
validation, the report-specific test, two-root deterministic build/check,
research-object link/accessibility checks, all affected content hashes, and
this independent reproduction/publication review. Do not publish or populate
human approval records as part of the remediation.

## Remediation Verification — 2026-07-16 UTC

Status: **PASS — F1–F3 resolved for internal reproduction/publication review;
human publication hold remains in force**

Evidence class: Static + Ran

This appended verdict supersedes the earlier remediation disposition for
F1–F3. It does not supersede the report's `DRAFT` lifecycle, authorize review
entry or publication, or represent human, scientific, external-peer,
assurance-steward, or release-owner approval.

### Finding Disposition

| Finding | Disposition | Renewed evidence |
| --- | --- | --- |
| F1 — retained H2637 reproduction | **CLOSED** | The accepted manifest, HBP, and pass-Parquet files are retained as report research objects with SHA-256 identities `756e324e…`, `378a8c1d…`, and `915f3b99…`. The repository procedure and its staged copy both authenticated these inputs and reproduced `h2637-ledger.json` with exact semantic JSON equality. The report-specific contract now executes the procedure against these retained source objects. |
| F2 — unstated staging prerequisite | **CLOSED** | Supplement S7 now explicitly creates `$stage/usersum` and copies `usersum/hillslope-hydrology-and-sediment-physics.md` before build/check. This review followed those instructions in two unrelated fresh roots; both builds and checks passed, and `diff -qr` found no difference between the complete staging trees. |
| F3 — unavailable exact prompt | **CLOSED** | The packet and `GW-OBJECT-EXECUTION-PROMPT` now resolve to the archived prompt at SHA-256 `5a740e7f…`. The packet hashes to `752b48c1…`; both objects are current in `report.yaml`, and the catalog binds the current report SHA-256 `47d1e0be…`. Staging retained byte-identical copies of the prompt and packet. |

### Renewed Reproduction And Assembly Evidence

- The documented staging workflow passed in
  `/tmp/assure05-review-a.yD3to3` and
  `/tmp/assure05-review-b.4iwxYo`; build/check passed in each and the complete
  roots were byte-identical.
- The staged H2637 procedure reproduced the retained H2637 ledger with exact
  semantic JSON equality after authenticating the three staged raw objects.
- The staged analytical procedure reproduced the retained two-day result with
  exact semantic JSON equality.
- Source validation passed with one selected production-domain report, zero
  public reports, lifecycle `DRAFT`, and `fixture_only=false`.
- `cargo nextest run --profile quick --test
  assurance_v2_groundwater_report_contract` passed 3/3 in run
  `fa69f704-80ea-4133-9411-84ce4beceea2`.
- The staged prompt, packet, manifest, HBP, and pass-Parquet bytes matched their
  current source identities. The current report hash matched the catalog
  binding.

### Human Hold Reverification

The required human boundary remains fail-closed. `human_report_lead` and
`scientific_approver` are null; the registry contains only the Codex agent
draft author; review is `DRAFT/not_started` with no approvals; publication is
`DRAFT`; release and transfer identities are null; and export and vendoring
remain false. The tracked public surface still contains exactly
`usersum/assurance/README.md`, and that file plus the three protected
catalog/template/export objects retain all four intake hashes.

Final internal-agent verdict:
`INTERNAL-AGENT-REVIEW-CLEAR-FOR-REQUIRED-HUMAN-REVIEW`.

Publication disposition: `HOLD-HUMAN-APPROVAL`.
