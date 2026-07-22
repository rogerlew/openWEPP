# Characterization

Static: the existing 12 planner tests were moved byte-for-byte before new
characterization. Ran: all 12 passed after the move. New target/error/floor
characterization remains the next test-first step and must land before
production decomposition.

Static: five new tests cover both retained supersession mappings, node semantic
weakening, every risk rank, node-argument shapes, regular/missing/directory/
UTF-8 and non-UTF-8 symlink manifest identities, Git success/failure, malformed
blob batches, command/confined/unsupported/package inventory dispatch, and the
target planner's committed-head + workspace + bound-context graph path.

Ran: the expanded exact planner namespace passed 17/17 with 133 skipped in
77.579 seconds. The initial compilation-only mismatch between JSON and
`Option<String>` was corrected before this passing run. No production line had
changed when the characterization passed.

Static: characterization was then strengthened with a real package-only Nextest
inventory, authority dispatch, confined/unsupported inventory failures, and a
canonical isolated `reconcile_intent_terminal` success path. Ran: the resulting
18-test namespace passes after the production extraction.

Static: review B correctly rejected the first workspace/head test because its
identical commits and editorial path could not discriminate graph-union defects.
The replacement builds an isolated base graph with a `legacy` reverse
dependent, a head graph with a `consumer` reverse dependent, and a common
`executor-fixture` root. A graph-sensitive Rust change must select all three
packages, both reverse dependencies, and three package-targeted nodes. A dual
invalid-revision case proves the base error is returned before the head is
read.

Ran: the corrected single test passed at current extracted source in 39.665
seconds. Ran: the exact same corrected test file passed 1/1 in 39.739 seconds
against pre-extraction commit `7c10da5e` in a disposable detached worktree.
The proof worktree and build cache were removed after the pass.
