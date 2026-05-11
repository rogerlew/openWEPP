# U.S. Geological Survey (1986). Professional Paper 1302: Basic Concepts of Kinematic-Wave Models (text extract)

**Source PDF**: /workdir/wepp-forest/references/USGS_PP1302_1986_Kinematic_Wave_Models.pdf

---
Basic Concepts of Kinematic-Wave Models
U. S. GEOLOGICAL SURVEY PROFESSIONAL PAPER

1302

Basic Concepts of Kinematic-Wave Models
#y JEFFREY E. MILLER

U. S. GEOLOGICAL SURVEY PROFESSIONAL PAPER

1302

UNITED STATES GOVERNMENT PRINTING OFFICE, WASHINGTON: 1984

UNITED STATES DEPARTMENT OF THE INTERIOR
WILLIAM P. CLARK, Secretary
GEOLOGICAL SURVEY
Dallas L. Peck, Director

Library of Congress Cataloging in Publication Data
Miller, Jeffrey E.
Basic concepts of kinetic-wave models.
(Geological Survey professional paper ; 1302)
Bibliography: p.
Supt. of Docs, no.: 119.16:1302
1. Water waves Mathematical models. I. Title.
II. Series.
TC172.M54
1983
627'.042
83-600251

For sale by the Distribution Branch, U.S. Geological Survey,
604 South Pickett Street, Alexandria, VA 22304

PREFACE
Kinematic-wave models are used extensively by the U.S. Geological Survey.
Both kinematic- and modified kinematic-wave models are used for channel and
overland-flow routing in the Precipitation-Runoff Modeling system and in the
Distributed Routing Rainfall-Runoff Model. The development of the theory
and application of kinematic waves is complex, but it is not readily available in
any one text. Therefore, the purpose of this report is to present the basic concepts of the kinematic-wave approximation of one-dimensional dynamic waves.
The report is intended for an audience of field hydrologists applying kinematicwave models in practical situations such as watershed models. This report does
not propose to assess or describe the current state of the art of approximate
hydraulic-flow routing techniques. It is intended to be used as a basic reference
on the theory and application of kinematic-wave models and modified
kinematic-wave models.
The kinematic-wave model is one of a number of approximations of the
dynamic-wave model. Because of the numerous approximations made in the
development of the models and the difficulty involved in applying the solution
techniques, the kinematic-wave approximation needs to be understood in relation to the other models. Therefore, a significant part of the report describes
the dynamic-wave model, including a detailed development of the governing
equations. This development is important because it shows the assumptions
made in describing unsteady flow with the full dynamic-wave equations. These
assumptions are inherent in any of the models that are approximations of the
dynamic-wave model, such as the kinematic-wave model.
Because of the intended audience, the equation developments are given in
detail so that they will be understood as easily as possible. There are many
basic concepts involved in the application of kinematic-wave models that can
be confusing. Instead of assuming that these concepts are obvious to the
reader, they are included in a supplemental information section.

m

CONTENTS
_
--

Abstract
Introduction
History of development of kinematic-wave theoryDevelopment of governing equations
Continuity equation
Equation of motion
Development of kinematic-wave approximations
Importance of assumptions and approximations Kinematic waves
Properties of kinematic waves
Solution techniques Finite-difference methods
Method of characteristics
Kinematic shock
Dynamic versus kinematic wavesApproximations of dynamic waves

1
1
1
2
3
4
7
7
7
8
8
9

-10
-12

Criteria describing limitations of applicability of
approximations -18
Further limitations of applicability
-20
Special topics and applications
-23
Summary and conclusions
-23
References
-£4
Supplemental information
-24
Equation development concepts
Momentum versus energy
-25
Coordinate systems
Friction-slope approximations
25
Methods for estimating a and m
26
Development of diffusion-analogy model
127
Development of step-backwater from dynamic-wave model 28
Kinematic-wave model used in the distributed routing
rainfall-runoff model
£9

ILLUSTRATIONS
Page

FIGURE 1. Definition sketch for the continuity equation
4

2. Longitudinal section showing prism and wedge storage

I 5

10

4. Water-surface profiles at time, t and t+At, showing distortion or steepening of the kinematic wave
5. Fixed crrid in the * t nlane ahnwinff Inmtirm nf crriH nninta and ivm-ritAm i and i

.

ill

Schematic of x-t plane showing typical characteristic paths for the kinematic-wave equations,
-412
with and without lateral inflow7. Water-surface profiles at time, t and t+At, and the characteristic paths between them showing the condition
necessary for shock formation
8. Comparison of kinematic- and dynamic-wave model results showing the upstream hydrograph and predicted hydrographs
at 20,000,40,000,60,000, and 80,000 feet
19
Four-point finite-difference grid
22
10. Use of log-log plot of discharge, Q, and flow area, A, to define the routing parameters, a and m

CONTENTS

VI

TABLE
TABLE

1. RELATIONS FOR ESTIMATING a and m based on physical characteristics of channel segments -

-27

