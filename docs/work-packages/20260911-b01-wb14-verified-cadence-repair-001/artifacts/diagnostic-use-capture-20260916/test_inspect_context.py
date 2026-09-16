#!/usr/bin/env python3
"""Focused fixtures for bounded context inspection; no corpus or model use."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile


HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("inspect_context", HERE / "inspect_context.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def byte_array(value):
    return "[" + ",".join(str(item) for item in value) + "]"


def add_member(root, ordinal, position, name, text):
    path = root / "rows" / f"{ordinal}.{position}.json"
    path.write_text(text)
    return {"event": "external_json_end", "path": [{"member": name, "position": position}],
            "file": str(path.relative_to(root)), "bytes": path.stat().st_size, "sha256": sha(path)}


def row(root, ordinal, kinds, members, scalars):
    events = root / "rows" / f"{ordinal}.events.jsonl"
    lines = ['{"event":"start_map","value":null}']
    for key, value in scalars.items():
        lines.append(json.dumps({"event": "map_key", "path": [{"member": key, "position": 0}], "value": key}))
        lines.append(json.dumps({"event": "string", "value": value}))
    lines.append('{"event":"end_map","value":null}')
    events.write_text("\n".join(lines) + "\n")
    return {"ordinal": ordinal, "kinds": kinds, "role": "related_b01", "event_file": str(events.relative_to(root)),
            "event_file_bytes": events.stat().st_size, "event_file_sha256": sha(events), "external_json": members}


def fixture(root, malformed=False):
    (root / "rows").mkdir(parents=True)
    input_bytes = b'{"transaction_id":"tx","day_index":4,"decimal":1.2300}'
    typed = '{"Ok":' + byte_array(input_bytes) + '}'
    parent = '{"Ok":' + byte_array(b'{"parent_day_index":4,"parent_interval_index":22,"cursor":7}') + '}'
    large = ('{"ordered_owner_joins":[1,2,3],"event_groups":[{"a":1},{"a":2}],'
             '"pending_parcels":[7],"phase_day_index":4,"blob":[' + ",".join("0" for _ in range(550000)) + "]}")
    if malformed:
        typed = '{"Ok":[1,256]}'
    guard_members = [add_member(root, 2, 0, "input_typed_bytes", typed), add_member(root, 2, 1, "beginning_typed_bytes", typed)]
    caller_members = [add_member(root, 3, 0, "input_typed_bytes", typed), add_member(root, 3, 1, "beginning_typed_bytes", typed),
                      add_member(root, 3, 2, "working_typed_bytes", typed), add_member(root, 3, 3, "parent_working_typed_bytes", parent),
                      add_member(root, 3, 4, "phase_diagnostics", large)]
    rows = [row(root, 2, ["surface_liquid_wb14_cadence_failure"], guard_members, {"day_index": "4", "interval_index": "22", "cursor": "7"}),
            row(root, 3, ["surface_liquid_wb14_cadence_caller_failure"], caller_members, {"day_index": "4", "interval_index": "22", "provisional": "true"})]
    (root / "summary.json").write_text(json.dumps({"status": "COMPLETE", "rows": rows}))


def main():
    with tempfile.TemporaryDirectory() as temporary:
        root = Path(temporary)
        export = root / "export"; fixture(export)
        result = module.inspect(export, root / "inspection")
        assert result["status"] == "COMPLETE"
        joins = result["operand_joins"]
        assert joins["guard_ordinal"] == 2 and joins["caller_ordinal"] == 3
        assert all(joins["equalities"].values())
        caller = next(item for item in result["rows"] if item["physical_ordinal"] == 3)
        input_member = next(item for item in caller["members"] if item["member"] == "input_typed_bytes")
        assert input_member["bytes"] == len(b'{"transaction_id":"tx","day_index":4,"decimal":1.2300}')
        assert input_member["inner_json"] == "decoded" and input_member["number_decoding"] == "exact Decimal"
        phase = next(item for item in caller["members"] if item["member"] == "phase_diagnostics")
        assert phase["source_bytes"] > 1 << 20 and phase["status"].endswith("semantically unvalidated")
        counts = {item["member"]: item for item in phase["top_level_members"]}
        assert counts["ordered_owner_joins"]["direct_array_item_count"] == 3
        assert counts["event_groups"]["direct_array_item_count"] == 2
        assert counts["pending_parcels"]["direct_array_item_count"] == 1
        malformed = root / "malformed"; fixture(malformed, malformed=True)
        try:
            module.inspect(malformed, root / "malformed-inspection")
        except module.ContextInspectionError:
            pass
        else:
            raise AssertionError("invalid byte unexpectedly accepted")
        print(json.dumps({"status": "PASS", "large_byte_array": phase["source_bytes"]}))


if __name__ == "__main__":
    main()
