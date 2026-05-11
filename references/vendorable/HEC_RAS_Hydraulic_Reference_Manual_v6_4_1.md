# USACE HEC-RAS Hydraulic Reference Manual v6.4.1 (text extract)

**Source PDF**: /workdir/wepp-forest/references/HEC_RAS_Hydraulic_Reference_Manual_v6_4_1.pdf

---
HEC-RAS Hydraulic Reference Manual
HEC-RAS Hydraulic Reference Manual

Exported on 10/18/2023

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

Table of Contents
1

Front Matter.....................................................................................................14

1.1

HEC-RAS River Analysis System ....................................................................14

1.1.1

Hydraulic Reference Manual ..........................................................................15

1.1.1.1 Version 6.0 Beta December 2020 ..................................................................15
1.2

HEC-RAS River Analysis System ....................................................................16

1.3

Hydraulic Reference Manual ..........................................................................16

1.3.1

Version 6.0 Beta December 2020 ..................................................................16

1.3.2

River Analysis System, HEC-RAS ...................................................................17

1.3.3

Terms and Conditions of Use:........................................................................17

1.3.3.1 Waiver of Warranty:.........................................................................................18
1.3.3.2 Limitation of Liability: .....................................................................................18
1.3.3.3 Indemnity: ........................................................................................................18
1.3.3.4 Assent:.............................................................................................................18
2

Forward............................................................................................................19

3

Introduction .....................................................................................................21

3.1

General Philosophy of the Modeling System ................................................21

3.2

Overview of Hydraulic Capabilities ................................................................21

3.3

HEC-RAS Documentation ...............................................................................22

3.4

Overview of This Manual ................................................................................23

4

Theoretical Basis for One-Dimensional and Two-Dimensional
Hydrodynamic Calculations ...........................................................................25

4.1

1D Steady Flow Water Surface Profiles.........................................................25

4.1.1

Equations for Basic Profile Calculations .......................................................25

4.1.2

Cross Section Subdivision for Conveyance Calculations.............................27

4.1.3

Composite Manning's n for the Main Channel..............................................29

4.1.4

Evaluation of the Mean Kinetic Energy Head ................................................30

4.1.5

Friction Loss Evaluation .................................................................................32

2

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

4.1.6

Contraction and Expansion Loss Evaluation.................................................33

4.1.7

Computation Procedure .................................................................................33

4.1.8

Critical Depth Determination ..........................................................................35

4.1.9

Applications of the Momentum Equation .....................................................37

4.1.10

Air Entrainment in High Velocity Streams .....................................................42

4.1.11

1D Steady Flow Program Limitations ............................................................42

4.2

1D Unsteady Flow Hydrodynamics................................................................44

4.2.1

Continuity Equation.........................................................................................44

4.2.2

Momentum Equation ......................................................................................45

4.2.3

Application of the 1D Unsteady Flow Equations within HEC-RAS ...............49

4.2.4

Implicit Finite Difference Scheme..................................................................51

4.2.4.1 Continuity Equation0 ......................................................................................52
4.2.4.2 Momentum Equation0 ....................................................................................54
4.2.4.3 Added Force Term ..........................................................................................55
4.2.4.4 Lateral Influx of Momentum...........................................................................56
4.2.4.5 Finite Difference Form of the Unsteady Flow Equations..............................57
4.2.4.6 Linearized, Implicit, Finite Difference Equations...........................................57
4.2.4.7 Flow Distribution Factor .................................................................................60
4.2.4.8 Equivalent Flow Path ......................................................................................61
4.2.4.9 Boundary Conditions ......................................................................................61
4.2.4.10 Interior Boundary Conditions (for Reach Connections) ...............................61
4.2.4.11 Upstream Boundary Conditions .....................................................................63
4.2.4.12 Downstream Boundary Conditions ................................................................64
4.2.4.13 Skyline Solution of a Sparse System of Linear Equations............................66
4.2.4.14 Computational Procedure ..............................................................................71
4.2.5

Semi-Implicit Finite-Volume Scheme.............................................................72

4.2.5.1 Hydraulic Equations (1D FV) ..........................................................................72
4.2.5.2 Numerical Methods (1D FV)...........................................................................74
4.3

2D Unsteady Flow Hydrodynamics................................................................78

3

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

4.3.1

Introduction .....................................................................................................78

