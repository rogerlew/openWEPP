# Grid40 QA review: provisional arm recorder

Evidence class: **Static.** Reviewed `run_pair_arm.py`,
`arm-recorder-unit-checks.json`, and `arm-recorder-process-checks.json` only.
The process checks used synthetic Python children with a synthetic source
identity. No frozen pair-freeze, binary, original case, solver, or result-bearing
arm was invoked.

## Findings

### Medium — the fixed eight-file partition is not enforced

`run_pair_arm.py:124-126` admits any nonempty declared partition with *at most*
eight names. The stated protocol is eight fixed child-output files, and the
per-file `RLIMIT_FSIZE` derives from division by eight. A freeze with fewer
names changes the declared partition while retaining that derived cap. Require
exactly eight unique child-output names in the frozen protocol, before creating
the arm directory. Keep `stdout` and `stderr` among them.

### Medium — output inventory detects, but does not contain, writes outside the arm directory

`run_pair_arm.py:82-90` inventories only paths below the arm directory after
the child begins. A child can write elsewhere, or create an output symlink and
write through it, before the next inventory check. `RLIMIT_FSIZE` limits files
created by the child but does not establish that all child output has the eight
declared paths. The future pair freeze must bind the actual output-directory
argument/environment and the detached executable's complete write boundary;
the pre-launch checks must demonstrate that the child cannot use other output
paths. An inventory failure must remain an invalid preserved arm, never a
successful measurement.

### Medium — early recorder failures have no durable arm receipt

`run_pair_arm.py:103-123` verifies the freeze, checks host capacity and reads
treatment admission before `directory.mkdir()` and before `receipt.json` is
written. A failure there (including a preexisting arm directory) raises without
an arm-local receipt recording the refusal. The atomic claim comment applies
only after successful directory creation. Preserve a small refusal receipt in
a predeclared parent-controlled location, or otherwise make the freeze define
the durable record for each pre-spawn refusal, while retaining no-retry
semantics.

## Accepted prospective controls

- The launch deadline check reserves `120 + 1200` seconds; each child receives
  120 seconds, 16 GiB address-space, and 64 MiB stack limits.
- The per-file bound is `(1 GiB - 2 MiB) / 8`; eight child files therefore
  total below 1 GiB, while `save()` limits each receipt to below 1 MiB and
  `output_inventory()` checks the arm aggregate. This is an admissible
  prospective subdivision of the fixed 1 GiB arm limit, provided the exact
  eight-file requirement is enforced. It must remain fixed after results.
- `verify()` binds recorder and snapshot-tool hashes, source snapshot, binary,
  input digest, five support links, and nineteen external build inputs. The
  eventual freeze still must name concrete paths/digests and pass this check
  before either arm starts.
- Treatment is blocked on a successful baseline receipt, bound admission and
  comparison sidecars, matching trace digest, ordinary-record count/order, and
  unchanged freeze hash. `directory.mkdir()` prevents replacing an already
  claimed arm after those gates.
- Process exceptions, nonzero exits, timeouts, inventory failures and final
  integrity failures occurring after receipt creation preserve a receipt and
  child files. The synthetic checks demonstrate only those recorder mechanics;
  they do not demonstrate the frozen binary, source identity, input replay,
  baseline correspondence, numerical validity, or scientific acceptance.

## Same-reviewer fix verification

Evidence class: **Static**, with the supplied `arm-recorder-fix-checks.json`
recording offline synthetic environment/refusal checks only.

The three recorder findings are resolved at code level:

- `run_pair_arm.py:147-152` requires exactly eight unique local child slots,
  requires `stdout` and `stderr`, and rejects aliases of the parent receipt
  before the atomic arm claim.
- `run_pair_arm.py:75-80` and `83-98` bind source/binary/entry-specific
  write-boundary review material, require no subprocesses and no other
  destinations, clear profiling/build/injection environment variables, and
  require every declared writer environment value to resolve to a permitted
  normalized arm-local filename. The supplied offline checks cover a reviewed
  writer path, escaping writer refusal, and profiling-flag refusal.
- `run_pair_arm.py:213-231` durably records pre-spawn/launcher refusals in a
  unique parent-controlled path without replacing either arm receipt. The
  supplied offline check covers this independent refusal record.

The concrete source-, binary-, and entry-specific exhaustive Rust writer
call-path/write-boundary review remains **PENDING**. No pair-freeze may claim
the `write_boundary` predicates, and no result-bearing launch may occur, until
that review names the actual reachable writers and paths, finds no subprocess
or other destination, is independently reviewed, and its bound review hash is
present in the freeze. This is not evidence of a defect in the revised recorder;
it is the required source-specific prerequisite for its new guard.

## Disposition

**HOLD — no launch recommendation.** Recorder finding fixes pass static
same-reviewer verification. Freeze the concrete source, binary, protocol,
exhaustive write boundary, five support links, nineteen external inputs, and
prerequisite evidence; then recheck the changed freeze controls before any
result-bearing baseline dispatch.

Inspection-tool failures: **0**.
