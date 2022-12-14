# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.985093 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  54.400 us |     100 ns |  55.300 us | 109.800 us |
| 02  |  22.700 us |  14.700 us |  35.100 us |  72.500 us |
| 03  |  17.300 us |  11.500 us |  28.100 us |  56.900 us |
| 04  |  88.900 us |   6.500 us |  89.700 us | 185.100 us |
| 05  | 119.200 us |  14.800 us | 130.400 us | 264.400 us |
| 06  |     100 ns |   3.200 us |   7.300 us |  10.600 us |
| 07  | 980.500 us |     600 ns | 981.400 us |   1.962 ms |
| 08  |  49.300 us |       0 ns | 314.900 us | 364.200 us |
| 09  |  61.100 us |     100 ns | 754.300 us | 815.500 us |
| 10  |  15.200 us |     100 ns |  32.200 us |  47.500 us |
| 11  |        --- |        --- |        --- |        --- |
| 12  |  47.200 us |   1.950 ms |   1.464 ms |   3.463 ms |
| 13  |   1.464 ms |  11.200 us |   1.522 ms |   2.997 ms |
| 14  | 176.500 us |  81.700 us |   4.299 ms |   4.557 ms |
| sum |   3.096 ms |   2.095 ms |   9.715 ms |  14.907 ms |

## Criterion benches

`0.989044 s` remaining

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
AoC 2022 - Day14        time:   [3.5533 ms 3.5590 ms 3.5655 ms]

AoC 2022 - All          time:   [10.940 ms 10.956 ms 10.974 ms]
```