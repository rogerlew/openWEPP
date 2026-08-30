# Fixed-point optimization evidence

Status: `QUALIFIED INCREMENT`

Evidence mode: `Static + Ran` as labeled.

## Retained baseline

Ran: canonical one-day profiling baseline at commit `8352f6c92` completed in
`416.94 s` test-body wall time with 588 accepted supports, 320 rejected trials,
124 fixed-point cap failures, 140 scaled comparison rejections, and zero exact
discrete/event comparison rejections. The accepted widths were 139x60,
111x120, 320x180, 12x240, 1x300, 3x420, 1x900, and 1x1800 seconds. Maximum
mass and energy residuals were `3.55271367880050093e-15 kg m^-2` and
`1.39698386192321777e-9 J m^-2`; receipt reseal maxima were
`9.98625182546675205e-10 J m^-2` and `1.06297193269710988e-11 K`.

## Initial limiting trace

Ran: the retained five-parent trace shows 420-second and 180-second trials at
the phase-transition seam exhausting all 96 Picard iterations. At the cap,
LSE, Stage 3, soil, and complete-boundary comparisons are all false; the first
Stage-3 difference is `layer.mass_swe_m`. Adjacent 120-second trials converge
in 20--29 iterations and exact-floor trials converge in 5--42 iterations.
This rejects a receipt-identity or operating-system orchestration diagnosis for
the first cap seam and points to the coupled nonlinear transition map.

## Current execution

Ran: exact-head canonical one-day cap aggregation completed in `415.59 s` test
body (`417.37 s` external wall) with the retained 588/320 behavior and closure.
Of 124 cap failures, 94 are Picard/finalization failures whose first Stage-3
difference is `layer.cold_content_j_m2`; in those rows LSE, soil, and complete
boundaries are already converged. Another 28 Picard failures have all coupled
maps still moving, and two are receipt-replay failures. The comparison audit
attributes 95 of 140 scaled rejections to `layer.refrozen_liquid_m`.

Artifact: `/tmp/adaptive_microstep_amendment/one-day-cap-signatures-v20.log`,
sha256 `c300119fd27a8b418ff3f8be61b4121e89ad8d7c856e9162d683d6569387ee6b`.
External-time artifact sha256:
`6beb6e4e04dd617c3f6842801dc91e8492aec9462f715de2df49d4178d85044d`.

Static: `fixed_point.rs` declines Stage-3 under-relaxation when any per-layer
liquid water, cold content, or refrozen liquid changes exact zero posture. The
decline occurs before its existing fingerprint, persistent-domain, aggregate
lifecycle, and cumulative closure proofs. These three quantities are numeric
state under `TOL-SNOWENERGY-003`, not exact schema/event/topology predicates.
Contract v27 therefore admits only their continuous zero crossing inside the
already-authorized convex iterate; no production solver change has yet been
made at this evidence point.

## Continuous phase-axis correction, focused result

Ran: the contract-derived zero-crossing vector failed 0/1 on the old guard
after both endpoint candidates independently passed persistent-domain and
cumulative-closure validation. Log sha256:
`c10ed05979b7f26f6125f8082d1c1a050db51ef0bcb8021d47abc6797644d81e`.
After removing only the liquid-water/cold-content/refrozen-liquid zero-posture
refusal, the complete covered convergence-policy set passes 17/17 (nextest run
`8cd7867c-521b-498b-9289-1ef8eba2b970`).

Ran: the five-parent real fixture then reduced cap failures from 5 to 4. The
former 180-second all-map Picard cap now converges; the 420-second cap reaches
finalization with LSE, soil, and boundaries converged and only
`2.24015093408525e-6 J m^-2` cold-content separation, while a refined
240-second attempt ends at `1.7255188140552496e-6 J m^-2`. Receipt-replay
identity failure remains unchanged. Test-body time was `81.77 s`; compilation-
inclusive external wall was `112.67 s`. Log sha256:
`81b505fd3051bf8acbbad044645b196889c4abd01ce6912e5829b27f8d7257c5`.

Static: this is a positive but insufficient focused reduction. The canonical
day rerun will measure the global count change and aggregate the maximum
native-unit separation of every remaining cap before any narrowly bounded
convergence amendment is considered.

## Rejected canonical zero-crossing trial

