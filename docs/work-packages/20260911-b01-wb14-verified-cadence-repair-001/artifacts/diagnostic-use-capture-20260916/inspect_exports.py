#!/usr/bin/env python3
"""File-backed structural inspection of a COMPLETE diagnostic export only.

This deliberately does not invoke restoration, a native importer, or a model.
It verifies retained bytes, shards each row by top-level member, and reports
availability as exported/structurally checked/semantically unvalidated.
"""
import argparse
from decimal import Decimal
import hashlib
import json
from pathlib import Path


class InspectionError(Exception):
    pass


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as raw:
        for chunk in iter(lambda: raw.read(65536), b""):
            value.update(chunk)
    return value.hexdigest()


def exported_path(export, relative):
    path = (export / relative).resolve()
    if export.resolve() not in path.parents:
        raise InspectionError("external path escapes export directory")
    return path


def no_duplicate_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise InspectionError("duplicate member blocks semantic decode")
        result[key] = value
    return result


def decode_small_json(path, limit=1 << 20):
    if path.stat().st_size > limit:
        raise InspectionError("member exceeds bounded decode limit")
    with path.open() as raw:
        return json.load(raw, parse_int=Decimal, parse_float=Decimal,
                         object_pairs_hook=no_duplicate_object)


def decode_small_member(path, limit=1 << 20):
    if path.stat().st_size > limit:
        raise InspectionError("member exceeds bounded decode limit")
    stack, keys, result = [], [], None
    for line in path.open():
        item = json.loads(line)
        if "value" not in item:
            continue
        event, value = item["event"], item["value"]
        if event == "map_key":
            if stack and isinstance(stack[-1], dict) and value in stack[-1]:
                raise InspectionError("duplicate member blocks semantic decode")
            if keys:
                keys[-1] = value
            continue
        if event in ("start_map", "start_array"):
            value = {} if event == "start_map" else []
        elif event == "number":
            value = int(value["number_lexeme"])
        elif event in ("end_map", "end_array"):
            stack.pop(); keys.pop(); continue
        if not stack:
            result = value
        elif isinstance(stack[-1], list):
            stack[-1].append(value)
        else:
            if keys[-1] in stack[-1]:
                raise InspectionError("duplicate member blocks semantic decode")
            stack[-1][keys[-1]] = value
        if event in ("start_map", "start_array"):
            stack.append(value); keys.append(None)
    if stack:
        raise InspectionError("truncated member event shard")
    return result


