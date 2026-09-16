#!/usr/bin/env python3
"""Bounded, file-backed export of B01/WB14 diagnostic rows from `physical`.

Rows are retained as ordered ijson events, never decoded/re-encoded as JSON
objects.  Number events are emitted as Decimal lexemes and map-member positions
are retained, so duplicate keys cannot be silently collapsed.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import sys
import time

import ijson

LIMIT = 1 << 30
DAY, INTERVAL, START_NS, END_NS = "4", "22", "385920000000000", "385980000000000"
SUCCESS = "b01_wb14_snow_free_pre_child_current_context_v1"
ERROR = "b01_wb14_pre_child_current_context_capture_error_v1"
ARCHIVE = "b01_wb14_committed_day_archive_record_v1"


class ExportError(Exception):
    pass


class OutputBudget:
    """Reserve every output byte before the write that creates it."""
    def __init__(self, maximum):
        self.maximum = maximum
        self.event_bytes = 0
        self.external_json_bytes = 0
        self.external_byte_bytes = 0

    @property
    def total(self):
        return self.event_bytes + self.external_json_bytes + self.external_byte_bytes

    def charge(self, category, size):
        if size < 0 or self.total + size > self.maximum:
            raise ExportError("selected export exceeds explicit disk limit")
        if category == "event":
            self.event_bytes += size
        elif category == "external_json":
            self.external_json_bytes += size
        elif category == "external_byte":
            self.external_byte_bytes += size
        else:
            raise ExportError("unknown output budget category")


class JsonSink:
    """Small streaming JSON writer; numbers are supplied as original Decimal text."""
    def __init__(self, path, budget):
        self.raw, self.digest, self.count, self.stack = path.open("xb"), hashlib.sha256(), 0, []
        self.budget = budget
    def put(self, data):
        self.budget.charge("external_json", len(data))
        self.raw.write(data); self.digest.update(data); self.count += len(data)
    def prefix(self):
        if not self.stack: return
        frame = self.stack[-1]
        if frame["map"]:
            if frame["key"] is None: raise ExportError("map value without key")
            if not frame["first"]: self.put(b",")
            self.put(json.dumps(frame["key"], ensure_ascii=True, separators=(",", ":")).encode()+b":")
            frame["key"] = None; frame["first"] = False
        else:
            if not frame["first"]: self.put(b",")
            frame["first"] = False
    def event(self, event, value):
        if event == "map_key": self.stack[-1]["key"] = value; return
        if event == "start_map": self.prefix(); self.put(b"{"); self.stack.append({"map":True,"first":True,"key":None}); return
        if event == "start_array": self.prefix(); self.put(b"["); self.stack.append({"map":False,"first":True,"key":None}); return
        if event == "end_map": self.put(b"}"); self.stack.pop(); return
        if event == "end_array": self.put(b"]"); self.stack.pop(); return
        self.prefix(); self.put((str(value) if event == "number" else json.dumps(value, ensure_ascii=True, separators=(",", ":"))).encode())
    def close(self):
        if self.stack: raise ExportError("truncated external JSON subtree")
        self.raw.close(); return self.count, self.digest.hexdigest()


class Reader:
    def __init__(self, raw):
        self.raw, self.digest, self.count, self.eof = raw, hashlib.sha256(), 0, False

    def read(self, size=-1):
        if size < 0:
            raise ExportError("unbounded parser read rejected")
        data = self.raw.read(size)
        self.digest.update(data)
        self.count += len(data)
        if size and not data:
            self.eof = True
        return data


def stat_id(stat):
    return {key: getattr(stat, key) for key in ("st_dev", "st_ino", "st_size", "st_mtime_ns", "st_ctime_ns")}


def file_hash(path):
    digest = hashlib.sha256()
    with path.open("rb") as raw:
        for chunk in iter(lambda: raw.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def encoded(event, value):
    # ijson supplies numbers as Decimal with use_float=False.  Do not convert.
    if event == "number":
        return {"number_lexeme": str(value), "encoding": "exact Decimal event"}
    return value


def wanted(meta):
    kinds = meta["kinds"]
    day, interval = meta["scalars"].get("day_index"), meta["scalars"].get("interval_index")
    support = meta["support_scalars"]
    coordinate = (day == DAY and interval == INTERVAL and meta["scalars"].get("capture_phase") == "snow_free"
                  and support.get("start_ns") == START_NS and support.get("end_ns") == END_NS)
    counts = {name: sum(member["member"] == name for member in meta["members"])
              for name in ("kind", "day_index", "interval_index", "capture_phase", "support", "prepared_support_constructor_operands")}
    support_counts = {name: meta["support_members"].count(name) for name in ("start_ns", "end_ns")}
    exact_target_members = all(counts[name] == 1 for name in ("kind", "day_index", "interval_index", "capture_phase", "support")) and all(support_counts[name] == 1 for name in support_counts)
    has_support = counts["prepared_support_constructor_operands"] == 1
    meta["selector_member_multiplicity"] = {**counts, **{"support." + name: count for name, count in support_counts.items()}}
    if not exact_target_members:
        meta["ambiguous_selector_members"] = True
    success = kinds == [SUCCESS] and coordinate and has_support and exact_target_members
    error = kinds == [ERROR] and coordinate and exact_target_members
    related = any(kind.startswith("b01_wb14_") or kind in ("surface_liquid_wb14_cadence_failure", "surface_liquid_wb14_cadence_caller_failure") for kind in kinds)
    return success or error or related, ("target_success" if success else "target_capture_error" if error else "related_b01")


def inventory_ordinals(inventory):
    summary = json.loads((inventory / "summary.json").read_text())
    if summary.get("status") != "COMPLETE":
        raise ExportError("inventory is not COMPLETE")
    records_path = inventory / "records.jsonl"
    records_bytes = records_path.read_bytes()
    rows, seen_ordinals, kinds_by_ordinal = set(), set(), {}
    count = 0
    for line in records_bytes.decode().splitlines():
        record = json.loads(line)
        ordinal = record.get("physical_ordinal")
        if (record.get("source_sha256") != summary.get("sha256_supplied") or record.get("provisional") is not True
                or type(ordinal) is not int or ordinal < 0 or ordinal >= summary["physical_rows"] or ordinal in seen_ordinals):
            raise ExportError("inventory records custody/ordinal validation failed")
        seen_ordinals.add(ordinal)
        count += 1
        kinds = record.get("kinds", [])
        kinds_by_ordinal[ordinal] = kinds
        if any(kind.startswith("b01_wb14_") or kind in ("surface_liquid_wb14_cadence_failure", "surface_liquid_wb14_cadence_caller_failure") for kind in kinds):
            rows.add(ordinal)
    if count != summary.get("selected_rows"):
        raise ExportError("inventory records count differs from COMPLETE summary")
    return summary, rows, kinds_by_ordinal, {"path": str(records_path), "bytes": len(records_bytes), "sha256": hashlib.sha256(records_bytes).hexdigest(), "records": count}


def export(source, inventory, destination, max_export_bytes, reserve_bytes):
    if max_export_bytes <= 0 or reserve_bytes < 0:
        raise ExportError("invalid disk limits")
    if destination.exists():
        raise ExportError("destination already exists")
    census, selected_ordinals, census_kinds, records_custody = inventory_ordinals(inventory)
    source_stat = source.stat()
    census_hash = census.get("sha256_supplied")
    if census_hash != census.get("expected_sha256") or census.get("expected_bytes") != source_stat.st_size:
        raise ExportError("inventory source custody is incomplete or mismatched")
    free = shutil.disk_usage(destination.parent).free
    required = max_export_bytes + reserve_bytes
    if free < required:
        raise ExportError(f"insufficient free disk: {free} < required {required}")
    destination.mkdir(parents=True)
    rows_dir, spool_dir = destination / "rows", destination / ".spool"
    rows_dir.mkdir()
    spool_dir.mkdir()
    summary = {"status": "INCOMPLETE", "source": str(source), "source_stat_before": stat_id(source_stat),
               "inventory": str(inventory), "inventory_records": records_custody, "inventory_source_sha256": census_hash, "inventory_physical_rows": census["physical_rows"], "inventory_selected_ordinals": sorted(selected_ordinals), "max_export_bytes": max_export_bytes, "reserve_bytes": reserve_bytes, "free_bytes_before": free,
               "format": "one ordered event JSONL file per physical ordinal; number_lexeme is exact Decimal text",
               "selection": {"day_index": 4, "interval_index": 22, "capture_phase": "snow_free", "support_start_ns": int(START_NS), "support_end_ns": int(END_NS), "success_kind": SUCCESS, "capture_error_kind": ERROR,
                             "related": "all b01_wb14 rows plus named cadence guard/caller failures; no ordinal-based selection"},
               "rows": [], "physical_rows": 0, "syntax_complete": False, "eof": False}
    receipt = destination / "summary.json"
    receipt.write_text(json.dumps(summary, indent=2) + "\n")
    started, current, row_file, reader = time.monotonic(), None, None, None
    budget = OutputBudget(max_export_bytes)
    stack, root_key, physical_seen, root_done = [], None, 0, False

    def write_row_line(item):
        if row_file is None:
            raise ExportError("selected row event without spool")
        data = (json.dumps(item, separators=(",", ":")) + "\n").encode()
        budget.charge("event", len(data))
        row_file.write(data.decode())
    try:
        with source.open("rb") as raw:
            before_fd = stat_id(os.fstat(raw.fileno()))
            reader = Reader(raw)
            for event, value in ijson.basic_parse(reader, use_float=False, buf_size=65536):
                depth = len(stack)
                parent = stack[-1] if stack else None
                if parent and parent.get("external_json"):
                    sink = parent["external_json"]
                    sink.event(event, value)
                    if event in ("start_map", "start_array"):
                        stack.append({"type": event[6:], "key": None, "position": 0, "physical": False, "row": True, "external_json": sink})
                    elif event in ("end_map", "end_array"):
                        frame = stack.pop()
                        if frame.get("external_root"):
                            size, sha = sink.close()
                            marker = {"event":"external_json_end","path":frame["path"],"file":frame["file"],"bytes":size,"sha256":sha}
                            write_row_line(marker)
                            current.setdefault("external_json",[]).append(marker)
                    continue
                if event == "map_key":
                    if not parent or parent["type"] != "map":
                        raise ExportError("map key outside object")
                    parent["key"] = value
                    parent["position"] += 1
                    if depth == 1:
                        root_key = value
                    if current is not None:
                        current["members"].append({"member": value, "position": parent["position"] - 1}) if depth == 3 else None
                        if depth == 3 and value == "kind":
                            current["kind_positions"].append(parent["position"] - 1)
                    if row_file is not None:
                        write_row_line({"path": [{"member": value, "position": parent["position"] - 1}], "event": event, "value": value})
                    continue
                if parent and parent.get("byte_payload"):
                    if event == "number":
                        text = str(value)
                        if not text.isdigit() or int(text) > 255:
                            raise ExportError("canonical_record contains non-byte value")
                        budget.charge("external_byte", 1)
                        parent["buffer"].append(int(text))
                        if len(parent["buffer"]) == 65536:
                            parent["raw"].write(parent["buffer"])
                            parent["digest"].update(parent["buffer"])
                            parent["buffer"].clear()
                        parent["count"] += 1
                        continue
                    if event == "end_array":
                        if parent["buffer"]:
                            parent["raw"].write(parent["buffer"])
                            parent["digest"].update(parent["buffer"])
                        parent["raw"].close()
                        marker = {"event": "external_byte_array_end", "path": parent["path"], "file": parent["file"], "bytes": parent["count"], "sha256": parent["digest"].hexdigest()}
                        write_row_line(marker)
                        current.setdefault("external_byte_arrays", []).append(marker)
                        stack.pop()
                        continue
                    raise ExportError("canonical_record byte array contains non-number event")
                if depth == 0:
                    if event != "start_map" or root_done:
                        raise ExportError("expected exactly one root object")
                if depth == 1 and root_key == "physical" and event == "start_array":
                    physical_seen += 1
                    if physical_seen != 1:
                        raise ExportError("duplicate top-level physical member")
                if parent and parent.get("physical") and event == "start_map":
                    ordinal = summary["physical_rows"]
                    summary["physical_rows"] += 1
                    if ordinal in selected_ordinals:
                        current = {"ordinal": ordinal, "kinds": [], "kind_positions": [], "scalars": {}, "support_scalars": {}, "support_members": [], "members": []}
                        spool = spool_dir / f"{ordinal}.events.jsonl"
                        row_file = spool.open("x", encoding="utf-8")
                if (current is not None and event in ("start_map", "start_array") and depth == 3 and parent and parent["type"] == "map"
                        and parent.get("key") not in ("support", "capture", "canonical_record", "archive_entry")
                        and ARCHIVE not in census_kinds.get(current["ordinal"], [])):
                    member = {"member":parent["key"],"position":parent["position"]-1}
                    path = rows_dir / f"{current['ordinal']}.{member['position']}.json"
                    sink = JsonSink(path, budget); sink.event(event,value)
                    marker={"event":"external_json_start","path":[member],"file":str(path.relative_to(destination)),"encoding":"streamed JSON exact Decimal lexemes"}
                    write_row_line(marker)
                    stack.append({"type":event[6:],"key":None,"position":0,"physical":False,"row":True,"external_json":sink,"external_root":True,"file":marker["file"],"path":[member]})
                    continue
                if (current is not None and event == "start_array" and depth == 3 and parent and parent["type"] == "map"
                        and parent.get("key") == "canonical_record" and census_kinds.get(current["ordinal"]) == [ARCHIVE]):
                    member = {"member": "canonical_record", "position": parent["position"] - 1}
                    binary = rows_dir / f"{current['ordinal']}.canonical_record.bin"
                    marker = {"event": "external_byte_array_start", "path": [member], "file": str(binary.relative_to(destination)), "encoding": "source-defined Vec<u8>; streamed bytes"}
                    write_row_line(marker)
                    stack.append({"type": "array", "key": None, "position": 0, "context_field": "canonical_record", "physical": False, "row": True,
                                  "byte_payload": True, "raw": binary.open("xb"), "buffer": bytearray(), "digest": hashlib.sha256(), "count": 0, "file": marker["file"], "path": [member]})
                    continue
                if row_file is not None:
                    write_row_line({"path": [], "event": event, "value": encoded(event, value)})
                # Capture only selector fields at row top level. Duplicate values are
                # represented as lists, never resolved by last-value-wins.
                if current is not None and depth == 3 and parent and parent["type"] == "map":
                    key = parent.get("key")
                    if key in ("kind", "day_index", "interval_index", "capture_phase") and event in ("string", "number"):
                        scalar = str(value)
                        if key == "kind":
                            current["kinds"].append(scalar)
                        else:
                            current["scalars"].setdefault(key, scalar)
                            if current["scalars"][key] != scalar:
                                current.setdefault("duplicate_selector_values", {}).setdefault(key, []).append(scalar)
                if current is not None and depth == 4 and parent and parent["type"] == "map":
                    key = parent.get("key")
                    if (parent.get("context_field") == "support"
                            and key in ("start_ns", "end_ns") and event in ("string", "number")):
                        scalar = str(value)
                        current["support_members"].append(key)
                        current["support_scalars"].setdefault(key, scalar)
                        if current["support_scalars"][key] != scalar:
                            current.setdefault("duplicate_selector_values", {}).setdefault("support." + key, []).append(scalar)
                if event in ("start_map", "start_array"):
                    stack.append({"type": event[6:], "key": None, "position": 0, "context_field": parent.get("key") if parent else None,
                                  "physical": depth == 1 and root_key == "physical", "row": current is not None})
                elif event in ("end_map", "end_array"):
                    if not stack:
                        raise ExportError("unexpected container end")
                    frame = stack.pop()
                    if frame.get("physical"):
                        pass
                    if event == "end_map" and current is not None and depth == 3 and parent and parent.get("row"):
                        # This is the physical-row end; decide after all row events
                        # have reached its disk spool.
                        row_file.close()
                        keep, role = wanted(current)
                        spool = spool_dir / f"{current['ordinal']}.events.jsonl"
                        if keep:
                            size = spool.stat().st_size
                            final = rows_dir / spool.name
                            spool.replace(final)
                            current.update({"role": role, "event_file": str(final.relative_to(destination)), "event_file_bytes": size,
                                            "event_file_sha256": file_hash(final)})
                            summary["rows"].append(current)
                        else:
                            spool.unlink()
                        current, row_file = None, None
                    if depth == 1 and event == "end_map":
                        root_done = True
            after_fd = stat_id(os.fstat(raw.fileno()))
        after_path = stat_id(source.stat())
        summary.update(source_stat_after=after_path, source_fd_stat_after=after_fd,
                       source_sha256=reader.digest.hexdigest(), source_bytes=reader.count, eof=reader.eof,
                       syntax_complete=True, selected_event_bytes=budget.event_bytes,
                       external_json_bytes=budget.external_json_bytes,
                       external_byte_bytes=budget.external_byte_bytes,
                       selected_total_bytes=budget.total)
        if not root_done or stack or physical_seen != 1 or not reader.eof:
            raise ExportError("incomplete root/physical/EOF traversal")
        if (reader.digest.hexdigest() != census_hash or reader.count != census["expected_bytes"]
                or summary["physical_rows"] != census["physical_rows"]):
            raise ExportError("source/hash/row count differs from completed inventory")
        if {row["ordinal"] for row in summary["rows"]} != selected_ordinals:
            raise ExportError("selected census ordinal missing or semantically unclassifiable")
        if any(row["kinds"] != census_kinds[row["ordinal"]] for row in summary["rows"]):
            raise ExportError("streamed kind differs from completed census")
        if not any(row["role"] in ("target_success", "target_capture_error") for row in summary["rows"]):
            raise ExportError("current census has no complete target success or capture-error row")
        if before_fd != after_fd or before_fd != after_path:
            raise ExportError("source identity changed during export")
        summary["status"] = "COMPLETE"
    except Exception as error:
        summary["error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        if row_file is not None:
            row_file.close()
        for frame in stack:
            if frame.get("external_json") and not frame["external_json"].raw.closed:
                frame["external_json"].raw.close()
            if frame.get("byte_payload") and not frame["raw"].closed:
                frame["raw"].close()
        summary.update(selected_event_bytes=budget.event_bytes,
                       external_json_bytes=budget.external_json_bytes,
                       external_byte_bytes=budget.external_byte_bytes,
                       selected_total_bytes=budget.total)
        summary.update(elapsed_seconds=time.monotonic() - started,
                       peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                       rlimit_as_bytes=list(resource.getrlimit(resource.RLIMIT_AS)))
        receipt.write_text(json.dumps(summary, indent=2) + "\n")
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("inventory", type=Path, help="completed inventory.py output directory")
    parser.add_argument("destination", type=Path)
    parser.add_argument("--max-export-bytes", type=int, required=True)
    parser.add_argument("--reserve-bytes", type=int, default=1 << 30)
    args = parser.parse_args()
    soft, hard = resource.getrlimit(resource.RLIMIT_AS)
    if soft < 0 or hard < 0 or max(soft, hard) > LIMIT:
        parser.error("invoke under prlimit --as=1073741824:1073741824")
    result = export(args.source, args.inventory, args.destination, args.max_export_bytes, args.reserve_bytes)
    print(json.dumps({"status": result["status"], "physical_rows": result["physical_rows"], "selected_rows": len(result["rows"]), "source_sha256": result.get("source_sha256")}))


if __name__ == "__main__":
    main()
