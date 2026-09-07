# Administrative agent tools

`find-agents --for <paths>` discovers applicable instruction chains.
`.venv/bin/python tools/agents/context_report.py selection.json [--revision SHA]`
reports explicit recursive reading selections. Each selection has `readings`
(ID -> path, reason, optional inclusive `lines`, `exposures`, `requires` IDs)
and `roles` (role -> bootstrap/expansion ID lists). Whole-file is the default.
Automatic repository instructions count as declared exposures; unknown runtime
context stays UNOBSERVED. This tool does not discover semantic obligations.

## Capture

Before future candidate measurement or reversion, retain recoverable bytes:

```sh
.venv/bin/python tools/agents/evidence_bundle.py capture selection.json --root /absolute/source --destination /outside/checkout/new-bundle
.venv/bin/python tools/agents/evidence_bundle.py verify /outside/checkout/new-bundle
.venv/bin/python tools/agents/evidence_bundle.py restore /outside/checkout/new-bundle --destination /outside/checkout/new-restoration
```

Caller must select every relevant source/build input explicitly (including
tracked deletions and both names of renames); source coverage is declared, not
inferred. No directory recursion or secret discovery. Selection JSON:

```json
{
  "files": {"source": ["main.c", "Makefile"], "executable": ["program"],
    "fixture": ["input.txt"], "protocol": ["protocol.md"], "result": ["result.txt"]},
  "build_command": ["make", "program"],
  "toolchain": "exact compiler version and external identity",
  "commands": [["./program", "input.txt"]],
  "environment": ["CFLAGS", "LANG"],
  "external_dependencies": [{"identity": "exact library/toolchain identity", "limitation": "not bundled"}],
  "limitations": "explicit reproduction limitations",
  "retention": "named owner; through disposition plus explicit audit-release boundary"
}
```

All files are explicitly relative to `--root`; stage external inputs into a
declared source directory before capture if necessary. Do not select credentials,
tokens or `.env` files; review command/metadata values for secrets. Environment is
restricted to the constant allowlist in the utility and captured from the process.
Commands/toolchain are recorded assertions, never executed or automatically verified.
Results must exist: premeasurement capture may use an explicitly labeled NOT RUN
result record, then create a new immutable postmeasurement bundle with actual results.
Never mislabel the premeasurement record as measured evidence.

Bundle layout is manifest.json, manifest.sha256 and blobs/<sha256>, not a tar
archive. Verification validates member paths before restoration. No self-hash:
the detached manifest digest covers manifest bytes; per-file hashes cover blobs.
Hashes detect corruption, not malicious replacement or semantic equivalence.
Keep a trusted compact manifest digest separately in the package.
Symlinks are captured as link bytes, never followed; only relative targets
contained within their restored group are allowed. Unsupported hardlinks,
special files/modes, index conflicts, missing inputs, changed inputs/index,
corruption and existing/unsafe destinations fail. Use a quiescent source tree;
two reads plus metadata checks detect observed mutation, not adversarial atomic
filesystem snapshots. Capture/restore destinations must have an existing parent.
Partial failed destinations are retained for inspection and are never overwritten.

Restoration creates source/, executable/, fixture/, protocol/, result/ and
custody.json. source/.git is a fresh repository with the selected staged blobs
restored independently from worktree bytes. Original Git objects/history are
not required; base/status/index metadata explains the original selected state.
No original history/HEAD reconstruction is claimed. Source restoration, measured
executable restoration, rebuildability and bit-reproducible rebuilding are four
distinct claims; this tool proves byte custody, not the latter two. External
tools/libraries remain explicit reproduction limitations.

Owner retains the external bundle through disposition and a named audit-retention
boundary (default: explicit owner audit release). No automatic deletion, archive
service, committed executable or bulk archive. Test fixtures use disposable
temporary directories and delete only their own generated data.

## Identity comparison

`identity membership.json --root PATH` emits hashes for exactly three nonempty
membership lists: experiment, evidence_claim, publication. Include all relevant
dependencies, including Markdown when it defines protocol/authority/claims.
`compare before.json --destination after.json` reports changed identities only.
The accountable change-impact decision follows testing-and-gate-strategy.md
section 10. The utility never grants reuse, signoff or acceptance.

Run `.venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py'`.
