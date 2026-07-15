# SNOTEL Pilot Evidence Inventory

Status: `complete` for baseline inventory; characterization:
`INSUFFICIENT_EVIDENCE`.

Static: the pilot is bounded to snow water equivalent, station-derived snow
depth, and derived bulk density at five SNOTEL climates. It does not cover
frost, runoff, erosion, channels, watersheds, or application fitness.

## Retained Evidence

| Identity | Class and role | Availability and reproduction | Interpretation |
| --- | --- | --- | --- |
| `d673b2e6...a133f3` observation manifest | Empirical dataset inventory | Tracked; references some untracked absolute acquisition paths and null hashes | Admitted observations exist, but raw acquisition replay is incomplete |
| Five tracked normalized site CSVs | Empirical observations | Tracked and individually hashed in the manifest | SWE is direct; depth is station-derived; density compounds both uncertainties |
| `820162ad...6c020` characterization JSON | Empirical dataset characterization | Tracked | Supports data inspection, not model corroboration by itself |
| `fc5657fe...f13f01` cross-SNOTEL JSON and `3b6c8018...ea24a` summary | Comparative/empirical diagnostic | Retained; historical tool can be run only with substantial build/runtime context | Retrospective rubric; explicit no-promotion disposition |
| `f511c11d...5747a` activation JSON | Verification/comparative activation evidence | Retained and guarded by integration test | Shows selected implementation/default and conservation, not general validity |
| `6b3b1796...2d17` diagnostic guard | Verification policy | Directly reproducible with nextest | Preserves diagnostic-only and flag-not-target posture |
| `471f8dbe...382df` activation guard | Verification and retained-report consistency | Directly reproducible with nextest | Confirms current source/report bindings, not observation agreement |

The five sites span maritime, intermountain, continental, and high-elevation
settings using DAYMET, GRIDMET, CLIGEN, or PRISM forcing. The retained fixture
inventory reports 70,999 daily observation rows and 13,590 paired density rows
across the five sites. Absolute SWE, depth, and density magnitudes remain
forcing-limited; timing and decomposed response signatures are more
forcing-robust under ADR-0028.

## Material Gaps

- The evaluation was retrospective; thresholds and model variants were not a
  prospectively registered held-out campaign.
- The observation manifest contains local absolute paths for raw downloads and
  null identities for some acquisition/characterization inputs.
- Full command, dependency, executable, configuration, and raw-source replay is
  not preserved as one portable bundle.
- The historical rubric includes comparative flags and diagnostic scores, not
  a reviewed public representational claim with application accuracy criteria.
- No independent external hydrologist has reviewed this baseline dossier.
- Observation, forcing, scale, and derived-density uncertainty are described
  but not propagated into a claim-specific acceptance analysis.

Disposition: the retained evidence is informative and publishable as a
`CANDIDATE`, but cannot sustain a favorable empirical characterization. The
pilot uses `INSUFFICIENT_EVIDENCE`; its verification profile is reported
separately. The exact historical activation report closes three mandatory
selector/rollback/phase-partition obligations. It does not close
current-release result lineage, numerical solution verification for the
reported quantities, or independent release-realization reproduction. Those
mandatory rows are `BLOCKED` or `NOT_RUN`, so the mechanical aggregate is
`BLOCKED`, not `PASS`.