4.3.2

Hydraulic Equations........................................................................................80

4.3.2.1 Mass Conservation .........................................................................................80
4.3.2.2 Momentum Conservation...............................................................................81
4.3.2.3 Turbulence Modeling ......................................................................................83
4.3.2.4 Wind Surface Stress .......................................................................................84
4.3.2.5 Diffusion Wave Approximation to the Shallow Water Equations.................87
4.3.3

Grid and Dual Grid ...........................................................................................88

4.3.3.1 Connectivity.....................................................................................................90
4.3.4

Subgrid Bathymetry ........................................................................................91

4.3.5

Numerical Methods ........................................................................................92

4.3.5.1 Face-Normal Gradient.....................................................................................93
4.3.5.2 Face-Tangential Velocity ................................................................................93
4.3.5.3 Cell Velocity.....................................................................................................94
4.3.5.4 Cell Velocity Gradient .....................................................................................95
4.3.5.5 Diffusion-Wave Equation Solver.....................................................................95
4.3.5.6 Local Inertia Approximation to the Shallow Water Equations .....................98
4.3.5.7 Eulerian-Lagrangian Shallow Water Equation Solver................................. 100
4.3.5.8 Eulerian Shallow Water Equation Solver..................................................... 106
4.3.5.9 Matrix Solvers .............................................................................................. 110
5

Basic Data Requirements ............................................................................ 114

5.1

Geometric Data ............................................................................................ 114

5.1.1

Study Limit Determination........................................................................... 115

5.1.2

The River System Schematic ...................................................................... 115

5.1.3

Cross Section Geometry.............................................................................. 117

5.1.4

Optional Cross Section Properties.............................................................. 119

5.1.5

Reach Lengths ............................................................................................. 124

5.1.6

Energy Loss Coefficients............................................................................. 124

5.1.6.1 Table 3‑1 Manning's n Values ..................................................................... 125

4

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

5.1.6.2 Table 3-3 Subcritical Flow Contraction and Expansion Coefficients ....... 134
5.1.7

Stream Junction Data .................................................................................. 135

5.2

Steady Flow Data ......................................................................................... 136

5.2.1

Flow Regime................................................................................................. 136

5.2.2

Boundary Conditions ................................................................................... 136

5.2.3

Discharge Information ................................................................................. 137

5.3

Unsteady Flow Data ..................................................................................... 137

5.3.1

Boundary Conditions1 ................................................................................. 138

5.3.2

Initial Conditions .......................................................................................... 138

6

Overview of Optional Capabilities ............................................................... 139

6.1

Multiple Profile Analysis.............................................................................. 139

6.2

Multiple Plan Analysis ................................................................................. 139

6.3

Optional Friction Loss Equations................................................................ 140

6.4

Cross Section Interpolation......................................................................... 141

6.5

Mixed Flow Regime Calculations................................................................ 142

6.6

Modeling Stream Junctions ........................................................................ 145

6.6.1

Energy Based Junction Method .................................................................. 145

6.6.2

Momentum Based Junction Method .......................................................... 151

6.7

Flow Distribution Calculations .................................................................... 154

6.8

Split Flow Optimization................................................................................ 156

6.9

Pressurized Pipe Flow ................................................................................. 157

6.10

Estimating Ungaged Area Inflows .............................................................. 163

6.10.1

Theory ........................................................................................................... 164

6.10.2

Optimization of Ungaged Inflow ................................................................. 165

6.10.3

Simultaneous Optimization of Independent Reaches ............................... 166

6.10.4

Sequential Optimization .............................................................................. 166

6.11

Modeling Precipitation and Infiltration ....................................................... 167

6.11.1

Deficit and Constant .................................................................................... 167

6.11.2

Curve Number .............................................................................................. 168

5

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

6.11.3

Green-Ampt .................................................................................................. 171

7

Modeling Bridges ......................................................................................... 176

7.1

General Bridge Modeling Guidelines .......................................................... 176

7.1.1

Cross Section Locations for Bridges .......................................................... 176

7.1.2

Defining Ineffective Flow Areas .................................................................. 179

7.1.3

Contraction and Expansion Losses ............................................................ 182

7.2

Hydraulic Computations through the Bridge.............................................. 183

7.2.1

Low Flow Computations.............................................................................. 183

