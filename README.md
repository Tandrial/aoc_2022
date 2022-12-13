# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.99887 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  60.800 us |       0 ns |  61.700 us | 122.500 us |
| 02  |  26.300 us |  14.500 us |  44.500 us |  85.300 us |
| 03  |  18.500 us |  11.800 us |  29.600 us |  59.900 us |
| 04  |  93.000 us |   6.700 us |  93.800 us | 193.500 us |
| 05  | 135.300 us |  15.900 us | 147.000 us | 298.200 us |
| 06  |     200 ns |   5.000 us |   6.800 us |  12.000 us |
| 07  |   1.023 ms |     700 ns |   1.024 ms |   2.048 ms |
| 08  |  55.200 us |       0 ns | 339.200 us | 394.400 us |
| 09  |  62.300 us |       0 ns | 827.900 us | 890.200 us |
| 10  |  10.100 us |     100 ns |  26.400 us |  36.600 us |
| 11  |        --- |        --- |        --- |        --- |
| 12  |  67.100 us |   2.035 ms |   1.764 ms |   3.867 ms |
| 13  |   1.530 ms |  13.500 us |   1.748 ms |   3.291 ms |
| sum |   3.082 ms |   2.103 ms |   6.114 ms |  11.300 ms |

## Criterion benches

`0.9932326 s` remaining

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
AoC 2022 - Day13        time:   [1.6139 ms 1.6160 ms 1.6184 ms]

AoC 2022 - All          time:   [6.7574 ms 6.7674 ms 6.7780 ms]
```