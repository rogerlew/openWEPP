"""Fail-closed reconstruction of the retained local parent chronology.

This checks the observed parent payload only.  It deliberately does not infer a
later WB14 parent-final replay, global checkpoint finalization, or clock commit.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re

from reconstruct_credits import reject_constant, require, unique_object

TAG = "B01_INTEGRATED_PARENT"
NS = 1_000_000_000
PARENT_END = 1_800 * NS
PREFIX_SUPPORTS = [(i * 60 * NS, (i + 1) * 60 * NS) for i in range(6)]
SUCCESSOR_SUPPORTS = [(360 * NS, 420 * NS), (420 * NS, PARENT_END)]


def support(value):
    return int(value["start_ns"]), int(value["end_ns"])


def records(raw):
    found = []
    pattern = re.compile(r"(?<![A-Z0-9_])" + TAG + r"(?=\s|$)")
    for line_number, line in enumerate(raw.decode().splitlines(), 1):
        match = pattern.search(line)
        if match:
            value = json.loads(line[match.end():].strip(), object_pairs_hook=unique_object,
                               parse_constant=reject_constant)
            require(isinstance(value, dict), f"{TAG} line {line_number}: expected object")
            found.append(value)
    require(len(found) == 1, f"{TAG}: expected exactly one record, got {len(found)}")
    return found[0]


def canonical_bytes(value):
    require(isinstance(value, list) and all(isinstance(byte, int) and 0 <= byte <= 255 for byte in value),
            "canonical JSON byte inventory")
    return json.loads(bytes(value), object_pairs_hook=unique_object, parse_constant=reject_constant)


def reconstruct(raw):
    record = records(raw)
    require(record["record_version"] == 1, "parent record version")
    require(record["boundary"] == "native mixed-phase accepted parent after complete terminal execution",
            "parent observation boundary")
    parent = record["parent"]
    fixture = parent["fixture_inputs"]
    require(fixture["parity_profile"] == "native-mixed-phase" and fixture["production_only"] is True,
            "native fixture identity")
    require(fixture["hard_boundary_ns"] == 60 * NS and support(fixture["parent_support"]) == (0, PARENT_END),
            "native parent support/boundary")
    require(parent["beginning_native_v3_resident_present"] is True and parent["beginning_native_v4_resident_present"] is True,
            "native residency at beginning")

    beginning_clock = canonical_bytes(parent["beginning_clock_canonical_json"])
    ending_clock = canonical_bytes(parent["ending_clock_canonical_json"])
    beginning_checkpoint = canonical_bytes(parent["beginning_parent_checkpoint_canonical_json"])
    ending_checkpoint = canonical_bytes(parent["ending_parent_checkpoint_canonical_json"])
    parent_id = beginning_clock["parent_transaction_id"]
    require(len(parent_id) == 64 and all(ch in "0123456789abcdef" for ch in parent_id), "parent identity shape")
    require(support(beginning_clock["parent_support"]) == (0, PARENT_END), "beginning clock parent support")
    require(support(ending_clock["parent_support"]) == (0, PARENT_END), "ending clock parent support")
    require(beginning_clock["parent_transaction_id"] == ending_clock["parent_transaction_id"] == parent_id,
            "clock parent identity join")
    require(beginning_clock["committed"] is False and ending_clock["committed"] is False,
            "local clock remains uncommitted")
    require(beginning_clock["accepted_until"] == "0" and ending_clock["accepted_until"] == str(PARENT_END),
            "clock advancement")
    require((beginning_clock["segment_ordinal"], beginning_clock["slab_ordinal"], beginning_clock["event_ordinal"]) == (0, 0, 0),
            "beginning clock ordinals")
    require((ending_clock["segment_ordinal"], ending_clock["slab_ordinal"], ending_clock["event_ordinal"]) == (2, 8, 2),
            "ending clock ordinals")
    require(beginning_checkpoint["parent_transaction_id"] == ending_checkpoint["parent_transaction_id"] == parent_id,
            "checkpoint parent identity join")
    require(beginning_checkpoint["finalized"] is False and ending_checkpoint["finalized"] is False,
            "local checkpoint remains unfinalized")

    slabs = parent["subslabs"]
    require(len(slabs) == 6, "six represented-prefix slabs")
    prefix_ids = []
    for index, slab in enumerate(slabs):
        core = slab["core_without_destination_receipts"]
        require(support(core["support"]) == PREFIX_SUPPORTS[index], "represented-prefix slab support/order")
        prefix_ids.append(core["accepted_slab_sha256"])
        destinations = slab["ordered_destination_receipts"]
        require([(row["ofe_id"], row["tile_id"]) for row in destinations] == [("ofe-1", "forest"), ("ofe-1", "open")],
                "represented-prefix destination inventory/order")

    groups = parent["event_groups"]
    require(len(groups) == 1, "one observed terminal event group")
    group = groups[0]
    require(group["ordinal"] == 0 and group["pre_active_lanes"] == [1] and group["post_active_lanes"] == [] and group["terminating_lanes"] == [1],
            "terminal event lane transition")
    require(group["tick"] == str(360 * NS), "terminal event tick")
    event = group["accepted_event_receipt"]
    require(event["ordinal"] == 0 and event["parent_transaction_id"] == parent_id and event["tick"] == group["tick"],
            "accepted event identity")
    require(len(group["candidates"]) == 1 and support(group["candidates"][0]["support"]) == PREFIX_SUPPORTS[-1],
            "terminal event candidate support")

    parcels = parent["terminal_parcels"]
    require(len(parcels) == 1, "one terminal parcel")
    parcel = parcels[0]
    require(parcel["posture"] == "Consumed" and parcel["parent_transaction_id"] == parent_id and support(parcel["support"]) == PREFIX_SUPPORTS[-1],
            "terminal parcel consumption/parent join")
    require(group["produced_unconsumed_parcel_digests"] == [parcel["parcel_digest"]], "event to consumed parcel join")
    require(group['produced_unconsumed_parcels'] == [{k:v for k,v in parcel.items() if k != 'posture'}],
            'complete produced-to-consumed parcel payload and inventory')

    successors = parent["snow_free_successor_receipts"]
    require(len(successors) == 2, "two snowfree successor receipts")
    successor_ids = []
    for index, receipt in enumerate(successors):
        require(receipt["successor_ordinal"] == index and support(receipt["support"]) == SUCCESSOR_SUPPORTS[index],
                "snowfree successor support/order")
        require(receipt["parent_transaction_id"] == parent_id, "snowfree successor parent join")
        successor_ids.append(receipt["accepted_slab_sha256"])
    require(successors[0]["receiver_pending"] is True and successors[1]["receiver_pending"] is False,
            "snowfree receiver progression")

    projection = parent["finalized_parent_canonical_projection"]
    require(projection["parent_transaction_id"] == parent_id, "final projection parent identity")
    owner_ids = {'bgc', 'hydrology', 'land_surface_energy', 'snow', 'soil_thermal', 'surface_liquid', 'vegetation'}
    def owners(rows):
        require(len(rows) == 7 and {r['owner_id'] for r in rows} == owner_ids, 'complete seven-owner inventory')
        return {r['owner_id']: (r['state_digest'], r['state_bytes']) for r in rows}
    beginning_owners = owners(projection['beginning_complete_owners'])
    require(beginning_owners == owners(beginning_clock['complete_owner_set']), 'projection beginning clock complete owners')
    for checkpoint in (beginning_checkpoint, ending_checkpoint):
        require(set(checkpoint['beginning_complete_owners']) == owner_ids, 'checkpoint beginning owner inventory')
        require(beginning_owners == {key: (row['state_sha256'], row['state_bytes'])
                                    for key, row in checkpoint['beginning_complete_owners'].items()},
                'projection beginning checkpoint complete owners')
    checkpoints = projection["accepted_segment_checkpoints"]
    segments = projection["accepted_segments"]
    require(parent["finalized_accepted_segment_count"] == len(checkpoints) == len(segments) == 8,
            "complete eight-slab final projection")
    expected_supports = PREFIX_SUPPORTS + SUCCESSOR_SUPPORTS
    expected_ids = prefix_ids + successor_ids
    require([support(row["support"]) for row in checkpoints] == expected_supports, "final projection support chronology")
    require([row["slab_id"] for row in checkpoints] == expected_ids, "final projection slab identity join")
    require(all(row["parent_transaction_id"] == parent_id for row in checkpoints), "final projection checkpoint parent join")
    require(ending_checkpoint['accepted_segments'] == checkpoints,
            'ending checkpoint complete accepted segment payloads')
    for segment, checkpoint, receipt in zip(segments, checkpoints, ending_clock['accepted_slab_receipts']):
        require(segment['accepted_slab_receipt'] == receipt, 'actual accepted segment full clock receipt')
        for name in ('receipt_id', 'slab_id', 'parent_transaction_id', 'slab_ordinal', 'segment_id', 'support'):
            require(receipt[name] == checkpoint[name], 'actual accepted segment/checkpoint identity ' + name)
        require(receipt['duration_bits'] == checkpoint['duration_s_bits'], 'actual accepted segment duration')
        require(set(segment) == {'accepted_slab_receipt'} | (set(checkpoint) - {
            'receipt_id', 'slab_id', 'parent_transaction_id', 'slab_ordinal', 'segment_id', 'support', 'duration_s_bits'}),
            'actual accepted segment full payload field inventory')
        for name in segment:
            if name != 'accepted_slab_receipt':
                require(segment[name] == checkpoint[name], 'actual accepted segment/checkpoint payload ' + name)
    require(owners(projection['ending_complete_owners']) == owners(ending_clock['complete_owner_set']),
            "final projection ending owner join")

    # The clock records the complete chronology: six represented slabs, the
    # observed snow transfer, two snow-free slabs, then a terminal zero-duration
    # vegetation ownership transition.  The latter is a clock receipt, not a
    # second observed terminal-event group.
    slab_receipts = ending_clock["accepted_slab_receipts"]
    event_receipts = ending_clock["accepted_event_receipts"]
    require(len(slab_receipts) == 8 and len(event_receipts) == 2, "complete clock receipt inventory")
    require([row["slab_ordinal"] for row in slab_receipts] == list(range(8)), "clock slab ordinal inventory")
    require([support(row["support"]) for row in slab_receipts] == expected_supports, "clock slab support chronology")
    require([row["slab_id"] for row in slab_receipts] == expected_ids, "clock slab identity join")
    require([row["receipt_id"] for row in slab_receipts] == [row["receipt_id"] for row in checkpoints],
            "clock/projection slab receipt join")
    require(all(row["parent_transaction_id"] == parent_id for row in slab_receipts + event_receipts),
            "clock receipt parent identity")
    require([(row["ordinal"], row["tick"], row["class"]) for row in event_receipts]
            == [(0, str(360 * NS), "OwnershipTransfer"), (1, str(PARENT_END), "OwnershipTransfer")],
            "two clock event inventory")
    require(event_receipts[0] == event,
            "observed event to first clock receipt join")
    chronological = slab_receipts[:6] + [event_receipts[0]] + slab_receipts[6:] + [event_receipts[1]]
    require(chronological[0]["begin_clock"] == beginning_clock["begin_clock_digest"]
            and chronological[0]["begin_owner_set"] == beginning_clock["begin_owner_set_digest"],
            "chronology beginning clock/owner join")
    for prior, following in zip(chronological, chronological[1:]):
        require(prior["end_clock"] == following["begin_clock"], "chronological clock digest chain")
        require(prior["end_owner_set"] == following["begin_owner_set"], "chronological owner digest chain")
    require(chronological[-1]["end_clock"] == ending_clock["accepted_clock_digest"], "chronology ending clock join")
    require(ending_clock["active_segment_start"] == ending_clock["active_segment_end"] == str(PARENT_END),
            "terminal zero-duration active segment")
    require(successors[0]["beginning_complete_owner_set_sha256"] == event_receipts[0]["end_owner_set"],
            "first snowfree owner/event join")
    require(successors[0]["ending_complete_owner_set_sha256"] == successors[1]["beginning_complete_owner_set_sha256"],
            "snowfree owner chain")
    require(successors[1]["ending_complete_owner_set_sha256"] == event_receipts[1]["begin_owner_set"],
            "second snowfree/terminal event join")
    require(successors[0]["ending_pending_terminal_parcel_set_sha256"]
            == successors[1]["beginning_pending_terminal_parcel_set_sha256"], "snowfree pending-parcel chain")
    return {
        "passed": True,
        "input_sha256": hashlib.sha256(raw).hexdigest(),
        "represented_prefix_slabs": 6,
        "snowfree_successor_slabs": 2,
        "final_projection_segments": 8,
        "observed_terminal_event_groups": 1,
        "clock_event_receipts": 2,
        "terminal_parcel_consumed": True,
        "checkpoint_finalized": False,
        "clock_committed": False,
        "later_parent_final_replay_authenticated": False,
        "scope": "Exact local parent chronology, clock/checkpoint identity joins, terminal parcel consumption, and final-projection inventory only; later WB14 parent-final replay, global commit, parent-final installation, and conservation remain open.",
    }


def control_cases(raw):
    original = records(raw)
    cases = []
    def poison_clock(value):
        clock = canonical_bytes(value["parent"]["ending_clock_canonical_json"])
        clock["event_ordinal"] = 99
        value["parent"]["ending_clock_canonical_json"] = list(
            json.dumps(clock, separators=(",", ":")).encode()
        )

    def poison_clock_receipt(value):
        clock = canonical_bytes(value["parent"]["ending_clock_canonical_json"])
        clock["accepted_slab_receipts"][0]["receipt_id"] = "0" * 64
        value["parent"]["ending_clock_canonical_json"] = list(json.dumps(clock, separators=(",", ":")).encode())

    def poison_owner_join(value):
        clock = canonical_bytes(value["parent"]["ending_clock_canonical_json"])
        clock["accepted_event_receipts"][0]["end_owner_set"] = "0" * 64
        value["parent"]["ending_clock_canonical_json"] = list(json.dumps(clock, separators=(",", ":")).encode())

    def poison_second_event(value):
        clock = canonical_bytes(value["parent"]["ending_clock_canonical_json"])
        clock["accepted_event_receipts"][1]["tick"] = str(420 * NS)
        value["parent"]["ending_clock_canonical_json"] = list(json.dumps(clock, separators=(",", ":")).encode())

    mutations = [
        ("missing_required_tag", lambda lines: [line for line in lines if TAG not in line]),
        ("duplicate_required_tag", lambda lines: lines + [next(line for line in lines if TAG in line)]),
        ("clock_ordinal", poison_clock),
        ("clock_receipt", poison_clock_receipt),
        ("owner_join", poison_owner_join),
        ("second_event", poison_second_event),
        ("prefix_support", lambda value: value["parent"]["subslabs"][0]["core_without_destination_receipts"]["support"].__setitem__("end_ns", "1")),
        ("parcel_digest", lambda value: value["parent"]["terminal_parcels"][0].__setitem__("parcel_digest", "0" * 64)),
        ("extra_produced_parcel", lambda value: value['parent']['event_groups'][0]['produced_unconsumed_parcels'].append(copy.deepcopy(value['parent']['event_groups'][0]['produced_unconsumed_parcels'][0]))),
        ("group_event_payload", lambda value: value['parent']['event_groups'][0]['accepted_event_receipt'].__setitem__('begin_owner_set', '0'*64)),
        ("projection_slab", lambda value: value["parent"]["finalized_parent_canonical_projection"]["accepted_segment_checkpoints"][7].__setitem__("slab_id", "0" * 64)),
        ("actual_segment_slab", lambda value: value["parent"]["finalized_parent_canonical_projection"]["accepted_segments"][0]["accepted_slab_receipt"].__setitem__("slab_id", "0" * 64)),
        ("actual_segment_payload", lambda value: value["parent"]["finalized_parent_canonical_projection"]["accepted_segments"][0].__setitem__("material_transfers", [])),
        ("beginning_owner_digest", lambda value: value["parent"]["finalized_parent_canonical_projection"]["beginning_complete_owners"][0].__setitem__("state_digest", "0"*64)),
    ]
    for name, mutate in mutations:
        if name in ("missing_required_tag", "duplicate_required_tag"):
            lines = raw.decode().splitlines()
            changed = ("\n".join(mutate(lines)) + "\n").encode()
        else:
            value = copy.deepcopy(original)
            mutate(value)
            changed = (TAG + " " + json.dumps(value) + "\n").encode()
        try:
            reconstruct(changed)
        except (ValueError, KeyError, TypeError, OverflowError, AssertionError, json.JSONDecodeError) as error:
            cases.append({"case": name, "rejected": True, "error": str(error)})
        else:
            cases.append({"case": name, "rejected": False})
    return cases


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("stdout", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--controls", action="store_true")
    args = parser.parse_args()
    raw = args.stdout.read_bytes()
    try:
        result = reconstruct(raw)
        if args.controls:
            result["controls"] = control_cases(raw)
            result["controls_passed"] = all(case["rejected"] for case in result["controls"])
            require(result["controls_passed"], "parent reconstruction controls")
    except (ValueError, KeyError, TypeError, OverflowError, AssertionError, json.JSONDecodeError) as error:
        result = {"passed": False, "error": f"{type(error).__name__}: {error}"}
    result["input"] = str(args.stdout)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "controls"}))
    raise SystemExit(0 if result["passed"] else 1)


if __name__ == "__main__":
    main()
