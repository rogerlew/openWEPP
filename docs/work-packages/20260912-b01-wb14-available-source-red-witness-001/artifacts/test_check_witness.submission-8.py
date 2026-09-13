"""Named source-shaped controls for the captured boundary packet only."""
import copy, hashlib, json, struct, tempfile, unittest
from pathlib import Path
from check_witness import WitnessError, check, u128

HERE = Path(__file__).parent
PACKET = HERE / "current-run-boundary-packet.json"
RAW = Path("/workdir/openwepp-experiments/b01-wb14-available-source-red-witness/raw-capture-20260912-1")
BINARY = Path("/workdir/openwepp-experiments/b01-wb14-available-source-red-witness/artifacts/openwepp_runner-identity-stamped.frozen")
LISTING = Path("/workdir/openwepp-experiments/b01-wb14-available-source-red-witness/artifacts/compiled-listing-identity-stamped.stdout")

def row(p, i): return p["records"][i]["record"]
def typed(r, k): return json.loads(bytes(r[k]["Ok"]))
def put(r, k, x):
    encoded = {"Ok": list(json.dumps(x, separators=(",", ":")).encode())}
    if r.get(k) == encoded: raise AssertionError("typed mutation made no byte change")
    r[k] = encoded
def blob(p, i, k, edit):
    r = row(p, i); x = typed(r, k); old = json.dumps(x, separators=(",", ":")); edit(x)
    if json.dumps(x, separators=(",", ":")) == old: raise AssertionError("typed mutation made no value change")
    put(r, k, x)
def edit_parent(p, edit): return blob(p, 1, "parent_working_typed_bytes", edit)
def input_pair(p, edit):
    xs = [typed(row(p, i), "input_typed_bytes") for i in (0, 1)]
    old = copy.deepcopy(xs[0]); edit(xs[0]); edit(xs[1])
    if xs[0] == old or xs[0] != xs[1]: raise AssertionError("input mutation is not coherent")
    for i, x in enumerate(xs): put(row(p, i), "input_typed_bytes", x)
def flip(x, key):
    if not isinstance(x[key], list) or len(x[key]) != 32: raise AssertionError("expected digest")
    old = x[key][0]; x[key][0] ^= 1
    if x[key][0] == old: raise AssertionError("digest mutation made no change")
def cumulative_all(p, field, value):
    # All seven producer copies change together, so owner-copy equality cannot
    # mask the exact-positive-zero predicate.
    if value == 0.0:
        if struct.pack(">d", value) == struct.pack(">d", 0.0): raise AssertionError("signed-zero mutation lost its bit change")
    for i in (0, 1): blob(p, i, "beginning_typed_bytes", lambda x: x["continuations"][0].__setitem__(field, value))
    blob(p, 1, "working_typed_bytes", lambda x: x["continuations"][0].__setitem__(field, value))
    def owner(x):
        for name in ("persistent_beginning_state", "candidate_state"):
            x[name]["continuations"][0][field] = value
        a = x["per_ofe_authorities"]["ofe-1"]
        a["beginning_cursor"][field] = value; a["working"][field] = value
    edit_parent(p, owner)
def identity_path(path):
    def edit(p):
        def owner(x):
            target = x["per_ofe_authorities"]["ofe-1"]["authority"]
            for key in path[:-1]: target = target[key]
            flip(target, path[-1])
        edit_parent(p, owner)
    return edit

