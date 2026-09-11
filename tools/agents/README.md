# Administrative agent tools

## External review packet

Use the owner's existing ChatGPT Pro exchange with
[the role document](../../docs/standards/chatgpt-pro-role.md) and
[new-thread kickoff](../../docs/work-packages/templates/chatgpt-pro-kickoff.md).
After a scoped local commit, export explicitly chosen source and evidence:

```sh
.venv/bin/python tools/agents/review_packet.py --root /workdir/openWEPP \
  --base BASE_COMMIT --head REVIEWED_COMMIT \
  --path path/to/changed-file --context AGENTS.md \
  --context docs/standards/chatgpt-pro-role.md \
  --attach /tmp/openwepp_selected_run_result.md \
  --output /tmp/openwepp_checkpoint_review.md
```

Repeat `--path` for each selected changed file (both paths of a rename),
`--context` for needed committed surrounding source/instructions/authority, and
`--attach` for explicit local text evidence. Review these selections for secrets
before transfer. The packet includes full selected files at the reviewed commit,
the selected diff, an inventory of all changed paths, and attachment hashes.
Other changed bodies and worktree/index/untracked bytes are NOT included unless
explicitly attached. No dependency discovery, recursive directory export, network
upload, command execution from inputs, or review/validation verdict occurs.
LFS pointers are labeled as unavailable payloads; the packet does not fetch them.
Attachments are supplied evidence, not proof that their asserted commands ran.

For detached experiments, include their actual retained source/patches and
identities using explicit attachments or existing evidence tooling. A primary
checkout commit is not the experiment identity. Large/binary source kits belong
in that existing tooling; the text exporter fails at its default 1 MiB limit
instead of silently truncating. It refuses directories, symlink files, binary or
non-UTF-8 text, invalid/missing paths, and existing output files. It reads local
attachments without an atomic snapshot: use quiescent files. Hashes identify the
included bytes, not their independent truth or a trusted publisher.

The owner transfers the packet and saves the returned original review response.
The executor incorporates attributable findings and reviewed source in package.md;
there is no automated verdict importer. After fixes, use the previously reviewed
commit as `--base`, the corrected commit as `--head`, and attach the original
finding IDs plus correction evidence for the same reviewer conversation. Keep
the package's current checkpoint budget; exporting or opening a thread does not
renew it. No additional maintained handoff record is required.

Run `.venv/bin/python -m unittest discover -s tools/agents -p 'test_review_packet.py'`.

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
contained within their restored group are allowed; parent-traversing (`..`)
targets are rejected even when they appear lexically contained. Unsupported hardlinks,
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
