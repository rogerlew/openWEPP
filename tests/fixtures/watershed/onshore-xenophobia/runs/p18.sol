9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:08.297974
# Source Data: Surgo
#
# Mukey: 62711
# Major Component: 27032838 (comppct_r = 85.0)
# Texture: silt loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80724135   H1             36.0    28.0        0.0         0.0          0.9    14.0    14.2     8.6     5.5
# 80724136   H2            109.0    28.0        0.0         0.0         1.05    14.0    42.7    34.4     2.0
# 80724137   H3            152.0   300.0        0.0        28.0          1.3     2.5    96.0     7.2     0.6
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
# 80724135::wilt_pt estimated from wfifteenbar_r and rock
# 80724135::field_cap estimated from wthirdbar_r and rock
# 80724136::wilt_pt estimated from wfifteenbar_r and rock
# 80724136::field_cap estimated from wthirdbar_r and rock
# 80724137::wilt_pt estimated from wfifteenbar_r and rock
# 80724137::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-04-28 19:18:16.990513
# Source File: soils/62711.sol
#
# Replacements
# --------------------------
# luse -> forest moderate sev fire
# stext -> silt loam
# ki -> 1000000
# kr -> 0.0001
# shcrit -> 1.5
# avke -> 13
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
0	 'forest moderate sev fire'	 'silt loam'	 1.3 	 0.3
'Jimbo silt loam'	 'SIL'	 4	 0.16	 0.75	 1000000	 0.0001	 1.5
	200.0	 0.9	 13	 10.0	 0.2777	 0.1063	 14.2	 14.0	 5.5	 17.5	 12.5	 0.08804	 0.5321	 0.002655	 1.595	 179.1	 0.1376	 0.223
	360.0	 0.9	 100.8	 10.0	 0.2777	 0.1063	 14.2	 14.0	 5.5	 17.5	 12.5	 0.08804	 0.5321	 0.002655	 1.595	 179.1	 0.1376	 0.223
	1090.0	 1.05	 100.8	 1.0	 0.24	 0.08	 42.7	 14.0	 2.0	 10.0	 2.5	 0.08051	 0.4733	 0.005394	 1.497	 97.66	 0.1247	 0.2299
	1600.0	 1.3	 1080.0	 1.0	 0.104	 0.028	 96.0	 2.5	 0.6	 2.5	 55.0	 0.05466	 0.4274	 0.02978	 2.924	 1.079e+03	 0.05466	 0.06926
1 10000.0 0.01
