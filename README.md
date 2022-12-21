# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  79.400 us |     100 ns |  81.700 us | 161.200 us |
| 02  |  26.700 us |  15.100 us |  39.600 us |  81.400 us |
| 03  |  16.700 us |  12.000 us |  27.700 us |  56.400 us |
| 04  |  97.300 us |   6.800 us |  98.100 us | 202.200 us |
| 05  | 173.500 us |  16.400 us | 187.500 us | 377.400 us |
| 06  |     100 ns |   4.800 us |   6.700 us |  11.600 us |
| 07  |   1.029 ms |     500 ns |   1.030 ms |   2.060 ms |
| 08  |  53.600 us |       0 ns | 327.500 us | 381.100 us |
| 09  |  55.200 us |       0 ns | 863.000 us | 918.200 us |
| 10  |  12.100 us |     100 ns |  30.800 us |  43.000 us |
| 11  |  28.200 us |  28.600 us |  16.423 ms |  16.480 ms |
| 12  |  57.700 us |   1.978 ms |   1.554 ms |   3.590 ms |
| 13  |   1.497 ms |  11.500 us |   1.554 ms |   3.062 ms |
| 14  | 200.400 us | 114.700 us |   4.227 ms |   4.542 ms |
| 15  | 503.300 us |     200 ns | 535.000 us |   1.038 ms |
| 16  | 982.600 us |  50.250 ms | 120.567 ms | 171.801 ms |
| 17  |     100 ns |  59.120 ms |  85.707 ms | 144.827 ms |
| 18  | 689.600 us | 448.500 us |   4.202 ms |   5.341 ms |
| 19  | 625.400 us |  22.158 ms |  21.520 ms |  44.304 ms |
| 20  | 145.700 us |   5.522 ms |  67.379 ms |  73.047 ms |
| 21  | 886.600 us | 545.700 us |   1.834 ms |   3.266 ms |
| sum |   7.161 ms | 140.235 ms | 328.199 ms | 475.596 ms |

`0.5244035s` remaining

## Criterion benches

```
AoC 2022 - Day01        time:   [29.263 µs 29.386 µs 29.515 µs]
AoC 2022 - Day02        time:   [13.947 µs 13.974 µs 14.002 µs]
AoC 2022 - Day03        time:   [24.872 µs 24.906 µs 24.946 µs]
AoC 2022 - Day04        time:   [75.316 µs 75.684 µs 76.084 µs]
AoC 2022 - Day05        time:   [129.66 µs 129.83 µs 130.01 µs]
AoC 2022 - Day06        time:   [9.4908 µs 9.4974 µs 9.5044 µs]
AoC 2022 - Day07        time:   [698.67 µs 699.67 µs 700.79 µs]
AoC 2022 - Day08        time:   [295.32 µs 296.20 µs 297.34 µs]
AoC 2022 - Day09        time:   [707.96 µs 712.51 µs 718.02 µs]
AoC 2022 - Day10        time:   [17.636 µs 17.657 µs 17.680 µs]
AoC 2022 - Day11        time:   [19.433 µs 19.478 µs 19.535 µs]
AoC 2022 - Day12        time:   [1.8280 ms 1.8317 ms 1.8356 ms]
AoC 2022 - Day13        time:   [1.4609 ms 1.4626 ms 1.4645 ms]
AoC 2022 - Day14        time:   [3.9756 ms 3.9793 ms 3.9829 ms]
AoC 2022 - Day15        time:   [49.464 µs 49.524 µs 49.586 µs]
AoC 2022 - Day16        time:   [168.37 ms 168.90 ms 169.46 ms]
AoC 2022 - Day17        time:   [143.80 ms 144.07 ms 144.35 ms]
AoC 2022 - Day18        time:   [4.1492 ms 4.1654 ms 4.1880 ms]
AoC 2022 - Day20        time:   [72.320 ms 72.483 ms 72.682 ms]
AoC 2022 - Day21        time:   [1.7750 ms 1.7797 ms 1.7850 ms]
AoC 2022 - Day22        time:   missing
AoC 2022 - Day23        time:   missing
AoC 2022 - Day24        time:   missing
AoC 2022 - Day25        time:   missing

AoC 2022 - All          time:   [462.94 ms 464.06 ms 465.22 ms]
```

`0.53594 s` remaining