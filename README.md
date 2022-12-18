# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  69.500 us |     100 ns |  70.500 us | 140.100 us |
| 02  |  24.500 us |  15.000 us |  36.700 us |  76.200 us |
| 03  |  21.000 us |  11.600 us |  31.800 us |  64.400 us |
| 04  |  92.200 us |   7.200 us |  93.000 us | 192.400 us |
| 05  | 164.800 us |  17.000 us | 180.100 us | 361.900 us |
| 06  |       0 ns |   3.100 us |   9.200 us |  12.300 us |
| 07  | 973.200 us |     500 ns | 974.100 us |   1.947 ms |
| 08  |  54.800 us |       0 ns | 324.400 us | 379.200 us |
| 09  |  57.200 us |       0 ns | 852.800 us | 910.000 us |
| 10  |  12.600 us |       0 ns |  29.000 us |  41.600 us |
| 11  |            |            |            |            |
| 12  |  47.300 us |   2.015 ms |   1.558 ms |   3.620 ms |
| 13  |   1.626 ms |  15.900 us |   1.711 ms |   3.353 ms |
| 14  | 191.800 us | 129.100 us |   3.829 ms |   4.150 ms |
| 15  | 472.100 us |     200 ns | 487.200 us | 959.500 us |
| 16  | 911.000 us |  79.597 ms | 119.067 ms | 199.575 ms |
| sum |   4.718 ms |  81.812 ms | 129.254 ms | 215.785 ms |

`0.78421426s` remaining

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
AoC 2022 - Day16        time:   [199.81 ms 200.12 ms 200.45 ms]

AoC 2022 - All          time:   [209.85 ms 210.15 ms 210.46 ms]
```

`0.78985 s` remaining