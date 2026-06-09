# H1-H39 Runner Evidence

Evidence mode: Ran

Delegated runner: `gpt-5.3-codex-spark` worker following
`.codex/agents/comparator_suite_runner.toml`.

Command:

```text
tools/owcmp/owcmp batch h1-h39-semantic --baseline-dir /tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions --candidate-dir /tmp/hphys0300_full_20260605T155527Z/hillslope_output --candidate-year-offset 2012 --output-root docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39
```

First delegated attempt:

- Exit code: `1`.
- Hard blocker: direct `tools/owcmp/owcmp` used `/usr/bin/python3`, which lacked
  `pyarrow`.
- Evidence path:
  `docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/logs/H1.stderr.txt`.
- Resolution: `tools/owcmp/owcmp` now re-execs through `.venv/bin/python` when
  available, and failure paths now write `summary.md`.

Second delegated attempt:

- Exit code: `0`.
- Execution verdict: `PASS`.
- Semantic pass count: `0/39`.
- Structural row/key failures: `0`.
- First divergent: `hillslope=1 key=[1,254,2016]`.

Artifact paths:

- Summary JSON:
  `/home/workdir/openWEPP/docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/summary.json`
- Summary Markdown:
  `/home/workdir/openWEPP/docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/summary.md`
- Command log:
  `/home/workdir/openWEPP/docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/command-log.json`
- Logs directory:
  `/home/workdir/openWEPP/docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/logs`
- Reports directory:
  `/home/workdir/openWEPP/docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/reports`

Focus-column metrics:

| Column | Hillslope Fails | Total Fails | Mean Abs Diff Mean | Max Abs Diff | Max Rel Diff | Max Abs Hillslope | Max Abs Key |
|---|---:|---:|---:|---:|---:|---:|---|
| RM | 39 | 7097 | 0.2560855095903018 | 27.959999999999997 | 1.0 | 6 | [1,139,2014] |
| Snow-Water | 39 | 10391 | 2.8994315028494784 | 65.50683982565039 | 1.0 | 37 | [1,106,2015] |
| Total-Soil | 39 | 52185 | 56.0100716038276 | 317.13012867850887 | 0.9031415984876613 | 24 | [1,260,2016] |
| SoilWaterTotal | 39 | 52185 | 56.0100716038276 | 317.13012867850887 | 0.9031415984876613 | 24 | [1,260,2016] |
| Ep | 39 | 42688 | 0.6336571797708913 | 7.100843908920885 | 1.0 | 7 | [1,185,2015] |
| Es | 1 | 470 | 0.010140431715611934 | 1.828582943108198 | 1.0 | 6 | [1,160,2016] |
| Dp | 38 | 10961 | 0.050443549018045604 | 0.24479985054954173 | 1.0 | 37 | [1,179,2015] |
| Q | 0 | 0 | 7.739858510220364e-17 | 2.0816681711721685e-14 | 1.0 | 38 | [1,143,2014] |
| latqcc | 39 | 38462 | 0.28588237171068176 | 3.023091871829949 | 1.0 | 33 | [1,83,2015] |