7.2.1.1 Class A Low Flow......................................................................................... 184
7.2.1.2 Yarnell Equation ........................................................................................... 188
7.2.1.3 FHWA WSPRO Method ................................................................................ 190
7.2.1.4 Class B Low Flow......................................................................................... 192
7.2.1.5 Class C Low Flow......................................................................................... 192
7.2.2

High Flow Computations............................................................................. 192

7.2.3

Combination Flow ........................................................................................ 199

7.3

Selecting a Bridge Modeling Approach ...................................................... 200

7.3.1

Low Flow Methods....................................................................................... 200

7.3.2

High Flow Methods...................................................................................... 201

7.4

Unique Bridge Problems and Suggested Approaches .............................. 201

7.4.1

Perched Bridges........................................................................................... 202

7.4.2

Low Water Bridges....................................................................................... 202

7.4.3

Bridges on a Skew ....................................................................................... 203

7.4.4

Parallel Bridges ............................................................................................ 205

7.4.5

Multiple Bridge Opening .............................................................................. 206

7.4.6

Modeling Floating Pier Debris ..................................................................... 206

7.5

Bridge Modeling in 2D ................................................................................. 207

7.5.1

Simplified 1D/2D Bridge Modeling ............................................................. 208

7.5.2

Detailed Bridge Modeling ............................................................................ 208

8

Modeling Culverts ........................................................................................ 209

6

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

8.1

General Culvert Modeling Guidelines ......................................................... 209

8.1.1

Types of Culverts ......................................................................................... 210

8.1.2

Cross Section Locations for Culverts ......................................................... 211

8.1.3

Expansion and Contraction Coefficients for Culverts ............................... 214

8.1.4

Limitations of the Culvert Routines in HEC-RAS ........................................ 215

8.2

Culvert Hydraulics........................................................................................ 215

8.2.1

Introduction to Culvert Terminology........................................................... 215

8.2.2

Flow Analysis for Culverts........................................................................... 217

8.2.3

Computing Inlet Control Headwater ........................................................... 219

8.2.4

Computing Outlet Control Headwater ........................................................ 220

8.2.5

FHWA Full Flow Equations .......................................................................... 223

8.2.6

Direct Step Water Surface Profile Computations ...................................... 224

8.2.7

Normal Depth of Flow in the Culvert........................................................... 225

8.2.8

Critical Depth of Flow in the Culvert ........................................................... 226

8.2.9

Horizontal and Adverse Culvert Slopes ...................................................... 227

8.2.10

Weir Flow ...................................................................................................... 227

8.2.11

Supercritical and Mixed Flow Regime Inside of Culvert ............................ 227

8.2.12

Multiple Manning’s n Values Inside of Culvert........................................... 228

8.2.13

Partially Filled or Buried Culverts ................................................................ 229

8.2.14

Comparison to the USGS Culvert Procedures............................................ 229

8.3

Culvert Data and Coefficients ..................................................................... 231

8.3.1

Culvert Shape and Size ................................................................................ 231

8.3.2

Culvert Length .............................................................................................. 233

8.3.3

Number of Identical Barrels ........................................................................ 233

8.3.4

Manning's Roughness Coefficient .............................................................. 233

8.3.5

Entrance Loss Coefficient ........................................................................... 236

8.3.6

Exit Loss Coefficient.................................................................................... 238

8.3.7

FHWA Chart and Scale Numbers ................................................................ 239

8.3.8

Culvert Invert Elevations .............................................................................. 250

7

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

8.3.9

Weir Flow Coefficient................................................................................... 251

9

Modeling Multiple Bridge and Culvert Openings........................................ 252

9.1

General Modeling Guidelines ...................................................................... 252

9.2

Multiple Opening Approach......................................................................... 252

9.2.1

Locating the Stagnation Points................................................................... 254

9.2.2

Computational Procedure for Multiple Openings ...................................... 255

9.2.3

Limitations of the Multiple Opening Approach .......................................... 256

9.3

Divided Flow Approach................................................................................ 256

10

Modeling Gated Spillways, Weirs and Drop Structures ............................. 258

10.1

General Modeling Guidelines for Inline Structures .................................... 258

10.1.1

Cross Section Locations ............................................................................. 259

10.1.2

Expansion and Contraction Coefficients.................................................... 262

10.2

Hydraulic Computations through Gated Spillways .................................... 263

