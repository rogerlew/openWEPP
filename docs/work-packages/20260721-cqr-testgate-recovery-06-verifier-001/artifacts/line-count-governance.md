# Line-Count Governance

Static: scaffold `verifier.rs` is 2,689 lines, above the 2,000-line WARN
threshold and below the 3,000-line block. New characterization should use the
declared test-only split; production decomposition must remain below 3,000.

Static: the final production file is 2,693 lines, with only the four-line
test-only include added. The 324-line characterization resides in the declared
split. The WARN remains recorded and the 3,000-line production block is not
approached; a future production module-boundary split remains appropriate.
