# QA Attempt 1

Evidence class: Ran.

- Provider run:
  [`30175384859`](https://github.com/rogerlew/openWEPP/actions/runs/30175384859)
- Source/workflow head:
  `2f16072bd86ed2ae858625a31c39769d7ad8b3a5`
- Qualification TESTGATE run: `30173294509`
- Result: `EXECUTION_FAILED`
- Child exit: `2`
- Control receipt SHA-256:
  `429030b2d148744d7fc3df2beaddc576d1c37c0e716ef347c0dbd1a557162c91`
- Partial index SHA-256:
  `e9f09c64ae53e8efd51cc724e4b291a7d11d4d43d2f97c76cc975bc9c4290605`

Priority preflight passed exact source, workflow, repository, TESTGATE run, and
current-main admission. Forest1 labels were exact, occupancy remained `CLEAR`,
and no TESTGATE overlap occurred.

The instrumented `full` Nextest profile failed before `science-manual` or any
downstream inventory, JUnit, merged LCOV, CRAP, snowbench, report, publication,
or `quality_evidence_id` was produced. Only
`local/nextest-full.log` existed, at 2,686,826 bytes.

The exact failing test and log tail are not recoverable. The control artifact
indexed but intentionally omitted logs larger than 256 KiB, forest1 cleanup
removed raw attempt contents, and the provider did not materialize the
observation-job log. No exact test failure is inferred.

Disposition: repository-owned evidence-lifecycle defect. The unchanged head
will not be rerun. A corrected head must retain a full log digest and bounded
diagnostic tail, pass TESTGATE, then receive one new QA attempt.

Read-only forensic evidence is retained at
`/home/workdir/openWEPP-quality-history/20260725-order7-qa-run-30175384859`.
