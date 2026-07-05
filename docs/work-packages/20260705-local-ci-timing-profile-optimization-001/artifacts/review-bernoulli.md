# Subagent Review - Bernoulli

Evidence class: `Static/Ran`.

Reviewer: `Bernoulli` (`019f3130-dbf1-7c40-a9af-86ae9e3c73ba`)

Scope: read-only review of the local-CI timing/profile optimization package,
including `tools/local_ci/nextest_timing.py`, `.config/nextest.toml`, package
artifacts, and agent-facing guidance.

## Findings

1. **High - Timing tool can record false nextest evidence from stale JUnit.**

   Evidence: `nextest_timing.py run` executed a command and then parsed the
   supplied JUnit path without proving that file was produced by the command.
   Local ignored history demonstrated the failure mode: a `true` command could
   record passed nextest tests from a pre-existing JUnit file.

   Failure mode: agents can create durable timing/gate records for a command
   that did not run nextest, or parse an old profile's JUnit after a failed or
   no-output run.

2. **Medium - Required concurrency evidence is not fully in package artifacts.**

   Evidence: `package.md` requires concurrency artifacts to record commands, but
   `empirical-concurrency.md` recorded method, caps, wall times, and decisions
   without the exact sweep commands/filtersets. Exact commands existed only in
   ignored local history.

   Failure mode: future agents cannot reproduce or audit the cap decisions from
   committed package evidence alone.

3. **Medium - Nextest group semantics are under-described, making "cap 4" easy
   to misread.**

   Evidence: `.config/nextest.toml` sets `max-threads = 4` for fixture groups,
   while matching overrides use `threads-required = 2`.

   Failure mode: later tuning may incorrectly treat these as four-way fixture
   caps. The frost cap is also justified by a very small non-snowbench subset,
   so it should not be described as strong evidence for expensive frost
   fixtures.

4. **Low - Active kickoff prompt lacks the required subagent authorization
   wording.**

   Evidence: package governance requires explicit authorization in `package.md`
   and active kickoff prompt. The package had it in `package.md`, but not in
   `prompts/active/kickoff.md`.

   Failure mode: future replay from the kickoff prompt alone is not
   self-contained for delegated review.

## Commands Reported By Reviewer

- `git status --short`
- `find`
- `sed` / `nl`
- `rg`
- `git diff --stat`
- `git diff --name-only`
- `git diff -- ...`
- `git ls-files --others --exclude-standard`
- `git check-ignore`
- `cargo nextest --version`
- `cargo nextest run --help`
- `cargo nextest list --help`
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/local_ci/nextest_timing.py --help`
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/local_ci/nextest_timing.py run --help`
- `ls`, `tail`, and `sed` reads under `target/local-ci-history`