SELECTED FACTORS FOR CONVERTING INCH-POUND UNITS
TO THE INTERNATIONAL SYSTEM OF UNITS (SD
For those readers who may prefer to use the International System of units (SD rather than inchpound units, the conversion factors for the terms used in this report are given below.
Multiply inch-pound unit

Cubic foot per second
Foot
Foot per mile
Foot per second

By

To obtain SI unit

0.02832
0.3048
0.1894
0.3048

cubic meter per second
meter
meter per kilometer
meter per second

DEFINITION OF SYMBOLS
a
A
Af
b
B
c
C
f
F
F0
g
h
hf
H
i
j
K
L
L0
m
n
P
Pc

acceleration or coefficient in the wetted perimeter-flow area
rea
relationship,
cross-sectional flow area,
area of flow at pipe-full conditions,
coefficient in the wetted perimeter flow-area relationship,>,
water-surface width,
wave celerity,
Chezy friction coefficient or Courant number,
force,
Froude number = u/Vg y,
Froude number for normal flow = v0/^jg~y t
gravitational acceleration,
height of water surface above datum or piezometric head,I,
friction loss,
total energy head,
finite-difference grid location in the ^-direction,
finite-difference grid location in the time direction,
kinematic-flow number,
length of wave,
length of channel segment or overland-flow plane,
mass or coefficient in the steady uniform flow equation
approximation,
Manning's roughness coefficient,
wetted perimeter,
channel parameter,

q
Q
Q,
*ff

fi
Sa
Se
S^S0
<
T
u
U0

Wt
Wx
X

y

%

2
a

y

0
p
C
TO

discharge per unit width (can be either flow in a channel or
di
la
lateral
inflow),
10
discharge,
di
di
discharge at pipe-full conditions,
U.
hydraulic radius = A/P,
h:
acceleration
slope,
a<
energy
slope = -dH/dx,
ei
fr
friction slope,
b« slope = -dz/dx,
bed
time,
ti
w
wave
period,
fl velocity,
flow
steady-state normal flow velocity,
St
time derivative weighting factor,
ti
spacial derivative weighting factor,
sF
horizontal distance,
h(
vertical
stream depth,
v(
n<normal depth,
height of stream bottom above datum,
h(
coefficient in the steady uniform flow equation approximacc
tion,
ti
specific
weight of water,
sp
angle
of the channel bed with the horizontal,
ai
diffusion
coefficient,
di
d<
density
of fluid, and
mean
m
longitudinal shear stress.

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS
By JEFFREY E. MILLER
ABSTRACT

The kinematic-wave model is one of a number of approximations of
the dynamic-wave model. The dynamic-wave model describes onedimensional shallow-water waves (unsteady, gradually varied, openchannel flow). This report provides a basic reference on the theory
and applications of the kinematic-wave model and describes the limitations of the model in relation to the other approximations of the
dynamic-wave model. In the kinematic-wave approximation, a number of the terms in the equation of motion are assumed to be insignificant. The equation of motion is replaced by an equation describing
uniform flow. Thus, the kinematic-wave model is described by the
continuity equation and a uniform-flow equation such as the wellknown Chezy or Manning formulas. Kinematic-wave models are
applicable to overland flow where lateral inflow is continuously
added and is a large part of the total flow. For channel-routing
applications, the kinematic-wave model always predicts a steeper
wave with less dispersion and attenuation than actually occurs. The
effect of the accumulation of errors in the kinematic-wave model
shows that the approximations made in the development of the
kinematic-wave equations are not generally justified for most
channel-routing applications. Modified flow-routing models can be
used which help to stop the accumulation of errors that occur when
the kinematic-wave model is applied.

INTRODUCTION

Kinematic-wave theory describes a distinctive type
of wave motion that can occur in many one-dimensional flow problems (Lighthill and Whitham, 1955, p.
281). The theory is described in this report as an
approximation of dynamic-wave theory applied to
water-routing problems.
The purpose of this report is to provide a basic reference on the theory and applications of the kinematicwave model and to describe the limitations of the
model in relation to the dynamic-wave model. The
kinematic-wave model is also described in relation to
the other hydraulic-routing techniques that are approximations of the dynamic-wave model. Unless
specifically indicated, the channels described in this
report are prismatic, and most of the explanations of
the behavior of the models will use examples that include no lateral inflow. These simplifications make the
behavior of the models easier to illustrate because
changes in numerically routed hydrographs are not
masked by other influences. The development, assumptions and approximations, properties, criteria for
application, current applications, and solution techniques of kinematic-wave theory in water routing will
be examined.

The kinematic-wave model is one of a number of approximations of the dynamic-wave model. The dynamicwave model describes one-dimensional shallow-water
HISTORY OF DEVELOPMENT OF
waves (unsteady, gradually varied, open-channel flow)
KINEMATIC-WAVE THEORY
and consists of the continuity equation and the equation of motion with appropriately prescribed initial
The development of kinematic-wave theory occurred
and boundary conditions. In most approximations of late in the history of the development of the theory of
dynamic waves, the continuity equation and an approx- open-channel flow. It is based on the early developimation of the equation of motion are solved. In the ments in the study of steady varied flow and the later
kinematic-wave approximation, a number of the terms developments in the study of unsteady flow.
in the equation of motion are assumed to be insignifiAccording to Bakhmeteff (1932, p. 299), the theory of
cant; therefore, the equation of motion simply states steady varied flow is usually associated with J. M.
that the friction slope is equal to the bed slope. It is Belanger. Bakhmeteff stated that Belanger, in his
replaced by an equation describing uniform (which also 1828 paper, covers the "subject of varied flow in a
implies steady) flow. Thus, the kinematic-wave model remarkably complete and comprehensive manner."
is described by the continuity equation and a uniform- Bakhmeteff listed earlier references on varied flow,
flow equation, such as the well-known Chezy or Mann- such as Dubuat in 1779, Venturoli in 1818, and paring formulas, plus the usually imposed initial and ticularly Massetti in 1827. He also stated that Poncelet
boundary conditions.
is believed to have developed the equation of varied

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

flow about the same time as Belanger and that Euler
developed the foundations of hydromechanics in 1755.
Bakhmeteff further stated that "while Belanger and
his followers deducted the varied flow equation from
the general Newtonian equation of motion," Coriolis in
1836 "made use of the principle of conservation of
energy and thus was the first to suggest the reasoning
which since has been followed in textbooks on hydraulics in establishing the so-called Bernoulli equation."
According to Yevjevich (1975, p. 2-5), the study of
unsteady flow started with two French mathematicians Laplace, in 1775, and Lagrange, in 1781. The
Lagrange celerity formula was reported in 1788, providing impetus for further studies. Barre de Saint
Venant presented the equation of motion and the
water-continuity equation in 1871. These equations
are now known as the Saint Venant partial-differential
equations of unsteady flow. Also in 1871, Saint Venant
attempted to integrate the continuity equation and the
equation of motion by setting the channel slope equal
to the energy line. This approach is similar to
kinematic-wave theory.
Lighthill and Whitham (1955) stated that, several
writers independently have given the theory of continuous kinematic waves as applied to flood movement.
Complete treatments were given by Boussinesq in
1877 and by Forcheimer in 1930 (in his book
Hydraulik). These two authors referred to an unpublished report by Kleitz in 1858, Breton in 1867, and
Graeff in 1875 as pioneers of the theory. Lighthill and
Whitham also stated that Seddon, in 1900 discussed
the problem with reference to the Mississippi and its
tributaries and that Seddon, although unaware of the
earlier work, showed an understanding of the variety
of mechanism that govern the relationship between
flow area and discharge.
The principal theoretical work on kinematic waves
was done by Lighthill and Whitham (1955). They
named the theory "kinematic wave" and investigated
the general properties of waves and shock waves based
on the theory. They also provided a detailed treatment
of the relationship between kinematic- and dynamicwave movement in rivers.
Application of kinematic-wave theory to channel
routing has been described by Weinmann and Laurenson (1979), Henderson (1963), and Brakensiek (1967).
Cunge (1969) showed that the Muskingum routing
method can be considered.a finite-difference approximation of the kinematic-wave model. Thus, kinematicwave theory has been widely applied to river routing
for a number of years. The theory also has been applied
to the problems of water routing in irrigation systems
(see, for example, Cunge and Woolhiser, 1975) and
storm water runoff (selected references are the Storm

Water Management Model, U.S. Environmental Protection Agency, 1971, and Dawdy and others, 1978).
Overland flow is viewed as wide shallow-channel
flow and is often analyzed on a unit width basis with
lateral inflow originating from rainfall excess. Henderson (1966, p. 394) stated that Japanese engineers
applied kinematic-wave theory to the problem of overland runoff. Since then, a large amount of work has
been done on the overland-flow problem. Selected examples are Henderson and Wooding (1964), Woolhiser
and Liggett (1967), Kibler and Woolhiser (1970),
Schaafce (1970), Li and others (1975), and Borah and
others (1980).
Many numerical solution techniques have been
developed to solve the kinematic-wave equations.
Usually these techniques involve finite-difference (see,
for example, Kibler and Woolhiser, 1970) or method of
characteristics (see, for example, Borah and others,
1980) Schemes. Cunge (1969), the U.S. Environmental
Protection Agency (1971), and Li and others (1976)
have applied modified kinematic-wave approximations
in which adjusted slope terms are used or controlled
numerical dispersion is added to the solution for
channel-routing problems. In the time since the paper
by Lighthill and Whitham was written, a large amount
of work has been done in solving the kinematic-wave
equations, applying the theory to channel and overland flow, determining when the theory is applicable,
and describing the properties of waves based on the
theory and solution techniques.
DEVELOPMENT OF GOVERNING EQUATIONS

The equations describing unsteady open-channel
flow are a continuity equation and an equation of motion. The continuity equation is formulated based on
the principle of conservation of mass. The principle
states that the difference between the rates of inflow
and outflow is equal to the rate of change in storage.
The development of the equation of motion involves
Newton's second law and the principles of conservation
of energy and momentum. Newton's second law is
f=ma,

(1)

where
f force,
77i = mass,
and
a = acceleration.
The equation of motion can be derived from equation 1
directly, or it can be derived using conservation of
energy or momentum concepts. Hie development of the

DEVELOPMENT OF GOVERNING EQUATIONS

continuity equation and the equation of motion used in
this report follows Henderson (1966). The treatment
given by Henderson is developed in detail and the
equation of motion is based directly on Newton's second law instead of on the energy or momentum forms.
It uses an orthogonal gravity-oriented coordinate system. The relationship between momentum and energy
and a description of commonly used coordinate systems
are described in the section "Supplemental Information" under "Equation Development Concepts."
The following references give equation developments
for both the continuity equation and the equation of
motion. Derivation of the equation of motion, based on
the principle of conservation of energy, is given by
Gilcrest (1950) and Chow (1959). The derivation of the
equation of motion, based on the principle of conservation of momentum, is given by Liggett (1975), Theurer
(1975), and Viessman and others (1977).

(2)

The right-hand side will be positive if the volume is
decreasing. (Q may be changing with time as well as
with x.)
Considering the change in waterrsurface level, the
volume of water is changing at the rate of
dh
(BAx).
dt

(3)

This term will be positive if the volume is increasing.
Equations 2 and 3 must be equal in magnitude and opposite in sign. Therefore, an expression of continuity is
dx

(4)

dt

Dividing by Ax yields

CONTINUITY EQUATION

The development of the continuity equation involves
the following variables:
A = cross-sectional flow area,
B = water-surface width,
h = height of water surface above datum or
piezometric head,
q = discharge per unit width,
Q = discharge,
t = time,
v = flow velocity,
x = horizontal distance, and
y = vertical stream depth.
Consider a control volume enclosed by the water surface, the channel boundary surface, and two cross sections separated by a small distance, Ax, as shown in
figure 1. Without lateral inflow or outflow.

dQ

(5)

Equation 5 is the unsteady continuity equation. A form
of equation 5 for more general cross-section shapes
(where 57- is substituted for
dt

dQ dA
-~-

(6)

Two additional alternative forms are developed as
follows. When a channel is prismatic and rectangular,
Q=B q. Substituting into equation 5 and dividing by B,
the first alternative form, is

dx

(7)

dt

The second alternative form, which is useful for
visualizing each term, can be written by substituting
dQ d(A v)
, , .,
- , .
,. _
-r-i-= j. and depth, y, for h in equation 5:

-^ ** r

X\\NXNSxk\\V

d(A v)
~
Datum

FIGURE 1. Definition sketch for the continuity
equation.

dy

(8)

or
(9)

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

The three terms in the form of the continuity equation
given above are known as prism storage, wedge
storage, and rate of rise terms. The physical meaning
of the first two terms is shown in figure 2. Equation 9 is
valid only for prismatic channels.
EQUATION OF MOTION

The development of the equation of motion involves
the variables used in the continuity equation and the
following additional variables:
a = acceleration,
C = Chezy friction coefficient,
f = force,
H = total energy head,
g = gravitational acceleration,
m = mass,
P = wetted perimeter,
R = hydraulic radius =A/P,
Sa = acceleration slope,
Se - energy slope = -dH/dx,
Sf = friction slope,
S0 = bed slope = -dz/dx,
z = height of stream bottom above datum,
7 = specific weight of water,
6
angle of the channel bed with the horizontal,
Q = density of fluid, and
TO = mean longitudinal shear stress.
To develop the equation of motion, the forces on a fluid
element in a channel section will be determined based
in assumptions appropriate to one-dimensional shallowwater waves. Then the remaining acceleration and
mass terms will be developed and substituted into
Newton's second law.
Considering the channel sections shown in figure 3
and assuming that the pressure distribution is hydrostatic, the total horizontal hydrostatic pressure on the
element shown is equal to the difference in the two
hydrostatic pressures, 7 Ah, over the area of the element. The opposite sign is used because a negative Ah

will produce flow in the positive jc-direction. The crosssectional flow area of the element is
A=ABy.

(10)

There is only a small variation in area across Ax tfAh/y
and Az/y are small. The hydrostatic force difference is
then
-7 A A/i.

(ID

This force is resisted by a shear force equal to r0 over
the wetted perimeter along Ax at an angle 6 to the
horizontal. The net force in the direction of the flow is
f= -7 A Ah-r0 P Ax cose.

(12)

Following the assumption of small slopes, cos 0 = 1;
therefore, equation 12 can be written
/=-7 AAh-r0 PAx.

(13)

This is the force term in Newton's second law.
Again, considering the channel section in figure 3,
the mass term in Newton's second law is
m=Q A Ax.

(14)

The acceleration term in Newton's second law is developed as follows. Noting that a=dv/dt, from the theory of
partial differentiation,
dv _ dx dv dv
dt ~ dt dx dt'

(15)

Equation 15 describes the rate at which the velocity v
will change in the eyes of an observer moving in the
direction x with velocity dx/dt. If dv/dt is to be interpreted as a fluid acceleration, the observer must be
moving with the fluid itself. Thus, dx/dt=v and
dv

dv . dv

(16)

Wedge storage

Substituting equations 13, 14, and 16 into Newton's
second law (eq 1) results in the following:
7 A Ah TO P Ax=

(17)

Rearranging,
T =-

FIGURE 2. Longitudinal section showing prism and wedge storage.

PAx

(18)

DEVELOPMENT OF GOVERNING EQUATIONS
-Art

Datum

CROSS SECTION

LONGITUDINAL SECTION

T0

HYDROSTATIC PRESSURE DIAGRAMS
FIGURE 3. Definition sketches for the equation of motion.

Setting AIP=R and taking the limit as A*-*0 so that
A/t = dh
Ax
dx '
dv

(19)

(see Henderson, 1966, p. 21). The velocity distribution
coefficients for both energy and momentum are ignored
in this report unless specifically stated otherwise. Taking the partial derivative with respect to the downstream direction, the definition of the friction slope is

or, noting that y=Qg,

"'
v dv . 1 dv

(20)

At this point, before the equation of motion can be
developed further, three basic relationships two for
the friction slope and one for the longitudinal shear
stress must be introduced. The definition of the total
energy head is
(21)
This definition ignores the velocity distribution coefficient. This is often done in unsteady flow models
because the coefficient is only of secondary importance

-dH
dx

(22)

dx

or, by the rules of differentiation,
dH_ ,dh v dv
- (+

v

(23)

or
dH_ fdh L v dv
dx ~ ( dx

(24)

The two remaining relationships are approximations
when applied to the unsteady nonuniform flow situation. The derivation of these relationships is shown in
the section "Supplemental Information" under "Equa-

6

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

tion Development Concepts." They are the friction
slope based on the Chezy equation,

s,=- C*R

Setting h=z+y, equation 24 can be rewritten

(25)

dx

and the longitudinal shear stress expressed using the
Chezy equation,

Substituting S0 =^,
ox

(26)

Continuing the development of the equation of motion, equation 24 is substituted into equation 20:

Finally, substituting equation 35 into equation 29 and
rearranging, a commonly seen form of the equation of
motion is obtained:

dH

v* _ o0~dx~~j>dx~~g~di'
dy v dv 1 dv
o~
_ C*R~

(27)

Setting equation 26 equal to equation 27,

R_»»

n,m . i dv

Significant
(28)

terms for:
steady uniform
flow,

Rearranging,

gdt* C*R

(29)

or

se+sa+sf =o,

S = dH
dx

(30)

:=g(S0-Sf).

(32)
where Se is not equal to Sf, Instead,
(33)

TT-. Sa is a source of energy loss or gain

due to the unsteadiness of the flow.

The above arrangement was pointed out by Henderson
(1966). It shows that additional terms must be evaluated as the complexity of a given water-routing problem increases. Equation 36 can be rearranged into
another commonly seen form by multiplying by g and
using S^ instead of the Chezy equation:
(37)

(31)

and

s+s=-s

steady nonuniform
flow, ^
unsteady nonuniform
flow.

where Se is the energy slope, Sa is the acceleration
slope, and Sf is the friction slope. Here, Henderson
(1966) made a careful distinction between Se and Sf, In
the steady flow case, Se is the same as Sf except opposite in sign. However, in the unsteady flow case, Se
and S are defined as

where Sa=

(36)

Equation 37 is the one-dimensional form of the equation of motion describing unsteady open-channel flow
with zero lateral inflow. Other more complex applications may require modification of certain terms of additional terms to describe situations such as nonprismatic
channels, lateral inflow, and flood-plain storage. Twoor three-dimensional flow requires an entirely different
development. According to Viessman and others (1977,
p. 562), the convective terms gdy/dx and vdv/dx in
equation 37 describe the changes in kinetic energy and
potential energy (also called the inertia terms), the
term dvldt accounts for the local acceleration of the
fluid (also called the pressure-differential term), and

IMPORTANCE OF ASSUMPTIONS AND APPROXIMATIONS

Q=aAm,
(40)
the terms g S0 and g Sf account for the component of
gravitational force in the direction of flow and friction
along the channel. Equations 6 and 37 are sometimes in which a and m are coefficients defined for each channel cross section. Methods for estimating a and m are
called the Saint Venant equations.
given under "Supplemental Information." Usually, in
an application of kinematic-wave theory, equation 40
DEVELOPMENT OF
and the unsteady continuity equation (eq. 6) are solved
KINEMATIC-WAVE APPROXIMATIONS
simultaneously. Equations 6 and 40 are called the
kinematic-wave equations.
Kinematic-wave approximations to equation 37 were
Recalling the unsteady continuity equation (eq. 6),
first named and described in detail by Lighthill and dO dA
=0, and noting that the kinematic-wave
Whitham (1955). Waves whose motion is adequately
dx
dt
described by these approximations are known as
and that
" dt can be written
"kinematic waves" as opposed to dynamic waves as celerity, c, is equal to dA
described by equation 37. The various equations and dA
, the wave celerity can be substituted into
relationships describing kinematic waves constitute
at
"kinematic-wave theory." When this theory is applied equation 6 to obtain what is often called the kinematicto channel-water routing or overland-flow routing, it is wave equation,
often called "kinematic-flood routing," "kinematic
model," or "kinematic flow" (Miller and Gunge, 1975).
(41)
To apply kinematic-wave theory, the shallow-water
c dt
dx
wave is assumed to be long and flat so that the friction
slope, Sft is nearly equal to the bed slope, S0, in equa*J A

tion 37.Thus, the remaining terms,

, i, and

sometimes called the secondary terms, are assumed to
be insignificant. This implies that there is a balance
between the gravitational and frictional forces. The
resulting equation becomes
0=g(S0-Sf)

(38)

or
(39)
From equation 39 it is obvious that the friction slope is
parallel to the bed slope and, therefore, does not
change with flow conditions. Thus, it follows that q is a
function of y or A alone and can be evaluated using any
of the uniform-flow equations, such as the Chezy or
Manning formulas, in which S0 is substituted for S?
The terms neglected in the development of equation 39
are the inertia and pressure-differential terms the
dynamic terms. Kinematics is usually defined as the
description of motion without considering the forces
giving rise to motion. The kinematic-wave model is so
named because it is based primarily on the continuity
equation and only approximates the dynamic equation
with a uniform-flow formula.
By defining the friction slope with a uniform-flow formula, equation 39 can be represented for application to
a specific channel or overland-flow plane by a general
power relationship of the form

IMPORTANCE OF
ASSUMPTIONS AND APPROXIMATIONS

To avoid applying the kinematic-wave theory to
situations that it does not adequately describe, the
assumptions and approximations made to develop the
theory need to be understood. Because the kinematicwave equations are based on the dynamic-wave equations, the assumptions and approximations made in
the development of the latter need to be examined first.
The details in the following descriptions, except as
otherwise referenced, for dynamic waves are taken
from Liggett (1975) and for kinematic waves are taken
from Lighthill and Whitham (1955), Yen (1979), Miller
and Cunge (1975), and Ponce and Simons (1977).
DYNAMIC WAVES

Slope and hydrostatic pressure approximations. In
equation 11 the pressure distribution is assumed to be
hydrostatic. The pressure distribution will depart from
hydrostatic when the bed slope becomes steep. Chow
(1959, p. 32-33) showed that for uniform flow in steep
channels the pressure at any depth must be corrected
by the factor cos2 6. While this factor is only approximate for unsteady flows, it provides an indication of
the magnitude of errors involved. The correction
(cos2 6) decreases the pressure by an amount less than 1
percent until 6 is nearly 6 degrees, or a slope of about 1
in 10. Ordinary channels usually have slopes that are

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

8

much less than 6 degrees. Therefore, this approximation commonly is insignificant.
For the pressure distribution to be hydrostatic, it is
also necessary that the vertical components of acceleration be negligible. This is true if the flow is not rapidly
varied and for long shallow-water waves where the
slope of the wave front or back is small. However, this
may not be true for shorter, deeper water waves. As the
waves approach oscillatory waves, a correction should
be made to the wave celerity (wave speed). In order to
maintain a 2-percent accuracy in the wave celerity, the
ratio of the depth of the wave to the length of the wave
must be less than or equal to 0.055 (Liggett, 1975,
p. 44). Because the depth of a flood wave is usually less
than a few tens of feet and the length is usually
measured in miles, the ratio is much less than 0.055
and this approximation is commonly insignificant.
Friction approximations. Two friction-slope approximations are described in the section "Supplemental Information" under "Equation Development Concepts."
The first approximation is the common use of the
Chezy and Manning formulas to estimate the friction
slope in unsteady flow even though they were developed for steady uniform flow. Results of unsteady flow
modeling indicate that they are applicable. One major
problem is how to estimate the resistance or friction
coefficient. An error in the estimation of the resistance
coefficient leads to an error in the predicted velocity or
flow of the same magnitude. The second approximation
is the assumption of steady flow made to exprss the
longitudinal shear stress using the Chezy formula.
Again, the results of unsteady flow modeling indicate
that this approximation is adequate.
KINEMATIC WAVES

The additional assumptions and approximations
made in kinematic-wave theory can be summarized by
noting that Q is assumed to be a function of y alone.
This means that Sf=S0 and the other three slope terms,
called the secondary terms, in equation 36 are negligible. Thus, the bed slope, S0, is assumed to be large
enough and the water wave long and flat enough so
that the change in depth and velocity with respect to
dv

5r?

distance (-^- and -5-) and the change in velocity with
dx
dx
respect to time (-£) are negligible when subtracted
at
from S0 in equation 36.
Henderson (1966, p. 364) pointed out that for natural
floods in steep rivers whose slopes are of the order of
10 feet per mile or more, the values of the secondary
terms are small. He also provides some typical values

for the slope terms taken from an actual river in steep
alluvial country as follows:
So,

dy
~&T'

26

0.05

JLflH
g dx '

_L^H
g dt

0.125-0.25
(feet per mile)

0.05

Gunaratnam and Perkins (1970, p. 45) also reported
the following values, taken from Schaake (1965), for
gutter and overland flow.
So,
182
212

dy
dx'

JL
g dx 1

9.8
4.9
16.4
1.64
(feet per mile)

g
4.9
1.64

Values for the other slope terms (secondary terms) are
small compared to S0, which indicate the rationale
behind the assumptions made in the development of
kinematic-wave theory.

PROPERTIES OF KINEMATIC WAVES

The properties or behavior of kinematic waves were
first described in detail by Lighthill and Whitham
(1955). They were described further by Henderson
(1963 and 1966).
Unlike dynamic waves which propagate both downstream and upstream at characteristic speeds as stated
in the equation, c=v±^/gy (positive downstream and
negative upstream); kinematic waves propagate downstream only. Lighthill and Whitham (1955, p. 282)
pointed out that the continuity equation (eq 6) is of the
first order because for kinematic waves Q is a function
of y alone; thus, kinematic waves have only one wave
speed.
According to Henderson (1966, p. 365-367), the wave
speed in a prismatic channel with no lateral inflow can
be determined as follows. If a monoclinal wave is
brought to rest by the superposition of a velocity equal
and opposite to the wave velocity .(or celerity), c, the,
discharge through the now stationary wave is constant
and equal to
(v-c)A

(42)

or Q cA=constant discharge through wave. Taking
the derivative,

SOLUTION TECHNIQUES

dQ-cdA=0,

(43)

9

When the Manning formula is used, equation 52 by a
similar procedure becomes

and rearranging,
c=5/3i>.
_dQ
c=
dA

(44)

This is called the Kleitz-Seddon law (Lighthill and
Whitham, 1955, p. 282). Another form may be obtained
by noting that dA=B dy:
1 dQ
B dy

(45)

(53)

The relationship between the wave celerity and
mean velocity can be evaluated when the characteristics of specific channels are defined using equation 40.
From equation 44 it can be seen that the wave celerity
can be determined if equation 40 is differentiated with
respect to A as follows:
j =a m
dA

(54)

Setting Q=A v in equation 44,
Substituting equation 40,
c= dCAp)
dA

dA +A~dA
j_ A dv
~dA

(46)

or
c=v+A- dv
dA

(47)

The wave celerity can be evaluated further for wide
rectangular channels when using a specific uniformflow equation such as the Chefzy or Manning formulas.
For example, using the Chezy formula to describe the
relationship between velocity and depth and substituting R=A/P, the following form of the equation is
obtained:
(48)

Taking the derivative with respect to A and noting
that for wide rectangular channels P=B and is approximately constant,
dv =1/2CP-1/2 S''2A-1/2 ;
dA

(49)

multiplying both sides by A,
A"2=1/2CV S0 ;

(50)

p

substituting equation 48 into 50,

U/l

c=m-r-=m v.
A

(55)

Therefore, m determines the relationship between the
wave celerity and the mean velocity. Table 1 in the
"Supplemental Information" section shows that for
some typical prismatic channels the wave celerity is 1
to 1.67 times as fast as the mean velocity.
On streams where stage-discharge ratings have been
developed at representative channel sections, the
kinematic-wave celerity can be estimated using equadQ
tion 45. The slope p- can be measured directly from
the rating curve. This often provides a useful approximation of wave celerities in natural channels.
Equation 44 gives the speed at which a point of constant Q and y is moving. Since this speed depends on y
alone, points on the wave profile at the same depth, for
example B and B' at time, t, in figure 4, move at the
same velocity, and at a later time, t+At, the chord
length B B' remains constant. Thus, the wave does
not lengthen or disperse and does not subside as it
moves downstream (see point A in fig. 4). However, it
does distort because dQ/dy, and thus v, increases with
y. This was described by Henderson (1963 and 1966,
p. 370). If the wave traveled far enough, point A would
overtake point B' forming a vertical wall of water, or
surge. This result of kinematic-wave theory was named
"kinematic shock" by Lighthill and Whitham (1955).
Before kinematic shock can be discussed, the different
solution techniques for the kinematic-wave equations
need to be introduced.

(51)
SOLUTION TECHNIQUES

and, substituting equation 51 into equation 47,
c=v+l/2v=3/2v.

(52)

The equations describing dynamic or kinematic
waves are partial-differential equations for which

10

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

Water surface
profile
at time t

Water surface
profile
at time r + At

FIGURE 4. Water-surface profiles at time, t and t+At, showing distortion or steepening of the kinematic wave.

there are no analytic solutions except in a few idealized
situations. Thus, numerical techniques such as finitedifference methods, the method of characteristics, or
finite-element methods are employed. The first two are
most commonly used and will be discussed here. The
details involved in applying these techniques are
described by Liggett and Cunge (1975). The application
of numerical solution techniques to partial-differential
equations is a complex topic. While a complete description of the concepts involved is not included in this
report, an overview of the basic concepts is given
because the techniques are required to use kinematicwave models.
Partial-differential equations, in general, can be
classified as elliptic, parabolic, or hyperbolic. Equilibrium or steady-state problems generally are described
by equations that are elliptic. Propagation for marching problems are described by parabolic and hyperbolic
equations. The equations can be classified using the
concept of characteristics. Elliptic equations have no
real characteristic directions, parabolic equations have
one characteristic direction, and hyperbolic equations
have two real characteristic directions. The details involved in classifying partial-differential equations are
given by Ames (1977, p. 3-9). The equations describing
kinematic waves behave as parabolic equations although, since they are not of the second order they are
not actually parabolic. The equations describing
dynamic waves are hyperbolic. It has been noted previously that kinematic waves propagate in only one

characteristic direction while dynamic waves propagate in two characteristic directions. The difference in
the number of propagation directions is an example of
how the solutions to the equations in different classifications can behave differently. This needs to be considered in the formulation of the solutions.
FINITE-DIFFERENCE METHODS

The basic concepts behind finite-difference approximations to partial-differential equations are given by
Ames (1977). The approximations can be developed in
the following manner. If, in the x-y plane, a Taylor
series for an arbitrary point u(x+Ax,y) is developed
about u(x,y), the approximation for du/dx can be
developed.

(A*)2 d*u(x,y)
2\
dx*

(56)
where O[ | A* |"] is read "terms on the order of Ax to the
nth power." This is the notation for the truncation er-

SOLUTION TECHNIQUES

ror of this approximation. Simplifying by neglecting
all terms of higher order than one and solving for
du/dx.
du(x,y) ^ u(x+Ax,y)- u(x,y) Q[ | Ax \ 2]
dx
Ax
Ax

(57)

or
du
dx

(58)

where the notation used in equation 58 is typical for
approximation of du/dx in space and time for marching
problems on a fixed grid in the x-t plane. Points on the
grid are located by multiplying the grid spacing, Ax or
At, by the corresponding i or j terms.
Equation 58 is a one-dimensional forward difference.
The same procedure can be repeated for u(x Ax,y) to
determine a backward-difference approximation. If the
backward and forward differences are written to include
the second order terms and then one is subtracted from
the other, a central difference of the second order,
O[ | Ax | 2], is obtained. Finally, if the same procedures
are followed except that the two differences are added,
a second order approximation for da u/dx? is obtained.
Following the above procedures, approximations for
the terms in the partial-differential equations describing one-dimensional unsteady flow can be developed.
To apply finite-difference techniques, a fixed grid in
the x t plane is usually employed as shown in figure 5.
Generally, in unsteady flow problems such as river
routing, the velocity and depth (v, y) or discharge and
area (Q, A) are known for all grid points along the
^-direction at the initial time, £=0. Also, the inflow
hydrograph is known (either y or Q) and sometimes the
outflow hydrograph is known. To solve for the points
between, either an explicit or implicit scheme is em-

At

Ax
'-1,7

'/ 7

Initial time

FIGURE 5. Fixed grid in the x-t plane showing location of grid
points and counters, i and j.

11

ployed. Generally, when an explicit scheme is used,
the approximations are written in such a way that
unknowns at each point in the next time step can be
solved for individually and then the procedure is
repeated for each succeeding row.
Explicit schemes are often subject to a certain condition that is expressed as a "mesh ratio" (ratio of time
step to distant step size) to produce a stable finitedifference scheme (O'Brien and others, 1950). Stability
refers to the ability of the scheme to prevent numerical
errors from growing in an unbounded or uncontrolled
manner. This stability condition is called the Courant
or Courant-Friedrichs-Lewy stability condition. It
states, for application to the equations in this report,
that the wave celerity must be less than the ratio of
distance step to time step in the finite-difference
scheme. In practical applications, this condition can
limit the size of time step that can be used.
Generally, when an implicit scheme is used,
unknowns for more than one point or each succeeding
row of points in time are solved for at once. Finitedifference equations involving more than one unknown
point in the unknown time step are developed so that
each grid point along the ^-direction is included. The
scheme is designed so that, when the equations are
written for an entire row, there are an equal number of
unknowns and equations. Then a Gaussian elimination or other matrix-solver is used to solve for unknowns at all points simultaneously. One of the advantages of implicit schemes over explicit schemes is that
they are generally not subject to the restrictive mesh
ratio conditions for stability.
To apply the finite-difference methods, care must be
taken to describe the initial and boundary conditions
properly. In the application of a kinematic-wave model
all of the many different hydrographs and ranges of
flow are described by the same partial-differential
equations (eqs. 6 and 40). The many possible flows are
distinguished only by the initial and boundary conditions and by the channel parameters, such as the friction coefficient and channel shapes. In the application
of a kinematic-wave model, initial values for the
unknowns are required at all grid locations and values
for the unknowns are required at the upstream boundary of the channel for all time steps modeled. The actual unknowns specified depend on the formulation of
the model but are usually some combination of velocity, depth, discharge, or area. Only one is specified for
each initial or boundary condition. The application of a
dynamic-wave model requires the specification of more
complex boundary conditions because the equations
have two characteristic directions. Therefore, initial
conditions and both upstream and downstream boundary conditions must be specified.

12

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

Finite-difference approximations for the kinematicwave equations have been presented by Brankensiek
(1967), Li and others (1975), Li and others (1976),
Dawdy and others (1978), and Kibler and Woolhiser
(1970). As an example, the explicit scheme by Dawdy
and others (1978) is described under "Supplemental Information." A linear, implicit finite-difference solution
for the full dynamic-wave equations is presented by
Land (1978) and Reefer (1976).
It should be noted that the application of numerical
methods to partial-differential equations is not straightforward. Finite-difference schemes are often developed
that appear to be accurate and yet do not produce useful
results. However, a very similar scheme may produce
very good results (Roache, 1976, p. 3).
METHOD OF CHARACTERISTICS

The method of characteristics has been used to solve
the equations describing dynamic and kinematic
waves. An overview of the development of the method
of characteristics for application to the kinematic-wave
equations is presented here. The method applied to the
full dynamic-wave equations is described by Abbott
(1975) and Theurer (1975). The development given here
is taken from Eagleson (1970) and Borah and others
(1980). A detailed discussion of the concepts behind the
method of characteristics is given by Crandall (1956, p.
352-355). The method involves locating the characteristic paths and then integrating the ordinary differential equations that apply along the characteristic
paths. When applied to wave motions, the method
describes the space-time locus of discontinuity in the
partial derivatives, with respect to space and time, of
the dependent variables (in this case, A and Q). This
locus defines the path of wave propagation (the characteristic path) along which the phenomenon is described
by an integrable ordinary differential equation.
The development of the method of characteristics for
the kinematic-wave equations involves a system of
four equations in four unknowns written in matrix
form. The four unknowns are the partial derivatives,
with respect to space and time, of the dependent varidA dA dQ
, dQ . , ,,
,
,
U1
ables,
- ~T-, and ?*-. After the system of
at , ox
at
ox
equations is written in matrix form, the remaining
concepts in the development are based on Cramer's
rule for solving simultaneous linear equations.
Cramer's rule gives the solution in the form of quotients of determinants.
The application of the method of characteristics to
kinematic-wave equations converts the two equations,
one of which is a partial-differential equation, to ordinary differential equations. One describes the path of

the characteristic in the x-t plane, and the other
describes the changes in area or discharge along the
characteristic path. The path of the characteristic is
determined by the speed of the kinematic wave (wave
celerity). This speed is dependent on depth only; thus,
for prismatic channels in which there is no lateral inflow, the speed is constant and the characteristic is a
straight line (fig. 6). For nonprismatic channels or
situations in which there is lateral inflow, the speed
varies due to the varying depth and the characteristic
is not a straight line. As pointed out earlier, the
kinematic wave does not disperse or attenuate. Therefore, for the simple case of no lateral inflow, the
metho'd of characteristics involves simply tracking
point discharges or areas (sometimes velocity and
depth are used) to downstream locations based on the
wave celerity. The method is developed as follows.
When applying kinematic-wave theory to overland
flow, where lateral inflow must be considered, the continuity equation (eq. 6) is written to include lateral
inflow:
dQ , dA
~--

(59)

where q is lateral inflow in the form of either rainfall
excess or overland flow to a receiving channel. Following the simplifications made in the kinematic-wave
theory, the equation of motion (eq. 40) is written, based
on a uniform-flow equation, as Q=aAm. dQ/dt can be
determined from equation 40:
(60)
dt

dt

In addition to equations 59 and 60, two more equations
can be added using the definition of a total differential:
dQ=-

dx

dt

(61)

FIGURE 6. Schematic of x t plane showing typical characteristic
paths for the kinematic-wave equations, with and without lateral
inflow.

SOLUTION TECHNIQUES

and
dA=-^-d*+ dA
dx
dt

(62)

Equations 59, 60, 61, and 62 can be written in matrix
form:
dA/dt
q
dA/dx
0
dQ/dt = dA
dQ
dQ/dx

1
001
ct7n.Am~l 0 10
dt
d* 0 0
0
0 dt dx

13

other determinants in equation 63 also equal zero.
Now, if the columns of the coefficient matrix are
replaced one at a time by the column vector on the
right-hand side of equation 63, the determinant set
equal to zero, and solved, the following are obtained.
Replacing the first column or third column
(67)

replacing the second column,
(63)

dA = .
The discontinuity in the four derivatives (or the path of
the characteristic line) is determined by the indeterminacy in their solution. The derivatives in equation
63 do not exist along the characteristic paths and,
therefore, there is no unique solution to 63 along the
characteristic paths. According to Cramer's rule, a
solution to the four equations exists and is unique
unless the determinant
1
001
am A -1 0-10
dt
dx 0 0 =0
0
0 d* d
or the ordinary differential equation

amA
A

1

dt=°'
rt

and, replacing the fourth column,
jT =qam A"1"1 .

(65)

Equation 65 has two roots for the wave speed =dx/dt=c:
c=am Am~1

(54)

c=0.

(66)

(69)

Dividing equation 68 by equation 54,
dA
dx

(64)

(68)

q

(70)

Equations 67 through 70 apply only along the characteristic path defined by equation 55.
Borah and others (1980) integrated and discretized
these equations assuming that q remains constant over
small time and space increments. The solution is nearly analytic and involves very little numerical dispersion. However, the discretization does introduce some
approximations. The characteristic equations are
solved conveniently by setting Ax or At and then computing the corresponding At or Ax value. When lateral
inflow is included, the characteristic equations are

and
(71)

Except for the conditions of infinite roughness zero
slope and zero depth the root in equation 66 is trivial.
Thus, equation 54 is used to describe the kinematicwave characteristic. As shown earlier, equation 54 can
be rewritten
c=m v

(55)

Since m in the approximation to the uniform-flow equation is usually larger than 1, it can be seen from equation 55 that the wave speed is greater than the stream
velocity and propagates in the positive or downstream
direction only. According to Cramer's rule, when equation 55 holds, there is no solution at all unless the

or
(72)

When lateral inflow is not included, equations 71 and
72 are not defined. The characteristic equations for
zero lateral inflow are
(73)

or
(74)

14

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

The equations for area or discharge along the characteristics are
(75)
or
(76)

One possible procedure for solving the above equations
is to determine the downstream location to which an
upstream hydrograph is to be routed (define Ax). From
the known Ax, At can be computed from equation 72 or
74, the cross-sectional area of the flow at the new location (x+Ax, t+At) can be computed from equation 75,
and the discharge can be computed using equation 40.
To follow this procedure, the upstream hydrograph
must be known at each increment of time at the initial
distance x. Then the hydrograph can be computed at
the downstream distance x+Ax.
KINEMATIC SHOCK

When the kinematic-wave equations are solved using
the method of characteristics (where numerical dispersion is not a problem), kinematic shocks do occur. Some
investigators have attempted to determine whether
these shocks occur physically. However, Henderson
(1966, p. 370) pointed out that while the steepening of
the wave front may suggest that a surge will form, as
the wave front becomes steeper the neglected secondary slope terms in equation 37 will become significant.
The effect of these terms is to introduce dispersion and
attenuation that will retard the steepening of the wave
front, probably delaying and preventing any formation
of a surge. Henderson's point indicates that shocks are
simply a result of the approximations made in the
development of the theory.
In the development of the kinematic-wave equations,
the secondary slope terms in equation 37 are assumed
to be small enough to be neglected when the bed slope
is large. However, even though these terms are small,
the effect of neglecting them is cumulative. In figure 4,
the steepening of the wave front is small because of the
small differences in celerity at each depth in the kinematic wave. However, the steepening is continuous
because there are no terms in the kinematic-wave
model to prevent it. The peak always has a greater
depth and, therefore, a faster mean velocity and wave
celerity than a lower point on the front face of the
hydrograph. Thus, the peak continuously overtakes
the lower depths of the hydrograph and shock occurs,
making the solution unreasonable. The inclusion of the

secondary slope terms in the equation of motion would
introduce a small amount of dispersion and attenuation that would tend to offset the tendency to steepen.
This would slow the steepening of the wave front continuously, although by a very small amount. Again, if
the wave front eventually does become steeper, the
secondary slope terms become even more significant
and will probably prevent the formation of a surge.
After any significant travel distance, the kinematicwave approximation will always tend to steepen the
routed wave more than predicted by the full dynamic
equations due to the cumulative effect of neglecting
the secondary terms in the equation of motion.
Kibler and Woolhiser (1970) and Li and others (1976)
describe the conditions that cause kinematic shock formation based on the paths of the characteristics of the
kinematic-wave equations. The equation describing
the path of the characteristics is dx/dt=c. This is the
path of wave propagation the path that an observer
would follow if he was moving with the wave. When
the channel shape remains constant and there is no
lateral inflow, the wave celerity, c, is a constant for a
specific depth. Therefore, the characteristic paths are
straight lines. The slope of the lines is dependent on
the depth of the flow. Water-surface profiles at two
times and the characteristic paths between them are
plotted in figure 7, showing the condition necessary for
shock formation (adapted from Peter E. Smith, written
commun., 1981). This condition is that the characteristic paths must cross. The characteristic paths cross because a deeper and, thus, faster element of the flood
wave overtakes a shallower and, thus, slower element
of the wave. This causes a severe steepening of the
wave front and results in kinematic shock.
The condition for an intersection of the characteristic
paths is that the slope, dx/dt, of the upper characteristic, such as path 2 in figure 7, must be greater than
that of the lower characteristic, such as path 1 in figure
7. This condition can occur with or without lateral inflow. Following Kibler and Woolhiser (1970, p. 8), this
condition can be described as
(77)

where
h and

= height of water above datum at a particular point corresponding to a time, tlt and
a later time, t2,

and
q = constant lateral inflow.

KINEMATIC SHOCK

15

r + Af

CHARACTERISTIC PATHS
I

WATER SURFACE PROFILES

FIGURE 7. Water-surface profiles at a time, t and t+At, and the characteristic paths between them showing the condition necessary for
shock formation.

An illustration of the cumulative effect of neglecting
the secondary slope terms in the kinematic-wave approximation is provided Ponce, Li, and Simons (1978).
These authors developed a criterion that determines
when the kinematic-wave model provides an adequate
approximation of the diffusion- and dynamic-wave
models. If the criterion is meet, the kinematic-wave
model will provide 95-percent accuracy in the wave
amplitude after one propagation period. Thus, for the
usual situation in which the wave amplitude is being
attenuated, the amplitude predicted by the diffusionor dynamic-wave models would be at least 95 percent of
the amplitude predicted by the kinematic-wave model.

However, this is for one propagation period. After two
propagation periods, the amplitude would be in error
by 5 percent of a wave that was already in error by 5
percent. This criterion again indicates that the error is
cumulative.
Usually, kinematic shocks only occur when the
method of characteristics is used to solve the
kinematic-wave equations (eqs. 6 and 40) because, in
this case, the characteristics solution is a nearly
analytic solution that, therefore, involves very little
numerical dispersion. When numerical solutions such
as finite-difference methods are used, the solution
often includes a significant amount of numerical

16

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

dispersion. This numerical dispersion has an effect on
the solution similar to the secondary slope terms in
equation 37. That is, it tends to introduce attenuation
and dispersion. Numerical dispersion generally prevents the formation of kinematic shocks. While numerical dispersion is similar to the actual dispersion occurring in water waves, there is no control over how much
dispersion and attenuation occurs. Therefore, it does
not provide an accurate model unless the dispersion is
controlled.

The wave models and terms included in each are as
follows:
1.
2.
3.
4.
5.

Model
Kinematic wave
Diffusion wave
Steady dynamic wave
Dynamic wave
Gravity

Terms
IV

m + rv
ii + in + iv
I + II + HI + IV
i + ii + m

The terms are named as follows: I is the local inertia
term, II is the convective inertia term, III is the
pressure-differential term, and IV is simply the bed
DYNAMIC VERSUS KINEMATIC WAVES
and friction slopes. Ponce and Simons (1977) provided a
To mathematically describe dynamic waves requires detailed description of the properties of the five approxboth the continuity equation and the full equation of imations based on their application of the theory of
motion, whereas to describe kinematic waves requires linear stability for flow in prismatic wide channels. A
the continuity equation and only a simplified form of brief review of the properties of the diffusion-, steady
the equation of motion. Dynamic waves propagate at dynamic-, and gravity-wave models follows (properties
two-wave celerities whereas kinematic waves pro- of the kinematic-wave approximation and the dynamicpagate at one-wave celerity. According to Lighthill and wave model were described previously).
Diffusion waves and kinematic waves propagate
Whitham (1955), the properties of both waves are
downstream
only and at an equal celerity. However,
demonstrated in flood waves but the kinematic propunlike
kinematic
waves, diffusion waves attenuate as
erties of the waves dominate. This is indicated by the
they
propagate
downstream.
Steady dynamic waves also
observed fact that the main part of the flood wave
propagate
downstream
only
and attenuate. However,
moves more slowly than the speed predicted for
dynamic waves. Dynamic waves have a high rate of at- the wave celerity and attenuation are not the same as
tenuation; therefore, the faster wave moving down the for diffusion waves. Like dynamic waves, gravity
river ahead of the kinematic wave attenuates quickly wave propagate at two wave celerities, c=v+\lgy and
and is, therefore, less significant a short distance c=v \]gy. However, unlike dynamic waves, they gendownstream. Miller and Gunge (1975, p. 196) stated erally attenuate only weakly.
There are two other approximations that are of inthat the largest part of the flood wave moves as a
terest.
Both are based on the diffusion-wave model.
kinematic wave because of the domination of the bedThese
approximations are the Muskingum-Cunge
slope term in the equation of motion. Therefore, the
model
and
the diffusion-analogy model.
speed of a flood wave may often be reliably predicted by
the speed of a kinematic wave for wide channels,
dy
c=3/2u, or from equation 45.
: -^-=
..
(79)
dx*
dt
dx
APPROXIMATIONS OF DYNAMIC WAVES

The kinematic-wave approximation is only one of a
number of possible approximations of dynamic waves.
Ponce and Simons (1977) described a number of approximations by considering the various possible combinations of the slope terms included in the equation of motion (eq. 37) to be solved along with the continuity
equation (eq. 6). They write equation 37 and number
each term in the following form:
I
II
III
IV
Term
Equation
1 dv v dv dy , 0 0 N _
/P70,
»
..
r i
- -I- T -K&r" o ) =0.
i78i
of motion
g dt g dx dx ^ °

in which /*= diffusion coefficient.

d*v
In the diffusion-analogy model, tthe term p-r^- is used
to approximate the diffusion introduced by the

term

in the diffusion-wave model. The diffusion-analogy
model is developed under "Suplemental Information."
The Muskingum-Cunge model is an adaptation of the
Muskingum method in which the numerical dispersion
is controlled based on the hydraulic characteristics of
the channel. The Muskingum-Cunge model is described
under "Special Topics and Applications."
The standard-step method commonly applied in stepbackwater analysis for steady, gradually varied flow is
also an approximation of the dynamic-wave model. The

CRITERIA DESCRIBING LIMITATIONS OF APPLICABILITY OF APPROXIMATIONS

development of the step-backwater approximation is
given under "Supplemental Information."
Yen (1979) compared kinematic-, diffusion-, and
steady dynamic-wave approximations to dynamic
waves. He stated that the kinematic model is the least
accurate and, even though the steady dynamic wave
includes another term over the diffusion wave, it is not
as accurate as the latter. This, he point out, is because
for the typical case of downstream propagation terms I
and II are usually of about the same order of magnitude
but have opposite signs. Thus, neglecting both of them
is more accurate than including only one.
When balancing the computation time and complexity
against the accuracy required for each water-routing
application, the kinematic-wave, the diffusion-wave or
analogy, and the dynamic-wave models appear to be
most commonly used. The remaining approximations
usually entail almost as much complexity in their solutions and less accuracy when compared to the next
more accurate model. Thus, the more accurate model
is chosen.
CRITERIA DESCRIBING LIMITATIONS OF
APPLICABILITY OF APPROXIMATIONS

Criteria for determining when kinematic-wave approximations of dynamic waves are acceptable have
been examined by a number of authors. Miller and
Cunge (1975, p. 197) showed the development of one
criterion based on the Froude number (F). Setting
dynamic- and kinematic-wave celerities equal produces a criterion for the condition of flow at which
dynamic and kinematic waves are of equal importance.
When the Froude number is equal to 2, the dynamic
and kinematic waves have the same celerity. According to Miller and Cunge (1975), when F is less than 2,
dynamic waves are rapidly dampened out (F is less
than 3/2 when Manning's equation is used). Kinematicwave theory does not adequately describe highly supercritical flows because the dynamic-wave celerity
diverges from the kinematic-wave celerity. This
criterion was first pointed out by Lighthill and
Whitham (1955, p. 294).
The applicability of the kinematic-wave model for
the overland-flow problem of a rising hydrograph due
to lateral inflow over one plane or channel segment
was described by Woolhiser and Liggett (1967) using
the kinematic-flow number, K; K was defined as:
(80)

where

17

y0

normal depth for the maximum steady-state
discharge at the end of the flow plane or channel with a maximum rate of lateral inflow,
F0 = Froude number for normal flow 00A/£&,
L0 = length of overland-flow plane or channel segment,

and
v0 = steady-state normal flow velocity.
When K is large, the kinematic-wave theory is accurate.
When K=W, there is approximately 10 percent error
in the rising hydrograph from an overland-flow plane.
It is noted that K is often much larger than 10 and,
therefore, the kinematic-wave model is adequate for
most overland-flow problems.
The criterion of Woolhiser and Liggett should only be
applied to the rising hydrograph problem where no
front face is formed. Because of the continuous lateral
inflow in the problem discussed by the authors, the discharge is always greatest at the downstream end of the
overland-flow plane or channel segment; therefore, no
front face is produced. For applications where a front
face is formed due to a change in slope, roughness, or
lateral inflow along the plane or channel, the criterion
is not applicable because the criterion indicates that a
longer channel length is a case where the kinematicwave approximation gives better results. However, for
problems that produce a hydrograph with a front face,
the further downstream a wave propagates the more
the wave has steepended and the less accurate it
becomes (fig. 4).
Ponce, Li, and Simons (1978) applied the theory of
linear stability to examine the characteristics of
dynamic-wave approximations. Following this work,
they developed criteria for the application of kinematic-,
diffusion-, or dynamic-wave models. For kinematicwave models, they stated that for at least 95-percent
accuracy in wave amplitude after one propagation
period, the following inequality should hold:
(81)
where T is the wave period or length of time required
for one wave to pass, and v and yare the mean stream
velocity and depth corresponding to an average discharge computed by assuming uniform flow. The
amplitude is measured from a mean discharge and
after one wave period. For the diffusion-wave model to
be an adequate approximation of dynamic waves, the
following criterion must be met:

18

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS
(82)

If equation 82 is less than 30, then the full dynamicwave model must be applied to accurately account for
the rate of travel and amount of attenuation.
Henderson (1963) presented the following qualitative
criterion based on the bed slope. Flood waves on intermediate slopes are best described by the complete
equation of motion (eq. 37 or 78), flood waves on gentle
slopes are adequately described by the diffusion-wave
model (excluding terms I and II in eq. 78), and flood
waves on steep slopes are adequately described by the
kinematic-wave model (excluding terms I, II, and III in
eq. 78).
The criteria described here for determining the applicability of the different models apply to different situations. Miller and Gunge (1975) showed that the
kinematic-wave approximation should not be used for
highly supercritical flows. Most overland-flow and
channel problems involve subcritical flows and, therefore, the criterion is not generally useful. Woolhiser
and Liggett (1967) showed when the kinematic-wave
approximation is accurate for simple one-plane or
channel applications. This is not a typical application
so this criterion is not generally useful. The criteria
presented by Ponce, Li, and Simons (1978) are useful
for diffusion waves, but, as discussed earlier, because
the errors are cumulative, the criterion for kinematic
waves is not generally useful. The criteria presented
by Henderson (1963) are not quantitative and, therefore, provide only general guidelines.
FURTHER LIMITATIONS OF APPLICABILITY

Because kinematic waves do not attenuate but
always steepen or distort and because flood waves generally do attenuate and often do not steepen significantly, it was suspected that the errors incurred in the
kinematic-wave approximation may continuously accumulate. To illustrate the cumulative effect of the errors
in the kinematic-wave model, a comparison of solutions using both a kinematic-wave model and a
dynamic-wave model was developed.
The comparison was developed for a 100-feet wide
rectangular channel with a bed slope, So, equal to 0.01
and a Chezy friction coefficient, C, equal to 25 (which is
approximately equivalent to a Manning roughness
coefficient ranging from 0.060 to 0.068 depending on
the depth of flow. A sinusoidal hydrograph was used as
input where the wave period, T, was equal to 10,000
seconds. The input hydrograph varied from approximately 250 to 700 cubic feet per second. Thus, criteria

for the kinematic-wave approximation were checked
by the following method. Using equation 81 at a reference flow of 400 cubic feet per second, which results in
a depth of 1.38 feet and a mean velocity of 2.90 feet per
second.
T $(-44=210 ;>171;

(83)

and, using equation 80 with L0 equal to the length of
channel traversed in one wave period (wave celerity
times wave period),
(84)

Both of the above criteria are met. The kinematic-wave
model should be adequate to predict the propagation of
the given wave.
The kinematic-wave model used in the comparison
was programmed based on a method of characteristics
solution presented by Borah and others (1980), as
described in the previous section. However, the shockfitting techniques used by the above authors were not
included. Instead, the solution, as described by the
kinematic-wave equations alone, was allowed to progress without adjustment for shock formation. If the
author's modified technique had been used, a much
more accurate result would have been obtained.
The dynamic-wave model programmed for this application is a linear implicit finite-difference model. It is a
simple formulation for application to prismatic channels only. It provides a convenient means to test
results using different time and distance steps. To
minimize the effects of numerical dispersion, the model
was run using a Courant number nearly equal to 1 and
numerous tests were run using smaller and smaller
time and distance steps. Because the model solves a
linearized form of the dynamic-wave equations, relatively small time and distance steps were required to
minimize the numerical dispersion. The model was further tested by comparing results with a linear implicit
finite-difference model developed by Keefer (1976) and
documented by Land (1978). The results from these two
dynamic-wave models were the same.
Because the downstream boundary condition was not
known, an 80,000-foot channel was modeled for a
distance of 200,000 feet. It was assumed that the solution at 80,000 feet would not be affected by the downstream boundary. This assumption was tested using a
much shorter reach and only insignificant differences
were noted.
The results of the application of kinematic and
dynamic models are shown in figure 8. The input

FURTHER LIMITATIONS OF APPLICABILITY

19

800

700

600
Kinematic
wave model
HI
HI
LL

Dynamic
wave model

O
CO

r>
o

500

z

IU
(D
CC.
<
I
O
CO

400

300

200
5000

10,000

15,000

20,000

25,000

30,000

35.000

40,000

TIME, IN SECONDS

FIGURE 8. Comparison of kinematic- and dynamic-wave model results showing the upstream hydrograph and predicted hydrographs at 20,000, 40,000, 60,000, and 80,000 feet.

hydrograph is shown along with the predicted hydrographs at downstream distances of 20,000, 40,000,
60,000, and 80,000 feet. The length of the input hydrograph is 37,500 feet. The model was run using a time
step equal to 50 seconds and a distance steep equal to
200 feet, giving a Courant number of approximately
1.1. (To compute the Courant number, the wave celerity
was determined from the time of travel of the peak discharge.) At a distance of 20,000 feet, the kinematicwave solution shows significant steepening (the
change in the discharge is more rapid on the front face
of the hydrograph). At a distance of 40,000 feet,
kinematic shock has already occurred (more than one
discharge is predicted at the same point and time). As
the wave moves farther downstream, the wave peak
completely overtakes the front edge of the wave. The
kinematic-wave model also failed to predict a significant amount of attenuation and dispersion. The criterion for accuracy after one wave period given by Ponce
and others (1978) is shown to be approximately correct;

however, the errors continue to accumulate as the
hydrograph is routed beyond one wave period until the
solution becomes unreasonable. The criterion states
that the wave amplitude, after one wave period, should
be within 95 percent of the correct wave amplitude as
compared to some average flow rate and depth.
Although the wave meets of all criteria for application
of the kinematic-wave model, it can be seen that the
error resulting from the kinematic-wave approximations in both the steepening and the attenuation is
cumulative as the wave moves downstream.
Li and others (1976) examined the conditions under
which kinematic shock occurs. They showed that along
the rising limb of a wave with small lateral inflows,
shock will always occur. In a kinematic-wave model,
the deeper parts of the wave have a faster wave celerity than the shallow parts. Thus, on the front face or
rising limb of the wave, the peak is moving faster than
the lower face and the wave continuously steepens until shock occurs. In a dynamic-wave model, attenuation

20

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

and dispersion, although small at each computation of Sf for application of the kinematic-wave model.
step, will generally prevent the steepening of the wave. Equation 40 is written, using the Manning equation,
Henderson (1966, p. 370) stated that as the wave as
front becomes steeper, the secondary slope terms come
into play and introduce dispersion and attenuation,
(85)
thus delaying or preventing altogether the formation
of a surge. However, based on the above comparison of
kinematic- and dynamic-wave models for channel-rout- where, from equation 78,
ing problems, the secondary slope terms appear to be
vdv
significant continuously, and application of a
(86)
kinematic-wave model to channel problems always
gives a steeper wave with less attenuation and dispersion than actually occurs. This result would not be im- The continuity equation was solved in a normalized
portant, however, in applications where there is no for- form of equation 6. The equation was normalized by
mation of a wave front in the channel. An example dividing by Q at pipe-full conditions based on an adwould be a channel of one slope where the flow origi- justed friction slope. The friction slope is computed
based on the actual flow conditions (not full). Weightnates from evenly distributed lateral inflow.
If a large amount of lateral inflow or channel storage ing factors Wx and Wt were used to weight the average
were included in the problem, the error involved in ap- of the finite differences at the i and i+1 distance steps
plying the kinematic-wave model would not be as and they andy'+l time steps. The resulting equation is
large. Thus, in overland-flow routing where the routing distances are short, the slopes steep, and the
(87)
lateral inflow large, the kinematic-wave model probably will produce adequate results. However, for most
channel-routing applications where the distances are where
significant and the lateral inflow is not a large percentage of the total flow, the errors accumulate and the
kinematic-wave approximation is not adequate. This is
true even when the wave meets the criteria which have
Ax Af
been set for application of the kinematic-wave approxiW
mation. Many channel-routing models are called
kinematic-wave models even though they are modified
W
models that include some adjustment to the analytic
solution of the kinematic-wave equations, such as
numerical dispersion, shock fitting, or adjustments to
the bed slope. Many modified models may produce
a=
much more adequate results.
^f
'
.

SPECIAL TOPICS AND APPLICATIONS

Three approaches to water routing in which the limitations of kinematic-wave approximations are somewhat relaxed will be presented in this section. These
modified kinematic-wave models include the .Storm
Water Management Model solution, the MuskingumCunge solution, and an implicit four-point finitedifference method.
In the development of the Storm Water Management
Model (U.S. Environmental Protection Agency, 1971),
it was decided to adjust the friction slope to more adequately describe flow in storm pipes at slopes less than
0.001. This was done by including terms II and III in
addition to term IV in equation 78 to adjust the value

Q
^i

Af = area of flow at pipe-full conditions,
and
Qf = discharge at pipe-full conditions, computed
from equations 85 and 86.
Combining equation 85 and 86 results in a nonlinear
equation that must be solved using an iterative method
(the Newton-Raphson method is suggested in the model
documentation). The finite-difference form of equations
85 and 86 is

21

SPECIAL TOPICS AND APPLICATIONS

method with variable parameters is presented here as
described by Ponce and Yevjevich (1978). The basic
equation is

2gAx

(88)

In the computation scheme, equation 88 is used to
compute Qf based on flow conditions at the previous
time step. Then equation 87 is solved iteratively. When
this scheme was tested, it was found that an iterative
procedure to correct values of velocity and depth was
needed to prevent oscillations in the solution. Finally,
an average value of Q was used based on the last two
iterations, in a total of four, applied to each point. In
the above formulation, both Wx and Wt were set equal to
0.55 because it was found that this value provided the
best hydrograph peak attenuation and insured stability
of the solution. Improved results for pipes with small
slopes were obtained with this scheme. Li and others
(1976) have presented a similar scheme in which all
terms in equation 78 are used to adjust the friction
slope. The two schemes described in the Storm Water
Management Model and by Li and others are not simply
dynamic- or steady dynamic-wave models as the number of terms included in equation 78 would suggest.
Unlike the conventional solution of routing equations
where the continuity equation and equation of motion
are solved simultaneously, these methods use the additional terms simply to adjust the friction slope used in
the kinematic-wave approximation. Some problems
have been noted in the application of methods in which
Sf is adjusted. On the backface of the hydrograph, the
additional terms can reduce Sf to such an extent that
an unreasonable flow is predicted. Therefore, when Sf
is predicted to be less than S0, the solution generally is
returned to the kinematic-wave solution for these parts
of the hydrograph.
A modified Muskingum method which attempts to
quantify the numerical dispersion which results from
the fact that the Muskingum method is a finitedifference approximation to the kinematic-wave equations was presented by Gunge (1969). Gunge showed
that the determination of the term D=-

in the
C Ax
method sets a weighting factor in the finite-difference
approximation so that the resulting numerical dispersion approximates the dispersion predicted by the
diffusion-wave model. The parameter, D, determines
the weighting given to the two differences in time, one
at the upstream point and one at a distance Ax down30
stream, to approximate the value of 57- in equation
at
41. The larger the value of D, the greater the amount of
dispersion in the solution. The Muskingum-Gunge

(89)
where

rl ~
_ l+C-D
1+C+D '

(90)

-1+C+D
1+C+D '

(91)

l-C+D
l+C+D '

(92)

C= c At
- =Courant number,

(93)

and

D=

S0 cAx

(94)

where c and q are the wave celerity and unit width discharge. These values need to be determined for each
computation step. Usually, At is fixed and Ax and S0 are
specified for each channel reach. The values of c and q
are determined as follows:
-dQ

(95)

and
(96)
According to the authors, Qt+u+i is computed by iteration. After an initial gues for Q,+1 J+1 has been determined by a three-point average of values at grid points
G»/)» (*+!>./)» and (i,j+l\ values for c and q are determined using a four-point average of values of Q and A
at grid points (i, j), (i+l,j), (i, j+D, and (i+lj+l). This
iteration is continued until values of c and q have been
converged upon. The authors state that this method
was shown to be sufficiently accurate for the simulation of flood flows.
A modified kinematic-wave model using an implicit
four-point finite-difference method is presented by
Alley and Smith (1982, p. 12-15) in a documentation of
the recently modified, distributed routing rainfallrunoff model. The scheme is similar to the Muskingum-

22

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

Cunge method in that it attempts to approximate a tent with the notation used in the remainder of this
diffusion-wave model by adjusting the numerical dis- report. It is used here to be consistent with previous
persion resulting from the solution technique. The documentations of this method such as Dawdy and
method requires an iterative procedure to solve for the others (1978). The relationship between the two notaunknown flow area.
tions is as follows: a denotes the location i, j; b denotes
In the implicit method, equation 6 is represented by a the location i+l,j; c denotes the location (j+1; and d
finite-difference equation using quantities at all four denotes the location i+1, j+l. The equation can be
corners of the computational box and a weighting fac- written as
tor for the space derivative. Applying the finitedifference method requires that each model segment be
subdivided into distance intervals. A distance interval,
Ax
Ax, and a time interval, At, form the computational box
for the finite-difference method. The value of Ax varies
.
(97)
for segment to segment, but the value of At is constant
2A*
for all segments.
Four points of a computational box (part of the finite- where Wx is the spatial derivative weighting factor that
difference grid in the x t plane) are represented in is assigned a value between 0.5 and 1.0 by the user.
figure 9. The purpose of this finite-difference method is The amount of numerical dispersion to be included in
to solve A and Q at point d, given values of A and Q at the solution is controlled by Wx. A value of Wx near 0.5
points a, b, and c. This notation for location of points in will include very little numerical dispersion, resulting
the finite-difference grid in the x t plane is inconsis- in a solution similar to that of the kinematic-wave

f + Af

x - Ax
FIGURE 9. Four-point finite-difference grid.

23

REFERENCES

equations. A value of Wx near 1.0 will result in a significant amount of dispersion. In this method, the user
can control the amount of dispersion in the solution.
Whereas no technique for predicting the correct value
for Wx is given, the value of Wx can be determined by
calibration or it can be determined based on the suggestions provided in the documentation of the modified
distributed routing rainfall-runoff model (Alley and
Smith, 1982, p. 33).
Equation 97 has two unknowns, Qd and Ad, which are
related by equation 40. By substituting Qd=o(Ad)m into
equation 96, the resulting equation is nonlinear with
one unknown, Ad, and can be rearranged into the
following form:
(98)
where
(99)
0.5 AJC

(100)

overbank storage is considered, the errors resulting
from application of the kinematic-wave model may be
masked by the overall computational strategy. This is
especially true when the routing distances involved
are short. However, significant problems occur where
the theory is applied to channel routing. In particular,
the cumulative error caused by neglecting the secondary slope terms in the equation of motion has been
shown.
Li and others (1976) concluded that the applicability
of the kinematic-wave model to river flood routing is
very limited because of shock formation. However, it
appears that the model is limited in application to
channel problems even before kinematic shock occurs.
The kinematic-wave model always predicts a steeper
wave with less dispersion and attenuation than actually occurs. This only applies, however, to waves that
have front faces. The approximations made in the
development of the kinematic-wave equations are not
generally justified for most channel-routing applications. However, many modified kinematic-wave
models can provide adequate results.
REFERENCES

and

(101)
The above nonlinear equation for Ad is solved with an
iterative procedure using Newton's second order
method for finding the roots of an equation. The procedure converges rapidly to a correct solution if a good
first estimate is made for the unknown area. To speed
convergence, the first estimate is obtained by using a
modification of the explicit method described under
"Supplemental Information" (Dawdy and others,
1978).
Because of errors involved in applying the kinematicwave model to channel problems, a modified scheme,
such as one of the three described here, should be used
instead. These schemes will help stop the accumulation of errors that occur when the kinematic-wave
model is applied.
SUMMARY AND CONCLUSIONS

Kinematic-wave theory is applicable to overland flow
where lateral inflow is continuously added. Where
relatively large lateral inflows are involved such as
overland-flow problems, and where relatively large

Abbott, M. B., 1975, Method of characteristics, in Mahmood, K., and
Yevjevich, V., eds., Unsteady flow in open channels: Water
Resources Publications, p. 63-88.
Alley, W. M., and Smith, P. E., 1982, Distributed routing rainfallrunoff model Version II: U.S. Geological Survey Open-File
Report 82-344, 201 p.
Ames, W. P., 1977, Numerical methods for partial-differential equations: New York, Academic Press, 365 p.
Bakhmeteff, B. A., 1932, Hydraulics of open channels: New York,
McGraw-Hill, 329 p.
Borah, D. K., Prasad, S. N., and Alonso, C. V., 1980, Kinematic wave
routing incorporating shock fitting: Water Resources Research,
v. 16, no. 3, p. 529-541.
Brakensiek, D. L., 1967, Kinematic flood routing: Transactions of
the American Society of Agricultural Engineers, v. 10, no. 3,
p. 340-343.
Chow, V. T., 1959, Open-channel hydraulics: New York, McGrawHill, 680 p.
Crandall, S. H., 1956, Engineering analysis: New York, McGrawHill, 417 p.
Cunge, J. A., 1969, On the subject of a flood propagation computation
method (Muskingum method): Journal Hydraulic Research, v. 7,
no. 2, p. 205-230.
Cunge, J. A. and Woolhiser, D. A., 1975, Irrigation systems, in Mahmood, K. and Yevjevich, V. eds., Unsteady Flow in open channels: Water Resources Publications, p. 509-537.
Dawdy, R. D., Schaake, J. C., and Alley, W. M., 1978, User's guide for
distributed routing rainfall-runoff model: U.S. Geological Survey
Water-Resources Investigations 78-79,146 p.
Eagleson, P. S., 1970, Dynamic hydrology: New York, McGraw-Hill,
462 p.
Gilcrest, B. R., 1950, Flood routing, in Rouse, H., ed., 1949, Engineering Hydraulics: Proceedings of the Fourth Hydraulics Conference, Iowa Institute of Hydraulic Research, chapter 10.

24

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

Gunaratnam, D. J., and Perkins, F. E., 1970, Numerical solution of
unsteady flows in open channels: Massachusetts Institute of
Technology, Department of Civil Engineering, Hydrodynamics
Laboratory Report No. 127, 216 p.
Henderson, F. M., 1963, Flood waves in prismatic channels: Journal
of the Hydraulics Division of the American Society of Civil
Engineers, v. 89, HY 4, p. 39-67.
___1966, Open-channel flow: New York, McMillan, 522 p.
Henderson, F. M., and Wooding, R. A., 1964, Overland flow and
ground-water flow from a steady rainfall of finite duration: Journal of Geophysical Research, v. 69, no. 8, p. 1531-1540.
Keefer, T. N., 1976, Comparison of linear systems and finite difference flow routing techniques: Water Resources Research, v. 12,
no. 5, p. 997-1006.
Kibler, D. F., and Woolhiser, D. A., 1970, The kinematic cascade as a
hydrologic model: Colorado State University, Hydrology Paper
No. 39, 28 p.
Land, L. F., 1978, Unsteady streamflow simulation using a linear
implicit finite-difference model: U.S. Geological Survey WaterResource Investigations 78-59, 59 p.
Li, R. M., Simons, D. B., and Stevens, M. A., 1975, Nonlinear kinematic wave approximation for water routing: Water Resources
Research, v. 11, no. 2, p. 245-252.
Li, R. M., Simons, D. B., Shiao, L. S., and Chen, Y. H., 1976, Kinematic wave approximation for flood routing: Rivers 76; Third
Annual Symposium on Inland Waterways for Navigation, Flood
Control, and Water Diversion; Colorado State University, v. 1,
p. 377-398.
Liggett, J. A., 1975, Basic equations of unsteady flow, in Mahmood,
K., and Yevjevich, V., eds., Unsteady flow in open channels:
Water Resources Publications, p. 29-62.
Liggett, J. A., and Cunge, J. A., 1975, Numerical methods of solution
of the unsteady flow equations, in Mahmood, K., and Yevjevich,
V., eds., Unsteady flow in open channels: Water Resources Publications, p. 89-180.
Lighthill, M. J., and Whitham, G. B., 1955, On kinematic waves, I.
Flood movement in long rivers: Proceedings of the Royal Society,
Series A, v. 229, p. 281-316.
Miller, W. A., and Cunge, J. A., 1975, Simplified equations of
unsteady flow, in Mahmood, K., and Yevjevich, V., eds., Unsteady
flow in open channels: Water Resources Publications, p. 183-249.
O'Brien, G. G., Hyman, M. A., and Kaplan, S., 1950, A study of the
numerical solution of partial-differential equations: Journal of
Mathematics and Physics, v. 29, no. 4, p. 223-251.
Ponce, V. M., Li, R. M., and Simons, D. B., 1978, Applicability of
kinematic and diffusion models: Journal of the Hydraulic Division of the American Society of Civil Engineers, v. 104, HY 3,
p. 353-360.
Ponce, V. M., and Simons, D. B., 1977, Shallow wave propagation in
open-channel flow: Journal of the Hydraulics Division of the
American Society of Civil Engineers, v. 103, HY 12, p. 1461-1476.
Ponce, V. M., and Yevjevich, V., 1978, Muskingum-Cunge method
with variable parameters: Technical Note, Journal of the
Hydraulics Division of the American Society of Civil Engineers,
v. 104, HY 12, p. 1663-1667.
Roache, P. J., 1976, Computational fluid dynamics: Albuquerque, N.
Mex., Hermosa Publishers, 446 p.
Schaake, J. C., 1965, Synthesis of inlet hydrograph: Johns Hopkins
University, Ph. D. Dissertation.
___1970, Modeling urban runoff as a deterministic process, in
Treatise on Urban Water Systems: Proceedings, Institute on Urban Water Systems, Fort Collins, Colo., chap. VI-A p. 343-383.
Theurer, F. D., 1975, A solution for unsteady open-channel flow:
Colorado State University, Ph. D. Dissertation, 127 p.
U.S. Environmental Protection Agency, 1971, Storm water manage-

ment model: Water Pollution Control Research Series 11024
DOC 07/71, v. 1-Final Report, p. 113-135.
Viessman, W., Knapp, J. W., Lewis, G. L., and Harbaugh, T. E., 1977,
Introduction to hydrology: New York, Harper and Row, 704 p.
Weinmann, P. E., and Laurenson, E. M., 1979, Approximate flood
routing methods: a review: Journal of the Hydraulics Division of
the American Society of Civil Engineers, v. 105, HY 12, p. 15211536.
Woolhiser, D. A., and Liggett, J. A., 1967, Unsteady, one-dimensional
flow over a plane the rising hydrograph: Water Resources
Research, v. 3, no. 3, p. 753-771.
Yen, B. C., 1973, Open-channel flow equations revisited: Journal of
the Engineering Mechanics Division of the American Society of
Civil Engineers, v. 99, EMS, p. 979-1009.
___1979, Unsteady flow mathematical modeling techniques, in
Shen, H. W., ed., Modeling of rivers: New York, John Wiley,
p. 13-1 to 13-33.
Yevjevich, Vujica, 1975, Introduction, in Mahmood, K., and Yevjevich,
V., eds., Unsteady flow in open channels: Water Resources Publications, p. 1-27.

SUPPLEMENTAL INFORMATION
EQUATION DEVELOPMENT CONCEPTS

The development of the continuity equation and the
equation of motion requires the use of some basic concepts. These concepts include the relationship between
momentum and energy, the different coordinate systems that can be used, and some friction-slope approximations.
MOMENTUM VERSUS ENERGY

The development of the equation of motion can be
based on Newton's second law (eq. 1) directly, or it can
be based on the conservation of energy or of momentum.
The relationship between equation 1, the concept of
conservation of energy, and the concept of conservation
of momentum is as follows. If both sides of equation 1
are multiplied by a length parallel to the direction of
the force and the acceleration, the energy equation is
obtained. It states that the work done as a body moves
a given length is equal to the kinetic energy acquired
by that body. If both sides of equation 1 are multiplied
by an elapsed time period, the momentum equation is
obtained. It states that the impulse, or force multiplied
by time, applied to a body is equal to the momentum, or
mass multiplied by velocity (mass times acceleration
multiplied by time equals mass times velocity), acquired by it.
The equation of motion is often called the momentum
equation. However, because it can be derived based on
conservation of momentum or energy, it is not clearly a
momentum equation. Chow (1959, p. 50-52) discussed
the relationship between the energy and momentum
equations for steady, gradually varied flow. He showed

25

SUPPLEMENTAL INFORMATION

that the two equations can be practically the same
when applied to certain flow problems. However, the
equations use different velocity distribution coefficients, although they are nearly equal, and the equations involve different meanings of the frictional
losses. In the energy equation, the frictional losses are
determined by the internal energy dissipated in the
whole mass of water in the reach. In the momentum
equation, the frictional losses are determined by the
losses due to external forces exerted on the water by
the walls of the channel. Yen (1973) showed the relationship between the one-dimensional equation of motion described in this report (Saint Venant equation)
and the general momentum and energy equations for
open-channel flow. Yen derived the general equations
from the Navier-Stokes equations. He stated that the
Saint Venant equation is a simplification and approximation of the general equations. However, he also
stated that there is no simple way to reduce the general energy equation into the Saint Venant form without making serious and questionable assumptions.
Therefore, the equation of motion is probably more
clearly a momentum equation than an energy equation. To avoid confusion on this point, the term "equation of motion" is used in this report to describe the
Saint Venant equation.

stress can be expressed using the well-known Chezy
formula. This approach is used because no similar relationship exists for unsteady flow.
The Chezy formula is

COORDINATE SYSTEMS

Rearranging and assuming that 6 is small so that
cos 6 si,

The continuity equation and the equation of motion
can be developed using either "gravity oriented" or
"natural" coordinate systems. In the gravity-oriented
system the x-axis is along the horizontal direction of
the channel, and in the natural system the x-axis is
along the longitudinal direction of the channel (at an
angle to the horizontal equal to the bed slope). Both
systems are orthogonal and are equally acceptable.
Some authors have used nonorthogonal systems, for
example with the x-axis horizontal and depth normal
to the channel bed instead of vertical. However, nonorthogonal coordinate systems can result in flow equations that are rather complicated and less suitable for
practical purposes (Yen, 1979, p. 13-3).

u=C

(102)

This equation was developed empirically for steady
uniform flow. However, a relationship for Sf is developed for application to the unsteady nonuniform flow
case by simply rearranging equation 101 as follows:
(25)

C?R '

The acceleration term used in the development of the
equation of motion is given in equation 16. However, for
the steady flow situation the derivative with respect to
time in the acceleration term is zero so that:
a=v dv
dx'

(103)

Substituting equations 12, 14, and 103 into equation 1,
(104)

Ah

(105)

Substituting R=-p and taking the limit as Ax 0 so that
Ah
Ax

dh
and noting that T=@g,
dx
V dv dh.

(106)

Substituting equation 24 into the previous equation,
T0=VRSf .

(107)

FRICTION-SLOPE APPROXIMATIONS

Rearranging, a second relationship for Sf is obtained.
Two friction-slope relationships, which are derived
from steady uniform and steady nonuniform flow
assumptions, are required for the development of the
equation of motion. Because these relationships are
not developed based on the more general unsteady nonuniform flow assumptions, they are approximations.
However, they are used in the unsteady flow equations
so that the friciton slope and the longitudinal shear

S=

'

(108)

Setting equation 108 equal to equation 25,

(109)

26

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

and, rearranging, r0 is expressed using the Chezy formula:

can be determined from a log-log plot of Q and A as
shown in figure 10. For the example data shown in
figure 10, a=1.20 and

(26)

log 9.5-log 2.5 _
m~ log 5.0-log 1.8 ~

(110)

The parameters a and m are used to define the relationship between flow discharge, Q, and flow area, A,
in the kinematic-wave routing scheme. The following
equation is used:

If enough channel data are available to define the
relationship between wetted perimeter, P, and flow
area, A, then the following procedure can be used to
estimate a and m. A log-log plot of P and A, similar to
that shown in figure 10, can be used to define a and b in
the following equation:

Q=oAm.

P=oA».

METHODS FOR ESTIMATING a AND m

(40)

The log form of equation 40 is log Q=loga+m logA.
This is a linear relation in which the intercept is
loga=logQ when A=l, and m is the slope of the relation. If cross-section properties and stage versus discharge data are known for a given channel, a and m

100

I

I

I

I

I

I

(Ill)

Then equations for estimating a and m can be devised,
following the steps below:
A

A

(112)

P a A*'

I M

o
o
UJ
CO
QC
UJ
tL

111
UJ
LL
O
00

D
U

I

I

I

I

I

I

I

100
FLOW AREA. IN SQUARE FEET
FIGURE 10. Use of log-log plot of discharge, Q, and flow area, A, to define the routing parameters a and m.

27

SUPPLEMENTAL INFORMATION

where #=hydraulic radius. Substituting into Manning's formula,

This is solved simultaneously with the continuity
equation as follows (from eq 8):

(113)

(119)

-r

aAb

and, rearranging,
(114)

In this development, the Chezy formula will be used to
describe the friction slope. Therefore,
(120)

From equation 114 it can be seen that a and m in equation 40 are
a=-1.49
no

(115)

and, for wide channels, As By and R=y so that,
following equation 120, Q=CBym S^ or v=
Substituting equation 118 into 120,
(121)

and
m=5/3-2/36.

(116)

The distributed routing rainfall-runoff model (Dawdy
and others, 1978, p. 14) provides some approximate
relations for determining a and m for channel shapes
that can be described as rectangular, trapezoidal, circular, or triangular cross sections. These approximations are given in table 1.

for wide channels,
(122)
Differentiating with respect to x, equation 122 becomes

DEVELOPMENT OF DIFFUSION-ANALOGY MODEL

The diffusion-analogy model is based on the diffusionwave approximation of the dynamic-wave model and
the continuity equation. From equation 78 the form of
the equation of motion used in the diffusion-wave
model is

(123)
Substituting into the continuity equation (eq. 119),

(117)
(124)
or
Recalling that S^Sb *- and v=C(y S/P and rearrang(118)

ing,

TABLE 1. Relations for estimating a and m based on physical characteristics of channel segments
Definition of channel parameter Pc

Type of segment

Wide rectangular or trapezoidal
Circular pipe

1.49
n

1.67

Width of conduit at about mean
depth of flow.

1.0

Diameter of pipe.

1.33

Width at 1-foot depth.

BASIC CONCEPTS OF KINEMATIC-WAVE MODELS

28

dy_

(125)

plified to ordinary differential equations describing
only variation with respect to x:

Multiplying by S^ISf1*,

(129)

dx
(126)

and
(130)

or, again substituting v=
(127)
It has been shown that for kinematic waves, when using the Chezy formula, c=3/2u, and choosing the diffusion coefficient to be
(128)

Equation 129 simply states that the discharge does not
vary with distance. This is true within each computation of the step-backwater procedure. The remainder of
the development is concerned with equation 130.
Recalling that
dt/2
..,«<
dx

s <

At

_n
dv
" "^l/ *
At

(131)

the first term in equation 130 can be rearranged as
follows:

equation 127 becomes
(79)

dt

This equation is the diffusion equation and in the application to flow routing is called the diffusion analogy.
A diffusion equation with discharge instead of depth
has as the dependent variable also can be developed
based on some additional assumptions.
DEVELOPMENT OF STEP-BACKWATER FROM
DYNAMIC-WAVE MODEL

The development of the step-backwater procedure is
given to show that it is one of the many approximations of the dynamic-wave model. The step-backwater
procedure is the standard-step method of computing
steady, gradually varied flow as described by Chow
(1959, p. 265-268). The dynamic-wave model includes
the continuity equation and the equation of motion as
follows:
(119)

v dv_ 1 dt/2 _
g dx 2g dx
dx
Therefore, equation 130 becomes

(133)
Approximating the differentials with a simple finite
difference in which the subscript 1 refers to a channel
cross section at some distance x and subscript 2 refers
to a section a distance Ax downstream,

(134)
Multiplying by Ax and rearranging,

. (135)
The friction slope can be written as

and
dy _v_ dv _1_ dv
dx g dx g dt'

Ax '
(36)

Following the assumption of steady flow, all terms
describing the variation with respect to time are zero.
Thus, equations 119 and 36 can be rearranged and sim-

(132)

(136)

where Af =friction loss.
The bed slope can be written as
Ax

Ax

(137)

29

SUPPLEMENTAL INFORMATION

Substituting equations 136 and 137 into equation 135,
(138)
Rearranging,
(139)
Equation 130 is the basis of the step-backwater procedure. Usually, an estimate of the velocity distribution coefficient at both sections and an estimate of the
eddy losses is included in a step-backwater procedure.
However, they are missing here because they were ignored in the development of the unsteady flow model
and are often ignored in practical applications of unsteady flow models.
KINEMATIC-WAVE MODEL USED IN THE DISTRIBUTED
ROUTING RAINFALL-RUNOFF MODEL

The kinematic-wave equations are solved using a
finite-difference approximation. The explicit scheme
used is generally stable for any channel segment
length, x, and any time step, t. There are some exceptions to this when there is a large inflow at the initial
time.
The continuity equation to be solved using a finitedifference scheme is
dA
dt

q'
nr

dx

n
At -r£
Ob = kinematic wave speed
8=m-r*Ax Ab
Ax/At

(40)

The model computes a and m for channel shapes that
can be approximated by rectangular, trapezoidal, circular, or triangular cross sections. These approximations are given in table 1.
A rectangular grid of points is used to approximate Q
and A at the downstream end of the channel using Q
and A at the upstream end of the channel to drive the
computations. The notation used here is consistent with
previous documentations of this method as described
under "Special Topics and Applications". Figure 9
shows the four-point finite-difference grid usod to solve
for Q and A at point d the downstream end of the

/nm
(140)

The parameter, 6, determines whether the characteristic path passing through point d in figure 9 passes
above or below point a. If 0 is greater than or equal to
unity, the water wave has traveled through the entire
reach, Ax, during At and the finite-difference equations
are written across time between points a and c. The
continuity equation is approximated as
Aa "iA

T~.

Ax

=q.

(141)

Rearranging, the equations are
Ax

(142)

and
(143)

(59)

The relationship between A and Q is approximated by
Q=aAm.

channel segment at the next time step. Points a and b
are the upstream and downstream ends of the channel
segment at the present time step and point c is the upstream end of the channel at the next time step. Q and
A are known at points a, 6, and c at every time step.
In an attempt to maintain a generally stable solution, the model contains two different finite-difference
equations and selects the appropriate one at each point
in the solution. The decision depends on the parameter.

This involves only grid points a, c, and d. If 6 is less
than unity, the water has traveled a distance less than
the reach length, Ax, during Aland the finite-difference
equations are written across space between points a
and b. The continuity equation is approximated as
Ad Ab
At

Ax

-=q.

(144)

Rearranging, the equation are
At,

(145)

and
(146)

