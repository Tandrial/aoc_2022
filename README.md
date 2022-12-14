# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.985007 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  58.400 us |     100 ns |  58.500 us | 117.000 us |
| 02  |  24.400 us |  15.000 us |  36.800 us |  76.200 us |
| 03  |  17.300 us |  12.000 us |  28.400 us |  57.700 us |
| 04  | 107.400 us |   6.400 us | 108.200 us | 222.000 us |
| 05  | 129.400 us |  14.400 us | 140.800 us | 284.600 us |
| 06  |     100 ns |   4.000 us |   6.700 us |  10.800 us |
| 07  |   1.021 ms |     500 ns |   1.022 ms |   2.044 ms |
| 08  |  54.800 us |     100 ns | 343.300 us | 398.200 us |
| 09  |  56.100 us |       0 ns | 777.000 us | 833.100 us |
| 10  |  14.100 us |     100 ns |  32.100 us |  46.300 us |
| 11  |        --- |        --- |        --- |        --- |
| 12  |  51.600 us |   1.974 ms |   1.480 ms |   3.506 ms |
| 13  |   1.495 ms |  11.200 us |   1.553 ms |   3.060 ms |
| 14  | 173.200 us | 127.600 us |   4.035 ms |   4.336 ms |
| sum |   3.203 ms |   2.166 ms |   9.623 ms |  14.993 ms |

## Criterion benches

`0.988741 s` remaining

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

AoC 2022 - All          time:   [11.237 ms 11.259 ms 11.285 ms]
```