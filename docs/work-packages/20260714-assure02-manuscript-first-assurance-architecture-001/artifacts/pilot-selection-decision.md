# Pilot Selection Decision

Decision: select the daily linear groundwater-reservoir recurrence for the
ASSURE-02 nonpublic manuscript prototype.

Date: 2026-07-14

## Scientific Question

Does the assessed openWEPP realization implement the authorized daily linear-
reservoir storage, baseflow, and deep-seepage recurrence, preserve its units and
timing, and carry generated baseflow through the production watershed consumer
without loss, substitution, or double counting?

## Claim Envelope

The prototype may conclude only about:

- formulation traceability to the authorized Srivastava linear-reservoir
  equations and pinned legacy lineage;
- code behavior for the tested analytical vectors and domain guards;
- numerical closure of the retained 731-day H2637 recurrence reconstruction;
- integration of generated baseflow/deep seepage through HBP and watershed
  consumers; and
- source stability from the frozen assessed commit to the current documentation
  checkout where exact path comparison supports it.

It may not conclude that:

- openWEPP groundwater predictions have been empirically corroborated across
  watersheds;
- the Priest River performance statistics reported by Srivastava et al. are
  current openWEPP results;
- coefficients are transferable or calibrated for a user's watershed;
- baseflow, lateral subsurface flow, and `chan.inp` `cbase` are interchangeable;
  or
- openWEPP is fit for a particular application.

## Why This Is A Scientific Prototype

The original routine has a peer-reviewed hydrologic rationale and observational
evaluation. The openWEPP question in this prototype is deliberately earlier in
the evidence chain: whether the software realizes that bounded formulation and
delivers its outputs correctly. This is a legitimate manuscript-sized
verification study with a clear referent and quantitative result. It also makes
the missing next study obvious: an independent current-release empirical
evaluation against admitted watershed observations.
