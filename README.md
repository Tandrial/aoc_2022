# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  60.300 us |     100 ns |  60.400 us | 120.800 us |
| 02  |  23.500 us |  14.500 us |  37.300 us |  75.300 us |
| 03  |  19.000 us |  13.800 us |  29.700 us |  62.500 us |
| 04  |  86.400 us |   6.100 us |  87.200 us | 179.700 us |
| 05  | 164.400 us |  13.400 us | 176.300 us | 354.100 us |
| 06  |     100 ns |   3.100 us |   7.500 us |  10.700 us |
| 07  | 990.900 us |     500 ns | 991.900 us |   1.983 ms |
| 08  |  55.800 us |     100 ns | 327.200 us | 383.100 us |
| 09  |  44.800 us |     100 ns | 831.900 us | 876.800 us |
| 10  |  11.000 us |     100 ns |  29.200 us |  40.300 us |
| 11  |            |            |            |            |
| 12  |  55.700 us |   2.028 ms |   1.548 ms |   3.632 ms |
| 13  |   1.516 ms |  10.800 us |   1.574 ms |   3.101 ms |
| 14  | 177.200 us | 111.300 us |   3.886 ms |   4.175 ms |
| 15  | 434.100 us |     200 ns | 452.600 us | 886.900 us |
| sum |   3.639 ms |   2.202 ms |  10.041 ms |  15.883 ms |

`0.981233s` remaining

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

AoC 2022 - All          time:   [12.099 ms 12.119 ms 12.142 ms]
```

`0.987881 s` remaining