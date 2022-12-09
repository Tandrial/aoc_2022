# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.99597 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  44.800 us |     100 ns |  44.900 us |  89.800 us |
| 02  |  24.800 us |  15.200 us |  38.100 us |  78.100 us |
| 03  |  20.600 us |  14.700 us |  34.500 us |  69.800 us |
| 04  |  91.000 us |   6.600 us |  91.800 us | 189.400 us |
| 05  | 167.900 us |  15.500 us | 179.900 us | 363.300 us |
| 06  |       0 ns |   3.200 us |   6.700 us |   9.900 us |
| 07  |   1.006 ms |     500 ns |   1.007 ms |   2.014 ms |
| 08  |  59.500 us |     100 ns | 343.900 us | 403.500 us |
| 09  |  50.500 us |       0 ns | 761.500 us | 812.000 us |
| sum |   1.465 ms |  55.900 us |   2.509 ms |   4.030 ms |

## Criterion benches

`0.997922 s` remaining

```
AoC 2022 - Day01        time:   [29.263 µs 29.386 µs 29.515 µs]
AoC 2022 - Day02        time:   [14.006 µs 14.035 µs 14.062 µs]
AoC 2022 - Day03        time:   [25.173 µs 25.267 µs 25.373 µs]
AoC 2022 - Day04        time:   [75.316 µs 75.684 µs 76.084 µs]
AoC 2022 - Day05        time:   [129.37 µs 130.05 µs 130.90 µs]
AoC 2022 - Day06        time:   [10.116 µs 10.153 µs 10.197 µs]
AoC 2022 - Day07        time:   [698.67 µs 699.67 µs 700.79 µs]
AoC 2022 - Day08        time:   [295.32 µs 296.20 µs 297.34 µs]
AoC 2022 - Day09        time:   [712.71 µs 716.82 µs 720.36 µs]

AoC 2022 - All          time:   [2.0767 ms 2.0780 ms 2.0794 ms]
```