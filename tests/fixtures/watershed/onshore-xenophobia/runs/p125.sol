9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:07.899491
# Source Data: Surgo
#
# Mukey: 62627
# Major Component: 27032679 (comppct_r = 85.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80723715   Oe     X       13.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80723716   H1             36.0     9.0        0.0         0.0         1.15    33.5    34.2    10.3     4.5
# 80723717   H2            152.0     3.0        0.0         0.0          1.2    45.0    26.1     7.8     2.0
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
# 80723716::wilt_pt estimated from wfifteenbar_r and rock
# 80723716::field_cap estimated from wthirdbar_r and rock
# 80723717::wilt_pt estimated from wfifteenbar_r and rock
# 80723717::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-04-28 19:18:17.469548
# Source File: soils/62627.sol
#
# Replacements
# --------------------------
# luse -> forest high sev fire
# stext -> clay loam
# ki -> 1500000
# kr -> 6.00E-05
# shcrit -> 0.5
# avke -> 14
# bd ->
# ksflag -> 0
# ksatadj -> 1
# ksatfac -> 100
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 2
# keffflag -> 1
# lkeff -> 0.1
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
1	 'forest high sev fire'	 'clay loam'	 100 	 0.3
'Blachly clay loam, 30 to 50 percent slopes'	 'CL'	 3	 0.16	 0.75	 1500000	 6e-05	 0.5
	200.0	 1.15	 14	 10.0	 0.3374	 0.2092	 34.2	 33.5	 4.5	 17.5	 2.5	 0.1107	 0.4975	 0.007388	 1.368	 39.84	 0.1789	 0.3071
	360.0	 1.15	 32.4	 10.0	 0.3374	 0.2092	 34.2	 33.5	 4.5	 17.5	 2.5	 0.1107	 0.4975	 0.007388	 1.368	 39.84	 0.1789	 0.3071
	1600.0	 1.2	 10.8	 1.0	 0.4491	 0.2663	 26.1	 45.0	 2.0	 15.0	 12.5	 0.1239	 0.5094	 0.008381	 1.321	 29.8	 0.2054	 0.3396
1 10000.0 0.01
