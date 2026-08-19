#!/usr/bin/env python3
"""Independent scalar CLM5 root-zone equation calculator (no Rust imports)."""

import math
import struct
import sys


def hx(value: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def solve(liquid: float, thickness: float, porosity: float, ksat: float,
          psi_sat: float, b: float, top: float, lateral: float) -> list[str]:
    capacity = porosity * thickness
    if liquid > math.nextafter(capacity, math.inf):
        raise ValueError("WaterAbovePoreCapacity")
    theta = liquid / thickness
    s = min(1.0, max(0.0, theta / porosity))
    if s == 0.0:
        s = 0.0
    s_psi = max(0.01, s)
    psi = max(psi_sat * math.pow(s_psi, -b), -1e8)
    exponent = 2.0 * b + 3.0
    conductivity = min(ksat, ksat * math.pow(s, exponent))
    node = top + 0.5 * thickness
    return [hx(v) for v in (s, s_psi, psi, exponent, conductivity, node,
                             -1000.0 * node, 1000.0 * (node + lateral))]


if __name__ == "__main__":
    print(" ".join(solve(*map(float, sys.argv[1:9]))))