def inspect(export, destination):
    summary = json.loads((export / "summary.json").read_text())
    if summary.get("status") != "COMPLETE":
        raise InspectionError("export is not COMPLETE")
    if destination.exists():
        raise InspectionError("destination exists")
    destination.mkdir(parents=True)
    result = {"status": "INCOMPLETE", "export_summary_sha256": digest(export / "summary.json"),
              "rows": [], "limits": "structural/custody inspection only; semantically unvalidated"}
    try:
        for row in summary["rows"]:
            events = export / row["event_file"]
            if events.stat().st_size != row["event_file_bytes"] or digest(events) != row["event_file_sha256"]:
                raise InspectionError(f"event hash mismatch ordinal {row['ordinal']}")
            row_dir = destination / str(row["ordinal"])
            row_dir.mkdir()
            member, handle, depth, members = None, None, 0, []
            with events.open() as raw:
                for line in raw:
                    event = json.loads(line)
                    top = {(item["member"], item["position"]) for item in row.get("members", [])}
                    if event["event"] == "map_key" and depth == 1 and event.get("path") and (event["value"], event["path"][0]["position"]) in top:
                        if handle:
                            handle.close()
                        member = event["value"]
                        position = event["path"][0]["position"]
                        name = f"{position:04d}-{member}.events.jsonl"
                        handle = (row_dir / name).open("x")
                        members.append({"member": member, "position": position, "file": name})
                    if handle and not (event["event"] == "end_map" and depth == 1):
                        handle.write(line)
                    if event["event"] in ("start_map", "start_array"):
                        depth += 1
                    elif event["event"] in ("end_map", "end_array"):
                        depth -= 1
            if handle:
                handle.close()
            if len({(item["member"], item["position"]) for item in members}) != len(members):
                raise InspectionError(f"duplicate member-position ordinal {row['ordinal']}")
            external = []
            external_json = []
            decoded_members = []
            for item in members:
                if not (item["member"].endswith("_canonical_json") or item["member"].endswith("_typed_bytes")):
                    continue
                shard = row_dir / item["file"]
                try:
                    value = decode_small_member(shard)
                    if item["member"].endswith("_typed_bytes"):
                        if not isinstance(value, dict) or list(value) != ["Ok"] or not isinstance(value["Ok"], list):
                            raise InspectionError("typed bytes must be {'Ok':[...]}" )
                        values = value["Ok"]
                    else:
                        values = value
                    if not isinstance(values, list) or any(type(byte) is not int or not 0 <= byte <= 255 for byte in values):
                        raise InspectionError("explicit byte field has non-byte value")
                    path = row_dir / (item["file"] + ".bin")
                    path.write_bytes(bytes(values))
                    decoded_members.append({"member": item["member"], "source_event_file": item["file"], "decoded_file": path.name,
                                            "bytes": len(values), "sha256": digest(path), "status": "decoded/structurally checked/semantically unvalidated"})
                except InspectionError as error:
                    decoded_members.append({"member": item["member"], "source_event_file": item["file"], "status": "present/exported/not decoded", "limitation": str(error)})
            for marker in row.get("external_byte_arrays", []):
                payload = exported_path(export, marker["file"])
                if payload.stat().st_size != marker["bytes"] or digest(payload) != marker["sha256"]:
                    raise InspectionError(f"external byte hash mismatch ordinal {row['ordinal']}")
                external.append({"path": marker["path"], "file": marker["file"], "bytes": marker["bytes"], "sha256": marker["sha256"], "status": "exported; structural byte hash checked; semantically unvalidated"})
            for marker in row.get("external_json", []):
                payload = exported_path(export, marker["file"])
                if payload.stat().st_size != marker["bytes"] or digest(payload) != marker["sha256"]:
                    raise InspectionError(f"external JSON hash mismatch ordinal {row['ordinal']}")
                starts, ends = [], []
                with events.open() as raw:
                    for line in raw:
                        event = json.loads(line)
                        if event.get("event") == "external_json_start" and event.get("file") == marker["file"]:
                            starts.append(event)
                        if event.get("event") == "external_json_end" and event.get("file") == marker["file"]:
                            ends.append(event)
                if len(starts) != 1 or len(ends) != 1 or starts[0].get("path") != marker["path"] or ends[0] != marker:
                    raise InspectionError(f"external JSON marker mismatch ordinal {row['ordinal']}")
                item = {"path": marker["path"], "file": marker["file"], "bytes": marker["bytes"],
                        "sha256": marker["sha256"], "member_position": marker["path"][0],
                        "status": "present/exported/structurally checked/semantically unvalidated"}
                try:
                    decoded = decode_small_json(payload)
                    item.update(decoded_type=type(decoded).__name__, number_decoding="exact Decimal",
                                status="present/exported/decoded/structurally checked/semantically unvalidated")
                except InspectionError as error:
                    item.update(status="present/exported/not decoded/semantically unvalidated", limitation=str(error))
                external_json.append(item)
            archive_check = None
            if row["kinds"] == ["b01_wb14_committed_day_archive_record_v1"]:
                entry = next((row_dir / item["file"] for item in members if item["member"] == "archive_entry"), None)
                if entry is None or len(external) != 1:
                    raise InspectionError(f"archive entry/binary missing members={members} external={len(external)}")
                decoded = decode_small_member(entry)
                if decoded.get("canonical_uncompressed_len") != external[0]["bytes"] or decoded.get("content_sha256") != external[0]["sha256"]:
                    raise InspectionError("archive entry length/hash differs from binary")
                archive_check = {"day_index": next((item["member"] for item in members if item["member"] == "day_index"), None),
                                 "canonical_uncompressed_len": decoded["canonical_uncompressed_len"], "content_sha256": decoded["content_sha256"], "status": "structurally checked; semantically unvalidated"}
            result["rows"].append({"physical_ordinal": row["ordinal"], "kind": row["kinds"], "role": row["role"],
                                   "top_level_members": members, "external_byte_arrays": external,
                                   "external_json": external_json,
                                   "decoded_members": decoded_members,
                                   "archive_integrity": archive_check,
                                   "status": "present/exported/structurally checked/semantically unvalidated"})
        result["status"] = "COMPLETE"
    except Exception as error:
        result["error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        (destination / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("export", type=Path)
    parser.add_argument("destination", type=Path)
    arguments = parser.parse_args()
    print(json.dumps({"status": inspect(arguments.export, arguments.destination)["status"]}))
