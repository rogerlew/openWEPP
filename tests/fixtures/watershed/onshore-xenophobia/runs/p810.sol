9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:08.741437
# Source Data: Surgo
#
# Mukey: 62717
# Major Component: 27032846 (comppct_r = 85.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80724154   Oi     X        5.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80724155   H1             20.0     9.0        0.0        23.0         0.68    31.0    27.1    17.4    10.0
# 80724156   H2             53.0     9.0        0.0         3.0         0.68    30.0    26.4    16.9     5.0
# 80724157   H3             94.0     9.0        0.0        20.0         0.83    27.5    31.1    23.1     2.0
# 80724158   H4            119.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 80724155::wilt_pt estimated from wfifteenbar_r and rock
# 80724155::field_cap estimated from wthirdbar_r and rock
# 80724156::wilt_pt estimated from wfifteenbar_r and rock
# 80724156::field_cap estimated from wthirdbar_r and rock
# 80724157::wilt_pt estimated from wfifteenbar_r and rock
# 80724157::field_cap estimated from wthirdbar_r and rock
# 80724158::using default rock content of 55.5%
# 80724158::ksat_r estimated from rosetta2
# 80724158::wilt_pt estimated from rosetta2
# 80724158::field_cap estimated from rosetta2
# 80724158::bd estimated from sand, vfs, and clay
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
# Build Date: 2026-04-28 19:18:26.648680
# Source File: soils/62717.sol
#
# Replacements
# --------------------------
# luse -> forest low sev fire
# stext -> clay loam
# ki -> 1500000
# kr -> 5.00E-05
# shcrit -> 0.5
# avke -> 18
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
0	 'forest low sev fire'	 'clay loam'	 1.3 	 0.3
'Keel cobbly clay loam, 25 to 45 percent slopes'	 'CB-CL'	 4	 0.23	 0.75	 1500000	 5e-05	 0.5
	200.0	 0.68	 18	 10.0	 0.414	 0.2062	 27.1	 31.0	 10.0	 50.0	 38.4	 0.1208	 0.6283	 0.004916	 1.403	 319.3	 0.2105	 0.3241
	530.0	 0.68	 32.4	 1.0	 0.299	 0.1271	 26.4	 30.0	 5.0	 42.5	 12.7	 0.1194	 0.6264	 0.004673	 1.412	 326.3	 0.2074	 0.3186
	940.0	 0.83	 32.4	 1.0	 0.3439	 0.1379	 31.1	 27.5	 2.0	 40.0	 34.0	 0.1093	 0.5719	 0.004956	 1.424	 181.8	 0.1837	 0.2968
	1200.0	 1.52	 59.8734	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