10.2.1

Radial Gates ................................................................................................. 263

10.2.2

Sluice Gate ................................................................................................... 265

10.2.3

Overflow Gates............................................................................................. 266

10.2.4

Low Flow through the Gates ....................................................................... 267

10.2.4.1 Submerged Weir Flow through the Gates .................................................. 268
10.3

Uncontrolled Overflow Weirs....................................................................... 269

10.3.1

Submerged Weir Flow.................................................................................. 270

10.4

Modeling Lateral Structures........................................................................ 271

10.4.1

Hager’s Lateral Weir Equation..................................................................... 274

10.5

Drop Structures ............................................................................................ 275

11

Floodplain Encroachment Calculations...................................................... 279

11.1

Encroachment Methods .............................................................................. 279

11.1.1

Encroachment Method 1 ............................................................................. 279

11.1.2

Encroachment Method 2 ............................................................................. 280

11.1.3

Encroachment Method 3 ............................................................................. 281

11.1.4

Encroachment Method 4 ............................................................................. 282

8

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

11.1.5

Encroachment Method 5 ............................................................................. 283

11.2

Bridge, Culvert, and Multiple Opening Encroachments ............................. 284

11.3

General Modeling Guidelines for Floodway Analysis ................................ 285

12

Estimating Scour at Bridges........................................................................ 286

12.1

General Modeling Guidelines ...................................................................... 286

12.2

Computing Contraction Scour..................................................................... 287

12.2.1

Contraction Scour Conditions ..................................................................... 287

12.2.2

Determination of Live-Bed or Clear-Water Contraction Scour .................. 287

12.2.3

Live-Bed Contraction Scour......................................................................... 288

12.2.4

Clear-Water Contraction Scour ................................................................... 289

12.3

Computing Local Scour at Piers ................................................................. 290

12.3.1

Computing Pier Scour With The CSU Equation.......................................... 290

12.3.2

Computing Pier Scour With The Froehlich Equation ................................. 294

12.4

Computing Local Scour at Abutments ....................................................... 295

12.4.1

The HIRE Equation ....................................................................................... 295

12.4.2

Froehlich’s Equation .................................................................................... 297

12.4.3

Clear-Water Scour at Abutments ................................................................ 298

12.5

Total Scour Depths Inside The Bridge ........................................................ 300

13

Modeling Ice-covered Rivers ....................................................................... 301

13.1

Modeling Ice Covers with Known Geometry .............................................. 301

13.2

Modeling Wide-River Ice Jams.................................................................... 304

13.2.1

Solution Procedure ...................................................................................... 307

14

Stable Channel Design Functions ............................................................... 309

14.1

Uniform Flow Computations ....................................................................... 309

14.1.1

Cross Section Subdivision for Conveyance Calculations0 ....................... 310

14.1.2

Bed Roughness Functions........................................................................... 310

14.2

Stable Channel Design................................................................................. 320

14.2.1

Copeland Method......................................................................................... 320

14.2.2

Regime Method............................................................................................ 326

9

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

14.2.3

Tractive Force Method ................................................................................ 328

14.3

Sediment Transport Capacity ..................................................................... 336

14.3.1

Background .................................................................................................. 337

14.3.2

Fall Velocity .................................................................................................. 339

14.3.3

Correction for Fine Sediment ...................................................................... 341

14.3.4

Sediment Gradation ..................................................................................... 342

14.3.5

Hydraulic Parameters .................................................................................. 345

14.3.6

Bed Load Stations........................................................................................ 346

14.3.7

Output ........................................................................................................... 346

14.3.8

Sediment Transport Functions ................................................................... 346

15

Performing a Dam Break Study with HEC-RAS .......................................... 355

15.1

Inflow Flood Routing a Through Reservoir ................................................. 355

15.1.1

Full Dynamic Wave Routing......................................................................... 357

15.1.2

Level Pool Routing ....................................................................................... 358

15.2

Estimating Dam Breach Parameters .......................................................... 360

15.2.1

Causes and Types of Dam Failures ............................................................ 360

15.2.2

Estimating Breach Parameters ................................................................... 362

15.2.2.1 Simplified Physical Breaching Method....................................................... 377
15.2.3

Recommended Approach............................................................................ 383

15.2.4

Example Application .................................................................................... 385

15.3

