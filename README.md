# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

`0.995978 s` remaining

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  58.200 us |     100 ns |  58.300 us | 116.600 us |
| 02  |  34.000 us |  15.200 us |  46.900 us |  96.100 us |
| 03  |  18.500 us |  11.700 us |  30.600 us |  60.800 us |
| 04  |  87.800 us |   6.600 us |  88.600 us | 183.000 us |
| 05  | 124.300 us |  18.600 us | 137.500 us | 280.400 us |
| 06  |       0 ns |   3.200 us |   6.500 us |   9.700 us |
| 07  |   1.004 ms |     500 ns |   1.006 ms |   2.011 ms |
| 08  |  57.400 us |     100 ns | 331.400 us | 388.900 us |
| 09  |  53.300 us |       0 ns | 821.800 us | 875.100 us |
| sum |   1.438 ms |  56.000 us |   2.528 ms |   4.022 ms |

## Criterion benches

`0.9978815 s` remaining

```
AoC 2022 - Day01        time:   [29.263 µs 29.386 µs 29.515 µs]
AoC 2022 - Day02        time:   [14.006 µs 14.035 µs 14.062 µs]
AoC 2022 - Day03        time:   [25.173 µs 25.267 µs 25.373 µs]
AoC 2022 - Day04        time:   [75.316 µs 75.684 µs 76.084 µs]
AoC 2022 - Day05        time:   [129.37 µs 130.05 µs 130.90 µs]
AoC 2022 - Day06        time:   [10.116 µs 10.153 µs 10.197 µs]
AoC 2022 - Day07        time:   [698.67 µs 699.67 µs 700.79 µs]
AoC 2022 - Day08        time:   [295.32 µs 296.20 µs 297.34 µs]
AoC 2022 - Day09        time:   [732.77 µs 733.31 µs 733.85 µs]

AoC 2022 - All          time:   [2.1142 ms 2.1185 ms 2.1229 ms]
```