"""Fail-closed reconstruction of the retained local parent chronology and replay.

This verifies the observed parent payload and retained WB14 parent replay.  It
does not authenticate a global checkpoint finalization or installation.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import struct

from reconstruct_credits import reject_constant, require, unique_object

TAG = "B01_INTEGRATED_PARENT"
REPLAY_TAG = "B01_INTEGRATED_ACCEPTED_PARENT_REPLAY"
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


def replay_record(raw):
    found = []
    pattern = re.compile(r"(?<![A-Z0-9_])" + REPLAY_TAG + r"(?=\s|$)")
    for line_number, line in enumerate(raw.decode().splitlines(), 1):
        match = pattern.search(line)
        if match:
            value = json.loads(line[match.end():].strip(), object_pairs_hook=unique_object,
                               parse_constant=reject_constant)
            require(isinstance(value, dict), f"{REPLAY_TAG} line {line_number}: expected object")
            found.append(value)
    require(len(found) == 1, f"{REPLAY_TAG}: expected exactly one record, got {len(found)}")
    require(found[0]["record_version"] == 1
            and found[0]["boundary"] == "accepted publication-history support with retained WB14 parent replay",
            "accepted parent replay boundary")
    return found[0]["accepted_support"]


def hex32(value):
    require(isinstance(value, list) and len(value) == 32 and all(isinstance(v, int) and 0 <= v <= 255 for v in value),
            "digest byte inventory")
    return bytes(value).hex()


def canonical_bytes(value):
    require(isinstance(value, list) and all(isinstance(byte, int) and 0 <= byte <= 255 for byte in value),
            "canonical JSON byte inventory")
    return json.loads(bytes(value), object_pairs_hook=unique_object, parse_constant=reject_constant)


def u(value, width):
    require(isinstance(value, int) and 0 <= value < 1 << (width * 8), "unsigned integer inventory")
    return value.to_bytes(width, "big")


def f64_bits(value):
    require(isinstance(value, (int, float)) and value == value and abs(value) != float("inf"),
            "finite floating value")
    return struct.unpack(">Q", struct.pack(">d", float(value)))[0]


def framed_sha256(domain, fields):
    """Independent OPENWEPP_CANONICAL_FRAMED_SHA256_V1 reconstruction."""
    require(isinstance(domain, str) and len(domain.encode()) < 1 << 16, "hash domain")
    hasher = hashlib.sha256()
    hasher.update(b"OPENWEPP\0")
    hasher.update(u(1, 2))
    hasher.update(u(len(domain.encode()), 2))
    hasher.update(domain.encode())
    for tag, value in fields:
        require(isinstance(tag, str) and isinstance(value, bytes), "hash field inventory")
        hasher.update(u(len(tag.encode()), 2))
        hasher.update(tag.encode())
        hasher.update(u(len(value), 4))
        hasher.update(value)
    return hasher.digest()


def digest(value):
    if isinstance(value, str):
        require(len(value) == 64 and all(char in "0123456789abcdef" for char in value), "hex digest inventory")
        return bytes.fromhex(value)
    return bytes.fromhex(hex32(value))


def cursor_digest(cursor):
    return framed_sha256("openwepp-wb14-persistent-cursor-v1", [
        ("day_index", u(cursor["day_index"], 8)),
        ("next_interval_index", u(cursor["next_interval_index"], 1)),
        ("cumulative_supply_m_bits", u(f64_bits(cursor["cumulative_supply_m"]), 8)),
        ("cumulative_infiltration_m_bits", u(f64_bits(cursor["cumulative_infiltration_m"]), 8)),
    ])


def working_digest(working):
    return framed_sha256("openwepp-wb14-parent-working-state-v1", [
        ("accepted_until_ns", u(working["accepted_until_ns"], 16)),
        ("next_child_ordinal", u(working["next_child_ordinal"], 4)),
        ("cumulative_supply_m_bits", u(f64_bits(working["cumulative_supply_m"]), 8)),
        ("cumulative_infiltration_m_bits", u(f64_bits(working["cumulative_infiltration_m"]), 8)),
        ("receipt_chain_sha256", digest(working["receipt_chain_sha256"])),
    ])


def parent_id(authority):
    fields = [("coupled_parent_transaction_sha256", digest(authority["coupled_parent_transaction_sha256"])),
              ("parent_day_index", u(authority["parent_day_index"], 8)),
              ("parent_interval_index", u(authority["parent_interval_index"], 1)),
              ("support_start_ns", u(authority["support_start_ns"], 16)),
              ("support_end_ns", u(authority["support_end_ns"], 16)),
              ("parent_beginning_owner_sha256", digest(authority["parent_beginning_owner_sha256"])),
              ("beginning_cursor_sha256", digest(authority["beginning_cursor_sha256"])),
              ("schema_sha256", digest(authority["schema_sha256"])),
              ("ofe_id_sha256", digest(authority["ofe_id_sha256"])),
              ("production_lane_id", u(authority["production_lane_id"], 4)),
              ("surface_liquid_configuration_sha256", digest(authority["surface_liquid_configuration_sha256"])),
              ("wb14_configuration_sha256", digest(authority["wb14_configuration_sha256"])),
              ("wb14_model_definition_sha256", digest(authority["wb14_model_definition_sha256"])),
              ("effective_conductivity_m_s_bits", u(authority["effective_conductivity_m_s_bits"], 8)),
              ("matric_potential_m_bits", u(authority["matric_potential_m_bits"], 8)),
              ("storage_capacity_m_bits", u(authority["storage_capacity_m_bits"], 8))]
    ordinary = framed_sha256("openwepp-wb14-parent-interval-v1", fields)
    prefix = authority["inactive_prefix"]
    if prefix is None:
        return ordinary
    return framed_sha256("openwepp-wb14-parent-interval-native-prefix-v1", [
        ("ordinary_parent_id", ordinary), ("inactive_prefix_sha256", digest(prefix["proof_sha256"]))])


def child_inputs_digest(receipt):
    encoded = bytearray()
    for transition in receipt["transitions"]:
        for name in ("cumulative_supply_m_bits", "cumulative_infiltration_m_bits", "interval_supply_m_bits",
                     "interval_duration_s_bits", "effective_conductivity_m_s_bits", "matric_potential_m_bits",
                     "storage_capacity_m_bits"):
            encoded.extend(u(transition[name], 8))
    return framed_sha256("openwepp-wb14-child-inputs-v1", [
        ("ordered_transition_bits", bytes(encoded)),
        ("proposed_upper_bound_s_bits", u(receipt["proposed_upper_bound_s_bits"], 8)),
    ])


def verify_wb14_replay(authority, cursor, final_working, children, parent_receipt, persistent_cursor,
                       prefix_receipts, terminal_group):
    """Recompute the source-defined WB14 authority, child receipts, chains, and final receipt."""
    require(cursor_digest(cursor) == digest(authority["beginning_cursor_sha256"]), "replay beginning cursor digest")
    require(parent_id(authority) == digest(authority["parent_id"]), "replay source-defined parent identity")
    prefix = authority["inactive_prefix"]
    if prefix is None:
        initial_chain = framed_sha256("openwepp-wb14-parent-receipt-chain-begin-v1", [
            ("parent_id", digest(authority["parent_id"])), ("beginning_cursor_sha256", digest(authority["beginning_cursor_sha256"]))])
        initial_until = authority["support_start_ns"]
    else:
        require(prefix["coupled_parent_transaction_sha256"] == authority["coupled_parent_transaction_sha256"]
                and prefix["parent_beginning_owner_sha256"] == authority["parent_beginning_owner_sha256"]
                and prefix["parent_support_start_ns"] == authority["support_start_ns"]
                and prefix["parent_support_end_ns"] == authority["support_end_ns"], "replay inactive-prefix authority join")
        initial_chain = framed_sha256("openwepp-wb14-parent-receipt-chain-native-prefix-v1", [
            ("parent_id", digest(authority["parent_id"])), ("beginning_cursor_sha256", digest(authority["beginning_cursor_sha256"])),
            ("inactive_prefix_sha256", digest(prefix["proof_sha256"]))])
        initial_until = prefix["prefix_end_ns"]
        require(prefix["prefix_end_ns"] == support(prefix_receipts[-1]["support"])[1], "replay inactive-prefix end support")
        require(hex32(prefix["prefix_ending_owner_sha256"]) == terminal_group["accepted_event_receipt"]["end_owner_set"],
                "replay inactive-prefix ending owner join")
        require([row["wb14_ofe_topology"] for row in prefix_receipts] == [["ofe-1"]] * len(prefix_receipts),
                "replay inactive-prefix OFE topology")
        topology = u(len("ofe-1"), 4) + b"ofe-1"
        prefix_chain = framed_sha256("openwepp-wb14-native-inactive-prefix-receipt-chain-v1", [
            ("ordered_subslab_receipts", b"".join(digest(row["receipt_sha256"]) for row in prefix_receipts)),
            ("accepted_terminal_groups", digest(terminal_group["accepted_group_receipt_sha256"])),
            ("ofe_topology", topology)])
        require(prefix_chain == digest(prefix["coupled_receipt_sha256"]), "replay inactive-prefix coupled receipt digest")
        prefix_proof = framed_sha256("openwepp-wb14-native-inactive-prefix-v1", [
            ("coupled_parent_transaction_sha256", digest(prefix["coupled_parent_transaction_sha256"])),
            ("parent_beginning_owner_sha256", digest(prefix["parent_beginning_owner_sha256"])),
            ("prefix_ending_owner_sha256", digest(prefix["prefix_ending_owner_sha256"])),
            ("parent_support_start_ns", u(prefix["parent_support_start_ns"], 16)),
            ("prefix_end_ns", u(prefix["prefix_end_ns"], 16)),
            ("parent_support_end_ns", u(prefix["parent_support_end_ns"], 16)),
            ("coupled_receipt_sha256", prefix_chain)])
        require(prefix_proof == digest(prefix["proof_sha256"]), "replay inactive-prefix proof digest")
    current = {"accepted_until_ns": initial_until, "next_child_ordinal": 0,
               "cumulative_supply_m": cursor["cumulative_supply_m"],
               "cumulative_infiltration_m": cursor["cumulative_infiltration_m"],
               "receipt_chain_sha256": list(initial_chain)}
    ordered = []
    for receipt in children:
        require(receipt["parent_id"] == authority["parent_id"]
                and receipt["ofe_id_sha256"] == authority["ofe_id_sha256"]
                and receipt["production_lane_id"] == authority["production_lane_id"]
                and receipt["surface_liquid_configuration_sha256"] == authority["surface_liquid_configuration_sha256"]
                and receipt["wb14_configuration_sha256"] == authority["wb14_configuration_sha256"]
                and receipt["wb14_model_definition_sha256"] == authority["wb14_model_definition_sha256"]
                and receipt["effective_conductivity_m_s_bits"] == authority["effective_conductivity_m_s_bits"]
                and receipt["matric_potential_m_bits"] == authority["matric_potential_m_bits"]
                and receipt["storage_capacity_m_bits"] == authority["storage_capacity_m_bits"], "replay child immutable identity")
        require(receipt["parent_beginning_owner_sha256"] == authority["parent_beginning_owner_sha256"]
                and receipt["parent_beginning_cursor_sha256"] == authority["beginning_cursor_sha256"],
                "replay child parent owner/cursor join")
        require(receipt["ordinal"] == current["next_child_ordinal"]
                and receipt["support_start_ns"] == current["accepted_until_ns"], "replay child sequence")
        require(receipt["cumulative_supply_m_bits"] == f64_bits(current["cumulative_supply_m"])
                and receipt["cumulative_infiltration_m_bits"] == f64_bits(current["cumulative_infiltration_m"]),
                "replay child beginning cumulative state")
        require(receipt["accepted_duration_s_bits"] == f64_bits(
            (receipt["support_end_ns"] - receipt["support_start_ns"]) / NS), "replay child accepted duration")
        require(receipt["interval_excess_m_bits"] == f64_bits(
            struct.unpack(">d", u(receipt["interval_supply_m_bits"], 8))[0]
            - struct.unpack(">d", u(receipt["interval_infiltration_m_bits"], 8))[0]),
            "replay child interval excess")
        require(digest(receipt["predecessor_receipt_sha256"]) == digest(current["receipt_chain_sha256"]),
                "replay child predecessor chain")
        require(child_inputs_digest(receipt) == digest(receipt["child_inputs_sha256"]), "replay child input digest")
        require(working_digest(current) == digest(receipt["beginning_working_state_sha256"]), "replay child beginning working digest")
        ending_supply = current["cumulative_supply_m"] + struct.unpack(">d", u(receipt["interval_supply_m_bits"], 8))[0]
        ending_infiltration = current["cumulative_infiltration_m"] + struct.unpack(">d", u(receipt["interval_infiltration_m_bits"], 8))[0]
        body = framed_sha256("openwepp-wb14-child-body-v1", [
            ("parent_id", digest(authority["parent_id"])), ("ofe_id_sha256", digest(authority["ofe_id_sha256"])),
            ("production_lane_id", u(authority["production_lane_id"], 4)),
            ("surface_liquid_configuration_sha256", digest(authority["surface_liquid_configuration_sha256"])),
            ("wb14_configuration_sha256", digest(authority["wb14_configuration_sha256"])),
            ("wb14_model_definition_sha256", digest(authority["wb14_model_definition_sha256"])),
            ("ordinal", u(receipt["ordinal"], 4)), ("support_start_ns", u(receipt["support_start_ns"], 16)),
            ("support_end_ns", u(receipt["support_end_ns"], 16)),
            ("beginning_working_state_sha256", digest(receipt["beginning_working_state_sha256"])),
            ("ending_cumulative_supply_m_bits", u(f64_bits(ending_supply), 8)),
            ("ending_cumulative_infiltration_m_bits", u(f64_bits(ending_infiltration), 8)),
            ("child_inputs_sha256", digest(receipt["child_inputs_sha256"])),
            ("accepted_coupled_slab_sha256", digest(receipt["accepted_coupled_slab_sha256"])),
            ("child_beginning_complete_owner_set_sha256", digest(receipt["child_beginning_complete_owner_set_sha256"])),
            ("pending_routed_parcels_before_sha256", digest(receipt["pending_routed_parcels_before_sha256"])),
            ("pending_routed_parcels_after_sha256", digest(receipt["pending_routed_parcels_after_sha256"])),
        ])
        chain = framed_sha256("openwepp-wb14-child-chain-v1", [
            ("predecessor_receipt_chain_sha256", digest(receipt["predecessor_receipt_sha256"])),
            ("child_body_sha256", body)])
        current = {"accepted_until_ns": receipt["support_end_ns"], "next_child_ordinal": receipt["ordinal"] + 1,
                   "cumulative_supply_m": ending_supply,
                   "cumulative_infiltration_m": ending_infiltration,
                   "receipt_chain_sha256": list(chain)}
        require(working_digest(current) == digest(receipt["ending_working_state_sha256"]), "replay child ending working digest")
        receipt_digest = framed_sha256("openwepp-wb14-child-receipt-v1", [
            ("child_body_sha256", body), ("ending_working_state_sha256", digest(receipt["ending_working_state_sha256"])),
            ("ending_receipt_chain_sha256", chain),
            ("parent_beginning_owner_sha256", digest(authority["parent_beginning_owner_sha256"])),
            ("parent_beginning_cursor_sha256", digest(authority["beginning_cursor_sha256"]))])
        require(receipt_digest == digest(receipt["receipt_sha256"]), "replay source-defined child receipt digest")
        ordered.append(receipt_digest)
    require(current == final_working, "replay final working state")
    require(parent_receipt["parent_id"] == authority["parent_id"]
            and parent_receipt["parent_beginning_owner_sha256"] == authority["parent_beginning_owner_sha256"]
            and parent_receipt["beginning_cursor_sha256"] == authority["beginning_cursor_sha256"]
            and parent_receipt["coupled_parent_transaction_sha256"] == authority["coupled_parent_transaction_sha256"]
            and parent_receipt["parent_day_index"] == authority["parent_day_index"]
            and parent_receipt["parent_interval_index"] == authority["parent_interval_index"]
            and parent_receipt["inactive_prefix_sha256"] == (None if prefix is None else prefix["proof_sha256"]),
            "replay parent receipt authority join")
    ending_cursor = {"day_index": parent_receipt["parent_day_index"], "next_interval_index": parent_receipt["parent_interval_index"] + 1,
                     "cumulative_supply_m": current["cumulative_supply_m"], "cumulative_infiltration_m": current["cumulative_infiltration_m"]}
    require(cursor_digest(ending_cursor) == digest(parent_receipt["ending_cursor_sha256"]), "replay ending cursor digest")
    require(persistent_cursor == ending_cursor, "replay persistent cursor payload")
    require([digest(value) for value in parent_receipt["ordered_child_receipt_sha256"]] == ordered,
            "replay parent ordered children")
    digest_receipt = framed_sha256("openwepp-wb14-parent-receipt-v1", [
        ("parent_id", digest(authority["parent_id"])),
        ("coupled_parent_transaction_sha256", digest(authority["coupled_parent_transaction_sha256"])),
        ("parent_beginning_owner_sha256", digest(authority["parent_beginning_owner_sha256"])),
        ("beginning_cursor_sha256", digest(authority["beginning_cursor_sha256"])),
        ("ending_cursor_sha256", digest(parent_receipt["ending_cursor_sha256"])),
        ("support_start_ns", u(authority["support_start_ns"], 16)), ("support_end_ns", u(authority["support_end_ns"], 16)),
        ("ordered_child_receipts", b"".join(ordered)), ("receipt_chain_sha256", digest(current["receipt_chain_sha256"]))])
    require(parent_receipt["receipt_chain_sha256"] == current["receipt_chain_sha256"], "replay parent receipt chain")
    require(digest_receipt == digest(parent_receipt["receipt_sha256"]), "replay source-defined parent receipt digest")


def reconstruct(raw):
    record = records(raw)
    require(record["record_version"] == 1, "parent record version")
    require(record["boundary"] == "native mixed-phase accepted parent after complete terminal execution",
            "parent observation boundary")
    parent = record["parent"]
    fixture = parent["fixture_inputs"]
    require(fixture == {
        "capture_terminal_failure": False,
        "expect_dynamic_proposal": False,
        "hard_boundary_ns": 60 * NS,
        "include_child_17": False,
        "initial_cold_delta_k_bits": 0,
        "parent_support": {"start_ns": "0", "end_ns": str(PARENT_END)},
        "parity_profile": "native-mixed-phase",
        "production_only": True,
        "runtime_swe_m_bits": f64_bits(0.0006),
        "second_lane_swe_m_bits": None,
        "solid_reappearance": False,
        "terminal_event": True,
    }, "native fixture exact input inventory")
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

    accepted = replay_record(raw)
    require(support(accepted["support"]) == SUCCESSOR_SUPPORTS[1]
            and accepted["parent_transaction_id"] == parent_id
            and accepted["accepted_slab_sha256"] == successor_ids[1], "accepted later support identity")
    child_pairs = json.loads(bytes(accepted["wb14_child_replay_bytes"]), object_pairs_hook=unique_object)
    parent_pairs = json.loads(bytes(accepted["wb14_parent_replay_bytes"]), object_pairs_hook=unique_object)
    require(isinstance(child_pairs, list) and len(child_pairs) == 1 and isinstance(child_pairs[0], list)
            and len(child_pairs[0]) == 2 and child_pairs[0][0] == "ofe-1", "child replay OFE inventory")
    require(isinstance(parent_pairs, list) and len(parent_pairs) == 1 and isinstance(parent_pairs[0], list)
            and len(parent_pairs[0]) == 2 and parent_pairs[0][0] == "ofe-1", "parent replay OFE inventory")
    child = child_pairs[0][1]
    parent_replay = parent_pairs[0][1]
    authority = child["authority"]
    receipt = parent_replay["receipt"]
    require(hex32(authority["coupled_parent_transaction_sha256"]) == parent_id
            and hex32(receipt["coupled_parent_transaction_sha256"]) == parent_id, "replay coupled parent identity")
    require(authority["support_start_ns"] == 0 and authority["support_end_ns"] == PARENT_END
            and receipt["support_start_ns"] == 0 and receipt["support_end_ns"] == PARENT_END, "replay parent support")
    children = child["receipts"]
    require([(row["ordinal"], row["support_start_ns"], row["support_end_ns"]) for row in children]
            == [(0, 360 * NS, 420 * NS), (1, 420 * NS, PARENT_END)], "physical child chronology")
    require([hex32(row["receipt_sha256"]) for row in children] == [hex32(value) for value in receipt["ordered_child_receipt_sha256"]],
            "parent replay ordered child identity")
    require(hex32(authority["parent_beginning_owner_sha256"]) == beginning_clock["begin_owner_set_digest"],
            "replay parent beginning owner join")
    require([hex32(row["child_beginning_complete_owner_set_sha256"]) for row in children]
            == [successors[0]["beginning_complete_owner_set_sha256"], successors[1]["beginning_complete_owner_set_sha256"]],
            "replay child owner joins")
    require([hex32(row["accepted_coupled_slab_sha256"]) for row in children] == successor_ids,
            "replay child coupled slab joins")
    verify_wb14_replay(authority, child["beginning_cursor"], child["working"], children, receipt,
                       parent_replay["persistent_cursor"],
                       [slab["core_without_destination_receipts"] for slab in slabs], group)
    require(receipt["receipt_sha256"] != [] and receipt["receipt_chain_sha256"] != []
            and accepted["wb14_parent_replay_bytes"] and accepted["wb14_child_replay_bytes"], "nonempty replay bytes are insufficient")
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
        "wb14_parent_replay_correspondence": True,
        "global_commit_authenticated": False,
        "installation_authenticated": False,
        "scope": "Exact local parent chronology and retained WB14 parent-replay correspondence only; global commit, installation, and conservation remain open.",
    }


def control_cases(raw):
    original = records(raw)
    cases = []
    def replace_tag(tag, value):
        lines = raw.decode().splitlines()
        pattern = re.compile(r"(?<![A-Z0-9_])" + tag + r"(?=\s|$)")
        for index, line in enumerate(lines):
            match = pattern.search(line)
            if match:
                lines[index] = line[:match.end()] + " " + json.dumps(value)
                return ("\n".join(lines) + "\n").encode()
        raise ValueError(f"{tag} absent in control source")
    def replace_parent(value):
        return replace_tag(TAG, value)
    def replay_envelope():
        pattern = re.compile(r"(?<![A-Z0-9_])" + REPLAY_TAG + r"(?=\s|$)")
        values = []
        for line in raw.decode().splitlines():
            match = pattern.search(line)
            if match:
                values.append(json.loads(line[match.end():].strip(), object_pairs_hook=unique_object,
                                        parse_constant=reject_constant))
        require(len(values) == 1, "control replay envelope inventory")
        return values[0]
    def mutate_replay(mutate):
        value = copy.deepcopy(replay_envelope())
        mutate(value)
        return replace_tag(REPLAY_TAG, value)
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
        ("missing_required_tag", lambda lines: [line for line in lines if not re.search(r"(?<![A-Z0-9_])" + TAG + r"(?=\s|$)", line)], f"{TAG}: expected exactly one record"),
        ("duplicate_required_tag", lambda lines: lines + [next(line for line in lines if re.search(r"(?<![A-Z0-9_])" + TAG + r"(?=\s|$)", line))], f"{TAG}: expected exactly one record"),
        ("clock_ordinal", poison_clock, "ending clock ordinals"),
        ("clock_receipt", poison_clock_receipt, "actual accepted segment full clock receipt"),
        ("owner_join", poison_owner_join, "observed event to first clock receipt join"),
        ("second_event", poison_second_event, "two clock event inventory"),
        ("fixture_swe", lambda value: value["parent"]["fixture_inputs"].__setitem__("runtime_swe_m_bits", 0), "native fixture exact input inventory"),
        ("prefix_support", lambda value: value["parent"]["subslabs"][0]["core_without_destination_receipts"]["support"].__setitem__("end_ns", "1"), "represented-prefix slab support/order"),
        ("parcel_digest", lambda value: value["parent"]["terminal_parcels"][0].__setitem__("parcel_digest", "0" * 64), "event to consumed parcel join"),
        ("extra_produced_parcel", lambda value: value['parent']['event_groups'][0]['produced_unconsumed_parcels'].append(copy.deepcopy(value['parent']['event_groups'][0]['produced_unconsumed_parcels'][0])), "complete produced-to-consumed parcel payload and inventory"),
        ("group_event_payload", lambda value: value['parent']['event_groups'][0]['accepted_event_receipt'].__setitem__('begin_owner_set', '0'*64), "observed event to first clock receipt join"),
        ("projection_slab", lambda value: value["parent"]["finalized_parent_canonical_projection"]["accepted_segment_checkpoints"][7].__setitem__("slab_id", "0" * 64), "final projection slab identity join"),
        ("actual_segment_slab", lambda value: value["parent"]["finalized_parent_canonical_projection"]["accepted_segments"][0]["accepted_slab_receipt"].__setitem__("slab_id", "0" * 64), "actual accepted segment full clock receipt"),
        ("actual_segment_payload", lambda value: value["parent"]["finalized_parent_canonical_projection"]["accepted_segments"][0].__setitem__("material_transfers", []), "actual accepted segment/checkpoint payload material_transfers"),
        ("beginning_owner_digest", lambda value: value["parent"]["finalized_parent_canonical_projection"]["beginning_complete_owners"][0].__setitem__("state_digest", "0"*64), "projection beginning clock complete owners"),
    ]
    for name, mutate, expected in mutations:
        if name in ("missing_required_tag", "duplicate_required_tag"):
            lines = raw.decode().splitlines()
            changed = ("\n".join(mutate(lines)) + "\n").encode()
        else:
            value = copy.deepcopy(original)
            mutate(value)
            changed = replace_parent(value)
        try:
            reconstruct(changed)
        except (ValueError, KeyError, TypeError, OverflowError, AssertionError, json.JSONDecodeError) as error:
            if name not in ("missing_required_tag", "duplicate_required_tag"):
                require(REPLAY_TAG not in str(error), f"{name}: replay record lost before target guard")
            require(expected in str(error), f"{name}: expected target guard {expected!r}, got {error}")
            cases.append({"case": name, "rejected": True, "error": str(error)})
        else:
            cases.append({"case": name, "rejected": False})
    replay_cases = [
        ("replay_missing", lambda: ("\n".join(line for line in raw.decode().splitlines()
                                           if not re.search(r"(?<![A-Z0-9_])" + REPLAY_TAG + r"(?=\s|$)", line)) + "\n").encode(),
         f"{REPLAY_TAG}: expected exactly one record"),
        ("replay_duplicate", lambda: raw + next(line.encode() + b"\n" for line in raw.decode().splitlines()
                                                  if re.search(r"(?<![A-Z0-9_])" + REPLAY_TAG + r"(?=\s|$)", line)),
         f"{REPLAY_TAG}: expected exactly one record"),
        ("replay_malformed", lambda: raw + (REPLAY_TAG + " {not-json}\n").encode(), "Expecting property name"),
        ("replay_child_mutation", lambda: mutate_replay(lambda value: value["accepted_support"]["wb14_child_replay_bytes"].__setitem__(0, 0)), "child replay OFE inventory"),
        ("replay_parent_digest", lambda: mutate_replay(lambda value: value["accepted_support"]["wb14_parent_replay_bytes"].__setitem__(0, 0)), "parent replay OFE inventory"),
        ("replay_owner_join", lambda: mutate_replay(lambda value: value["accepted_support"]["wb14_child_replay_bytes"].__setitem__(0, 0)), "child replay OFE inventory"),
    ]
    # The three byte-array cases above are deliberately replaced below with
    # structured, single-field mutations after decoding their canonical bytes.
    def replay_bytes_case(which, mutate):
        def run():
            def apply(value):
                encoded = value["accepted_support"][which]
                decoded = json.loads(bytes(encoded), object_pairs_hook=unique_object, parse_constant=reject_constant)
                mutate(decoded[0][1])
                value["accepted_support"][which] = list(json.dumps(decoded, separators=(",", ":")).encode())
            return mutate_replay(apply)
        return run
    replay_cases[3:] = [
        ("replay_child_mutation", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["receipts"][1].__setitem__("support_end_ns", 1)), "physical child chronology"),
        ("replay_parent_digest", replay_bytes_case("wb14_parent_replay_bytes", lambda parent: parent["receipt"].__setitem__("receipt_sha256", [0] * 32)), "replay source-defined parent receipt digest"),
        ("replay_owner_join", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["authority"].__setitem__("parent_beginning_owner_sha256", [0] * 32)), "replay parent beginning owner join"),
        ("replay_parent_chain", replay_bytes_case("wb14_parent_replay_bytes", lambda parent: parent["receipt"].__setitem__("receipt_chain_sha256", [0] * 32)), "replay parent receipt chain"),
        ("replay_persistent_cursor", replay_bytes_case("wb14_parent_replay_bytes", lambda parent: parent["persistent_cursor"].__setitem__("day_index", 1)), "replay persistent cursor payload"),
        ("replay_child_metadata", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["receipts"][0].__setitem__("wb14_configuration_sha256", [0] * 32)), "replay child immutable identity"),
        ("replay_child_duration", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["receipts"][0].__setitem__("accepted_duration_s_bits", 0)), "replay child accepted duration"),
        ("replay_prefix_chain", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["authority"]["inactive_prefix"].__setitem__("coupled_receipt_sha256", [0] * 32)), "replay inactive-prefix coupled receipt digest"),
        ("replay_prefix_proof", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["authority"]["inactive_prefix"].__setitem__("proof_sha256", [0] * 32)), "replay source-defined parent identity"),
        ("replay_prefix_owner", replay_bytes_case("wb14_child_replay_bytes", lambda child: child["authority"]["inactive_prefix"].__setitem__("prefix_ending_owner_sha256", [0] * 32)), "replay inactive-prefix ending owner join"),
    ]
    for name, build, expected in replay_cases:
        try:
            reconstruct(build())
        except (ValueError, KeyError, TypeError, OverflowError, AssertionError, json.JSONDecodeError) as error:
            require(expected in str(error), f"{name}: expected target guard {expected!r}, got {error}")
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
            result["no_op_positive_control"] = reconstruct(raw)["passed"]
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
