9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:08.238255
# Source Data: Surgo
#
# Mukey: 62631
# Major Component: 27032684 (comppct_r = 65.0)
# Texture: loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80723729   Oi     X        8.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80723730   H1             18.0    28.0       13.0         5.0          0.8    15.0    44.3    12.5     6.0
# 80723731   H2             38.0    92.0       38.0        38.0          0.8    10.0    66.6    11.9    2.25
# 80723732   H3             48.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 80723730::wilt_pt estimated from wfifteenbar_r and rock
# 80723730::field_cap estimated from wthirdbar_r and rock
# 80723731::wilt_pt estimated from wfifteenbar_r and rock
# 80723731::field_cap estimated from wthirdbar_r and rock
# 80723732::using default rock content of 55.5%
# 80723732::ksat_r estimated from rosetta2
# 80723732::wilt_pt estimated from rosetta2
# 80723732::field_cap estimated from rosetta2
# 80723732::bd estimated from sand, vfs, and clay
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
# Build Date: 2026-04-28 19:18:20.496409
# Source File: soils/62631.sol
#
# Replacements
# --------------------------
# luse -> forest moderate sev fire
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
# lkeff -> 1
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
0	 'forest moderate sev fire'	 'loam'	 1.3 	 0.3
'Yellowstone-Rock outcrop complex, 10 to 60 percent slopes'	 'ST-L'	 4	 0.23	 0.75	 1000000	 8e-05	 1
	180.0	 0.8	 20	 10.0	 0.3571	 0.1324	 44.3	 15.0	 6.0	 50.0	 42.6	 0.08944	 0.5453	 0.004918	 1.478	 253.5	 0.1478	 0.2518
	200.0	 0.8	 20	 10.0	 0.094	 0.032	 66.6	 10.0	 2.25	 50.0	 87.4	 0.08116	 0.5458	 0.009729	 1.449	 296.2	 0.1309	 0.2531
	380.0	 0.8	 331.2	 10.0	 0.094	 0.032	 66.6	 10.0	 2.25	 50.0	 87.4	 0.08116	 0.5458	 0.009729	 1.449	 296.2	 0.1309	 0.2531
	600.0	 1.52	 59.8734	 10.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
