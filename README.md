# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  76.600 us |     100 ns |  79.100 us | 155.800 us |
| 02  |  43.700 us |  18.200 us |  72.300 us | 134.200 us |
| 03  |  29.600 us |  11.800 us |  40.500 us |  81.900 us |
| 04  | 104.300 us |   6.500 us | 105.100 us | 215.900 us |
| 05  | 189.900 us |  18.400 us | 204.000 us | 412.300 us |
| 06  |       0 ns |  19.700 us |   6.700 us |  26.400 us |
| 07  |   1.035 ms |   2.900 us |   1.036 ms |   2.075 ms |
| 08  |  57.200 us |       0 ns | 338.500 us | 395.700 us |
| 09  |  87.900 us |       0 ns |   1.091 ms |   1.179 ms |
| 10  |  23.700 us |       0 ns |  46.800 us |  70.500 us |
| 11  |  31.400 us |  28.600 us |  16.570 ms |  16.630 ms |
| 12  |  48.700 us |   2.104 ms |   1.687 ms |   3.839 ms |
| 13  |   1.549 ms |  10.900 us |   1.604 ms |   3.164 ms |
| 14  | 226.800 us | 124.200 us |   4.473 ms |   4.824 ms |
| 15  | 679.800 us |     300 ns | 703.000 us |   1.383 ms |
| 16  | 905.700 us |  50.452 ms | 118.281 ms | 169.640 ms |
| 17  |       0 ns |  59.754 ms |  85.408 ms | 145.162 ms |
| 18  | 746.000 us | 517.800 us |   4.052 ms |   5.315 ms |
| 19  | 806.600 us |  22.445 ms |  21.612 ms |  44.864 ms |
| 20  | 208.100 us |   5.502 ms |  66.949 ms |  72.659 ms |
| 21  |   1.009 ms | 490.100 us |   1.926 ms |   3.426 ms |
| 22  | 114.600 us | 115.800 us | 216.600 us | 447.000 us |
| 23  | 167.000 us |       0 ns | 570.527 ms | 570.694 ms |
| 24  |     100 ns |     100 ns |     200 ns |     400 ns |
| 25  |     100 ns |     100 ns |     200 ns |     400 ns |
| sum |   8.142 ms | 141.623 ms | 897.035 ms |    1.047 s |

`-0.047s` remaining

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
AoC 2022 - Day22        time:   [210.07 µs 210.50 µs 210.98 µs]
AoC 2022 - Day23        time:   [587.06 ms 587.61 ms 588.15 ms]
AoC 2022 - Day24        time:   missing
AoC 2022 - Day25        time:   missing

AoC 2022 - All          time:   [1.0325 s 1.0339 s 1.0354 s]
```

`-0.0339 s` remaining