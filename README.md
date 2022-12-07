# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.997181 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  46.800 us |     100 ns |  47.700 us |  94.600 us |
| 02  |  31.500 us |  14.900 us |  44.400 us |  90.800 us |
| 03  |  16.900 us |  11.900 us |  32.300 us |  61.100 us |
| 04  |  91.400 us |   6.500 us |  92.300 us | 190.200 us |
| 05  | 176.300 us |  19.400 us | 190.700 us | 386.400 us |
| 06  |     100 ns |   4.000 us |  12.200 us |  16.300 us |
| 07  | 989.500 us |     500 ns | 990.500 us |   1.980 ms |
| sum |   1.352 ms |  57.300 us |   1.410 ms |   2.819 ms |


## Criterion benches

`0.9988963 s` remaining

```
AoC 2022 - Day01        time:   [29.263 µs 29.386 µs 29.515 µs]
AoC 2022 - Day02        time:   [14.600 µs 14.639 µs 14.680 µs]
AoC 2022 - Day03        time:   [25.625 µs 25.744 µs 25.876 µs]
AoC 2022 - Day04        time:   [75.316 µs 75.684 µs 76.084 µs]
AoC 2022 - Day05        time:   [129.37 µs 130.05 µs 130.90 µs]
AoC 2022 - Day06        time:   [10.116 µs 10.153 µs 10.197 µs]
AoC 2022 - Day07        time:   [698.67 µs 699.67 µs 700.79 µs]

AoC 2022 - All          time:   [1.1022 ms 1.1037 ms 1.1055 ms]
```