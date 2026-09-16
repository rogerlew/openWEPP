#!/usr/bin/env python3
"""Focused envelope test: exact decimals, duplicate keys, huge arrays and matches."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile


HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("export_selected", HERE / "export_selected.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
inventory_spec = importlib.util.spec_from_file_location(
    "inventory", HERE.parent / "stream-inventory-20260914" / "inventory.py"
)
inventory_module = importlib.util.module_from_spec(inventory_spec)
inventory_spec.loader.exec_module(inventory_module)
inspect_spec = importlib.util.spec_from_file_location("inspect_exports", HERE / "inspect_exports.py")
inspect_module = importlib.util.module_from_spec(inspect_spec)
inspect_spec.loader.exec_module(inspect_module)


def main():
    with tempfile.TemporaryDirectory() as temporary:
        root = Path(temporary)
        source = root / "observation.json"
        huge = ",".join(str(number) for number in range(150_000))
        binary_values = ",".join("7" for _ in range(70_000))
        binary_hash = hashlib.sha256(bytes((7,)) * 70_000).hexdigest()
        source.write_text(
            '{"physical":['
            '{"kind":"noise","numbers":[' + huge + ']},'
            '{"kind":"b01_wb14_snow_free_pre_child_current_context_v1","day_index":4,"interval_index":22,'
            '"capture_phase":"snow_free","support":{"start_ns":"385920000000000","end_ns":"385980000000000"},"prepared_support_constructor_operands":{"support":{"start_ns":"385920000000000","end_ns":"385980000000000","duration":1.2300}},"duplicate":"a","duplicate":"b","duplicate_nested":{"same":"a","same":"b","decimal":1.2300},"phase_diagnostics":[' + huge + ']},'
            '{"kind":"b01_wb14_pre_child_current_context_capture_error_v1","day_index":4,"interval_index":22,"capture_phase":"snow_free","support":{"start_ns":"385920000000000","end_ns":"385980000000000"},"detail":"kept"},'
            '{"kind":"b01_wb14_prepared_day_current_context_v1","day_index":4},'
            '{"archive_entry":{"canonical_uncompressed_len":70000,"content_sha256":"' + binary_hash + '"},"canonical_record":[' + binary_values + '],"capture":"x","day_index":0,"kind":"b01_wb14_committed_day_archive_record_v1"},'
            '{"archive_entry":{"canonical_uncompressed_len":2,"content_sha256":"06eb7d6a69ee19e5fbdf749018d3d2abfa04bcbd1365db312eb86dc7169389b8"},"canonical_record":[0,255],"day_index":1,"kind":"b01_wb14_committed_day_archive_record_v1"},'
            '{"archive_entry":{"canonical_uncompressed_len":2,"content_sha256":"06eb7d6a69ee19e5fbdf749018d3d2abfa04bcbd1365db312eb86dc7169389b8"},"canonical_record":[0,255],"day_index":2,"kind":"b01_wb14_committed_day_archive_record_v1"},'
            '{"archive_entry":{"canonical_uncompressed_len":2,"content_sha256":"06eb7d6a69ee19e5fbdf749018d3d2abfa04bcbd1365db312eb86dc7169389b8"},"canonical_record":[0,255],"day_index":3,"kind":"b01_wb14_committed_day_archive_record_v1"},'
            '{"kind":"surface_liquid_wb14_cadence_failure","day_index":4,"interval_index":22,"guard_typed_bytes":{"Ok":[1,2]}}'
            ']}'
        )
        digest = hashlib.sha256(source.read_bytes()).hexdigest()
        inventory = root / "inventory"
        inventory.mkdir()
        census = inventory_module.scan(source, inventory / "records.jsonl", digest, source.stat().st_size, 9)
        (inventory / "summary.json").write_text(json.dumps(census))
        result = module.export(source, inventory, root / "out", 64 * 1024 * 1024, 0)
        assert result["status"] == "COMPLETE"
        assert result["physical_rows"] == 9
        assert result["source_sha256"] == hashlib.sha256(source.read_bytes()).hexdigest()
        roles = [row["role"] for row in result["rows"]]
        assert "target_success" in roles and "target_capture_error" in roles
        assert sum(row["kinds"] == ["b01_wb14_committed_day_archive_record_v1"] for row in result["rows"]) == 4
        success = next(row for row in result["rows"] if row["role"] == "target_success")
        assert success["ordinal"] == 1
        events = (root / "out" / success["event_file"]).read_text()
        external = success["external_json"][0]
        assert "1.2300" in (root / "out" / external["file"]).read_text()
        nested = next(item for item in success["external_json"] if item["path"][0]["member"] == "duplicate_nested")
        assert (root / "out" / nested["file"]).read_text().count('"same"') == 2
        assert events.count('"value":"duplicate"') == 2
        archive = next(row for row in result["rows"] if row["ordinal"] == 4)
        payload = archive["external_byte_arrays"][0]
        assert (root / "out" / payload["file"]).read_bytes() == bytes((7,)) * 70_000
        inspection = inspect_module.inspect(root / "out", root / "inspection")
        assert inspection["status"] == "COMPLETE"
        inspected_success = next(row for row in inspection["rows"] if row["physical_ordinal"] == success["ordinal"])
        assert any(item["status"] == "present/exported/not decoded/semantically unvalidated" for item in inspected_success["external_json"])
        assert any(item.get("number_decoding") == "exact Decimal" for item in inspected_success["external_json"])
        assert result["external_byte_bytes"] >= 70_006
        assert result["selected_total_bytes"] == result["selected_event_bytes"] + result["external_json_bytes"] + result["external_byte_bytes"]
        assert not (root / "out" / "rows" / "0.events.jsonl").exists()
        try:
            module.export(source, inventory, root / "too-small", 1, 0)
        except module.ExportError:
            partial = json.loads((root / "too-small" / "summary.json").read_text())
            assert partial["status"] == "INCOMPLETE"
        else:
            raise AssertionError("too-small export limit unexpectedly passed")
        try:
            module.export(source, inventory, root / "generic-subtree-limit", 4096, 0)
        except module.ExportError:
            partial = json.loads((root / "generic-subtree-limit" / "summary.json").read_text())
            assert partial["external_json_bytes"] > 0
            assert partial["selected_total_bytes"] <= 4096
        else:
            raise AssertionError("generic subtree output limit unexpectedly passed")
        bad = root / "bad.json"
        bad.write_text(source.read_text().replace('"canonical_record":[7,7,7', '"canonical_record":[256,7,7', 1))
        bad_inventory = root / "bad-inventory"; bad_inventory.mkdir()
        bad_hash = hashlib.sha256(bad.read_bytes()).hexdigest()
        bad_census = inventory_module.scan(bad, bad_inventory / "records.jsonl", bad_hash, bad.stat().st_size, 9)
        (bad_inventory / "summary.json").write_text(json.dumps(bad_census))
        try:
            module.export(bad, bad_inventory, root / "bad-out", 64 * 1024 * 1024, 0)
        except module.ExportError:
            assert json.loads((root / "bad-out" / "summary.json").read_text())["status"] == "INCOMPLETE"
        else:
            raise AssertionError("invalid byte unexpectedly passed")
        (root / "out" / external["file"]).write_text("[]")
        try:
            inspect_module.inspect(root / "out", root / "corrupt-inspection")
        except inspect_module.InspectionError:
            pass
        else:
            raise AssertionError("external JSON integrity corruption unexpectedly passed")
        print(json.dumps({"status": "PASS", "selected_rows": len(result["rows"]), "source_bytes": source.stat().st_size}))


if __name__ == "__main__":
    main()
