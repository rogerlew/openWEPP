# ASSURE-05 Finding Disposition

Status: INTERNAL FINDINGS CLOSED; HUMAN REVIEW REQUIRED

Evidence class: Static + Ran

This ledger preserves the internal coding-agent findings and their resolution.
It is not peer review, scientific approval, publication approval, or release
authorization.

| Finding | Disposition | Resolution and verification |
| --- | --- | --- |
| DS-01: arithmetic residual mislabeled as an implementation residual | Accepted; resolved | The manuscript, supplement, result metadata, and figures now identify `1.7763568394002505e-15 m3` as Python binary64-versus-Decimal arithmetic residual. Rust evidence is separately limited to a passed `1.0e-12 m3` assertion allowance. Renewed domain review verified the correction. |
| DS-02: continuous producer-to-CLI-adapter-to-consumer traversal not freshly executed | Accepted; resolved by claim narrowing; follow-up retained | Every positive claim now distinguishes the writer/parser test from the hand-constructed watershed-consumer test and states that the complete nonzero CLI-adapter traversal was not run. A future continuous adapter test remains an explicit integration obligation. Renewed domain review found the bounded claims truthful. |
| DS-03 / F1: H2637 procedure depended on transient raw files | Accepted; resolved | The exact accepted manifest, HBP, and pass-Parquet files are version-bound research objects. The report contract and staged procedure authenticate them and semantically reproduce the retained H2637 ledger. |
| DS-04: Priest River calibration/evaluation period misstated | Accepted; resolved | The report now distinguishes 2005–2006 calibration, 2007–2009 evaluation, and full-period 2005–2009 summary metrics. Renewed domain review verified the attribution. |
| DS-05: storage and daily integrated volume units conflated | Accepted; resolved | Storage is stated in `m3`; daily `D`, `Qb`, and `Qs` are integrated timestep volumes in `m3`; only coefficients use `d^-1`. Renewed domain review verified the wording. |
| F2: staging instructions omitted the narrative seed | Accepted; resolved | The supplement gives the exact `mktemp`, directory creation, and model-narrative copy steps before build/check. Two final unrelated seeded roots built and checked identically. |
| F3: agent packet referred to a nonportable prompt | Accepted; resolved | The active prompt moved byte-for-byte to the archived path at unchanged SHA-256 `5a740e7f...`, is a declared public-safe research object, and is copied into staging. Packet, descriptor, and catalog identities were rebound in dependency order. |
| TV-A-01: archival semantic-difference summary retained the pre-remediation transfer wording and 11-object count | Accepted; resolved | The summary now states the separate-interface claim boundary, keeps continuous nonzero CLI-adapter traversal open, and records all 15 final public-safe research objects. Both terminal verifiers renewed the documentation check after correction. |
| TV-A-02: post-install source-drift rollback test could miss its transient backup marker | Accepted; resolved | The test-only fixture now extends the declared input rehash interval with an 8 MiB scratch dependency and uses a 60-second marker deadline. Production code is unchanged. Consecutive isolated runs, the 9/9 assembly target, and the final 59/59 affected-suite run passed; the contradictory failures remain recorded in terminal verification A. |

No accepted internal finding remains open against the bounded report. The
DS-02 follow-up is deliberately outside the evidence claimed by this version;
it must be closed before any later report claims continuous production adapter
traversal.

The remaining hold is institutional rather than technical: no valid named
human report lead, scientific reviewer, reproduction/publication reviewer,
assurance steward, or release owner has approved the exact source root. Agents
must not manufacture those records.
