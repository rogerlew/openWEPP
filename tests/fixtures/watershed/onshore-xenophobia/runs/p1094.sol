9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:09.416499
# Source Data: Surgo
#
# Mukey: 62731
# Major Component: 27032861 (comppct_r = 85.0)
# Texture: loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80724222   Oi     X        5.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80724223   H1             38.0     9.0       15.0         8.0          1.3    23.5    39.2    11.6     4.5
# 80724224   H2            104.0     9.0        0.0        38.0          1.3    30.0    33.5    10.0    1.75
# 80724225   H3            132.0     9.0        0.0        43.0          1.3    23.5    39.2    11.6    0.25
# 80724226   H4            142.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
#
# Restricting Layer:
# ksat threshold: 2.00000
# type: -
# ksat: -
#
# defaults applied to missing chorizon data:
# sandtotal_r  ->      66.800
# claytotal_r  ->       7.000
# om_r         ->       7.000
# cec7_r       ->      11.300
# sandvf_r     ->      10.000
# smr          ->      55.500
#
# Build Notes:
# 80724223::wilt_pt estimated from wfifteenbar_r and rock
# 80724223::field_cap estimated from wthirdbar_r and rock
# 80724224::wilt_pt estimated from wfifteenbar_r and rock
# 80724224::field_cap estimated from wthirdbar_r and rock
# 80724225::wilt_pt estimated from wfifteenbar_r and rock
# 80724225::field_cap estimated from wthirdbar_r and rock
# 80724226::using default rock content of 55.5%
# 80724226::ksat_r estimated from rosetta2
# 80724226::wilt_pt estimated from rosetta2
# 80724226::field_cap estimated from rosetta2
# 80724226::bd estimated from sand, vfs, and clay
# res_lyr_i None
#
# THIS FILE AND THE CONTAINED DATA IS PROVIDED BY THE UNIVERSITY OF IDAHO
# 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
# TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
# PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL UNIVERSITY OF IDAHO
# BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHERE IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS FILE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
#
#
# If you change the original contexts of this file please
# indicate it by putting an 'X' in the box here -> [ ]
#
#
#
# Source soil utility: 9002.0migration
# Build Date: 2026-04-28 19:18:11.836313
# Source File: soils/62731.sol
#
# Replacements
# --------------------------
# luse -> forest low sev fire
# stext -> loam
# ki -> 1000000
# kr -> 8.00E-05
# shcrit -> 1
# avke -> 20
# bd ->
# ksflag -> 0
# ksatadj -> 0
# ksatfac -> 1.3
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 4
# keffflag -> 1
# lkeff -> 10
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
0	 'forest low sev fire'	 'loam'	 1.3 	 0.3
'Klickitat stony loam, 50 to 75 percent south slopes'	 'ST-L'	 5	 0.16	 0.75	 1000000	 8e-05	 1
	200.0	 1.3	 20	 10.0	 0.4	 0.224	 39.2	 23.5	 4.5	 20.0	 53.8	 0.0936	 0.4348	 0.007059	 1.408	 22.71	 0.1446	 0.265
	380.0	 1.3	 32.4	 10.0	 0.4	 0.224	 39.2	 23.5	 4.5	 20.0	 53.8	 0.0936	 0.4348	 0.007059	 1.408	 22.71	 0.1446	 0.265
	1040.0	 1.3	 32.4	 1.0	 0.294	 0.168	 33.5	 30.0	 1.75	 17.5	 70.55	 0.1039	 0.4491	 0.006992	 1.377	 19.07	 0.1636	 0.2878
	1320.0	 1.3	 32.4	 1.0	 0.23	 0.104	 39.2	 23.5	 0.25	 12.5	 74.35	 0.0936	 0.4348	 0.007059	 1.408	 22.71	 0.1446	 0.265
	1600.0	 1.52	 59.8734	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
