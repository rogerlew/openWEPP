# ASSURE-04C Independent Review A

Evidence class: Static review plus cited ran evidence

## Initial disposition

HOLD. Reviewer A identified six blocking findings:

1. Staging creation preceded protected-target authorization, and path-based
   writes left a symlink/time-of-check-time-of-use boundary.
2. Replacement could occur before final source revalidation, while restoration
   or cleanup errors could be discarded after a new target became visible.
3. Manuscript wording changed beyond the package's mechanical-migration
   authority.
4. Inline values did not carry units, so authored suffixes could drift.
5. Manifest metadata could introduce Markdown or links.
6. Evidence proved bespoke Markdown/link handling but not the actual usersum
   renderer.

The reviewer also requested correction of the two-day figure caption and an
updated line-count disposition.

## Renewal disposition

HOLD. The reviewer confirmed the earlier quantity/unit, metadata, alternative,
portable-contract, caption, renderer, protected-surface, and write-set findings
closed, but found five remaining issues:

1. Generated-link checking returned to ambient canonical paths after opening a
   staging capability.
2. Preparation cleanup, post-install rollback evidence, and typed recovery
   errors were incomplete.
3. The H2637/reproducibility prose diff remained broader than necessary.
4. Bare/autolink URLs could bypass typed-link admission.
5. Audience lines were absent and the displayed narrative version did not use
   the usersum `X.Y` convention.

All five findings were accepted and remediated.

## Second renewal disposition

HOLD on governance evidence only. All requested technical items passed. The
reviewer found that `error.rs` was missing from the declared write set and saw
an older focused-count row while the parent was renewing the artifact. The
package now records the write-set exception/amendment, governs the 72-line
file, and carries the current 9/9 and 31/31 counts. Final confirmation is
pending.

## Final Phase 4 disposition

PASS. The reviewer confirmed the `error.rs` amendment/ordering exception,
72-line governance row, and current 9/9 assembly plus 31/31 focused evidence.
No Phase 4 blocker remains; heavy/full gates were not rerun by the reviewer.
