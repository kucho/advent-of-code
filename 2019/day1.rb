def fuel_for_module(mass)
  fuel = (mass / 3).round(2) - 2
  fuel > 0 ? fuel + fuel_for_module(fuel) : 0
end

def fuel_ecuation(arr)
  arr.map { |mod| fuel_for_module(mod) }.reduce(0) { |sum, el| sum + el }
end

input = [66_910,
         78_957,
         58_510,
         142_350,
         105_820,
         87_317,
         100_743,
         51_390,
         92_804,
         80_752,
         70_169,
         111_892,
         104_715,
         143_166,
         126_158,
         78_712,
         139_223,
         133_863,
         85_912,
         53_883,
         64_812,
         102_254,
         52_482,
         91_855,
         117_520,
         140_253,
         76_706,
         106_693,
         57_948,
         57_578,
         115_640,
         131_273,
         115_373,
         145_219,
         100_889,
         106_447,
         72_347,
         120_250,
         56_898,
         146_689,
         138_246,
         85_068,
         55_292,
         124_814,
         136_750,
         51_820,
         70_396,
         92_806,
         86_093,
         70_467,
         122_356,
         148_530,
         85_569,
         100_492,
         87_062,
         123_225,
         73_872,
         102_104,
         91_194,
         95_077,
         88_352,
         114_906,
         141_056,
         87_220,
         106_517,
         88_867,
         95_883,
         130_158,
         76_702,
         134_241,
         50_561,
         119_258,
         61_669,
         140_396,
         145_201,
         76_914,
         102_281,
         56_618,
         145_968,
         99_542,
         116_789,
         135_633,
         114_646,
         84_253,
         50_650,
         69_127,
         95_446,
         55_357,
         81_180,
         126_940,
         133_743,
         52_261,
         117_429,
         82_291,
         110_373,
         67_626,
         58_014,
         125_342,
         129_508,
         96_332]

p fuel_ecuation(input)
