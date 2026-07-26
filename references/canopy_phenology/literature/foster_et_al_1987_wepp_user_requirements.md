# USDA-Water Erosion Prediction Project (WEPP)
## User Requirements (Draft 6.3, September 1, 1987)

**Compiled by:** G. R. Foster, Project Leader, 1985-1987 and L. J. Lane, Project Leader, 1987-Continuing  
**Cooperating Agencies:**  
- USDA-Agricultural Research Service (ARS)  
- USDA-Soil Conservation Service (SCS)  
- USDA-Forest Service (FS)  
- USDI-Bureau of Land Management (BLM)  

**Project Leader:** L. J. Lane, Hydrologist, USDA-ARS Aridland Watershed Management Research Unit, 2000 East Allen Road, Tucson, AZ 85719 (602) 629-6381  
**Report Citation:** NSERL Report No. 1, National Soil Erosion Research Laboratory, USDA-Agricultural Research Service, W. Lafayette, Indiana 47907  

---

### Cover Schematics: WEPP Model Versions

#### Watershed Version Schematic
![Watershed Version](./wepp-figures/wepp-fig-0.1-watershed-version.png)

> **WCAG AAA Description:**  
> **Figure Type:** Conceptual schematic diagram.  
> **Visual Layout & Orientation:** A representative small watershed boundary enclosed in an irregular oval shape. Within the boundary, an eroding hillslope area is depicted at the upper reaches, showing soil surface depression and an underlying nonerodible layer. An overland flow profile follows the hillslope down into a concentrated flow channel. Along the concentrated flow channel, an impoundment structure (dam or sediment control basin) is shown intercepting flow before it reaches the watershed outlet.  
> **Labels & Annotations:** Text labels point to "Eroded Area", "Nonerodible Layer", "Overland Flow Profile", "Watershed Boundary", "Concentrated Flow Channel", "Impoundment", and "Sediment Yield" (exiting at the bottom outlet arrow).  
> **Functional Meaning:** Illustrates the Watershed Version of the WEPP model, which integrates overland hillslope erosion, concentrated flow channel routing, sediment deposition in impoundments, and total sediment yield delivery at the field/watershed outlet.

#### Overland Flow Profile Version Schematic
![Overland Flow Profile Version](./wepp-figures/wepp-fig-0.2-overland-flow-profile.png)

> **WCAG AAA Description:**  
> **Figure Type:** Conceptual profile cross-section diagram.  
> **Visual Layout & Orientation:** A continuous slope profile curve moving from upper left (high elevation, steep slope) to lower right (low elevation, flattening/concave slope).  
> **Labels & Annotations:** The upper steep section is labeled "Soil Loss" and "Interrill-Rill Erosion", indicating net detachment and transport. As the slope flattens horizontally at the base of the hill, the curve is labeled "Deposition", indicating sediment dropping out of suspension. The arrow continuing beyond the deposition zone is labeled "Sediment Yield", which discharges directly into a curved depression labeled "Concentrated Flow Channel".  
> **Functional Meaning:** Illustrates the Landscape Profile (Hillslope) Version of WEPP, which computes detachment by raindrop impact and thin surface flow (interrill/rill erosion) on convex/steep slopes and net deposition on concave/flattening slope segments, terminating where concentrated channel flow begins.

#### Grid Version Schematic
![Grid Version](./wepp-figures/wepp-fig-0.3-grid-version.png)

> **WCAG AAA Description:**  
> **Figure Type:** Conceptual spatial grid overlay diagram.  
> **Visual Layout & Orientation:** A field boundary (irregular enclosed polygon) superimposed with an orthogonal Cartesian grid (horizontal and vertical lines dividing the field into square grid cells). Flow arrows originate within individual grid cells (representing "Overland Flow Elements") and converge into two main branching lines representing "Concentrated Flow Channels". A circular feature near the outlet represents an "Impoundment".  
> **Labels & Annotations:** Labels point to "Field Boundary", "Overland Flow Element", "Concentrated Flow Channel", "Impoundment", and "Sediment Yield" exiting the channel network at the bottom boundary.  
> **Functional Meaning:** Illustrates the Grid Version of WEPP, which discretizes a complex field or watershed into uniform grid cells to calculate spatially distributed sheet and rill erosion, routing sediment and runoff across grid boundaries into concentrated flow networks and impoundments to determine total net sediment yield.

---

## PREFACE

For over a quarter century, federal and state agencies, private industry, and individuals have used the Universal Soil Loss Equation (USLE) to evaluate soil erosion caused by water. However, new environmental concerns, new erosion modeling applications, and the Food Security Act of 1985 now require enhanced capabilities. In addition to USLE's estimates of sheet and rill erosion, estimates are needed for erosion by concentrated flow, sediment delivery to locations on the landscape, and locations and amounts of deposition along the landscape and in waterways. Modern erosion theory, experimental research, computer technology, and availability of databases provide the potential for a significant advancement in erosion prediction technology.

The Water Erosion Prediction Project (WEPP) is a joint effort of several federal agencies and their cooperators. This Project will provide agencies with a new generation, process-oriented technology for estimating erosion by water. At the beginning of the Project, user needs were evaluated to develop these User Requirements that describe the functions and features that the new technology is to embody.

The following federal agencies are the principal cooperators in WEPP, support the Project, and concur with the User Requirements:

**Agency Concurrence Signatures:**  
- **TERRY B. KINNEY**, Administrator, USDA-Agricultural Research Service (Signed by M. E. Carter)  
- **WILSON SCALING**, Chief, USDA-Soil Conservation Service  
- **R. MAX PETERSON**, Chief, USDA-Forest Service (Signed by J. Lamar Beasley)  
- **ROBERT F. BURFORD**, Director, USDI-Bureau of Land Management  

---

## ABSTRACT

**Foster, G. R. (Compiler). 1987. User Requirements: USDA-Water Erosion Prediction Project (WEPP). NSERL Report No. 1. National Soil Erosion Research Laboratory. USDA-Agricultural Research Service. W. Lafayette, IN. 43 pp.**

The objective of the USDA-Water Erosion Prediction Project (WEPP) is to develop new generation water erosion prediction technology for use by the USDA-Soil Conservation Service, USDA-Forest Service, USDI-Bureau of Land Management, and other organizations involved in soil and water conservation and environmental planning and assessment. This improved erosion prediction technology is to be based on modern hydrologic and erosion science, process oriented, and conceptually a significant improvement over the Universal Soil Loss Equation (USLE). The model and computer programs implementing the model are to run on computers available in local offices of the user agencies. The first version of the technology, described in these User Requirements, is scheduled for delivery in 1989. Subsequently, the technology will undergo extensive testing and evaluation by user groups while research continues to develop and refine relationships in the model. Delivery of the version intended for wide use is expected in 1992. The technology, expected to have a life cycle of about 20 years, is to be modular so that new research findings can be incorporated without major revisions to the model.

Research to develop the technology will include laboratory and field studies on more than 75 cropland, rangeland, and forestland soils and site conditions across the US. This experimental research will be conducted to understand how inherent soil properties and those modified by climate and land use affect infiltration and erodibility. These studies will provide parameter values needed to ensure that the WEPP erosion prediction technology will apply across a broad range of climates, soils, topographies, and land uses.

The major applications for this technology are in conservation planning, project planning, and inventories and assessments. The model, which applies to field-sized areas, will be in three versions: a landscape profile (hillslope) version that applies to landscape profiles similar to those for the USLE except that the WEPP model considers depositional areas, a small watershed version that uses a representative landscape profile and considers waterways within the application area, and a grid version that computes sediment movement at all points and in all waterways over a field-sized area. The models is to describe the influence of climate, soil, topography, cover-management, and supporting practices on erosion, deposition, sediment yield, and sediment characteristics. Operational requirements for the model include: that it runs quickly on the computer and be easily used, broadly applicable, robust, and valid.

### PROCESS USED TO DEVELOP USER REQUIREMENTS

These User Requirements are a compilation of input from many sources in the cooperating agencies. Six major drafts of the User Requirements were written and reviewed. As a part of the process, members of the WEPP Core Team visited each of the four USDA-Soil Conservation Service Technical Centers (SCS) to receive first hand input from representatives of all levels of SCS. Erosion prediction committees, technical steering committees, and individuals in all of the cooperating agencies provided major input to the User Requirements. Discussions at WEPP Core Team meeting resolved differences between user expectations and the project's ability to meet certain potential requirements.

---

## CONTENTS (Table of Contents)

