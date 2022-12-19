# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  62.200 us |       0 ns |  63.100 us | 125.300 us |
| 02  |  24.500 us |  14.700 us |  36.900 us |  76.100 us |
| 03  |  21.200 us |  11.800 us |  32.200 us |  65.200 us |
| 04  |  81.300 us |   6.500 us |  82.100 us | 169.900 us |
| 05  | 169.300 us |  17.900 us | 183.900 us | 371.100 us |
| 06  |       0 ns |   3.000 us |   7.300 us |  10.300 us |
| 07  |   1.008 ms |     500 ns |   1.009 ms |   2.018 ms |
| 08  |  50.800 us |     100 ns | 338.400 us | 389.300 us |
| 09  |  73.800 us |       0 ns | 892.700 us | 966.500 us |
| 10  |   9.200 us |     100 ns |  25.700 us |  35.000 us |
| 11  |            |            |            |            |
| 12  |  46.400 us |   1.991 ms |   1.498 ms |   3.536 ms |
| 13  |   1.452 ms |  11.000 us |   1.509 ms |   2.973 ms |
| 14  | 257.300 us | 143.700 us |   4.109 ms |   4.510 ms |
| 15  | 508.100 us |     200 ns | 532.200 us |   1.040 ms |
| 16  | 965.100 us |  47.483 ms | 118.980 ms | 167.429 ms |
| 17  |     200 ns |  58.427 ms |  85.550 ms | 143.977 ms |
| 18  | 647.000 us | 562.600 us |   3.991 ms |   5.201 ms |
| 19  | 845.500 us |  36.712 ms |  35.224 ms |  72.782 ms |
| sum |   6.223 ms | 145.387 ms | 254.068 ms | 405.679 ms |

`0.594321s` remaining

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
AoC 2022 - Day11        time:   missing
AoC 2022 - Day12        time:   [1.8280 ms 1.8317 ms 1.8356 ms]
AoC 2022 - Day13        time:   [1.4609 ms 1.4626 ms 1.4645 ms]
AoC 2022 - Day14        time:   [3.9756 ms 3.9793 ms 3.9829 ms]
AoC 2022 - Day15        time:   [49.464 µs 49.524 µs 49.586 µs]
AoC 2022 - Day16        time:   [180.95 ms 181.64 ms 182.37 ms]
AoC 2022 - Day17        time:   [145.28 ms 145.53 ms 145.84 ms]
AoC 2022 - Day18        time:   [4.1492 ms 4.1654 ms 4.1880 ms]
AoC 2022 - Day19        time:   [66.158 ms 66.685 ms 67.235 ms]

AoC 2022 - All          time:   [397.15 ms 397.79 ms 398.45 ms]
```

`0.60221 s` remaining