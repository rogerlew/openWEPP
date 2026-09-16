#!/usr/bin/env python3
"""Bounded structural inspection of a completed B01/WB14 diagnostic export.

This tool only reads exporter output.  It never invokes restoration, a native
importer, or a model; all decoded values remain semantically unvalidated.
"""
import argparse
from decimal import Decimal
import hashlib
import json
from pathlib import Path

import ijson


INNER_LIMIT = 8 << 20


class ContextInspectionError(Exception):
    pass


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as raw:
        for chunk in iter(lambda: raw.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def inside(root, relative):
    path = (root / relative).resolve()
    if root.resolve() not in path.parents:
        raise ContextInspectionError("export member path escapes root")
    return path


def no_duplicates(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ContextInspectionError("duplicate keys block inner JSON decode")
        result[key] = value
    return result


def stream_u8_json(source, destination, typed):
    """Validate a JSON byte array and write it in fixed-size chunks."""
    digest, count, buffer = hashlib.sha256(), 0, bytearray()
    state = "typed_map" if typed else "array"
    key_seen = False
    try:
        with source.open("rb") as raw, destination.open("xb") as out:
            for event, value in ijson.basic_parse(raw, use_float=False, buf_size=65536):
                if state == "typed_map":
                    if event != "start_map":
                        raise ContextInspectionError("typed bytes must start with an object")
                    state = "typed_key"
                elif state == "typed_key":
                    if event != "map_key" or value != "Ok" or key_seen:
                        raise ContextInspectionError("typed bytes must be exactly {'Ok':[...]}" )
                    key_seen = True
                    state = "typed_array"
                elif state == "typed_array":
                    if event != "start_array":
                        raise ContextInspectionError("typed Ok must contain an array")
                    state = "values"
                elif state == "array":
                    if event != "start_array":
                        raise ContextInspectionError("canonical JSON must be a byte array")
                    state = "values"
                elif state == "values":
                    if event == "number":
                        text = str(value)
                        if not text.isdigit() or int(text) > 255:
                            raise ContextInspectionError("byte array contains a non-u8 value")
                        buffer.append(int(text)); count += 1
                        if len(buffer) == 65536:
                            out.write(buffer); digest.update(buffer); buffer.clear()
                    elif event == "end_array":
                        state = "typed_end" if typed else "done"
                    else:
                        raise ContextInspectionError("byte array contains a non-number event")
                elif state == "typed_end":
                    if event != "end_map":
                        raise ContextInspectionError("typed bytes object has extra members")
                    state = "done"
                elif state == "done":
                    raise ContextInspectionError("byte array has trailing JSON values")
            if state != "done":
                raise ContextInspectionError("truncated byte array")
            if buffer:
                out.write(buffer); digest.update(buffer)
    except Exception:
        if destination.exists():
            destination.unlink()
        raise
    return {"bytes": count, "sha256": digest.hexdigest(), "buffer_bytes": 65536}


def inner_json_summary(binary):
    size = binary.stat().st_size
    result = {"inner_json": "not decoded", "decoded_byte_limit": INNER_LIMIT}
    if size > INNER_LIMIT:
        result["limitation"] = "decoded bytes exceed bounded inner JSON limit"
        return result
    try:
        with binary.open() as raw:
            value = json.load(raw, parse_int=Decimal, parse_float=Decimal,
                              object_pairs_hook=no_duplicates)
    except (UnicodeDecodeError, json.JSONDecodeError, ContextInspectionError) as error:
        result["limitation"] = f"inner JSON not structurally decodable: {type(error).__name__}: {error}"
        return result
    result.update(inner_json="decoded", number_decoding="exact Decimal",
                  type=type(value).__name__)
    if isinstance(value, dict):
        result["top_level_keys"] = list(value)[:64]
        wanted = {"transaction_id", "day_index", "interval_index", "parent_day_index",
                  "parent_interval_index", "accepted_until_ns", "cursor", "provisional"}
        result["selected_small_fields"] = {key: str(value[key]) for key in wanted if key in value}
    elif isinstance(value, list):
        result["top_level_length"] = len(value)
    return result


def structural_joins(rows, destination):
    """Report only exact decoded-byte joins selected by source semantic kinds."""
    guard = next((row for row in rows if "surface_liquid_wb14_cadence_failure" in row["kind"]), None)
    caller = next((row for row in rows if "surface_liquid_wb14_cadence_caller_failure" in row["kind"]), None)

    def member(row, name):
        if row is None:
            return None
        return next((item for item in row["members"] if item["member"] == name and "decoded_file" in item), None)

    def equal(left, right):
        if left is None or right is None:
            return None
        left_path = destination / str(left["physical_ordinal"]) / left["decoded_file"]
        right_path = destination / str(right["physical_ordinal"]) / right["decoded_file"]
        if left_path.stat().st_size != right_path.stat().st_size:
            return False
        with left_path.open("rb") as left_raw, right_path.open("rb") as right_raw:
            while True:
                left_chunk, right_chunk = left_raw.read(65536), right_raw.read(65536)
                if left_chunk != right_chunk:
                    return False
                if not left_chunk:
                    return True

    if guard is None and caller is None:
        return {"status": "no guard/caller source-semantic rows present"}
    result = {"status": "structural byte equality only; semantically unvalidated",
              "guard_ordinal": None if guard is None else guard["physical_ordinal"],
              "caller_ordinal": None if caller is None else caller["physical_ordinal"],
              "guard_actual_indices": None if guard is None else guard["top_level_scalars"],
              "caller_actual_indices": None if caller is None else caller["top_level_scalars"]}
    for row in (guard, caller):
        if row is not None:
            for item in row["members"]:
                item["physical_ordinal"] = row["physical_ordinal"]
    result["equalities"] = {
        "guard_caller_input_exact_bytes": equal(member(guard, "input_typed_bytes"), member(caller, "input_typed_bytes")),
        "guard_caller_beginning_exact_bytes": equal(member(guard, "beginning_typed_bytes"), member(caller, "beginning_typed_bytes")),
        "caller_beginning_working_exact_bytes": equal(member(caller, "beginning_typed_bytes"), member(caller, "working_typed_bytes")),
    }
    result["current_input"] = None if caller is None else next((item.get("selected_small_fields") for item in caller["members"] if item["member"] == "input_typed_bytes"), None)
    result["parent_prefix_operands"] = None if caller is None else next((item.get("selected_small_fields") for item in caller["members"] if item["member"] == "parent_working_typed_bytes"), None)
    return result


def profile_large_json(source):
    """Count structure without constructing phase diagnostics."""
    scalar_count, container_count, selected, stack, descriptors = 0, 0, {}, [], []
    descriptors_truncated = False
    wanted = {"day_index", "interval_index", "capture_phase", "start_ns", "end_ns", "cursor", "provisional"}

    def scalar_path(frame):
        if frame["type"] == "map":
            return frame["path"] + "." + frame["pending"]["member"]
        return frame["path"] + "[" + str(frame["items"]) + "]"

    with source.open("rb") as raw:
        for event, value in ijson.basic_parse(raw, use_float=False, buf_size=65536):
            if event == "map_key":
                frame = stack[-1]
                if frame["type"] != "map":
                    raise ContextInspectionError("map key outside object")
                frame["pending"] = {"member": value, "position": frame["items"]}
                frame["items"] += 1
            elif event in ("start_map", "start_array"):
                parent = stack[-1] if stack else None
                descriptor = None
                if parent and parent["type"] == "array":
                    parent["items"] += 1
                if parent and len(stack) == 1 and parent["type"] == "map":
                    if len(descriptors) < 1024:
                        descriptor = {**parent["pending"], "type": "array" if event == "start_array" else "map"}
                    else:
                        descriptors_truncated = True
                if parent is None:
                    path = "$"
                elif parent["type"] == "map":
                    path = parent["path"] + "." + parent["pending"]["member"]
                    parent["pending"] = None
                else:
                    path = parent["path"] + "[" + str(parent["items"] - 1) + "]"
                container_count += 1
                stack.append({"type": "array" if event == "start_array" else "map", "items": 0,
                              "pending": None, "path": path, "descriptor": descriptor})
            elif event in ("end_map", "end_array"):
                frame = stack.pop()
                if frame["descriptor"] is not None:
                    key = "direct_array_item_count" if frame["type"] == "array" else "direct_map_member_count"
                    frame["descriptor"][key] = frame["items"]
                    descriptors.append(frame["descriptor"])
            elif event in ("string", "number", "boolean", "null"):
                scalar_count += 1
                if not stack:
                    raise ContextInspectionError("scalar outside JSON container")
                frame = stack[-1]
                path = scalar_path(frame)
                if frame["type"] == "array":
                    frame["items"] += 1
                else:
                    frame["pending"] = None
                leaf = path.rsplit(".", 1)[-1]
                if leaf in wanted and len(selected) < 256:
                    selected[path] = str(value)
    if stack:
        raise ContextInspectionError("truncated phase diagnostics JSON")
    return {"top_level_members": descriptors, "top_level_members_truncated": descriptors_truncated,
            "scalar_count": scalar_count, "container_count": container_count,
            "selected_small_fields": selected,
            "status": "present/exported/structurally counted/semantically unvalidated"}


def event_scalars(events):
    """Extract selected top-level scalar occurrences without rebuilding a row."""
    wanted = {"clock", "cursor", "provisional", "day_index", "interval_index", "capture_phase", "start_ns", "end_ns"}
    result, depth, pending = {}, 0, None
    with events.open() as raw:
        for line in raw:
            item = json.loads(line)
            event, value = item.get("event"), item.get("value")
            if event == "map_key":
                pending = value if depth == 1 else None
            elif event in ("start_map", "start_array"):
                depth += 1; pending = None
            elif event in ("end_map", "end_array"):
                depth -= 1; pending = None
            elif pending in wanted and event in ("string", "number", "boolean", "null"):
                if isinstance(value, dict) and "number_lexeme" in value:
                    value = value["number_lexeme"]
                result.setdefault(pending, []).append(value)
                pending = None
    return result


def inspect(export, destination):
    summary = json.loads((export / "summary.json").read_text())
    if summary.get("status") != "COMPLETE":
        raise ContextInspectionError("export is not COMPLETE")
    if destination.exists():
        raise ContextInspectionError("destination exists")
    destination.mkdir(parents=True)
    result = {"status": "INCOMPLETE", "export_summary_sha256": sha256(export / "summary.json"),
              "rows": [], "limits": "structural custody only; semantically unvalidated"}
    try:
        for row in summary["rows"]:
            events = inside(export, row["event_file"])
            if events.stat().st_size != row["event_file_bytes"] or sha256(events) != row["event_file_sha256"]:
                raise ContextInspectionError(f"event file custody mismatch ordinal {row['ordinal']}")
            row_dir = destination / str(row["ordinal"]); row_dir.mkdir()
            members = []
            for marker in row.get("external_json", []):
                source = inside(export, marker["file"])
                if source.stat().st_size != marker["bytes"] or sha256(source) != marker["sha256"]:
                    raise ContextInspectionError(f"external JSON custody mismatch ordinal {row['ordinal']}")
                path = marker["path"]
                if not isinstance(path, list) or len(path) != 1 or not isinstance(path[0], dict):
                    raise ContextInspectionError("external JSON marker has no top-level member position")
                name = path[0]["member"]
                common = {"member": name, "position": path[0]["position"], "source_file": marker["file"],
                          "source_bytes": marker["bytes"], "source_sha256": marker["sha256"]}
                if name.endswith("_canonical_json") or name.endswith("_typed_bytes"):
                    binary = row_dir / f"{common['position']:04d}-{name}.bin"
                    decoded = stream_u8_json(source, binary, typed=name.endswith("_typed_bytes"))
                    if decoded["bytes"] != binary.stat().st_size or decoded["sha256"] != sha256(binary):
                        raise ContextInspectionError("decoded byte receipt mismatch")
                    members.append({**common, "decoded_file": binary.name, "byte_decode": "strict u8 streamed",
                                    "status": "present/exported/byte-decoded/structurally checked/semantically unvalidated",
                                    **decoded, **inner_json_summary(binary)})
                elif name == "phase_diagnostics":
                    members.append({**common, **profile_large_json(source)})
                else:
                    members.append({**common, "status": "present/exported/not decoded/semantically unvalidated"})
            role = row.get("role", "")
            result["rows"].append({"physical_ordinal": row["ordinal"], "kind": row["kinds"], "role": role,
                                   "top_level_scalars": event_scalars(events), "members": members,
                                   "status": "present/exported/structurally checked/semantically unvalidated"})
        result["operand_joins"] = structural_joins(result["rows"], destination)
        result["status"] = "COMPLETE"
    except Exception as error:
        result["error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        (destination / "summary.json").write_text(json.dumps(result, indent=2, default=str) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("export", type=Path)
    parser.add_argument("destination", type=Path)
    arguments = parser.parse_args()
    print(json.dumps({"status": inspect(arguments.export, arguments.destination)["status"]}))
