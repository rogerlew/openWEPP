9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:07.911065
# Source Data: Surgo
#
# Mukey: 62598
# Major Component: 27032630 (comppct_r = 85.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80723564   Oi     X        3.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80723565   H1             28.0     9.0        0.0         3.0          1.1    31.0    35.4    10.6     5.5
# 80723566   H2             84.0     9.0        0.0         5.0          1.2    30.0    33.5    10.0    1.75
# 80723567   H3            152.0    92.0        0.0        28.0          1.3     2.5    81.1    15.6    0.35
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
# 80723565::wilt_pt estimated from wfifteenbar_r and rock
# 80723565::field_cap estimated from wthirdbar_r and rock
# 80723566::wilt_pt estimated from wfifteenbar_r and rock
# 80723566::field_cap estimated from wthirdbar_r and rock
# 80723567::wilt_pt estimated from wfifteenbar_r and rock
# 80723567::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-04-28 19:18:29.246759
# Source File: soils/62598.sol
#
# Replacements
# --------------------------
# luse -> forest moderate sev fire
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
0	 'forest moderate sev fire'	 'clay loam'	 1.3 	 0.3
'Saturn clay loam'	 'CL'	 4	 0.23	 0.75	 1500000	 5e-05	 0.5
	200.0	 1.1	 18	 10.0	 0.3582	 0.2144	 35.4	 31.0	 5.5	 25.0	 15.12	 0.1081	 0.5047	 0.006966	 1.384	 51.33	 0.1747	 0.3006
	280.0	 1.1	 32.4	 10.0	 0.3582	 0.2144	 35.4	 31.0	 5.5	 25.0	 15.12	 0.1081	 0.5047	 0.006966	 1.384	 51.33	 0.1747	 0.3006
	840.0	 1.2	 32.4	 1.0	 0.3805	 0.2075	 33.5	 30.0	 1.75	 17.5	 33.5	 0.1051	 0.4752	 0.006704	 1.388	 31.2	 0.1669	 0.2912
	1600.0	 1.3	 331.2	 1.0	 0.076	 0.016	 81.1	 2.5	 0.35	 2.5	 74.8	 0.05523	 0.4138	 0.02077	 1.803	 245.7	 0.05881	 0.1312
1 10000.0 0.01
