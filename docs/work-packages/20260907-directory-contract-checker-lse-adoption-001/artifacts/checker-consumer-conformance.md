# Checker and consumer conformance
Ran: .venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py tests/python/test_sc_contract_directory.py, cwd /workdir/openWEPP, exit 0: 71 passed. Log: logs/checker-conformance-initial.log. This is initial checker evidence; live migration and consumers remain pending.
Original legacy LSE strict PASS and unit PASS; original three pytest tests PASS (logs/baseline-*). Pytest 9.1.1 installed in repository .venv after actual missing-module failure; no global environment/dependency changes.

## Bounded implementation interpretations (prospective)
Front matter uses plain/quoted scalar or two-space dash list values, duplicate/malformed keys fail. Entry-only format metadata dispatches; absence is legacy, unknown cannot fall back. A repository is nearest .git ancestor, otherwise isolated fixture entry parent; fixtures needing parent traversal must declare .git root. Member paths never traverse; dependency traversal remains confined. No network fetch.
Required target fields accept Markdown links or a literal path#fragment; singular fields reject multiple links. Binding definitions always use exact specified HTML table-row grammar. Existing external document sections may resolve explicit link fragments using lowercase heading text, punctuation removed, spaces replaced by hyphens; ambiguous headings fail. New migrated sections use HTML anchors. An explicit anchor adjacent to its identically named heading is one intended section target, not two independent definitions. HTML comments are unsupported in directory authority and fail closed.
Schema N/A is admitted only for kernel keys with explicit `Non-kernel contract: <rationale>`; this LSE pilot uses no N/A. Structural check does not certify rationale/scientific adequacy.
Owned deferred rows use nonempty `owner:`, `next evidence gate:`, and `retained: path#anchor` in Notes. The retained target must remain normative; ownership and evidence meaning remain review duties.
Historical entry metadata follows the existing sidecar template: heading owns entry_id and title; bullet fields own status/source/date/verdict/bindings/provenance; prose owns summary. Every historical section requires an indexed source mapping; sidecar own status and mapped IDs must agree with BEI. Supersession points to a resolvable normative path#anchor.
Original Git source spans are identified by external commit-pinned source links; checker does not fetch or certify their content. Package preservation map binds local original Git object and exact hashes.

## Scope
One shared bounded parser/loader; no semantic dependency engine or science equivalence classifier. Qualifier-removal counterexample passes structural lint and is deliberately unacceptable to preservation review.
