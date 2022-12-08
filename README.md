# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.996839 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  44.300 us |     100 ns |  45.200 us |  89.600 us |
| 02  |  30.700 us |  14.800 us |  43.400 us |  88.900 us |
| 03  |  21.600 us |  11.700 us |  32.500 us |  65.800 us |
| 04  |  84.000 us |   6.700 us |  84.800 us | 175.500 us |
| 05  | 130.100 us |  14.400 us | 142.100 us | 286.600 us |
| 06  |     100 ns |   3.300 us |   6.600 us |  10.000 us |
| 07  |   1.017 ms |   1.600 us |   1.018 ms |   2.036 ms |
| 08  |  54.400 us |     100 ns | 353.700 us | 408.200 us |
| sum |   1.382 ms |  52.700 us |   1.726 ms |   3.161 ms |


## Criterion benches

`0.9986588 s` remaining

```
AoC 2022 - Day01        time:   [29.263 µs 29.386 µs 29.515 µs]
AoC 2022 - Day02        time:   [14.600 µs 14.639 µs 14.680 µs]
AoC 2022 - Day03        time:   [25.625 µs 25.744 µs 25.876 µs]
AoC 2022 - Day04        time:   [75.316 µs 75.684 µs 76.084 µs]
AoC 2022 - Day05        time:   [129.37 µs 130.05 µs 130.90 µs]
AoC 2022 - Day06        time:   [10.116 µs 10.153 µs 10.197 µs]
AoC 2022 - Day07        time:   [698.67 µs 699.67 µs 700.79 µs]
AoC 2022 - Day08        time:   [311.28 µs 311.92 µs 312.58 µs]

AoC 2022 - All          time:   [1.3371 ms 1.3392 ms 1.3413 ms]
```