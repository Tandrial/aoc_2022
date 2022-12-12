# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.993589 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  47.000 us |     100 ns |  47.800 us |  94.900 us |
| 02  |  27.800 us |  14.900 us |  39.200 us |  81.900 us |
| 03  |  22.700 us |  12.400 us |  33.900 us |  69.000 us |
| 04  |  92.500 us |   6.700 us |  93.400 us | 192.600 us |
| 05  | 171.600 us |  18.200 us | 185.800 us | 375.600 us |
| 06  |     100 ns |   3.200 us |   6.500 us |   9.800 us |
| 07  |   1.072 ms |     600 ns |   1.074 ms |   2.147 ms |
| 08  |  61.200 us |     100 ns | 344.700 us | 406.000 us |
| 09  |  66.000 us |     100 ns | 780.500 us | 846.600 us |
| 10  |  10.800 us |       0 ns |  28.700 us |  39.500 us |
| 11  |  --        |       --   |  --------- |  --------- |
| 12  |  61.700 us |     100 ns |   2.086 ms |   2.148 ms |
| sum |   1.633 ms |  56.400 us |   4.721 ms |   6.411 ms |

## Criterion benches

`0.99959685 s` remaining

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

AoC 2022 - All          time:   [4.0177 ms 4.0244 ms 4.0315 ms]
```