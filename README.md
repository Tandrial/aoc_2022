# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.996144 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  46.800 us |     100 ns |  46.900 us |  93.800 us |
| 02  |  23.300 us |  14.500 us |  35.200 us |  73.000 us |
| 03  |  15.000 us |  11.600 us |  26.600 us |  53.200 us |
| 04  |  97.500 us |   6.400 us |  98.300 us | 202.200 us |
| 05  | 116.900 us |  12.400 us | 128.000 us | 257.300 us |
| 06  |       0 ns |   3.100 us |   6.500 us |   9.600 us |
| 07  | 964.500 us |     500 ns | 965.400 us |   1.930 ms |
| 08  |  61.100 us |     100 ns | 332.000 us | 393.200 us |
| 09  |  44.600 us |     100 ns | 765.700 us | 810.400 us |
| 10  |   8.900 us |     100 ns |  24.400 us |  33.400 us |
| sum |   1.378 ms |  48.900 us |   2.429 ms |   3.856 ms |

## Criterion benches

`0.9979309 s` remaining

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

AoC 2022 - All          time:   [2.0674 ms 2.0691 ms 2.0710 ms]
```