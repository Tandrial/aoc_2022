# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  65.900 us |     200 ns |  66.700 us | 132.800 us |
| 02  |  23.600 us |  15.000 us |  36.700 us |  75.300 us |
| 03  |  23.100 us |  11.600 us |  34.000 us |  68.700 us |
| 04  |  99.800 us |   6.500 us | 100.600 us | 206.900 us |
| 05  | 129.800 us |  14.200 us | 141.100 us | 285.100 us |
| 06  |       0 ns |   3.000 us |   7.800 us |  10.800 us |
| 07  | 973.300 us |     500 ns | 974.300 us |   1.948 ms |
| 08  |  49.100 us |       0 ns | 323.700 us | 372.800 us |
| 09  |  54.500 us |       0 ns | 746.500 us | 801.000 us |
| 10  |  13.600 us |       0 ns |  29.900 us |  43.500 us |
| 12  |  51.400 us |   2.157 ms |   1.514 ms |   3.722 ms |
| 13  |   1.480 ms |  10.600 us |   1.535 ms |   3.026 ms |
| 14  | 184.500 us | 109.600 us |   3.880 ms |   4.174 ms |
| 15  | 484.700 us |     200 ns |  29.037 ms |  29.521 ms |
| sum |   3.633 ms |   2.328 ms |  38.429 ms |  44.390 ms |

`0.95560926s` remaining

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
AoC 2022 - Day15        time:   [28.933 ms 29.002 ms 29.080 ms]

AoC 2022 - All          time:   [40.397 ms 40.470 ms 40.553 ms]
```

`0.95953 s` remaining