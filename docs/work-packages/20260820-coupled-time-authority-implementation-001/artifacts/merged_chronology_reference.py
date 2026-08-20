#!/usr/bin/env python3
"""Positive V2 slab/event/slab restore chronology fixture; imports no Rust."""

import hashlib
import json


def h(label):
    return hashlib.sha256(label.encode()).hexdigest()


begin_owner, event_owner, end_owner = h("owners:A+B"), h("owners:A+B-terminal-C"), h("owners:A+C")
clock0, clock1, clock2, clock3 = (h(f"clock:{index}") for index in range(4))
actions = [
    {"kind": "slab", "support": [0, 5], "participants": ["A", "B"], "begin_owner": begin_owner, "end_owner": begin_owner, "begin_clock": clock0, "end_clock": clock1},
    {"kind": "event", "tick": 5, "transition": "B-to-C", "begin_owner": begin_owner, "end_owner": event_owner, "begin_clock": clock1, "end_clock": clock2},
    {"kind": "slab", "support": [5, 10], "participants": ["A", "C"], "begin_owner": event_owner, "end_owner": end_owner, "begin_clock": clock2, "end_clock": clock3},
]
restored = json.loads(json.dumps(actions, sort_keys=True))
owner, clock, cursor = begin_owner, clock0, 0
for action in restored:
    if action["begin_owner"] != owner or action["begin_clock"] != clock:
        raise SystemExit("merged chronology chain mismatch")
    if action["kind"] == "slab":
        if action["support"][0] != cursor or action["support"][1] <= cursor:
            raise SystemExit("slab support mismatch")
        cursor = action["support"][1]
    elif action["tick"] != cursor:
        raise SystemExit("event boundary mismatch")
    owner, clock = action["end_owner"], action["end_clock"]
actual = hashlib.sha256(json.dumps({"cursor": cursor, "owner": owner, "clock": clock, "actions": restored}, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
expected = "6b131695fda7f600344dc7c706f63e8c1cf86ef41ab72afd5583b8b76ff25971"
if actual != expected:
    raise SystemExit(f"merged chronology fixture mismatch: {actual}")
print(actual)