Ran: the canonical day retained exactly 588 accepted / 320 rejected trials and
the baseline accepted-width histogram. Fixed-point cap failures increased from
124 to 188 and test-body time regressed from `416.94 s` to `432.62 s`.
Maximum closure remained unchanged (`3.55271367880050093e-15 kg m^-2` mass,
`1.39698386192321777e-9 J m^-2` energy); receipt reseal maxima also remained
within the unchanged bounds. Remaining cap deltas reached
`1.0866278898902237e-5 J m^-2` cold content, `6.532427607536988e-5 m` SWE,
and `2.1024457700136168e-5 K`, so a narrow tolerance change is not supported.

Disposition: `REJECTED`. The candidate production/test/contract-v27 edits were
removed and authority returned to v26 before selecting the distinct v27
classification below. Retained diagnostic artifact:
`/tmp/adaptive_microstep_amendment/one-day-zero-crossing-v23.log`, sha256
`c582ad17136c6d317436ef44129de878f3fa99700a636d7d8b0180f6aa133cea`;
external-time sha256
`9f9578c3bca6feeb4c9273cb759e91adc44dc8729d05a9aff68d3be8dbd77751`.

## Selected factorization-lineage correction

Static: direct/composed discrepancies in per-layer `refrozen_liquid_m`
reproduce trial-duration chronology rather than an independent physical
endpoint. The field remains canonical/persisted material history. Future
physics consumes represented ice, liquid, cold content, and cumulative
physical ledgers; committed publication independently reconstructs refreeze
from beginning/ending ice and admitted fluxes. `SC-SNOWENERGY-001@27` excludes
only `snow.lanes[*][1].layers[*].refrozen_liquid_m` from cross-factorization
error estimation. No tolerance changed and no diagnostic surface is persisted.

Ran: the contract-first vector failed on the prior implementation, then passed
with the exact-path classification. The comparison projection suite passes
16/16, including explicit physical-state and discrete anti-evasion poisons;
committed-publication focus passes 21/21. Pre-implementation log sha256:
`958468a6fe15581f7660ecd70ab0317adc69b818bd28bf3f9d21a61ade535737`;
post-implementation projection/publication log sha256 values:
`12083d8a441ad5d6728d18dacfd15e05fb384b7794eb8b0084bb5f382c20aa2a`
and `54b0785901ee380f5f68f144a7bb0db1515b56fe6053c36bb27027c962328dfb`.
The seven affected canonical-contract binaries pass 47/47 (nextest run
`9d496043-80bb-446d-bcd2-b90c33745e0f`), and affected all-target/all-feature
crate checks pass. Log sha256 values:
`a2b7706975aeb322d5324a3589e19f733ba5f3976e360575fd99cc606cd5e590`
and `9e12f6be2563e7d41e986f06b332d29900a78cce75f410958fa9133e061cb93c`.

Ran: the five-parent real fixture passed in `38.70 s` body time. It accepted
the stable 1,800-second supports previously rejected by the tracer and retained
five fail-closed caps: one receipt replay plus duplicated 420-second and
180-second physical-map failures. Remaining scaled rejections were physical
deposition/temperature coordinates. Log sha256:
`c69e67e1f45d4f054630460e39887c8fffa83eb0911e19b488ea80e7daa682cf`.

Ran: the canonical one-day fixture passed all downstream gates with 504
accepted supports and 227 rejected trials, reductions of 84 (14.29%) and 93
(29.06%) from the retained 588/320 baseline. Accepted widths were 49x60,
112x120, 323x180, 11x240, 2x360, 1x420, 3x900, and 3x1800 seconds; 455/504
(90.28%) exceed the floor. Test-body wall time was `374.23 s` (`374.71 s`
external), down 42.71 s (10.24%) from the `416.94 s` baseline.

The remaining limiting reasons are 128 fixed-point caps and 45 scaled physical
comparison rejections: 5 snow deposition, 6 snow temperature, 1 persistent
surface-liquid, and 33 WB14 working-liquid. Exact discrete/event rejections
remain zero. Maximum mass/energy residuals were
`3.55271367880050093e-15 kg m^-2` and `9.31322574615478516e-10 J m^-2`.
Receipt reseal maxima were `9.98625182546675205e-10 J m^-2` and
`3.69482222595252097e-12 K`, within unchanged bounds. Log/time sha256 values:
`3930f691c7df2711791d1488b1b231209d79e079ea8ddfb7b5916119bf041ca1` and
`2531e4de80f3f681f4c32f382a8110f4ccfe745d9647fd17830903651ed563ea`.

Disposition: `ACCEPT`. This increment materially reduces the primary
accepted/rejected microstep blocker without tolerance relaxation or changes to
accepted physics, closure, custody, event, topology, receipt, rollback, or
fail-closed behavior. The 128 caps remain the next solver target.
