# Qualification Evidence Contract

Status: scaffold contract.

Every case report binds the subject-freeze ID, matrix version, case ID, attempt
IDs, exact argument arrays, environment projection, start/end times, process
spawn/exit trace, audit ID/status, planned and observed node IDs, per-node
receipt IDs, imported/rejected receipts with reasons, cache events, artifact
paths/digests, and result.

Crash/recovery evidence additionally binds the termination checkpoint, deleted
local paths, durable upload/index records, new environment identity, hash-chain
verification, and resumed process counts. Combined-run evidence binds the
single Nextest process, exact inventory, JUnit and LCOV digests, CRAP report,
functional/coverage node lineage, timing breakdown, and adoption threshold.

Provider evidence binds repository, base/head, event/ref, workflow/job, runner
identity, run/attempt IDs, queue preflight, job ordering, timestamps, uploaded
artifacts, independent verification, receipt, predicate, and attestation. Local
or hermetic results are labeled as such and never represented as provider
evidence.

Human summaries are renderings of machine records. Reviewers independently
reconstruct counts, hashes, and case verdicts rather than trusting summaries.