Downstream Flood Routing/Modeling Issues ........................................... 391

15.3.1

Cross Section Spacing and Hydraulic Properties ...................................... 391

15.4

Computational Time Step............................................................................ 395

15.5

Manning’s Roughness Coefficients ............................................................ 399

15.6

Downstream Storage, Tributaries, and Levees .......................................... 402

15.7

Modeling Bridge and Culvert Crossings ..................................................... 407

15.8

Modeling Steep Streams ............................................................................. 409

15.9

Drops in the bed Profile ............................................................................... 410

15.10

Initial Conditions and Low Flow .................................................................. 411

10

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

15.11

Downstream Boundary Condition Considerations .................................... 413

15.12

Using 2D Flow Areas for Dam Break Analyses .......................................... 414

16

References ................................................................................................... 417

17

Appendix....................................................................................................... 427

17.1

Flow Transitions in Bridge Backwater Analysis ......................................... 427

17.1.1

Conclusions From The Study ...................................................................... 429

17.1.1.1 Expansion Reach Lengths (Le on Figure)................................................... 430
17.1.1.2 Contraction Reach Lengths (Lc on Figure)................................................. 431
17.1.1.3 Expansion Coefficients................................................................................ 432
17.1.1.4 Contraction Coefficients ............................................................................. 432
17.1.1.5 Asymmetric Bridge Openings ..................................................................... 432
17.1.1.6 Vertical-Abutment Cases............................................................................. 432
17.1.2

Recommendations From The Study ........................................................... 432

17.1.2.1 Expansion Reach Lengths ........................................................................... 433
17.1.2.2 Contraction Reach Lengths......................................................................... 435
17.1.2.3 Expansion Coefficients0.............................................................................. 437
17.1.2.4 Contraction Coefficients0 ........................................................................... 437
17.2

Computational Differences Between HEC-RAS and HEC-2....................... 438

17.2.1

Cross Section Conveyance Calculations.................................................... 438

17.2.1.1 Testing Using HEC-2 Conveyance Calculation Approach ......................... 439
17.2.1.2 Testing Using HEC-RAS and HEC-2 Approach ........................................... 439
17.2.2

Critical Depth Calculations .......................................................................... 440

17.2.3

Bridge Hydraulic Computations .................................................................. 441

17.2.3.1 HEC-2 Special Bridge Methodology............................................................ 441
17.2.3.2 HEC-2 Normal Bridge Methodology............................................................ 442
17.2.4

Culvert Hydraulic Computations ................................................................. 443

17.2.5

Floodway Encroachment Calculations ....................................................... 443

17.2.6

New Computational Features in HEC-RAS ................................................. 444

11

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

17.3

Computation of the WSPRO Discharge Coefficient and Effective Flow
Length ........................................................................................................... 444

17.3.1

Effective Flow Length .................................................................................. 445

17.3.2

Coefficient of Discharge.............................................................................. 449

17.4

Sediment Transport Functions – Sample Calculations............................. 480

12

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

Welcome to the HEC-RAS Hydraulic Reference Manual.

 – 13

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

1 Front Matter

US Army Corps
of Engineers
Hydrologic Engineering Center

1.1 HEC-RAS
River Analysis System

Front Matter – 14

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

1.1.1 Hydraulic Reference Manual
1.1.1.1 Version 6.0 Beta
December 2020
Approved for Public Release. Distribution Unlimited           CPD-69

REPORT DOCUMENTATION PAGE

Form Approved
OMB No. 0704-0188

Public reporting burden for this collection of information is estimated to average 1 hour per response,
including the time for reviewing instructions, searching existing data sources, gathering and maintaining
the date needed, and completing and reviewing the collection of information. Send comments regarding
this burden estimate or any other aspect of this collection of information, including suggestions for
reducing this burden, to Washington Headquarters Services, Directorate for Information Operations and
Reports, 1215 Jefferson Davis Highway, Suite 1204, Arlington, VA 22202-4302, and to the Office of
Management and Budget, Paperwork Reduction Project (0704-0188), Washington, DC 20503.
1. AGENCY USE ONLY (Leave
blank)

2. REPORT DATE
December 2020

3. REPORT TYPE AND DATES COVERED
Computer Program Documentation

4. TITLE AND SUBTITLE
HEC-RAS, River Analysis System Hydraulic Reference Manual