1. [INTRODUCTION](#1-introduction)  
2. [GENERAL APPLICATIONS](#2-general-applications)  
   - 2.1 [SCS Applications](#21-scs-applications) (Conservation Planning, Project Planning, Inventory & Assessment)  
   - 2.2 [BLM Applications](#22-blm-applications)  
   - 2.3 [FS Applications](#23-fs-applications) (Project Planning & Assessment, Forest Planning, Conservation Planning)  
3. [GENERAL REQUIREMENTS](#3-general-requirements)  
   - 3.1 [Size Area](#31-size-area)  
   - 3.2 [Required Erosion, Deposition, and Sediment Yield Estimates](#32-required-erosion-deposition-and-sediment-yield-estimates)  
   - 3.3 [Major Factors](#33-major-factors)  
   - 3.4 [Applicability](#34-applicability)  
   - 3.5 [Other General Requirements](#35-other-general-requirements)  
4. [REQUIREMENTS FOR MAJOR FACTORS AFFECTING EROSION](#4-requirements-for-major-factors-affecting-erosion)  
   - 4.1 [Climate](#41-climate) (Erosive Agents, Frequency of Estimates, Climatic Inputs)  
   - 4.2 [Soil](#42-soil) (Soil Criteria, Soil Input Values)  
   - 4.3 [Topography](#43-topography) (Representations, Profile Version, Watershed Version, Grid Version, Resolution, Spatial Variability)  
   - 4.4 [Land Use](#44-land-use) (Cropland, Rangeland, Disturbed Forestlands, Management Systems, Supporting Practices, Modules)  
5. [OPERATIONAL REQUIREMENTS](#5-operational-requirements)  
   - 5.1 [Computational Time](#51-computational-time)  
   - 5.2 [Ease of Use](#52-ease-of-use)  
   - 5.3 [Applicability to Broad Range of Conditions](#53-applicable-to-broad-range-of-conditions)  
   - 5.4 [Robustness](#54-robustness)  
   - 5.5 [Validity & Validation Criteria](#55-validity)  
   - 5.6 [Ease of Explanation](#56-ease-of-explanation)  
6. [IMPLEMENTATION OF PROCEDURE](#6-implementation-of-procedure)  
   - 6.1 [Model Considerations](#61-model-considerations) (Structure, Hydrologic Elements, Family of Models, Similarities, Modification)  
   - 6.2 [Coding](#62-coding)  
   - 6.3 [Documentation](#63-documentation)  
7. [APPENDIX 1 - IMPORTANT SOIL PROPERTIES AFFECTING SOIL ERODIBILITY](#appendix-1---important-soil-properties-affecting-soil-erodibility)  
8. [APPENDIX 2 - SOILS LIST](#appendix-2---soils-list) (Tables 8.1 through 8.12)  
9. [APPENDIX 3 - TOPOGRAPHIC REPRESENTATIONS](#appendix-3---topographic-representations) (Figures 9.1 through 9.6)  
10. [APPENDIX 4 - EXAMPLE STRIP CROPPING SYSTEMS THAT WEPP IS TO CONSIDER](#appendix-4---example-strip-cropping-systems-that-wepp-is-to-consider) (Figures 10.1a through 10.1d)  
11. [APPENDIX 5 - HYDROLOGIC SITUATIONS NOT WELL TREATED BY WEPP](#appendix-5---hydrologic-situations-not-well-treated-by-wepp)  
12. [APPENDIX 6 - SOME ATYPICAL CROP ROTATIONS](#appendix-6---some-atypical-crop-rotations)  
13. [APPENDIX 7 - EXAMPLE OF A FUNCTION THAT ADDS ROBUSTNESS TO MODEL](#appendix-7---example-of-a-function-that-adds-robustness-to-model)  
14. [APPENDIX 8 - SIMILARITIES OF WEPP WITH OTHER MODELS THAT MIGHT BE USED BY SCS FOR EROSION COMPUTATIONS](#appendix-8---similarities-of-wepp-with-other-models-that-might-be-used-by-scs-for-erosion-computations)  
15. [COVER DESIGN & CORE TEAM DIRECTORY](#cover-design--core-team-directory)  

---

---

USER REQUIREMENTS
USDA-Water Erosion Prediction Project (WEPP)
Draft 6.3
# 1. INTRODUCTION:
The objective of this project is:
To develop new generation water erosion
prediction technology for use by the USDA-Soil
Conservation Service USDA-Forest Service, and
USDI-Bureau of Land Management, and other
organizations involved in soil and water conservation and environmental planning and assessment.
In particular, the project will develop
improved erosion prediction technology based on
modern hydrologic and erosion science that will
be process oriented and conceptually a significant
improvement over the Universal Soil Loss Equation (USLE). If the project is fully successful, the
user will choose the technology developed by this
project over the USLE.
The target group for the new technology is all
current USLE users and specifically USDA-Soil
Conservation Service (SCS). Field personnel
within SCS are the expected primary users group
although many others within and outside of SCS
will be interested in the technology. A major
secondary group of expected users is personnel in
the USDI-Bureau of Land Management (BLM)
and USDA-Forest Service (FS). Other agencies
including the USDA-Extension Service, U.S.
Department of Energy, U.S. Environmental Protection Agency, U.S. Federal Highway Administration, and various state and local agencies are
also prospective users of this new erosion prediction technology.
The USLE is by far the most widely used
method for predicting sheet and rill erosion. The
origin and form of the equation evolved from
A.W. Zingg’s 1940 equation for the effect of slope
length and steepness on erosion. The USLE is a
lumped model since it does not define separate
factor relationships for the fundamental hydrologic processes of rainfall, infiltration, and runoff
and the fundamental erosional processes of
detachment by raindrop impact, detachment by
flow, transport by rain splash, transport by flow,
and deposition by flow. Furthermore, the USLE
is empirical, depends on a large mass of data for
its relationships, and is very much a “black box”
model. Both its empirical origin and lumped
equation structure severely limit the potential for
increased accuracy and major improvement.
Scientifically the USLE is considered to be
mature technology.
When the USLE was developed more than 20
years ago, it had to be a simple mathematical
expression so that it could be solved with the
computational equipment available at the time.
However, small, computers are now readily available that can solve complex mathematical relationships quickly and easily in the field, local
office, and even the farmer’s home. Thus users
now have the potential for using erosion prediction technology based on modern science of
hydrologic and erosional processes that was
impractical two decades ago. The objective of
this project is to convert fundamental and available knowledge from these sciences to a form that
the user can conveniently apply as an alternative
to the USLE.
The output of this project may be a family of
models and computer programs rather than a single model and program. This document defines
the requirements that must be met to ensure that
the model(s) meet the users’ expectations and can
be applied as anticipated. The delivered model(s)
is expected to meet these requirements, and conversely, if the model(s) meets these requirements,
the project will be judged as having delivered the
intended product. Similarly, scientists and other
technical specialists wishing to directly contribute
to the model(s) are expected to provide information that meets the User Requirements.
The target date for completion of the project
defined by this User Requirements document is
August 1989. However, the prediction technology
including the required input values for all potential applications will not be complete at that time,
and therefore additional research and development will be required beyond 1989. The situation
is analogous to the development of the USLE,
which became available soon after 1960. Even
though the USLE was usable at its introduction,
research continued that expanded the applicability
and accuracy of the USLE within its original
structure. In the same way, the prediction procedure developed by this project will provide a
structure that will accommodate future

development and application to a broad range of
conditions. When the model(s) is delivered in
1989, parameter values will be provided so that
the model(s) will be usable for selected “key”
situations of major importance.
Development of this erosion prediction technology will be a multi-agency and multi-discipline
effort. A Core Team that has been formally
appointed to lead the development is:
L.J. Lane, ARS, Tucson, AZ
(Project Leader)
E.E. Alberts, ARS, Columbia, MS
J.E. Gilley, ARS, Lincoln, NE
J.M. Laflen, ARS, W. Lafayette, IN
A.D. Nicks, ARS, Durant, OK
W.J. Rawls, ARS, Beltsville, MD
J.R. Simanton, ARS, Tucson, AZ
L.T. West, ARS, W. Lafayette, IN
H.D. Fox, SCS, Tucson, AZ
C.S. Holzhey, SCS, Lincoln, NE
D.L. Schertz, SCS, Washington, D.C.
G.A. Weesies, SCS, W. Lafayette, IN
E.R. Burroughs, FS, Moscow, ID
J.O. Nordin, FS, Washington, D.C.
G.D. Wingate, BLM, Susanville, CA
In addition, J. M. Bradford, ARS, W. Lafayette, IN and K. G. Renard, ARS, Tucson, AZ
have been appointed as Resource Specialists to
the Core Team in the soils and rangeland areas,
respectively. The work is overseen by the ARS
Erosion Prediction Technology Committee
chaired by K. G. Renard, by the ARS National
Program Staff represented by W. D. Kemper,
Beltsville, MD, and by the SCS Erosion Prediction Committee chaired by K. W. Flach, Washington, D. C. A companion project led by L. J.
Hagan, ARS, Manhattan, KS, is underway to
develop new technology to estimate soil erosion
by wind.

## 2. GENERAL APPLICATIONS

The primary identified user of the erosion
prediction technology being developed by this
project will be the USDA-Soil Conservation Service (SCS) at all levels of its operations including
the local field office (District), Area Office, State
Office, National Technical Center, and National
Headquarters. The principal SCS user however
will be the local field. office. Another major identified user will be the USDI-Bureau of Land
Management (BLM), which will use the technology principally at its field office level and the
USDA-Forest Service (FS) at its Forest and District office levels. The needs of other potential
users will be accommodated to the degree that
their requirements match those listed in this document.

### 2.1 SCS Applications

Most SCS applications fall within the
categories of: (a) conservation planning, (b) project planning, and (c) inventory and assessment.

#### 2.1.1 Conservation Planning

In SCS conservation planning, the major application of this erosion prediction technology is to predict sheet and
rill erosion, concentrated flow erosion, and sediment yield at specific field sites. These predictions are made for existing conditions and for a
proposed set of alternative conservation practices
to provide erosion estimates used to guide the
selection of a resource management (soil conservation) system for the specific site. A resource
management system is usually being chosen to
control: (a) erosion to a tolerable rate for productivity maintenance, (b) on-site deposition to
prevent excessive adverse effects from deposition,
(c) sediment yield from fields to allowable rates
that prevent excessive off-site sedimentation, and
(d) sediment yield from fields to prevent excessive
degradation of off-site water quality.
Conservation planning in SCS occurs at the
field office level, and it usually takes place in the
field with the conservationist working directly
with the land user. Not only does the prediction
procedure provide the erosion estimates needed to
develop the conservation plan, it is a communication tool that helps the conservationist describe
the erosion process and how soil conservation
practices work to control erosion. Thus the technology must be based on concepts and principles
that can be easily understood and described by
the conservationist to the client. In this application, the major need is to provide “accurate” estimates using as few resources as possible and to
maximize the quality and quantity of service that
SCS field personnel provide to clients.

#### 2.1.2 Project Planning

In SCS project planning,
the prediction technology is used most frequently
as a tool to help: (a) determine the erosion control measures and their distribution over a project

area needed to meet specific project objectives, (b)
estimate sediment yield and sediment characteristics needed to design off-site water conveyance
and impoundment structures and to evaluate offsite impacts from sedimentation, and (c) estimate
sediment yield and sediment characteristics
needed to develop erosion control plans for
improvements in downstream water quality. The
prediction procedure is applied in the office using
field data collected by field personnel, and the
application may be more computer intensive than
for conservation planning. Comparing project
planning to conservation planning, project planning allows: (a) more expenditure of time and
effort for collecting input data, (b) more emphasis
on detail for representative subareas where computed results are expanded to the entire project
area, (c) more emphasis on sediment yield from
fields, and (d) greater use of interdisciplinary
teams, although field personnel typically collect
field data for input.

#### 2.1.3 Inventory and Assessment

As opposed to
planning, which considers "what if’ questions,
SCS inventory and assessment activities are concerned with determining erosion and sediment
yield for the present state. Also, rather than
application to specific fields, watersheds, and project areas, the analysis is usually computing erosion and sediment yield at selected sample points
and areas and aggregating these results for
county, regional, state, and national areal units.
The technology is usually applied in a “national”
office where considerable computer and data base
resources are available. The inputs frequently
come from existing data bases (e.g., climatic and
soil files), remotely sensed data, and surveys conducted by field office personnel. The use of the
technology and the analysis is usually conducted
by and under the direction of an SCS national
assessment staff.

### 2.2 BLM Applications

Most of the BLM applications are by field
office personnel needing erosion estimates to
develop range management plans that will reduce
sheet-rill erosion, concentrated flow erosion, and
sediment yield to acceptable levels to prevent
excessive loss of the productive capacity of rangeland soils and to prevent excessive off-site sedimentation. Development of range management
plans includes evaluating the effects of livestock
grazing systems and rangeland improvements
such as vegetation conversions, brush control,
mechanical/tillage treatments, and seedings on
erosion. This planning activity is similar to SCS’s
conservation planning.
Other important applications are in Watershed
Activity Planning and Watershed Condition
Analysis. Watershed Activity Planning is directly
analogous to SCS Project Planning and leads to
watershed improvement projects. Watershed
Analysis is analogous to SCS Inventory and
Assessment and is usually conducted on a
Resource Area or District scale.
The BLM requires technology that is easy to
use in areas where little supporting climatic, soil.
land use, and intensity of land use data may be
available. The need is also to have a tool that is
relatively simple and easy to use, comparable to
the SCS conservation planning needs.

### 2.3 FS Applications

Two branches of the USDA-Forest Service
are potential users of this erosion prediction technology. The branches are the National Forests
and the State and Private divisions of the FS.
The State and Private division of the FS advises
state and local governmental agencies and private
organizations and individuals. The principal FS
users will be resource specialists at the Forest and
District level. Applications by the FS are similar
to those of SCS, but vary slightly because of
specific direction for management of Federal
lands. Categories include: (a) project planning
and assessment, (b) forest planning, and (c) conservation planning.

#### 2.3.1 Project Planning and Assessment

In the
FS, project planning is generally conducted under
the guidance of the National Environmental Policy Act, the Multiple-Use Sustained-Yield Act,
and the National Forest Management Act of
1976. These acts established procedures for
evaluating resource management actions.
Most projects within the FS are evaluated by
an interdisciplinary team of resource specialists.
Jointly, they review major issues, list management
concerns, develop alternatives, collect necessary
resource information, and evaluate the alternatives. The preferred alternative is selected by the
appropriate line officer. All decisions are subject
to public review and appeal. Often it is the process that is appealed.
Types of projects include, but are not limited
to: range allotment planning, range improvements, road construction, mineral exploration
and development, timber harvest, reforestation,

wildlife habitat improvement projects, developed
recreation sites, trails, prescribed fire, wildfire restoration, off road vehicle use, borrow sites, and
land exchanges.
Most projects involve activities that do not
recur annually. Often they require disturbance
for a certain period followed by restoration and
then a long period of limited or no activity.

#### 2.3.2 Forest Planning

The National Forest
Mangement Act of 1976 required all Forests to
develop and evaluate alternative levels and mixes
of land management activities. Activities were
scheduled for 10 years, with projections for five
decades. Modeling of erosion and sediment was
included in many Forest Plans. Sediment values
were also used in some cases to evaluate mangement effects on other resources, such as fisheries.
Long-term monitoring is required to ensure compliance with legal requirements and to evaluate
assumptions. Environmental Impact Statements
are prepared for each Forest Plan.
Erosion and sediment estimates are made for
response areas within each Forest. The results
need to reflect impacts of management alternatives over large land areas. Results should also
reflect changes over time.

#### 2.3.3 Conservation Planning

This application
concerns evaluating the use of specific practices,
which is often accomplished by the FS watershed
specialist acting as an In-Service consultant to
Ranger Districts. Quick answers are needed in
the field with limited input information. “Ball
park” estimates should be possible without the
use of a computer.
Also included in this application is training of
non-watershed personnel.

## 3. GENERAL REQUIREMENTS

### 3.1 Size Area

The erosion prediction procedure from this
project is to apply to “field-sized” areas or conservation treatment units. Although the size of a
particular field to which the procedure applies
will vary with degree of complexity within a field,
the maximum size “field” is about a section (640
acres) although an area as large as 2,000 acres is
needed for some rangeland applications. On very
complex areas, the “field” may be much smaller
than 640 acres. The procedure will not apply to
areas having incised, permanent channels such as
classical gullies and stream channels. The
channels that the procedure is to include are those
farmed. over and known as concentrated flow or
“cropland ephemeral gullies.” Also, the procedure
is to apply to constructed waterways like terrace
channels and grassed waterways. In rangeland
and forest applications, “fields” can include gullies
up to the size of typical concentrated flow gullies
in 640 acre cropland fields. These channels are
about 3 to 6 ft wide by about 3 ft deep. The procedure is not to apply to headcut erosion, sloughing of gully sidewalls, or the effects of seepage on
erosion in concentrated flow channels.

### 3.2 Required Erosion, Deposition, and Sediment Yield Estimates

The procedure is to compute: (a) sheet-rill erosion and deposition by overland flow along
selected landscape profiles or over an entire field,
(b) concentrated flow (ephemeral gully) erosion
along selected channels or over the entire channel
network within a field, and (c) sediment yield and
its sediment characteristics from selected
watersheds within the field or at all outlet points
from the field. To meet these requirements, the
procedure is to include three basic versions: (a) a
representative landscape profile version, (b) a
watershed version, and (c) a grid version that covers the entire field:
In these User Requirements, “fineness” refers
to sediment characteristics. In general, fineness
can be an enrichment ratio based on specific surface area. However, at the request of the user,
the procedure is to compute erosion, deposition,
and transport for a minimum of five sediment
particle classes that can vary by diameter, density,
and composition by primary particles and organic
matter.
Estimates from the water erosion prediction
procedure should be in a form with respect to
space and time that they can be combined with
estimates from wind erosion prediction procedures to support evaluations of the combined
effects of wind and water erosion. This requirement can be met by computing average annual
erosion rates at any point in the field.

### 3.3 Major Factors

The procedure is to describe the influence on
erosion, deposition, and sediment yield of the
major factors: (a) climate, (b) soil, (c) topography, (d) cover-management, and (e) supporting
practices. The last two factors describe land use.

### 3.4 Applicability

Ultimately the procedure is to apply to all
U.S. locations including Alaska, Hawaii, and
Puerto Rico. Furthermore, the procedure should
be developed with the goal that it will apply .
worldwide. The procedure is to be process based
to meet this broad range of applicability. In particular, the effects of cropping and management
will be described by a component structure based
on canopy, ground cover, roughness, soil consolidation, and similar components. This project will
provide the basic relationships to meet this
requirement, but field application of the procedure will depend on the availability of parameter values. This project will determine parameter
values for a set of “key” soils, crops, management,
tillage, and supporting practices specified in a
later section.

### 3.5 Other General Requirements

The prediction technology must be easily used
with easily understood guidelines. Also, it must
use minimal and easily obtained inputs, which are
specified in a later section. When applied to conservation planning, the procedure must be portable for use in the office, truck, field, or client’s
house. The procedure when implemented on a
computer should be accessible by telephone if the
user desires special features or data not available
in a “standard” field version.

## 4. REQUIREMENTS FOR MAJOR FACTORS AFFECTING EROSION

### 4.1 Climate

#### 4.1.1 Erosive Agents

The procedure is to compute erosion, deposition, and sediment yield
caused by: (a) impacting waterdrops from rainfall
or sprinkler irrigation and (b) surface flow from
rainfall, sprinkler or surface irrigation, or
snowmelt, including runoff from snowbanks. The
procedure should also compute erosion of thawing soil.
4.1.2. Frequency of Estimates At the choice of
the user, values will be computed on the following
basis: (a) long term, average annual, (b) distribution by crop stages or seasons over an average or
typical year, (c) frequency distribution of annual
amounts, (d) frequency distribution by month, (e)
frequency distribution by event, (f) single design
storm, and (g) continuous simulation. These
computations may assume time invariant soil,
topographic, and land use conditions except in
cases where the cover conditions consistently
change over a period, like recovery following a
disturbance in a forest. The continuous simulation option is not expected to be used in conservation planning.

#### 4.1.3 Climatic (Weather) Inputs

Climatic
(weather) inputs can be, but are not limited to
values: (a) generated by a stochastic (random)
weather generator, (b) obtained from historical
weather records, or (c) derived from design storm
characteristics including additions of water by
sprinkler or surface irrigation. In any case, the
climatic inputs shall be retrievable from a
prerecorded record that can be directly accessed
by the computer program implementing the procedure, and use of the procedure shall require no
action by the user when the procedure is applied
within specified geographic regions. An objective
is to use a minimum number of storms in a given
application. Direct use of long term, daily, or
storm-by-storm historical weather data is least
desirable. In the case of design storms, the maximum information that should be expected from
the user is: (a) storm amount, (b) average intensity, (c) ratio of peak intensity to average intensity, and (d) time to peak. When design storms
are used, the user is to have the capability of
choosing parameter values for existing conditions
such as soil moisture.
The FS and the BLM collect weather information at a great number of sites, both remotely and
at manned stations. This information is maintained in the Administrative Forest Fire Information and Retrieval Management System
(AFFIRMS) and National Fire Weather Data
Library (NFWDF). The ideal situation would be
for the user to be able to select data from a
nearby site, and then, through a simple model,
have the data modified for any change in elevation and aspect.

### 4.2 Soil

The procedure must apply to a list of “key”
soils as agreed to by SCS, BLM, FS, and ARS.

#### 4.2.1 Soil Criteria

The "key” soils should include
a cross section of U.S. soils having a range of
properties thought to affect erodibility by raindrop impact and surface flow, sediment properties
at the point of detachment, infiltration, and
antecedent soil moisture. The procedure must
deal with eroded and uneroded soil phases. The
criteria for selecting specific soils for the ”key”
soils list is: (a) values of important properties are

uniformly spread over their range, (b) inclusion of
at least five soils within the textural classifications
of sandy loam, loamy sand, and sand (group of
soils where runoff limits erosion) and at least five
soils within the textural classifications of sandy
clay, silty clay, and clay (group of soils where
detachment limits erosion), (c) inclusion of several
soils with major erosion problems, (d) inclusion
of range soils, and (e) inclusion of soils susceptible to both water and wind erosion.
Selection of soils should consider differences
among soils according to their separate susceptibilities to interrill erosion and rill erosion and
according to characteristics of their sediment at
the point of detachment that are important in
transport and deposition processes. Ideally the
soils on the “key” soils list would reflect a range
of soil properties that affect soil erodibility and
transportability indices. These properties could
include: (a) texture, (b) organic matter, (c) aggregate size and stability, (d) soil structure, (e) bulk
density, (f) soil surface shear strength, (g) crust
thickness and penetration resistance, (h) water
retention, (i) Fe and Al oxides, (j) Na, Mg, Ca,
and K contents, (k) salinity characteristics, (1) acidity, (m) cation exchange capacity, (n) clay
mineralogy, and (0) morphology of soil particles
at and near the soil surface. Those properties
important in determining “base” erodibility values
are ones that are assumed to be time-invariant
and are not affected by land use with the exception of properties like aggregate size and stability,
susceptibility to crusting, soil strength, and bulk
density. These and similar properties must be
determined for a “standard” condition. A more
detailed and orderly listing of soil properties
important to soil erodibility is given in Appendix
1.
The Soils Lists given in Appendix 2 reflect a
consideration of these criteria and a range of
these properties. The Lists represent those soils
that when limited to about 20 to 30 cropland,
range, and forest soils would be ones that provide
a range of characteristics and are ones for which
the users would most want soil erodibility values
when the WEPP model is delivered in August
1989.
The key soils must include a cross section of
soils characteristic of mountainous rangeland and
forestland. The number of soils evaluated will be
limited but should represent a wide range in geographic location, geomorphic situation, vegetative
community, elevation, climatic conditions,
management, and rock fragment content. Slopes
between 20 and 75 percent should be represented.

#### 4.2.2 Soil Input Values

for the important timeinvariant soil properties needed by the procedure
should be prerecorded, and their input to the procedure shall require no action by the user other
than to specify a soil survey mapping unit. The
required properties could be available in a soils
data base that may be remote from the office site.
However, over time, the required soils data base
needed by a local office should be available on
the field office computer. The field user should
be able to optionally change soil properties from
those given in the data base, if on-site conditions
indicate that a change is desirable. The user can
also specify different soils at different locations in
the field.

### 4.3 Topography

#### 4.3.1 Types of Topographic Representations

Topography is to be considered in three ways.
One form of the procedure is the one that applies
to a given landscape profile. This profile is
chosen by the user in much the same way that a
landscape profile is chosen to apply the USLE.
The difference is that slope length would be the
distance from the origin of overland flow to a
concentrated flow channel or waterbody rather
than end at a depositional area. This application,
referred to as the profile version, considers sheet
and rill erosion, but not concentrated flow erosion. The second version is the watershed version, which applies to a given watershed within
the field. It considers sheet-rill erosion and deposition by overland flow, erosion and deposition by
concentrated flow channels, and deposition in
impoundments. The watershed version applies to
selected watersheds within the field being
analyzed, and it does not refer to a version that
can apply to large complex watersheds. The third
version, the grid version, completely covers the
field with grid elements and computes sheet and
rill erosion and deposition by overland flow for
each grid, erosion and deposition by concentrated
flow along each concentrated flow channel within
each grid, deposition in each impoundment within
the field, and sediment yield at all outlet points
from the field.

#### 4.3.2 Profile Version

The profile version is
applied to landscape profiles selected by the user.

##### 4.3.2.1 Output

The profile version is to compute: (a) net erosion or deposition, (b) rill erosion or deposition by flow, (c) interrill erosion,

(d) fineness of the eroded or deposited sediment,
(see Section 3.2 for a definition of fineness), (e)
sediment load, and (f) fineness of the sediment
load. As a minimum, this version outputs average soil loss for the slope length (sediment
yield/unit of slope length). Erosion rate for the
erosional parts of the profile and deposition rate
for the depositional parts of the slope are to be
computed for slope length segments located
along the slope length as specified by user input.
The procedure must have the capability to store
results from individual profiles within a field and
to use these values to compute weighted averages
for the field based on the fraction of the field that
each profile represents.

##### 4.3.2.2 Input

The user will have two options to
specify characteristics of the landscape profile.
The options are: (a) to specify a slope length,
average steepness, location of the major inflection
point, degree of curvature of the upper slope, and
degree of curvature of the lower slope and (b)
locations and slope steepnesses along the profile.
These features are indicated in Figure | of
Appendix 3.

#### 4.3.3 Watershed Version

To apply the watershed
version, the user chooses a watershed within the
field to apply the model. This version must at
least apply to the situations shown in Figures 2
through 5 of Appendix 3. Symmetry between
subwatersheds may be assumed.

##### 4.3.3.1 Output

The outputs for the slope length
profile portion of the watershed version is the
same information produced by the profile version.
The output for the concentrated flow portion is
to be the same as that for the profile portion
except that output is by channel reach rather than
by slope length segment.

##### 4.3.3.2 Input

Inputs for the profile portion of
the watershed version are to be the same as those
for the profile version. Inputs for the concentrated flow portion are to be: (a) channel cross
section properties, (b) locations along the channel
and grades at those locations, (c) information on
the outlet control to the channel, and (d) drainage
areas at the upper and lower ends of the channels.
The channel outlet control will consider backwater from: (a) uniform flow in the last reach of the
channel or uniform flow in a specific outlet channel at the end of the given channel, (b) critical
depth at the end of the channel, or (c) a natural
or constructed outlet control with a known rating
curve. The input for a within-field impoundment
will be the sideslopes that form the basin or coefficients for an area-depth curve and simple information on the rating function that describes flow
out of the impoundment.

#### 4.3.4 Grid Version

The grid version describes
erosion and sediment movement in grids that can
cover entire. regular or irregularly shaped fields
with boundaries that may not coincide with
watershed boundaries as shown in Figure 6 of
Appendix 3.

##### 4.3.4.1 Output

The outputs will be the same as
those for the watershed version except that the
values are based on grid areas rather than on
slope length or channel reach segments.

##### 4.3.4.2 Input

The input to the grid model will
be slope steepness and direction for each grid and
the grade and direction of a channel reach within
a grid. Properties of channel cross sections and
channel outlet controls will be input like those in
the watershed version. Topographic field data
needed to analyze a 640 acre field should be
obtainable within eight hours by three people. As
‘an alternative to collecting field data, the procedure shall accept data derived from 2 ft contour
maps or maps with a greater contour interval
provided an adequate definition of the slope profiles and concentrated flow paths can be determined.

#### 4.3.5 Resolution

The procedure is to use a
minimum number of slope length and channel
reach segments. The minimum should be that
needed to provide the detail of output information requested by the user and that needed for
satisfactory computational accuracy. Satisfactory
computational accuracy is achieved when computed values are within 10 percent of their “exact”
values. The purpose of this requirement is to
minimize run time while maintaining satisfactory
computational accuracy.

#### 4.3.6 Spatial Variability

The procedure is to
allow infiltration, soil erodibility, and other soil
factor values to vary as soil mapping units vary
and crop yield varies along a landscape profile,
along a concentrated flow channel profile, and
over a field. The procedure must be able to
accommodate. at least 10 combinations of land
use situations’ for a profile, channel, or field and
1 A land use situation or condition is defined. by differences
in crop, tillage, or sequence in a rotation that have a
significant effect on erosion.

ten strips, each having a different land use condition, in strip-type situations. Some of the most
complex strip cropping situations are illustrated
in Appendix 4. In addition to dealing with “conventional” strip cropping, the procedure is to consider skip-row farming common to some cotton
farming systems where four rows of crop are
alternated with two blank rows and other similar
combinations are used. Also, the procedure is to
accommodate the different land use situations
indicated in Figures 2 through 5 of Appendix 3.

### 4.4 Land Use

#### 4.4.1 Land Uses Considered by Prediction Procedure

The land uses to be considered by the
erosion prediction procedure produced by this
project include: cropland; vegetable land; hayland;
pastureland; orchards; vineyards; nurseries; rangeland; disturbed forest land; construction sites;
road surfaces, cuts, and fills; surface mines; hazardous waste sites; recreational; and other land uses
where surface flow occurs over the entire area and
erosional processes are only slightly affected by
"partial area” hydrology as illustrated in. Appendix 5. These lands may be non-irrigated,
sprinkler irrigated, or surface irrigated. This project will provide the basic relationships from a
component analysis to describe the important factors of land use that affect erosion. Although
parameter values unique to the land uses listed
above. may be required, this project will directly
provide only those for the “key” land _ uses
described below. Scientists contributing to the
project on their own may provide some of the
additional parameter values.

##### 4.4.1.1 Cropland

The procedure as delivered in
1989 is to work satisfactorily for: the rowcrops of
corn, soybeans, sorghum, and cotton; small grain;
hayland that may be periodically cultivated; and
pastureland that is occasionally renovated.. The
procedure as delivered in 1989 also will apply to
other land uses such as vegetable crops, orchards,
vineyards, and close grown legumes, but using
parameter values based on judgement rather than
data from specific field experiments on those
practices. The procedure is to apply to all land
uses as satisfactorily as does the USLE, recognizing that the USLE is applied to many situations
without supporting experimental data.

##### 4.4.1.2 Rangeland

The procedure will consider:
grasslands, brushland, savanna, tundra, sagebrush
lands, salt-desert shrublands, Southwest semidesert grasslands, lands, deserts, pinjon-juniper
woodlands, chaparral, and badlands. Rangelands
can be highly nonuniform in microtopography
and vegetation and a range site can have 15 to 20
separate species of plants. Rangelands are diverse
and vary greatly as a result of natural processes
that occurred over many years. The procedure is
to: be able to account for natural differences in
rangelands as well as differences caused by
specific grazing, vegetative, and mechanical
management practices. The procedure is to work
for rangelands common to all of the U.S, including steep, mountainous rangelands and alpine
areas.

##### 4.4.1.3 Disturbed forestlands

The procedure is
to apply to forest lands where overland flow
occurs and usual hydrologic procedures used to
describe overland flow hydrology can be reasonably applied. Overland flow does not occur in
many undisturbed forests and thus this procedure
would not apply to those situations. However,
the procedure is expected to apply in common
forest activities where harvesting, road construction and maintenance, site preparation, burning,
grazing, mining, and other activities have disturbed the cover and exposed mineral soil enough
that overland flow occurs. The procedure is also
expected to apply in undisturbed forests where
overland flow occurs.

##### 4.4.1.4 Management and tillage systems

On cultivated land, the procedure will accommodate:
(a) clean tillage systems (e.g., moldboard plow,
disk, field cultivate, plant, harvest); (b) conservation tillage systems (e.g., no-till, ridge till) identified by residue level, degree of disturbance, ridge
height, and other factors; (c) solid seeded, low
residue crops; (d) crop rotations with durations at
least through ten years [See Appendices 4 and 6
for some atypical rotations.] (e) multiple (double,
triple) cropping; (f) intercropping, (g) manure
additions, (h) silage and residue removal, (i)
mulching, (j) winter cover crops, (k) winter grazing, and (1) up to five hay cuttings. On rangeland, the management systems that the procedure
is to describe include: (a) grazing management,
(b) brush management, (c) range seeding, and (d)
conversion to and from rangeland. On forest
land, it is to consider typical practices.

##### 4.4.1.5 Supporting (mechanical) practices

These
practices for any land use include: (a) contouring, (b) ridging, (c) bedding, (d) hillside ditches,
(e) furrow diking, (f) terraces (level, gradient, and
tile outlet), (g) diversions, (h) strip cropping, (i)
divided slopes; (j) grass buffer strips, (k)

subsoiling and ripping, (1) land imprinting, (m)
root plowing, (n) pitting, (0) chaining, (p) tree
grubbing, and (q) prescribed burning and other
fire. Other practices having an impact on erosion
control that are to be considered include: (a)
controlled traffic, (b) water and sediment control
basins, (c) small impoundments (d) grassed waterways, (e) surface drains, (f) tile drains, (g) surge
irrigation, and (h) irrigation reorganization such
as changing length of run and slope.

#### 4.4.2 Representation of Land Use Effects

A
component method will be used to represent the
effects of land use on erosion and deposition.
The components considered to be most important are: (a) canopy; (b) ground cover (including
crop residue, stems and leaves, growing plants
touching the soil, rock fragments, and surface
litter); (c) surface roughness (random and oriented
-- tillage marks); (d) incorporated residue (short
term mechanical and long term effects); (e) below
ground biomass (incorporated residue and roots);
(f) land use residual ( e.g., soybean and meadow
effects); (g) seasonal (crop growth and soil effects
other than antecedent soil moisture and soil thawing); (h) soil thawing; (i) soil consolidation; (k) tillage (type, pulverization, and recency); and (I)
wetting, drying, traffic, and other recent events
affecting soil erodibility.
4.4.3, Major Land Use Modules The major
modules of the procedure needed to describe land
use will include: (a) crop growth, (b) tillage, (c)
residue decomposition, (d) grazing and (d) supporting practices.

##### 4.4.3.1 Crop module

This module is expected to
estimate: (a) canopy cover percentage and height
with time, (b) residue mass and coverage at harvest, and (c) below ground biomass per unit area
and its distribution with depth and time. These
growth characteristics will be described by nondimensional growth curves identified by major crop
type. The user is to have the option of changing
these curves, and plant growth will be sensitive to
environmental conditions in the continuous simulation mode of operation. For input, the user will
enter the names of the crops in a rotation. Planting and harvest dates, fertility level, yield, and
cropping history would be entered if actual values
differ significantly from typical local values that
have been prerecorded in a database. The procedure will use prestored values to the fullest
extent possible, but the user may change default
values, including those listed above plus expected
crop height, maximum canopy cover percentage,
ratio of residue mass to grain yield, and ratio of
root biomass to grain yield.

##### 4.4.3.2 Tillage module

The tillage module,
which represents soil disturbing activities, will
estimate: (a) surface roughness, bulk density, and
aggregate size at the time of tillage as a function
of implement properties, soil texture and organic
matter, production level, and moisture content at
time of tillage; (b) changes in roughness, bulk
density, and aggregate size with time as a function
of soil texture and organic matter, covermanagement, rainfall, freezing/thawing (overwintering), and erosion; and (c) burial of residue
and its distribution with depth as a function of
soil and implement properties. The user will have
the option to choose input by: (a) name of tillage
operation, which will use prestored information
or (b) properties and time of the tillage operation,
which will include “base” roughness value for a
standard condition, degree of pulverization, depth
of tillage, residue covered for a standard condition, ridge and row spacings, and ridge height.

##### 4.4.3.3 Residue decomposition module

The residue decomposition module is to estimate: (a)
mass and percent cover of residue including forest
litter and logging slash, on the soil surface at any
time and (b) mass and distribution with depth of
residue, roots, and other biomaterials in the soil
at any time. The module is to distinguish among
major differences according to soil, climate, crop,
and tillage. In particular, the component is to be
driven by: (a) soil temperature, (b) soil texture,
(c) soil moisture, and (d) type of residue. User
inputs to drive this component will be taken from
those listed for the other land use modules.
4.4.3.4. Grazing module: This module is to estimate the removal of above ground biomass and
changes in soil properties such as density and
roughness that affect infiltration and erosion
caused by animal grazing and traffic. Other
secondary effects such as loss of below ground
biomass and surface ground cover would also be
computed.

##### 4.4.3.5 Supporting practices module

Most supporting practices affect erosion by either reducing -
runoff, redirecting it, or directly reducing the
hydraulic forces of the flow. The procedure is to
accommodate increased infiltration and the resulting runoff reduction. Runoff redirection is to be
considered by flow path definition in the topographic inputs. The procedure is to accommodate at least two flow path configurations that

may occur any number of times during a cropping rotation. These flow path configurations are
defined by the user as input. The effect of practices like strip cropping and grass strips are to be
analyzed by their effect on infiltration, runoff rate
and amount, and the reduction of runoff’s shear
stress acting on the soil caused by an increase in
hydraulic roughness.

## 5. OPERATIONAL REQUIREMENTS

The primary application of this erosion prediction technology will be by field personnel who
will be using the procedure as a tool to assist land
managers in making soil conservation decisions.
Major factors important to these users are: (a)
computational time, (b) ease of use, (c) applicability to the broad range of conditions typically
encountered in field programs, (d) robustness, (e)
validity, and (f) ease of explanation to client.

### 5.1 Computational Time

Developers of the procedure are to strive for
computational efficiency and to have the procedure operate as quickly as possible. The procedure is to compute the frequency distribution of
annual soil loss values for the profile version at
the rate of one management practice per minute
and one practice per two minutes for the
watershed version running a single overland flow
profile and a single concentrated flow channel.
The rate can be proportionally slower for more
complex systems. Also, not more than 30
minutes per farm of actual user time (computer
time can be longer) is to be required in the office
to prepare and assemble needed information
before going to the field. Once in the field, no
more time can be used to collect and assemble
input information than would be required for the
USLE when the profile version is used. The criteria to be used by the developers for judging the
acceptability of an internal simplification in the
procedure are: (a) does the planning or assessment decision change - if not, use the procedure
that requires the least resources -- and do computed values for the primary output variable
change more than 10 percent -- if they do, is the
change of consequence?

### 5.2 Ease of Use

The procedure shall be easy to use, especially
for the infrequent user, by accepting simple inputs
that are commonly available and understood by
personnel in the local field office. It should
require little structured training or support. Also,
it shall be flexible and accept inputs on increasing
detailed and complex levels if the user determines
that more detail is needed or that default values
need changing. The user shall not have to
directly manipulate any mathematical equations
to use the procedure; all mathematical manipulations shall be done by a computer program. The
procedure is to be designed so that a maximum
amount of computations can be made once-andfor-all and stored for repeated use. Likewise, the
procedure shall be constructed so that data files
specific to a given local area (a county or subcounty unit) can be prepared and stored locally so
that the field office user only has to search and
retrieve minimal data with each application. In
so far as possible, the procedure shall use data
and data files used in other SCS, BLM, and FS
applications, and it should be compatible with
Geographic Information Systems (GIS). The output should display in an easily understood form
the consequence of alternative management
options.

### 5.3 Applicable to Broad Range of Conditions

The procedure, within the limits defined by
this document, must apply to all sheet-rill erosion
problems that local field office personnel
encounter. In particular, it must apply to all conditions covered by the USLE plus additional
ones. Similarly, this requirement also applies to
all concentrated flow erosion situations, but the
procedure is not for hydraulic design of waterways.

### 5.4 Robustness

The procedure must tolerate out-of-range
input data and combinations of inputs that might
cause problems. The procedure should use
asymptotic, "well behaved” functions to avoid
extremely incorrect values and the procedure
unexpectedly “blowing up.” See the Appendix 7
for an example. It must tolerate applications for
which it is not intended. However, the procedure
must alert the user to these excesses, alert for loss
of accuracy when inputs are over simplified such
as for slope shape, and check for incorrect data
entries. The procedure should alert the user to
the possibility of obtaining “additional” information with more detailed inputs.

### 5.5 Validity

The procedure must be sufficiently accurate to
lead to the planning and assessment decision that
would be made in the large majority of cases
sis

when full information is available. However,
more than accuracy is to be considered in establishing the validity of the procedure. The procedure is to be validated, and the validation process and its results are to be documented. The
prediction procedure is expected to be composed
of a number of modules. Each major module is
to be individually validated, and the procedure is
to be validated as a package.

#### 5.5.1 Validation Criteria

Validation is to be
based on the procedure meeting all of the following criteria. (a) The model is valid if it serves its
intended purpose as defined by these specific User
Requirements. (b) The model is based on scientific principles and represents a reasonable expression of current scientific understanding of erosion
processes that can be used in an applied procedure. (c) The procedure gives expected
responses that appear reasonable. For example,
the output varies qualitatively with ground cover
(or any variable or combination of variables) in
the way that is commonly accepted by erosion
experts. (d) The model gives results that are
more useful for agency program objectives than
those given by the USLE and applies to situations
not appropriate for the USLE. These situations
include deposition in furrows, especially as influenced by plant residue in the furrows; nonuniform distribution of cover between ridges and furrows; the acceleration of rill erosion above a critical steepness; the variation in slope length, slope
steepness, ground cover, and contouring relationships with climate, soil, topography, and land use;
erosion by surface irrigation; deposition on concave slopes; and concentrated flow erosion. (e€)
The model provides a reasonable representation
of data covering the range of conditions of the
“key” situations described above. (f) Judgements
on the "goodness of fit” of the estimates from the
procedure to observed data are to be based on the
data sets as a whole and not on a few specific and
isolated data sets. Quantitative measures of the
"goodness of fits” will be calculated and
presented, but a specific quantitative level of
accuracy figure is not being required because of
the great variation in the experimental data that
will be used in validation. However, the results
are to be at least as good with respect to observed
data and known relationships as those from the
USLE. (g) The model is able to “stand up” in
public hearings of management plans and assessments.
ll

### 5.6 Ease of Explanation

The procedure is to be based on a set of principles and concepts that can be explained by
local field personnel to the client. The procedure
is to be developed so that the user can easily
demonstrate how the major factors of climate,
soil, topography, and land use affect erosion.

## 6. IMPLEMENTATION OF PROCEDURE

The procedure described above must be implemented in some mathematical form that allows it
to be used. The procedure will be conceptualized
and described with logic and mathematical
expressions to form a model, which will be coded
in a computer program. A record and description of these activities is to be delivered.

### 6.1 Model Considerations

#### 6.1.1 Structure

The model is to be based on the
fundamental erosion processes of: (a) interrill
erosion (principally detachment by raindrop
impact and lateral transport by thin flow), (b) rill
and concentrated flow erosion (detachment by
flow), (c) sediment transport by flow, (d) deposition by flow, (e) deposition in impoundments,
and concentrated flow hydraulics. The model is
expected to include major modules for: (a) climate generation; (b) snow accumulation, (c)
snowmelt, (d) infiltration, (e) runoff, (f) soil temperature, (g) erosion, (h) soil moisture, (i) crop
growth, (j) plant residue, and (k) tillage. Implicit
in all of these modules except for the climate
module is the central role of soil and soil properties. Although the model will include these
modules, it is NOT intended to be used specifically as a model for crop yield, water quality, soil
moisture, runoff, stochastic (random) climate
variables, wind erosion, or erosion and sediment
yield from classical gullies, stream channels, or
large complex watersheds.
6.1.2. Hydrologic Elements Hydrologically, the
model is to apply to conditions where overland
flow is significant, and runoff and erosion is not
dominated by “partial area” hydrology. The
model will consider lateral subsurface flow and
baseflow using a simple travel time approach that
takes position on the landscape into account. It
will consider vertical water movement in the root
zone and tile drainage only to the extent needed
to compute surface runoff sufficiently accurate for
erosion computations. The hydrologic elements
will be: (a) overland flow (broad sheet flow and
concentrated flow in “furrows”), (b) concentrated

flow in major natural and constructed waterways
(ephemeral gullies plowed over within the crop
rotation, terrace and diversion channels, grassed
waterways, and rangeland gullies comparable to
within-field concentrated flow channels), (c) small
impoundments (underground tile outlet terrace
impoundments, level terraces without outlets,
water and sediment control basins, within-field
natural impoundments, farm ponds, and other
similar within-field structures and features), (d)
simple return, lateral, and base flow, and (e) simple tile drainage.

#### 6.1.3 Family of Models

A family of models is
permissible to meet the entirety of the user
requirements. All of the models in the family are
intended for use by SCS, FS, and BLM; research
models are not permissible in the family. Commonality within the family and with other models
is to be maintained where reasonable. In so far
as possible, relationships used in simplified versions in the family are to be derived from those in
the most detailed version. This statement of commonality implies that when parameter values
representing the assumptions used to derive the
simplified relationships are input into the detailed
relationships, they will give the same result as the
simple ones for a given situation. Commonality
in so far as possible is to be maintained with
components of other models where particular
dominant algorithms are emerging. The model
and its coding are to be modular to make changing of model modules or submodules simple and
easy. Inputs are to be common across this family
of models and with other models in wide use in
SCS in so far as possible.

#### 6.1.4 Similarities with Other Models

Since this
model will have several features that are similar
to those in existing models produced by ARS and
others that are being used by SCS, SCS _ has
asked for a brief description of several of these
models. Similarities of this model with other
models having similar components are described
in Appendix 8. A model is like a tool in a tool
chest. While a pair of pliers can sometimes be
used as a wrench, a wrench of the proper design
and size (i.e., the proper tool for the job) is usually chosen to tighten bolts. Similarly, this prediction model is to be developed as the best tool
for predicting erosion by the field user. Other
models are better for other purposes, e.g., EPIC
for estimating the impact of erosion on productivity.

#### 6.1.5 Modification of Model Modules

The various modules or model subcomponents are to be
constructed so that they can be easily maintained
and replaced. Also, the modules are to be constructed in a way to facilitate their use in combination with other models.

### 6.2 Coding

The main requirements presently identified for
the coding is that the model and its code will follow a structured design. The model will be programmed using structured programming procedures. The program must be developed in the
programming languages of FORTRAN 77 or C,
and machine dependent routines are to be
avoided to enhance ttransportability. The
language and other requirements related to programming are to be developed during further discussions with SCS, FS,:- and BLM. Also,
mnemonic variable names will be used. Additional details are to be developed on computer
requirements such as data base management,
menu vs. command driven, and input and output
screens.

### 6.3 Documentation

The main delivered product from the project
will be a computer program and documentation
transferred on tape, floppy disk, or electronically
between computers. However, written documentation is to accompany the computer file. The
documentation will describe the governing logic
and mathematical relationships on which the
model is built, validation of the model, information needed to install the computer program,
description of inputs and how to obtain them,
and instructions on how to run the program and
use the model. Also, information needed to
maintain the model and its code will also be provided. “User” type documentation will be jointly
developed by ARS and the user agencies.

---

## 7. APPENDIX 1 - IMPORTANT SOIL PROPERTIES AFFECTING SOIL ERODIBILITY

### I. Measurements on Disturbed Soil Samples (Sieved and/or Ground)

#### A. Time Invariant Properties
1. Primary clay
2. Primary silt
3. Primary sand
4. Coarse fragments
5. Carbon (Organic matter)
6. Total nitrogen
7. Iron
8. Aluminum
9. Silicon
10. Extractable bases
11. Soluble cations
12. Cation exchange capacity
13. Carbonates
14. Acidity
15. Soil consistency (Atterberg limits)

#### B. Aggregate Properties
1. Dry aggregate size
2. Aggregate stability indices

#### C. Mineralogical and Morphological Properties
1. Description and relative abundance of clay minerals
2. Description and relative abundance of amorphous materials
3. Morphology of coarse silt and sand primary particles
4. Morphology of coarse silt and sand-sized aggregates
5. Qualitative description of aggregation mechanics (e.g., abundance of fine root hairs, fungal hyphae)

#### D. Properties of Remolded Cores
1. **Infiltration related properties**
   - a. Moisture-tension relationship
   - b. Saturated hydraulic conductivity
   - c. Unsaturated hydraulic conductivity
   - d. Hydrophobic and hydrophilic conditions
   - e. Rock fragments
2. **Erosion related properties**
   - a. Cohesion and friction angle
   - b. Detachment by impact of single drop
   - c. Shear strength by fall cone
   - d. Shear strength by direct shear
   - e. Penetration resistance

---

### II. In-situ Soil Measurements Associated with Field Erodibility Testing

#### A. Interrill erodibility tests
1. Depth of Ap horizon
2. Bulk density
3. Microrelief
4. **Shear strength**
   - a. Fall cone
   - b. Penetration resistance
5. **Crust characteristics**
   - a. Thickness
   - b. Rupture resistance
6. Antecedent soil moisture

#### B. Rill erodibility tests
1. Depth of Ap horizon
2. Bulk density
3. **Shear strength**
   - a. Fall cone
   - b. Torvane
   - c. Penetration resistance

---
## 8. APPENDIX 2 - SOILS LIST

### I. Cropland Soils (List 4.0)

#### A. Table 8.1: List of Cropland Soils

| Category | No. | Soil Series | Taxonomic Classification |
|---|---|---|---|
| Category I (Highest Priority) | 1 | **Abilene (TX)** | fine, mixed, thermic Pachic Argiustoll |
| Category I (Highest Priority) | 2 | **Anselmo (NE)** | coarse-loamy, mixed, mesic Typic Haplustoll |
| Category I (Highest Priority) | 3 | **Bonifay (FL)** | loamy, siliceous, thermic Grossarenic Plinthic Paleudult |
| Category I (Highest Priority) | 4 | **Cecil (NC)** | clayey, kaolinitic, thermic Typic Hapludult |
| Category I (Highest Priority) | 5 | **Forman (ND)** | fine-loamy, mixed Udic Argiboroll |
| Category I (Highest Priority) | 6 | **Frederick (VA)** | clayey, mixed, mesic Typic Paleudult |
| Category I (Highest Priority) | 7 | **Grenada (MS)** | fine-silty, mixed, themic Glossic Fragiudalf |
| Category I (Highest Priority) | 8 | **Heiden (TX)** | fine, montmorillonitic, thermic Udic Chromustert |
| Category I (Highest Priority) | 9 | **Keith (NE)** | fine-silty, mixed, mesic Aridic Argiustoll |
| Category I (Highest Priority) | 10 | **Miami (IN)** | fine-loamy, mixed, mesic Typic Hapludalf |
| Category I (Highest Priority) | 11 | **Pierre (SD)** | very-fine, montmorillonitic, mesic Ustertic Camborthid |
| Category I (Highest Priority) | 12 | **Monona (IA)** | fine-silty, mixed, mesic Typic Hapludoll |
| Category I (Highest Priority) | 13 | **Palouse (WA)** | fine-silty, mixed, mesic Pachic Ultic Haploxeroll |
| Category I (Highest Priority) | 14 | **Ramona (CA)** | fine-loamy, mixed, thermic Typic Haploxeralf |
| Category I (Highest Priority) | 15 | **Salinas (CA)** | fine-loamy, mixed, thermic Pachic Haploxeroll |
| Category I (Highest Priority) | 16 | **Sverdrup (MN)** | sandy, mixed Udic Haploboroll |
| Category I (Highest Priority) | 17 | **Tifton (GA)** | fine-loamy, siliceous, thermic Plinthic Paleudult |
| Category I (Highest Priority) | 18 | **Walla Walla (WA)** | coarse-silty, mixed, mesic Typic Haploxeroll |
| Category I (Highest Priority) | 19 | **Williams (ND)** | fine-loamy, mixed Typic Argiboroll ; |
| Category I (Highest Priority) | 20 | **Woodward (OK)** | coarse-silty, mixed, thermic Typic Ustochrept |
| Category II (Lower Priority) | 1 | **Balcom (CA)** | fine-loamy, mixed, thermic Calcixerollic Xerochrept |
| Category II (Lower Priority) | 2 | **Chester (MD)** | fine-loamy, mixed, mesic Typic Hapludult |
| Category II (Lower Priority) | 3 | **Clarion (ITA)** | fine-loamy, mixed, mesic Typic Hapludoll |
| Category II (Lower Priority) | 4 | **Davidson (GA)** | clayey, kaolinitic, thermic Rhodic Paleudult |
| Category II (Lower Priority) | 5 | **Dunkird (NY)** | fine-silty, mixed, mesic Glossoboric Hapludalf |
| Category II (Lower Priority) | 6 | **Mexico (MO)** | fine, montmorillonitic, mesic Udollic Ochraqualf |
| Category II (Lower Priority) | 7 | **Morley (IL)** | fine, illitic, mesic Typic Hapludalf |
| Category II (Lower Priority) | 8 | **Portneuf (ID)** | coarse-silty, mixed, mesic Durixerollic Calciorthid |
| Category II (Lower Priority) | 9 | **Sharpsburg (IA)** | fine, montmorillonitic, mesic Typic Argiudoll |
| Category II (Lower Priority) | 10 | **Zahl (ND)** | fine-loamy, mixed Entic Haploboroll |

#### B. Table 8.2: Arrangement of Cropland Soils by Order and Suborder

| Order | Suborder | Warm Soils (Moist) | Warm Soils (Wet) | Warm Soils (Dry) | Cool Soils (Moist) |
|---|---|---|---|---|---|
| **Alfisols** (6) | **Udalfs**<br>**Aqualfs**<br>**Xeralfs** | Dunkirk, Grenada, Miami, Morley | Mexico | Ramona | — |
| **Aridisols** (2) | **Orthids** | — | — | Pierre, Portneuf | — |
| **Inceptisols** (2) | **Ochrepts** | Balcom, Woodward | — | — | — |
| **Mollisols** (13) | **Udolls**<br>**Ustolls**<br>**Borolls**<br>**Xerolls** | Clarion, Monona, Sharpsburg | — | Abilene, Anselmo, Keith<br><br>Palouse, Salinas, Walla Walla | Forman, Severdrup, Williams, Zahl |
| **Ultisols** (6) | **Udults** | Bonifay, Cecil, Chester, Davidson, Frederick, Tifton | — | — | — |
| **Vertisols** (1) | **Usterts** | — | — | Heiden | — |

#### C. Table 8.3: Arrangement of Cropland Soils by Texture

| Surface Texture Class | Soil Series in Texture Class |
|---|---|
| **Sand** | Bonifay |
| **Loamy sand** | Sverdrup, Tifton |
| **Sandy loam** | Anselmo, Cecil, Ramona |
| **Silt loam** | Chester, Grenada, Keith, Mexico, Miami, Monona, Palouse, Portneuf, Walla Walla, Sharpsburg |
| **Loam** | Balcom, Clarion, Williams, Woodward, Zahl |
| **Sandy clay loam** | Davidson, Dunkirk |
| **Silty clay loam** | Frederick |
| **Clay loam** | Abilene, Forman, Morley, Salinas |
| **Clay** | Heiden, Pierre |

---

### II. Rangeland Erodibility Soils (List 2.0)

These rangeland soils have been identified as target soils having a range of soil properties that make them important for determining erodibility values. Unlike cropland, erosion on rangeland must be studied as a combination of soil and site conditions. This list was developed primarily from soil erodibility considerations. The following list in Section III gives soil/site combinations that have been chosen for WEPP experiments where both soil and site conditions were considered along with operational factors such as availability of water.

#### A. Table 8.4: List of Rangeland Erodibility Soils

| No. | Soil Series | Taxonomic Classification |
|---|---|---|
| 1 | **Amarillo (TX)** | fine-loamy, mixed, thermic Aridic Paleustalf |
| 2 | **Bainville (MT)** | fine-silty, mixed (calcareous), mesic Ustic Torriorthent |
| 3 | **Blazon (WY)** | loamy, mixed (calcareous), frigid, shallow Ustic Torriorthent |
| 4 | **Cave (AZ)** | loamy, mixed, thermic, shallow Typic Paleorthid |
| 5 | **Colby (KS)** | fine-silty, mixed (calcareous), mesic Ustic Torriorthent |
| 6 | **Lucien (OK)** | loamy, mixed, thermic, shallow Typic Haplustoll |
| 7 | **Lucky Star (UT)** | loamy-skeletal, mixed Cryic Paleboroll |
| 8 | **Mohave (AZ)** | fine-loamy, mixed, thermic Typic Haplargid |
| 9 | **‘Morton (ND)** | fine-silty, mixed Typic Argiboroll |
| 10 | **Nannyton (ID)** | fine-loamy, mixed, mesic Typic Haplargid |
| 11 | **Parleys (UT)** | fine-silty, mixed, mesic Calcic Argixeroll |
| 12 | **Pierre (SD)** | very-fine, montmorillonitic, mesic Ustertic Camborthid |
| 13 | **Pratt (KS)** | sandy, mixed, thermic Psammentic Haplustalf |
| 14 | **Summit (OK)** | fine, montmorillonitic, thermic Vertic Argiudoll |
| 15 | **Tarrant (TX)** | clayey-skeletal, montmorillonitic, thermic Lithic Calciustoll |
| 16 | **Tillman (TX)** | fine, mixed, thermic Typic Paleustoll |
| 17 | **Vebar (ND)** | coarse-loamy, mixed Typic Haploboroll |
| 18 | **Venable (CO)** | fine-loamy, mixed Cumulic Cryaquoll |
| 19 | **Vernon (TX)** | fine, mixed, thermic Typic Ustochrept |
| 20 | **Vista (CA)** | coarse-loamy, mixed, thermic Typic Xerochrept |

#### B. Table 8.5: Arrangement of Rangeland Erodibility Soils by Order and Suborder

| Order | Suborder | Warm Soils (Moist) | Warm Soils (Wet) | Warm Soils (Dry) | Cool Soils (Moist) |
|---|---|---|---|---|---|
| **Alfisols** (2) | **Aqualfs**<br>**Ustalfs** | — | — | Amarillo, Pratt | — |
| **Aridisols** (4) | **Argids**<br>**Orthids** | — | — | Mohave, Nannyton<br>Cave, Pierre | — |
| **Entisols** (3) | **Orthents** | — | — | — | Bainville, Blazon, Colby |
| **Inceptisols** (2) | **Ochrepts** | — | — | Vernon, Vista | — |
| **Mollisols** (9) | **Udolls**<br>**Aquolls**<br>**Ustolls**<br>**Borolls**<br>**Xerolls** | Summit | Venable | Lucien, Tarrant, Tillman<br><br>Parleys | Lucky Star, Morton, Vebar |

#### C. Table 8.6: Arrangement of Rangeland Erodibility Soils by Texture

| Surface Texture Class | Soil Series in Texture Class |
|---|---|
| **Loamy sand** | Pratt |
| **Sandy loam** | Amarillo, Mohave, Vebar, Venable, Vista |
| **Loam** | Cave (gravelly), Lucien, Nannyton (gravelly) |
| **Silt loam** | Bainville, Colby, Lucky Star (gravelly), Morton, Parleys |
| **Clay loam** | Blazon |
| **Silty clay loam** | Summit |
| **Clay** | Pierre, Tarrant, Vernon, Tillman |

---

### III. Rangeland Soil/Site Combinations for Evaluation of Erodibility and Cover/Management Effects

#### A. Table 8.7: List of Rangeland Soil/Site Combination Soils

| No. | Soil/Site Combination | Taxonomic Classification |
|---|---|---|
| | **Bernardo (AZ)** | fine, mixed, thermic Ustollic Haplargid |
| | **Deaver (CO)** | fine, montmorillonitic (calcareous), mesic Typic Torriortent |
| | **Declo (ID)** | coarse-loamy, mixed, mesic Xerollic Calciorthid |
| | **Glasgow (ID)** | fine, montmorillonitic, mesic Xerollic Haplargid |
| | **Grant (OK)** | fine-silty, mixed, thermic Udic Argiustoll |
| | **Hackroy (NM)** | clayey, mixed, mesic Lithic Aridic Haplustalf |
| | **NSD-Area 11 (NV)*** | loamy-skeletal, mixed, thermic Typic Durorthid |
| | **NSD-Mercury (NV)*** | loamy-skeletal, mixed, thermic Typic Haplargid |
| | **Petescreek (CA)** | fine-loamy, mixed, frigid Pachic Agixeroll |
| | **Pierre (SD)** | very-fine, montmorillonitic, mesic Ustertic Cambothid |
| | **Pratt (OK)** | sandy, mixed, thermic Psammentic Haplustalf |
| | **Purves (TX)** | clayey, montmorillonitic, thermic Lithic Calciustoll |
| | **Querencia (NM)** | fine-loamy, mixed, thermic Ustollic Camborthid |
| | **Searla (ID)** | loamy-skeletal, mixed, frigid Calcic Agrixeroll |
| | **Shano (WA)** | coarse-silty, mixed, mesic Xerollic Camborthid |
| | **Vida (MT)** | fine-loamy, mixed Typic Argiboroll |
| | **Woodward (OK)** | coarse-silty, mixed, thermic Typic Ustochrept |

**NSD - No Series Designated. Series names have not been assigned to these sites. The name*

#### B. Table 8.8: Arrangement of Rangeland Soil/Site Combinations by Soil Order and Suborder

| Order | Suborder | Warm Soils (Dry) | Cool Soils (Moist) |
|---|---|---|---|
| **Alfisols** (2) | **Ustalfs** | Hackroy, Pratt | — |
| **Aridisols** (10) | **Argids**<br>**Orthids** | Bernardo, Glasgow, Nannyton, NSD-Mercury<br>Declo, NSD-Area 11, NSD-Walnut Gulch, Pierre, Querencia, Shano | — |
| **Entisols** (1) | **Orthents** | Deaver | — |
| **Inceptisols** (1) | **Ochrepts** | Woodward | — |
| **Mollisols** (5) | **Ustolls**<br>**Xerolls** | Grant, Purves<br>Petescreek, Searla | Vida |

#### C. Table 8.9: Arrangement of Range Soil/Site Combination Soils by Surface Texture

| Surface Texture Class | Soil/Site Combination Series |
|---|---|
| **Loamy Sand** | Pratt |
| **Sandy Loam** | Declo, Hackroy, NSD-Area 11 (gravelly), NSD-Walnut Gulch (gravelly), Querencia |
| **Loam** | Glasgow, Nannyton, NSD-Mercury (gravelly), Petescreek (gravelly), Searla (gravelly) |
| **Silt Loam** | Grant, Shano, Woodward |
| **Clay Loam** | Bernardo, Vida |
| **Silty Clay** | Deaver |
| **Clay** | Pierre, Purves (gravelly) |

---

### IV. Forest Land Mountainous Rangeland Soils

#### A. Table 8.10: List of Forest Land Mountainous Rangeland Soils

| Category | No. | Soil Series | Taxonomic Classification |
|---|---|---|---|
| Category I (Highest Priority) | 1 | **Aiken (CA)** | clayey, oxidic, mesic Xeric Haplohumult |
| Category I (Highest Priority) | 2 | **Auburn (CA)** | loamy, mixed, thermic Ruptic-lithic Xerochrept |
| Category I (Highest Priority) | 3 | **Awhahnee (CA)** | coarse-loamy, mixed thermic Mollic Haploxeralf |
| Category I (Highest Priority) | 4 | **Berks (PA)** | loamy-skeletal, mixed, mesic, Typic Dystrochrept |
| Category I (Highest Priority) | 5 | **Boise (ID)** | sandy-skeletal, mixed Typic Cryoboroll |
| Category I (Highest Priority) | 6 | **Clarksville (AR)** | loamy-skeletal, siliceous, mesic, Typic Paleudult |
| Category I (Highest Priority) | 7 | **Edneyville (NC)** | fine-loamy, mixed, mesic, Typic Hapludult |
| Category I (Highest Priority) | 8 | **Gogebic (MI)** | coarse-loamy, mixed, frigid, Alfic Fragiorthod |
| Category I (Highest Priority) | 9 | **Granile (CO)** | loamy-skeletal, mixed, Typic Cryoboralf |
| Category I (Highest Priority) | 10 | **Legault (CO)** | sandy-skeletal, mixed, Typic Cryochrept |
| Category I (Highest Priority) | 11 | **Nathrop (CO)** | loamy-skeletal, mixed, Argic Cryoboroll |
| Category I (Highest Priority) | 12 | **Neuns (CA)** | loamy-skeletal, mixed, mesic Dystric Xerochrept |
| Category I (Highest Priority) | 13 | **Owen Creek (WY)** | fine, montmorillonitic, Argic Cryoboroll |
| Category I (Highest Priority) | 14 | **Smithdale (MS)** | fine-loamy, siliceous, thermic, Typci Hapludult |
| Category I (Highest Priority) | 15 | **Tolo (OR)** | medial/loamy, mixed, frigid, Typic Vitrandept |
| Category I (Highest Priority) | 16 | **Tunbridge (VT)** | coarse-loamy, mixed, frigid, Typic Haplorthod |
| Category I (Highest Priority) | 17 | **Waca (CA)** | medial-skeletal, frigid Andic Xerumbrept- |
| Category I (Highest Priority) | 18 | **Wellston (OH)** | fine-silty, mixed, mesic, Ultic Hapludalf |
| Category II (Lower Priority) | 1 | **Adel (MT)** | fine-loamy, mixed Pachic Cryoboroll |
| Category II (Lower Priority) | 2 | **Cecil (NC)** | clayey, kaolinitic, thermic Typic Hapludult |
| Category II (Lower Priority) | 3 | **Cieneba (CA)** | loamy, mixed, nonacid, thermic, shallow Typic Xerorthent |
| Category II (Lower Priority) | 4 | **Cookport (WV)** | fine-loamy, mixed, mesic, Aquic Fragiudult |
| Category II (Lower Priority) | 5 | **Grenada (MS)** | fine-silty, mixed, thermic, Glossic Fragidualf |
| Category II (Lower Priority) | 6 | **Karta (AK)** | loamy-skeletal, mixed Humic Cryorthod |
| Category II (Lower Priority) | 7 | **Marlow (NH)** | coarse-loamy, mixed, frigid, Typic Haplorthod |
| Category II (Lower Priority) | 8 | **Rubicon (MI)** | sandy, mixed, frigid, Entic Haplorthod |
| Category II (Lower Priority) | 9 | **Sitka (AK)** | medial, Humic Cryorthod |
| Category II (Lower Priority) | 10 | **Wiekert (PA)** | loamy-skeletal, mixed, mesic, Lithic Dystrochrept |

#### B. Table 8.11: Arrangement of Forest Land Mountainous Rangeland Soils by Order and Suborder

| Order | Suborder | Warm Soils (Moist) | Cool Soils (Moist) |
|---|---|---|---|
| **Alfisols** (4) | **Udalfs**<br>**Xeralfs**<br>**Boralfs** | Grenada, Wellston<br>Awhahnee | Granile |
| **Entisols** (1) | **Orthents** | Cieneba | — |
| **Inceptisols** (7) | **Andepts**<br>**Ochrepts**<br>**Umbrepts** | Tolo<br>Auburn, Becks, Legault, Neuns, Weikert<br>Waca | — |
| **Mollisols** (4) | **Borolls** | — | Adel, Boise, Mathrop, Owen Creek |
| **Spodosols** (6) | **Orthods** | — | Gogebic, Karta, Marlow, Rubicon, Sitka, Tunbridge |
| **Ultisols** (6) | **Humults**<br>**Udults** | Aiken<br>Cecil, Clarksville, Cookport, Edneyville, Smithdale | — |

#### C. Table 8.12: Arrangement of Forest Land Mountainous Rangeland Soils by Surface Texture

| Surface Texture Class | Soil Series in Texture Class |
|---|---|
| **Loamy Sand** | Boise, Legault (stony), Rubicon |
| **Sandy Loam** | Awhahnee, Cecil, Cieneba, Edneyville, Gogebic, Granile (gravelly), Marlow, Smithdale, Tunbridge, Waca (cobbly) |
| **Loam** | Adel, Aiken, Auburn, Cookport, Karta (gravelly), Nathrop (stony), Neuns (gravelly) |
| **Silt Loam** | Berks, Clarksville (very cherty), Grenada, Owen Creek, Tolo, Weikert (channery), Wellston |
| **Medial\*** | Sitka |

*\*Medial: Volcanic derived; loamy by field texture; <60% ash, cinders, and pumice.*

---

---

## 9. APPENDIX 3 - TOPOGRAPHIC REPRESENTATIONS

The WEPP erosion prediction procedure is to represent topography in three different versions:
1. **Profile Version (Hillslope):** Computes interrill and rill erosion and deposition along a continuous slope profile from the top of a hill down to where concentrated flow begins or where deposition ends.
2. **Watershed Version:** Uses representative slope profiles for hillslope elements and routes runoff and sediment through concentrated flow channels (ephemeral gullies, grassed waterways, terraces) and impoundments (terraces, farm ponds, check dams) within a small watershed boundary.
3. **Grid Version:** Superimposes an orthogonal grid over a field or small watershed to spatially compute soil detachment, transport, and deposition across grid cells and channel elements.

A measure of slope convexity/concavity and slope length \(A_{aa}\) must be accounted for when defining topographic elements in these representations.

---

### Figure 9.1: Landscape Profile Representation Options
![Landscape Profile Representation Options](./wepp-figures/wepp-fig-9.1-profile-options.png)

> **WCAG AAA Description:**  
> **Figure Type:** Topographic slope profile schematic.  
> **Visual Layout & Orientation:** A set of cross-sectional slope profile curves illustrating different geometric representations of hillslope topography from upper crest to lower drainage.  
> **Labels & Annotations:** Curves are labeled showing uniform, convex, concave, and S-shaped (complex) slope configurations. Annotations indicate slope length \(X\) and elevation \(Z\), marking transition points where slope steepness changes.  
> **Functional Meaning:** Illustrates the Profile Version options in WEPP for representing hillslope geometry, allowing users to model uniform slopes or divide complex hillslope profiles into discrete slope elements with varying steepness and curvature.

---

### Figure 9.2: Simple First Order Watershed
![Simple First Order Watershed](./wepp-figures/wepp-fig-9.2-first-order-watershed.png)

> **WCAG AAA Description:**  
> **Figure Type:** Topographic watershed schematic diagram.  
> **Visual Layout & Orientation:** A planar aerial view of a pear-shaped boundary defining a simple first-order watershed draining toward the bottom of the diagram.  
> **Labels & Annotations:** The interior area is divided into two contributing hillslope flanks labeled "Overland Flow Element 1" (left flank) and "Overland Flow Element 2" (right flank). Arrows from both overland flow elements converge into a central vertical line running down the axis labeled "Concentrated Flow Channel". The channel terminates at the bottom vertex labeled "Watershed Outlet" where "Sediment Yield" discharges.  
> **Functional Meaning:** Illustrates the basic building block of the Watershed Version of WEPP: two overland flow hillslope planes draining laterally into a single central concentrated flow channel that delivers sediment and runoff to the watershed outlet.

---

### Figure 9.3a: Example Second Order Watersheds
![Example Second Order Watersheds](./wepp-figures/wepp-fig-9.3a-second-order-watershed.png)

> **WCAG AAA Description:**  
> **Figure Type:** Complex watershed routing schematic diagram.  
> **Visual Layout & Orientation:** An aerial schematic of a larger, second-order watershed composed of multiple first-order sub-watersheds and channel networks draining from top to bottom.  
> **Labels & Annotations:** Upper sub-areas represent first-order watersheds where overland flow elements feed into primary channels. These primary channels converge into a higher-order central channel labeled "Concentrated Flow Channel (2nd Order)". Along the upper reaches of the channels, shaded boxes represent "Impoundment Elements" (such as terraces or sediment retention basins). Flow routing arrows indicate the direction of runoff and sediment movement through the channels and impoundments toward the final "Watershed Outlet".  
> **Functional Meaning:** Demonstrates how WEPP scales from simple first-order watersheds to second-order watersheds by linking multiple hillslope elements, tributary channels, and impoundment structures in a distributed routing network to compute total sediment delivery.

---

### Figure 9.3b: Conventional Gradient Terrace System
![Conventional Gradient Terrace System](./wepp-figures/wepp-fig-9.3b-gradient-terrace-system.png)

> **WCAG AAA Description:**  
> **Figure Type:** Agricultural conservation practice schematic diagram.  
> **Visual Layout & Orientation:** A top-down perspective of a field slope fitted with a conventional gradient terrace system. Horizontal terrace ridges run across the slope perpendicular to the primary grade.  
> **Labels & Annotations:** Arrows labeled "Overland Flow" show sheet and rill runoff moving downslope across the field interval until intercepted by horizontal terrace channels labeled "Terrace Channel (Concentrated Flow)". Once intercepted, the flow turns 90 degrees and moves laterally along the gentle grade of the terrace channel toward a vertical drainage structure labeled "Grass Waterway or Underground Outlet".  
> **Functional Meaning:** Illustrates how WEPP models conventional gradient terraces: overland flow is intercepted at regular slope intervals, reducing slope length and interrill/rill erosion, and runoff is safely routed via concentrated channel flow to protected waterway outlets.

---

### Figure 9.3c: Ridge-Furrow Microtopography: Flow Stays Within Furrows
![Ridge-Furrow Microtopography](./wepp-figures/wepp-fig-9.3c-ridge-furrow-microtopography.png)

> **WCAG AAA Description:**  
> **Figure Type:** Tillage microtopography routing diagram.  
> **Visual Layout & Orientation:** A detailed view of field surface microtopography shaped by ridge-tillage or bedding, showing parallel ridges and furrows oriented slightly off-contour or down-slope.  
> **Labels & Annotations:** The raised earthen rows are labeled "Tillage Ridges", and the depressions between them are labeled "Furrows". Flow arrows labeled "Overland Flow (Within Furrows)" originate on the ridge sides (interrill detachment) and flow immediately into the adjacent furrows. Because the ridges are intact, the flow is confined entirely within the furrows, running parallel to the tillage direction toward a field boundary or drainage channel.  
> **Functional Meaning:** Shows how WEPP represents ridge-furrow microtopography when ridge height is sufficient to prevent cross-flow: interrill sediment moves laterally off ridge tops into furrows, where rill flow transports the sediment along the furrow grade without breaching the ridges.

---

### Figure 9.4: Example Fourth Order Watershed
![Example Fourth Order Watershed](./wepp-figures/wepp-fig-9.4-fourth-order-watershed.png)

> **WCAG AAA Description:**  
> **Figure Type:** Basin-scale channel network diagram.  
> **Visual Layout & Orientation:** A comprehensive schematic map of a large fourth-order watershed boundary containing a highly branched dendritic drainage network.  
> **Labels & Annotations:** Small peripheral tributaries are labeled "1st Order Channels", which merge to form "2nd Order Channels", which in turn merge into "3rd Order Channels", finally converging into a single large stem labeled "4th Order Main Channel". Various overland flow elements and sub-watershed boundaries are demarcated across the basin. The main stem flows out at the bottom vertex labeled "Watershed Outlet".  
> **Functional Meaning:** Depicts the upper spatial scale limit for the WEPP watershed version, illustrating how complex multi-branching stream networks up to fourth order are structured and routed to predict sediment yield at the basin scale.

---

### Figure 9.5a: Special Topographic and Land Use Cases: Furrow and Channel Flow
![Special Topographic and Land Use Cases: Furrow and Channel Flow](./wepp-figures/wepp-fig-9.5a-special-cases-furrow-channel.png)

> **WCAG AAA Description:**  
> **Figure Type:** Specialized channel routing diagram.  
> **Visual Layout & Orientation:** Schematic showing concentrated flow entering a channel or furrow with varying bed roughness and cross-sectional geometry.  
> **Labels & Annotations:** Labels highlight channel bed slope, wetted perimeter, and flow depth, with arrows showing sediment transport and localized scour or deposition along the furrow length.  
> **Functional Meaning:** Demonstrates WEPP's capability to model concentrated flow hydraulics and sediment routing in specialized agricultural furrows and field ditches where flow transitions from shallow overland flow to channelized flow.

---

### Figure 9.5b: Special Topographic and Land Use Cases: Overland Flow Specifications
![Special Topographic and Land Use Cases: Overland Flow Specifications](./wepp-figures/wepp-fig-9.5b-special-cases-overland-specifications.png)

> **WCAG AAA Description:**  
> **Figure Type:** Spatial overland flow boundary diagram.  
> **Visual Layout & Orientation:** An aerial schematic showing multiple distinct overland flow elements with differing soils and vegetative covers draining into a shared boundary.  
> **Labels & Annotations:** Labels demarcate "Element A (Upper Slope)" and "Element B (Lower Slope)", showing surface runoff and sediment routing vectors crossing from one management zone into another.  
> **Functional Meaning:** Illustrates WEPP's requirement to accommodate non-uniform hillslope profiles where overland flow crosses abrupt boundaries between different soil types, vegetative covers, or management practices.

---

### Figure 9.5c: Special Topographic and Land Use Cases: Split Channel Routing
![Special Topographic and Land Use Cases: Split Channel Routing](./wepp-figures/wepp-fig-9.5c-special-cases-split-channel.png)

> **WCAG AAA Description:**  
> **Figure Type:** Channel divergence schematic diagram.  
> **Visual Layout & Orientation:** A top-down schematic showing a concentrated flow channel dividing or splitting into multiple downstream distributary branches.  
> **Labels & Annotations:** Arrows show main stem flow bifurcating at a junction into "Branch 1" and "Branch 2", with annotations indicating the partition of water discharge and sediment load between the split channels.  
> **Functional Meaning:** Highlights special operational routing cases in WEPP where concentrated flow networks diverge (such as in braided streams, alluvial fans, or engineered diversion channels), requiring mass balance partitioning of sediment and water.

---

### Figure 9.5d: Special Topographic and Land Use Cases: Bench Terrace Systems
![Special Topographic and Land Use Cases: Bench Terrace Systems](./wepp-figures/wepp-fig-9.5d-special-cases-bench-terrace.png)

> **WCAG AAA Description:**  
> **Figure Type:** Stepped bench terrace profile diagram.  
> **Visual Layout & Orientation:** A cross-sectional profile of a steep hillside modified into a series of stepped, horizontal bench terraces separated by steep risers.  
> **Labels & Annotations:** Flat cultivated steps are labeled "Bench Terrace Shelf (Infiltration/Deposition Zone)", while the vertical or steep connecting slopes are labeled "Terrace Riser (Protected Grass/Stone Face)". Arrows show surface drainage controlled across each shelf.  
> **Functional Meaning:** Illustrates WEPP's representation of bench terracing on steep agricultural lands, where slope length and steepness are dramatically altered to eliminate overland acceleration and trap sediment on horizontal benches.

---

### Figure 9.5e: Special Topographic and Land Use Cases: Forest Roads and Ditches
![Special Topographic and Land Use Cases: Forest Roads and Ditches](./wepp-figures/wepp-fig-9.5e-special-cases-forest-road.png)

> **WCAG AAA Description:**  
> **Figure Type:** Forest infrastructure drainage diagram.  
> **Visual Layout & Orientation:** A cross-section and layout of a mountain forest road cut into a hillside, showing the road prism, inside drainage ditch, and culvert cross-drain.  
> **Labels & Annotations:** Labels point to "Cut Bank", "Road Running Surface", "Inside Ditch", and "Culvert Crossing". Arrows trace surface runoff from the road surface and cut bank concentrating in the ditch and discharging through the culvert onto the natural slope below.  
> **Functional Meaning:** Demonstrates WEPP's application to disturbed forest lands, specifically modeling high-erosion road networks where compacted road surfaces and ditches intercept subsurface flow and generate concentrated surface runoff.

---

### Figure 9.6: Grid Element Version and Its Representation of a Field
![Grid Element Version and Its Representation of a Field](./wepp-figures/wepp-fig-9.6-grid-element-version.png)

> **WCAG AAA Description:**  
> **Figure Type:** Spatial grid discretization and flow vector diagram.  
> **Visual Layout & Orientation:** A rectangular field plot overlaid with an \(N 	imes M\) Cartesian grid of square cells. Within each cell, arrows indicate the local direction of surface runoff.  
> **Labels & Annotations:** Individual cells are labeled "Grid Cell \((i, j)\): Overland Flow Element". Flow vector arrows within each cell point across cell boundaries toward adjacent lower-elevation cells based on local topographic slope. In cells where flow vectors converge from multiple directions, a darkened line represents the initiation of a "Concentrated Flow Channel". An enclosed group of cells near the bottom is labeled "Impoundment / Ponded Area". Total flow exits at the bottom right cell labeled "Field Outlet".  
> **Functional Meaning:** Illustrates the Grid Version's mathematical architecture: the field is resolved into discrete grid cells where water and sediment balance equations are solved locally, routing outputs from cell \((i, j)\) as inputs to adjacent downslope cells, dynamically generating concentrated flow networks and impoundment effects.

---
## 10. APPENDIX 4 - EXAMPLE STRIP CROPPING SYSTEMS THAT WEPP IS TO CONSIDER

Contour strip cropping systems can involve up to 10 strips in a field. A strip cropping system could involve the following:

### 1. 5-Year Strip Cropping Rotation Schedule
#### Figure 10.1a: 5-Year Strip Cropping Rotation Schedule
![5-Year Strip Cropping Rotation Schedule](./wepp-figures/wepp-fig-10.1a-stripcropping-5yr-rotation.png)

> **WCAG AAA Description:**  
> **Figure Type:** Rotational strip cropping schedule diagram.  
> **Visual Layout & Orientation:** A tabular and schematic layout of a sloping field divided into alternating contour strips undergoing a 5-year crop rotation cycle.  
> **Labels & Annotations:** Rows represent rotational years (Year 1 to Year 5), and columns represent adjacent field strips. Labels indicate alternating bands of cultivated grain crops and dense forage/meadow strips, showing how protective sod strips are shifted annually across the slope.  
> **Functional Meaning:** Demonstrates WEPP's capability to simulate 5-year contour strip cropping systems, where annual rotation of clean-tilled and meadow strips controls slope length and traps sediment downslope.

---

### 2. 6-Year Strip Cropping Rotation Schedule
#### Figure 10.1b: 6-Year Strip Cropping Rotation Schedule
![6-Year Strip Cropping Rotation Schedule](./wepp-figures/wepp-fig-10.1b-stripcropping-6yr-rotation.png)

> **WCAG AAA Description:**  
> **Figure Type:** Rotational strip cropping schedule diagram.  
> **Visual Layout & Orientation:** A schematic field layout illustrating a 6-year rotational strip cropping arrangement across a hillside.  
> **Labels & Annotations:** Annotations mark sequential strip assignments over a 6-year period, highlighting alternating strips of corn, small grain, and multi-year alfalfa/meadow sod. Flow arrows show runoff from grain strips being intercepted by downslope meadow strips.  
> **Functional Meaning:** Illustrates the representation of extended 6-year contour strip cropping in WEPP, evaluating long-term sediment trapping efficiency across multi-year forage cycles.

---

### 3. 6-Year Crop Rotation of Corn, Grain, and Meadow (C-G-M-M-M-M)
#### Figure 10.1c: 6-Year Crop Rotation of Corn, Grain, and Meadow (C-G-M-M-M-M)
![6-Year Crop Rotation of Corn, Grain, and Meadow (C-G-M-M-M-M)](./wepp-figures/wepp-fig-10.1c-stripcropping-6yr-cgmmmm.png)

> **WCAG AAA Description:**  
> **Figure Type:** Field strip rotation schedule and layout diagram.  
> **Visual Layout & Orientation:** A tabular and spatial schematic showing a sloping field divided into three equal-sized field units, each subdivided into alternating contour strips running horizontally across the slope.  
> **Labels & Annotations:** Columns represent field units, while rows indicate the 6-year progression. Within each unit, alternating strips are labeled with crop codes: "C" for Corn, "G" for Small Grain, and "M1", "M2", "M3", "M4" for 1st through 4th year Meadow. In any given year, high-erosion Corn strips are physically separated and buffered by dense Meadow strips downslope.  
> **Functional Meaning:** Illustrates how WEPP models contour strip cropping: evaluating the spatial interception of sediment from tilled corn strips by adjacent grass/meadow buffer strips across a 6-year rotational cycle in a 3-field layout.

#### Table 10.1: 6-Year Crop Rotation Schedule (C-G-M-M-M-M) in a 3-Field Arrangement

| Year | Field 1 Strips (Top to Bottom) | Field 2 Strips (Top to Bottom) | Field 3 Strips (Top to Bottom) |
|---|---|---|---|
| **Year 1** | Corn / Meadow-2 / Corn / Meadow-2 | Grain / Meadow-3 / Grain / Meadow-3 | Meadow-1 / Meadow-4 / Meadow-1 / Meadow-4 |
| **Year 2** | Grain / Meadow-3 / Grain / Meadow-3 | Meadow-1 / Meadow-4 / Meadow-1 / Meadow-4 | Corn / Meadow-2 / Corn / Meadow-2 |
| **Year 3** | Meadow-1 / Meadow-4 / Meadow-1 / Meadow-4 | Corn / Meadow-2 / Corn / Meadow-2 | Grain / Meadow-3 / Grain / Meadow-3 |
| **Year 4** | Meadow-2 / Corn / Meadow-2 / Corn | Meadow-3 / Grain / Meadow-3 / Grain | Meadow-4 / Meadow-1 / Meadow-4 / Meadow-1 |
| **Year 5** | Meadow-3 / Grain / Meadow-3 / Grain | Meadow-4 / Meadow-1 / Meadow-4 / Meadow-1 | Meadow-2 / Corn / Meadow-2 / Corn |
| **Year 6** | Meadow-4 / Meadow-1 / Meadow-4 / Meadow-1 | Meadow-2 / Corn / Meadow-2 / Corn | Meadow-3 / Grain / Meadow-3 / Grain |

---

### 4. 7-Year Strip Cropping Rotation with Corn and Soybeans (C-Sb)
#### Figure 10.1d: 7-Year Strip Cropping Rotation with Corn and Soybeans (C-Sb)
![7-Year Strip Cropping Rotation with Corn and Soybeans (C-Sb)](./wepp-figures/wepp-fig-10.1d-stripcropping-7yr-csb.png)

> **WCAG AAA Description:**  
> **Figure Type:** Tillage and crop strip alternation schematic.  
> **Visual Layout & Orientation:** A cross-sectional and aerial view of a slope planted to a 7-year rotation involving Corn, Soybeans, and protective forage strips.  
> **Labels & Annotations:** Alternating horizontal bands across the slope are labeled showing high-residue corn strips, low-residue soybean strips, and permanent or rotational grass buffer bands. Downslope flow arrows show runoff from soybean strips entering heavy residue or grass strips below.  
> **Functional Meaning:** Illustrates WEPP's requirement to model complex 7-year strip cropping rotations involving low-residue crops like soybeans buffered by high-residue or perennial forage strips.

---
## 11. APPENDIX 5 - HYDROLOGIC SITUATIONS NOT WELL TREATED BY WEPP

Certain complex hydrologic and subsurface flow phenomena are not primary drivers of surface erosion in typical field situations and are not well treated by the initial release of the WEPP model:

### 1. Partial Area Hydrology
#### Figure 11.1: Partial Area Hydrology
![Partial Area Hydrology](./wepp-figures/wepp-fig-app5-partial-area-hydrology.png)

> **WCAG AAA Description:**  
> **Figure Type:** Hillslope hydrological process diagram.  
> **Visual Layout & Orientation:** A cross-sectional profile of a hillside leading down to a stream channel at the valley bottom, illustrating spatial variations in surface runoff generation during a rainfall event.  
> **Labels & Annotations:** The upper and middle hillslope segments are labeled "High Intake Zone / Unsaturated Soil", showing raindrops infiltrating completely into the soil profile with "No Surface Runoff Generated". At the base of the slope adjacent to the stream, the water table rises to the surface, creating an area labeled "Saturated Contributing Area (Partial Area Runoff)". Here, rain falling on already saturated soil generates immediate "Saturation Overland Flow" that drains into the stream channel.  
> **Functional Meaning:** Illustrates partial area (variable source area) hydrology, where runoff originates only from dynamic saturated zones near channels rather than uniformly across the entire hillslope—a process not fully simulated by WEPP's standard infiltration-excess (Horton) overland flow routing.

---

### 2. Interflow and Restricting Layers
#### Figure 11.2: Interflow and Restricting Layers
![Interflow and Restricting Layers](./wepp-figures/wepp-fig-app5-interflow-restricting-layer.png)

> **WCAG AAA Description:**  
> **Figure Type:** Subsurface hydrological profile diagram.  
> **Visual Layout & Orientation:** A soil profile cross-section showing an upper permeable soil horizon overlying an impermeable subsoil restricting layer on a sloping hillside.  
> **Labels & Annotations:** Vertical arrows at the surface indicate "Rainfall Infiltration" entering the topsoil labeled "Permeable Horizon A". When infiltrating water encounters the dense lower layer labeled "Restricting Layer / Claypan / Bedrock", it cannot percolate downward and instead moves laterally downslope through the soil matrix as indicated by horizontal arrows labeled "Interflow / Subsurface Lateral Flow". Near the base of the slope, an arrow pointing upward is labeled "Return Flow / Seepage Exfiltration", where subsurface water re-emerges to become surface runoff.  
> **Functional Meaning:** Depicts lateral subsurface storm flow (interflow) and return flow seepage along restricting layers. WEPP primarily models surface erosion from Hortonian overland flow and does not fully treat the geotechnical instability, sloughing, or rill erosion caused by subsurface return flow exfiltrating on lower slope faces.

---

## 12. APPENDIX 6 - SOME ATYPICAL CROP ROTATIONS

In addition to standard Midwest grain and meadow rotations, WEPP must be capable of evaluating atypical and complex crop rotations utilized across various U.S. agricultural regions:
1. **Corn (3 yrs.) – Oats (2 yrs.) – Meadow (3 to 5 yrs.)**  
   *Application:* Upper Midwest dairy and livestock farming systems with extended forage cycles.
2. **Corn (3 yrs.) – Oats (1 yr.) – Meadow (4 to 5 yrs.)**  
   *Application:* Sloping erodible cropland requiring prolonged perennial sod cover between intensive grain cultivation cycles.
3. **Continuous Cotton with Winter Cover Crop (Vetch / Wheat / Rye)**  
   *Application:* Southern US Coastal Plain and Piedmont cropping systems where winter legume or cereal cover crops are turned under or killed as mulch prior to spring cotton planting.
4. **2-Year Sugar Beet – Dry Bean – Spring Wheat – Potato Rotations**  
   *Application:* Irrigated and dryland specialized commercial vegetable and root crop systems in the Pacific Northwest, Northern Plains, and Intermountain West involving intensive soil disturbance and low residue crops.
5. **Wheat – Fallow – Sorghum (3-Year Rotation with 11-Month Fallow Periods)**  
   *Application:* Great Plains semi-arid dryland farming systems where soil moisture conservation requires long fallow intervals between dryland cereal crops, leaving soil vulnerable to wind and water erosion if residue is not managed by conservation tillage.

---

## 13. APPENDIX 7 - EXAMPLE OF A FUNCTION THAT ADDS ROBUSTNESS TO MODEL

To ensure operational robustness and prevent numerical instability or unrealistic jump discontinuities in model outputs, parameter relationships in WEPP must be continuous and smooth across transition thresholds.

### Figure 13.1: Example of a Function That Adds Robustness to Model
![Example of a Function That Adds Robustness to Model](./wepp-figures/wepp-fig-app7-robustness-function.png)

> **WCAG AAA Description:**  
> **Figure Type:** Mathematical parameter relationship graph.  
> **Visual Layout & Orientation:** A two-dimensional Cartesian coordinate graph comparing two mathematical functions representing a soil erodibility parameter or adjustment factor across a boundary condition. The horizontal axis represents an input variable (such as soil moisture, residue cover, or temperature threshold), and the vertical axis represents the computed parameter adjustment factor \(Y\).  
> **Labels & Annotations:** A dashed line depicts a "Step Function (Non-Robust)", which remains flat at value \(Y_1\) and then jumps instantaneously vertically at threshold \(X_0\) to a higher value \(Y_2\). A solid smooth curve depicts a "Smooth Continuous Function (Robust / WEPP Target)", which begins curving gradually before \(X_0\) and asymptotically transitions to \(Y_2\) without sharp corners or discontinuities. Annotations highlight that step functions cause erratic model sensitivity and numerical oscillations during simulation, whereas smooth sigmoid or exponential transition curves maintain mathematical continuity and model robustness.  
> **Functional Meaning:** Illustrates the core modeling requirement that WEPP constitutive equations must avoid abrupt step functions and instead employ continuous, differentiable functions to ensure robust numerical convergence and predictable user results.

---

## 14. APPENDIX 8 - SIMILARITIES OF WEPP WITH OTHER MODELS THAT MIGHT BE USED BY SCS FOR EROSION COMPUTATIONS

To facilitate adoption by agency personnel, WEPP incorporates proven scientific concepts from existing hydrologic and erosion models while advancing beyond their structural limitations:

### I. USLE (Universal Soil Loss Equation)
- **A. Principal Application:** Predicting long-term average annual sheet and rill soil loss on specific field slopes under specified land use and management.
- **B. Similarities with WEPP:**  
  1. Utilizes baseline soil erodibility concepts adjusted for soil texture, organic matter, and structure.  
  2. Evaluates the mitigating effects of surface residue cover, canopy cover, and conservation practices (contouring, strip cropping, terracing) on soil detachment.  
  3. Applies to field-sized decision units and conservation planning plots.
- **C. Key Differences / WEPP Improvements:**  
  1. USLE is an empirical factor-based equation (\(A = R K L S C P\)) predicting only average annual detachment; WEPP is a process-based simulation model operating on daily or storm-event time steps.  
  2. USLE cannot predict sediment deposition on concave slopes or in terraces; WEPP explicitly models hydrodynamic detachment, transport capacity, and deposition along irregular landscape profiles.  
  3. USLE does not simulate concentrated flow (ephemeral gully) erosion or sediment yield delivery to waterways; WEPP routes runoff and sediment through channels and impoundments.

### II. CREAMS (Chemical, Runoff, and Erosion from Agricultural Management Systems)
- **A. Principal Application:** Field-scale model for evaluating nonpoint source pollution, sediment yield, and chemical transport from agricultural fields.
- **B. Similarities with WEPP:**  
  1. Separates soil erosion into interrill detachment (raindrop impact) and rill detachment (overland flow shear stress).  
  2. Utilizes Yalin's sediment transport capacity equation to govern sediment routing and deposition.  
  3. Models overland flow planes, concentrated flow channels, and impoundments within field boundaries.  
  4. Maintains a continuous daily water balance (soil moisture, evapotranspiration, percolation).
- **C. Key Differences / WEPP Improvements:**  
  1. CREAMS uses empirical SCS Curve Number and USLE-derived erodibility parameters; WEPP uses fundamental Green-Ampt infiltration and physically-derived soil erodibility parameters (\(K_i, K_r, \tau_c\)).  
  2. CREAMS requires static user inputs for plant growth and residue; WEPP includes dynamic crop growth, biomass accumulation, root development, and residue decomposition modules.  
  3. WEPP provides superior spatial resolution across complex grid and watershed configurations.

### III. EPIC (Erosion/Productivity Impact Calculator)
- **A. Principal Application:** Long-term simulation model assessing the impact of soil erosion on agricultural productivity and crop yields across hundreds of years.
- **B. Similarities with WEPP:**  
  1. Incorporates daily weather generators (precipitation, temperature, solar radiation) to simulate long-term climate variability.  
  2. Simulates daily crop growth, leaf area index (LAI), root depth, and nutrient cycling.  
  3. Tracks soil profile degradation, layering changes, and organic matter depletion over time.
- **C. Key Differences / WEPP Improvements:**  
  1. EPIC relies on USLE/MUSLE empirical energy factors for erosion calculations; WEPP utilizes fundamental hydrodynamic detachment and routing equations.  
  2. EPIC operates on a single lumped point/field slope; WEPP models distributed spatial erosion and deposition across complex topography and watersheds.

### IV. SWRRB (Simulator for Water Resources in Rural Basins)
- **A. Principal Application:** Predicting runoff, water yield, sediment yield, and agricultural chemical transport in large rural watersheds and river basins.
- **B. Similarities with WEPP:**  
  1. Simulates continuous daily water balance and hydrology across sub-basins.  
  2. Routes streamflow and sediment through complex stream channel networks and reservoir impoundments.  
  3. Utilizes weather generators and crop growth simulation modules.
- **C. Key Differences / WEPP Improvements:**  
  1. SWRRB is designed for large basins (up to several thousand square miles) using lumped Modified USLE (MUSLE) sediment equations; WEPP is tailored for field-sized and small watershed areas (up to 4th order) using physically-based erosion mechanics.  
  2. WEPP provides detailed spatial resolution of hillslope detachment and deposition, whereas SWRRB lumps hillslope processes into sub-basin averages.

### V. SPUR (Simulation of Production and Utilization of Rangelands)
- **A. Principal Application:** Comprehensive rangeland ecosystem model simulating forage production, livestock grazing, hydrology, and erosion on rangeland watersheds.
- **B. Similarities with WEPP:**  
  1. Models rangeland plant community dynamics, perennial grass growth, and shrub/forb competition.  
  2. Simulates the impacts of livestock grazing (biomass removal, trampling, compaction) on soil infiltration and cover.  
  3. Evaluates rangeland hydrology and sediment yield across watershed sub-units.
- **C. Key Differences / WEPP Improvements:**  
  1. SPUR uses Modified USLE equations for sediment yield; WEPP implements modern hydrodynamic rill/interrill equations calibrated specifically for rangeland soil/site combinations.  
  2. WEPP provides advanced modeling of surface crusting, rock fragment cover, and spatial rangeland microtopography.

### VI. ANSWERS (Areal Nonpoint Source Watershed Environment Response Simulation)
- **A. Principal Application:** Event-based distributed grid model for predicting runoff, erosion, and sediment transport from complex agricultural watersheds during storm events.
- **B. Similarities with WEPP:**  
  1. Utilizes a distributed grid cell architecture to capture spatial variability of soils, topography, and land use across a watershed.  
  2. Simulates infiltration-excess overland flow routing from cell to cell across grid boundaries.  
  3. Separates detachment by rainfall impact from detachment by overland flow shear stress.
- **C. Key Differences / WEPP Improvements:**  
  1. ANSWERS is primarily a single-storm event model requiring manual initialization of antecedent soil moisture; WEPP operates as a continuous daily simulation model maintaining automated water balance, plant growth, and soil crust/moisture dynamics between storms.  
  2. WEPP incorporates advanced channel routing and impoundment deposition algorithms not present in basic ANSWERS grid formulations.

### VII. AGNPS (Agricultural Non-Point Source Pollution Model)
- **A. Principal Application:** Single-storm event grid model evaluating nutrient, sediment, and chemical runoff from agricultural watersheds to prioritize conservation best management practices.
- **B. Similarities with WEPP:**  
  1. Discretizes watersheds into uniform square grid cells to identify critical erosion and pollution source areas.  
  2. Evaluates the spatial effectiveness of conservation practices (terraces, grassed waterways, ponds) across the watershed.  
  3. Routes sediment and runoff from headwater cells through channel networks to the basin outlet.
- **C. Key Differences / WEPP Improvements:**  
  1. AGNPS uses empirical USLE equations and SCS Curve Number hydrology within each grid cell; WEPP uses Green-Ampt infiltration and fundamental rill/interrill shear detachment equations.  
  2. AGNPS evaluates single storm events with static cover inputs; WEPP performs continuous simulation of soil, plant, and residue dynamics over multi-year rotations.

### VIII. SEDIMOT II (Sedimentology by Distributed Model Treatment)
- **A. Principal Application:** Design and evaluation of sediment control structures, detention basins, strip mine reclamation, and urban construction site erosion control.
- **B. Similarities with WEPP:**  
  1. Provides detailed hydrodynamic modeling of sediment deposition, particle size distribution trapping, and hydraulic routing in impoundments, ponds, and check dams.  
  2. Routes hydrographs and sediment graphs through grassed waterways and vegetated channels.  
  3. Evaluates the sediment trapping efficiency of alternative conservation structures.
- **C. Key Differences / WEPP Improvements:**  
  1. SEDIMOT II relies on design storm hydrographs and MUSLE/USLE detachment estimates from contributing disturbed areas; WEPP integrates continuous upland hillslope erosion processes directly with channel and impoundment routing.  
  2. WEPP models dynamic changes in channel bed erodibility, vegetative resistance, and seasonal scour over multi-year simulation periods.

---

## COVER DESIGN & CORE TEAM DIRECTORY

### Cover Design Note
**COVER DESIGN:** The logos on the cover identify the major cooperating agencies: USDA-Agricultural Research Service (ARS), USDA-Soil Conservation Service (SCS), USDA-Forest Service (FS), and USDI-Bureau of Land Management (BLM) in the Water Erosion Prediction Project. The schematics identify the three operational models of WEPP: the landscape profile (hillslope) version, the small watershed version, and the grid version.

---

### WEPP Core Team & Program Leaders Directory

#### WEPP Project Leadership
- **L. J. Lane**, Project Leader (1987–Continuing), Hydrologist, USDA-ARS Aridland Watershed Management Research Unit, 2000 E. Allen Rd., Tucson, AZ 85719. Phone: (602) 629-6381.
- **G. R. Foster**, Former Project Leader (1985–1987), Hydraulic Engineer / Professor, Head, Department of Agricultural Engineering, University of Minnesota, St. Paul, MN 55108. Phone: (612) 625-7733.

#### Core Team Members (1987 Draft 6.3)
- **J. E. Gilley**, Agricultural Engineer, USDA-ARS, Room 252, Agricultural Engineering Department, University of Nebraska, Lincoln, NE 68583-0832. Phone: (402) 472-2975.
- **E. E. Alberts**, Soil Scientist, USDA-ARS, Watershed Research Unit, 270 South Russell Avenue, Columbia, MO 652. Phone: (314) 875-5399.
- **J. M. Laflen**, Agricultural Engineer, USDA-ARS, National Soil Erosion Research Laboratory, Purdue University, Building SOIL, W. Lafayette, IN 47907. Phone: (317) 494-8673.
- **A. D. Nicks**, Agricultural Engineer, USDA-ARS, Water Quality and Watershed Research Laboratory, P.O. Box 1430, Durant, OK 74702. Phone: (405) 924-5066.
- **W. J. Rawls**, Hydrologist, USDA-ARS, Hydrology Laboratory, Room 139, Building 007, BARC-West, Beltsville, MD 20705. Phone: (301) 344-3511.
- **K. G. Renard**, Hydraulic Engineer, USDA-ARS, Aridland Watershed Management Research Unit, 2000 E. Allen Road, Tucson, AZ 85719. Phone: (602) 629-6381.
- **J. R. Simanton**, Hydrologist, USDA-ARS, Aridland Watershed Management Research Unit, 2000 E. Allen Road, Tucson, AZ 85719. Phone: (602) 629-6381.
- **M. A. Nearing**, Agricultural Engineer, USDA-ARS, National Soil Erosion Research Laboratory, Purdue University, Building SOIL, W. Lafayette, IN 47907. Phone: (317) 494-8673.
- **S. C. Gupta**, Professor, Department of Soil Science, University of Minnesota, St. Paul, MN 55108. Phone: (612) 625-1244.
- **C. R. Amerman**, Hydraulic Engineer, USDA-ARS, National Program Staff, Room 201, Building 005, BARC-West, Beltsville, MD 20705. Phone: (301) 344-3080.

#### Cooperating Agency Representatives & Technical Steering Committee
- **SCS Representative:** H. R. Gardner, Soil Conservationist, USDA-SCS, South National Technical Center, P.O. Box 6567, Fort Worth, TX 76115. Phone: (817) 334-5242.
- **FS Representative:** W. F. Megahan, Principal Research Hydrologist, USDA-FS, Intermountain Forest and Range Experiment Station, 316 E. Myrtle Street, Boise, ID 83702. Phone: (208) 334-1457.
- **BLM Representative:** M. R. Karl, Rangeland Hydrologist, USDI-BLM, Service Center (SC-370), P.O. Box 25047, Denver, CO 80225. Phone: (303) 236-0161.

---