class WitnessCases(unittest.TestCase):
    def run_case(self, name, expected, packet_edit=None, receipt_edit=None, run_edit=None, listing_edit=None):
        p = json.loads(PACKET.read_text()); receipt = json.loads((RAW / "receipt.json").read_text()); run = json.loads((RAW / "run.json").read_text())
        before = (copy.deepcopy(p), copy.deepcopy(receipt), copy.deepcopy(run))
        if packet_edit: packet_edit(p)
        if receipt_edit: receipt_edit(receipt)
        if run_edit: run_edit(run)
        if packet_edit: self.assertNotEqual(p, before[0], name + ": packet did not change")
        if receipt_edit: self.assertNotEqual(receipt, before[1], name + ": receipt did not change")
        if run_edit: self.assertNotEqual(run, before[2], name + ": run did not change")
        with tempfile.TemporaryDirectory() as td:
            d = Path(td); pp, rp, up, lp = d / "packet", d / "receipt", d / "run", d / "listing"
            pp.write_text(json.dumps(p, separators=(",", ":")))
            if run_edit:
                up.write_text(json.dumps(run, separators=(",", ":")))
                receipt["runner_receipt_sha256"] = hashlib.sha256(up.read_bytes()).hexdigest()
            else: up.write_text((RAW / "run.json").read_text())
            rp.write_text(json.dumps(receipt, separators=(",", ":")))
            lp.write_text(LISTING.read_text() if listing_edit is None else listing_edit(LISTING.read_text()))
            if expected == "positive":
                self.assertEqual(check(pp, rp, up, BINARY, lp)["witness_verdict"], "LIMITED_CHECKER_PASS_PENDING_REVIEW", name); return
            with self.assertRaises(WitnessError, msg=name) as caught: check(pp, rp, up, BINARY, lp)
            self.assertEqual(caught.exception.code, expected, name)

    def test_named_finite_case_table(self):
        cases = [
            ("current_packet_positive", "positive"), ("missing_record", "records", lambda p: p["records"].pop()),
            ("wrong_boundary_valid_json", "input", lambda p: input_pair(p, lambda x: x.__setitem__("day_index", 5))),
            ("raw_pair_mismatch", "input_pair", lambda p: blob(p, 1, "input_typed_bytes", lambda x: x.__setitem__("interval_index", 23))),
            ("capture_mismatch", "capture", lambda p: p["records"][1]["record"]["capture"].__setitem__("process", 0)),
            ("overflow_observation_loss", "observation", lambda p: p["top_level_scalars"].__setitem__("overflow", True)),
            ("poisoned_observation_loss", "observation", lambda p: p["top_level_scalars"].__setitem__("counts_poisoned", True)),
            ("truncated_input", "typed", lambda p: row(p, 0)["input_typed_bytes"]["Ok"].pop()), ("generic_panic", "error", lambda p: row(p, 1).__setitem__("error", "panic")),
            ("timeout_only", "receipt", None, lambda r: r.__setitem__("timeout", True)), ("wrong_binary", "receipt", None, lambda r: r.__setitem__("binary_sha256", "0" * 64)),
            ("zero_exit", "receipt", None, lambda r: r.__setitem__("exit_code", 0)), ("infrastructure", "receipt", None, lambda r: r.__setitem__("infrastructure_error", "x")),
            ("run_hash", "receipt", None, lambda r: r.__setitem__("runner_receipt_sha256", "0" * 64)), ("artifact_hash", "receipt", None, lambda r: r["artifact_sha256"].__setitem__("observations.json", "0" * 64)),
            ("wrong_argv", "selection", None, lambda r: r.__setitem__("argv", [])), ("zero_listing", "selection", None, None, None, lambda _: "\n0 tests, 0 benchmarks\n"),
            ("missing_beginning", "typed", lambda p: row(p, 0).pop("beginning_typed_bytes")), ("missing_parent", "typed", lambda p: row(p, 1).pop("parent_working_typed_bytes")),
            ("missing_working", "typed", lambda p: row(p, 1).pop("working_typed_bytes")), ("missing_ofe", "ofe", lambda p: edit_parent(p, lambda x: x.__setitem__("per_ofe_authorities", {}))),
            ("extra_ofe", "ofe", lambda p: edit_parent(p, lambda x: x["per_ofe_authorities"].__setitem__("other", copy.deepcopy(x["per_ofe_authorities"]["ofe-1"])))),
            ("wrong_ofe_guard_scalar", "guard", lambda p: row(p, 0).__setitem__("ofe_id", "other")), ("support", "support", lambda p: row(p, 1)["coupled_binding"].__setitem__("parent_support_end_ns", "387000000000001")),
            ("authority_support_start", "support", lambda p: edit_parent(p, lambda x: x["per_ofe_authorities"]["ofe-1"]["authority"].__setitem__("support_start_ns", "385200000000001"))),
            ("ordinal", "ordinal", lambda p: edit_parent(p, lambda x: x["per_ofe_authorities"]["ofe-1"]["working"].__setitem__("next_child_ordinal", 1))),
            ("prior_receipt", "ordinal", lambda p: edit_parent(p, lambda x: x["per_ofe_authorities"]["ofe-1"].__setitem__("receipts", [{"x": 1}]))),
            ("upper_bound", "support", lambda p: row(p, 1)["coupled_binding"].__setitem__("proposed_upper_bound_s_bits", 0)),
            ("u128_out_of_domain", "input", lambda p: input_pair(p, lambda x: x.__setitem__("transaction_id", str(2**128)))),
        ]
        for state in ("beginning", "working"):
            for field in ("cumulative_supply_m", "cumulative_infiltration_m"):
                key = "beginning_typed_bytes" if state == "beginning" else "working_typed_bytes"; index = 0 if state == "beginning" else 1
                cases.append((f"{state}_{field}_owner_copy", "beginning" if state == "beginning" else "parent", lambda p, i=index, k=key, f=field: blob(p, i, k, lambda x: x["continuations"][0].__setitem__(f, 1.0))))
        for field in ("cumulative_supply_m", "cumulative_infiltration_m"):
            for label, value in (("nonzero", 1.0), ("negative_zero", -0.0), ("nonfinite", float("inf")), ("nonnumeric", "zero")):
                cases.append((f"coherent_{field}_{label}", "cumulative", lambda p, f=field, v=value: cumulative_all(p, f, v)))
        cases += [
            ("transaction_authority_only", "identity", identity_path(("coupled_parent_transaction_sha256",))),
            ("transaction_prefix_only", "identity", identity_path(("inactive_prefix", "coupled_parent_transaction_sha256"))),
            ("transaction_binding_only", "identity", lambda p: flip(row(p, 1)["coupled_binding"], "coupled_parent_transaction_sha256")),
            ("missing_accepted_slab", "identity", lambda p: row(p, 1)["coupled_binding"].pop("accepted_slab_sha256")),
            ("malformed_accepted_slab", "identity", lambda p: row(p, 1)["coupled_binding"].__setitem__("accepted_slab_sha256", [1] * 31)),
            ("zero_accepted_slab", "identity", lambda p: row(p, 1)["coupled_binding"].__setitem__("accepted_slab_sha256", [0] * 32)),
            ("missing_parent_id", "identity", lambda p: edit_parent(p, lambda x: x["per_ofe_authorities"]["ofe-1"]["authority"].pop("parent_id"))),
            ("malformed_parent_id", "identity", lambda p: edit_parent(p, lambda x: x["per_ofe_authorities"]["ofe-1"]["authority"].__setitem__("parent_id", [1] * 31))),
        ]
        for name, expected, *edits in cases:
            with self.subTest(case=name): self.run_case(name, expected, *edits)
        self.assertEqual(u128(str(2**127 + 17), "u128"), 2**127 + 17)

if __name__ == "__main__": unittest.main(verbosity=2)