5. FUNDING NUMBERS

6. AUTHOR(S)
Gary W. Brunner
7. PERFORMING ORGANIZATION NAME(S) AND ADDRESS(ES)
US ARMY CORPS OF ENGINEERS
HYDROLOGIC ENGINEERING CENTER (HEC)
609 Second Street
Davis, CA 95616-4687

8. PERFORMING
ORGANIZATION
REPORT NUMBER
CPD-69

9. SPONSORING / MONITORING AGENCY NAME(S) AND
ADDRESS(ES)

10. SPONSORING /
MONITORING AGENCY
REPORT NUMBER

11. SUPPLEMENTARY NOTES
12a. DISTRIBUTION / AVAILABILITY STATEMENT
Approved for Public Release. Distribution is unlimited.

12b. DISTRIBUTION CODE

Front Matter – 15

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

13. ABSTRACT (Maximum 200 words)
The U.S. Army Corps of Engineers' River Analysis System (HEC-RAS) is software that allows you to
perform one-dimensional steady and unsteady flow river hydraulics calculations.
HEC-RAS is an integrated system of software, designed for interactive use in a multi-tasking, multi-user
network environment. The system is comprised of a graphical user interface (GUI), separate hydraulic
analysis components, data storage and management capabilities, graphics and reporting facilities.
The HEC-RAS system contains four one-dimensional hydraulic analysis components for: (1) steady flow
water surface profile computations; (2) unsteady flow simulation; (3) movable boundary sediment
transport computations; and (4) temperature and water quality constituent transport modeling. A key
element is that all four components use a common geometric data representation and common
geometric and hydraulic computation routines. In addition to the four hydraulic analysis components, the
system contains several hydraulic design features that can be invoked once the basic water surface
profiles are computed.
14. SUBJECT TERMS
water surface profiles, river hydraulics, steady and unsteady flow, One-dimensional
and two-dimensional hydrodynamics, computer program

15. NUMBER OF
PAGES
520
16. PRICE CODE

17. SECURITY
CLASSIFICATION OF REPORT
UNCLASSIFIED

18. SECURITY
CLASSIFICATION OF
THIS PAGE
UNCLASSIFIED

19. SECURITY
CLASSIFICATION OF
ABSTRACT

20. LIMITATION
OF ABSTRACT
UNLIMITED

UNCLASSIFIED

1.2 HEC-RAS
River Analysis System
1.3 Hydraulic Reference Manual
1.3.1 Version 6.0 Beta
December 2020
U.S. Army Corps of Engineers
Institute for Water Resources
Hydrologic Engineering Center
609 Second Street
Davis, CA 95616

Front Matter – 16

HEC-RAS Hydraulic Reference Manual – HEC-RAS Hydraulic Reference Manual

(530) 756-1104
(530) 756-8250 FAX
www.hec.usace.army.mil1

1.3.2 River Analysis System, HEC-RAS
The HEC-RAS executable code and documentation was developed with U.S. Federal Government resources
and is therefore in the public domain. It may be used, copied, distributed, or redistributed freely. However, it is
requested that HEC be given appropriate acknowledgment in any subsequent use of this work.
HEC cannot provide technical support for this software to non-Corps users. See our software vendors list (on
our web page) to locate organizations that provide the program, documentation, and support services for a
fee. However, we will respond to all documented instances of program errors. Documented errors are bugs
in the software due to programming mistakes not model problems due to user-entered data.
This document contains references to product names that are trademarks or registered trademarks of their
respective owners. Use of specific product names does not imply official or unofficial endorsement. Product
names are used solely for the purpose of identifying products available in the public marketplace.
Microsoft, Windows, and Excel are registered trademarks of Microsoft Corp.
ArcView is a trademark of ESRI, Inc.

1.3.3 Terms and Conditions of Use:
Use of the software described by this document is controlled by certain terms and conditions. The user must
acknowledge and agree to be bound by the terms and conditions of usage before the software can be
installed or used. The software described by this document can be downloaded for free from our internet site
(www.hec.usace.army.mil2).
The United States Government, US Army Corps of Engineers, Hydrologic Engineering Center ("HEC") grants to
the user the rights to install the HEC River Analysis System (HEC-RAS) "the Software" (either from a disk copy
obtained from HEC, a distributor or another user or by downloading it from a network) and to use, copy and/
