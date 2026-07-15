# ASSURE-03 Verification A — Scientific Communication And Preservation

Verification class: internal coding-agent verification; not external
scientific peer review

Evidence class: Static + Ran

Terminal verdict: **PASS**

The verifier made no repository content edits.

## Initial Re-verification

All substantive preservation and reader-facing checks passed:

- the renewed r4 identity reproduced exactly: 67 status rows, 40 present paths,
  full-index diff SHA-256 `d60c66de…`, and ordered content manifest SHA-256
  `a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8`;
- all 51 frozen v1 blobs recovered with exact identities and action agreement:
  27 removed and 24 preserved-or-revised;
- all nine retained SNOTEL, comparator, and activation evidence hashes matched;
- retained evidence independently reported 5 sites, 70,999 source rows, 13,590
  paired rows, 159,986 selector rows, 53,711 precipitation rows, and maximum
  conservation residual `5.551115123125783e-17 m`;
- public assurance contained exactly one regular file, matching the source
  template, and the export check passed with zero reports, zero documents, and
  `vendoring_authorized=false`;
- active-v1 searches found migration inventory, prohibited examples/lifecycle
  explanation, and fail-closed negative guards, but no active public v1 route;
- all 112 local documentation links resolved; and
- focused Nextest run `77af082d-c775-414f-931a-277c90813bff` passed the 2/2
  relevant preservation and public-science tests.

Direct Markdown lint/validation passed 27 changed documents and five evidence
artifacts. `git diff --check` passed.

### VA-003 — Stale artifact descriptions

Initial disposition: **HOLD**.

The migration inventory still said the integration target contained ten tests,
while the target contained 13. The `VA-002` disposition also described the
historical r3 wording state as current, although renewed r4 now supplied the
terminal source identity.

The parent accepted the finding and changed only package evidence: the inventory
now says 13 tests; `VA-002` says r4 superseded and closed the r3 ambiguity; and
the r3 bounded audit is explicitly chronology only.

## Terminal Recheck

Verdict: **PASS**.

The verifier confirmed both corrected descriptions and the `VA-003` disposition
are accurate. Direct Markdown lint/validation and `git diff --check` passed. The
non-artifact r4 source manifest remained exactly
`a5355bf907b0e23efae776ba3c464404e21e6c2f669d4ff1a39a00008c6248b8`.
No finding remains.

## Final Closure-Tree Recheck

Verdict: **PASS**; the prior Verification A result remains applicable.

The verifier independently reproduced current status SHA-256 `f59a166d…`,
40-path SHA-256 `13cf0251…`, full-diff SHA-256 `56e1dbed…`, and content
manifest SHA-256
`1178d3b69e83a4e612bedb94f038dce0dd7d18074c251bcb27775d870d407bd7`.
The r4 comparison is exactly 39 common paths, 31 unchanged rows, eight changed
closure-document rows, one removed active prompt, and one added archived
prompt. Reversing only `88,903` to `86,652` reproduces the r4 prompt hash.

The seven Core files total exactly 88,903 bytes. Package, plan, catalog,
disposition, handoff, and prospective main-roadmap states agree; ASSURE-04A is
next but requires a new operator instruction. All 18 protected paths match r4,
all four retired paths remain absent, and public assurance/science bytes did
not change. All 112 local links, Markdown lint/validation, retained CRAP
checksums, and `git diff --check` passed. No finding and no verifier edit.

## Claim Boundary

Stability remains **NOT RUN**. This verification does not support release
qualification, a retained candidate, a production release, WEPPcloud vendoring,
or external scientific peer review.
