# Contract-Test Implementation Evidence

Status: `not-applicable`

Evidence mode: `Static:` plus focused tests.

No `SC-*` amendment was required for W6, so no new canonical
contract-derived test was required.

W6 package coverage is provided by focused tests:

- `typed_publication_writer_reads_publication_frame_directly` proves the direct
  typed publication writer reads `WatershedPublicationFrame` fields directly and
  preserves existing output aliases/formulas for populated operands.
- `wshedw5_public_cli_uses_typed_network_and_publication_frames` proves the
  public watershed CLI remains on the typed network/publication path and forbids
  row-seed staging markers in that CLI.
- `carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate` proves
  the existing fixture now includes committed W6 watershed and hillslope launch
  runfiles.
- `onshore_xenophobia_committed_fixture_is_full_1305_hillslope_gate` proves the
  large fixture gate is committed, full, parser-readable, and source-local.
