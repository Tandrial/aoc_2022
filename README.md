# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  60.700 us |     100 ns |  61.600 us | 122.400 us |
| 02  |  25.500 us |  14.900 us |  38.500 us |  78.900 us |
| 03  |  12.700 us |  11.900 us |  23.800 us |  48.400 us |
| 04  |  94.900 us |   6.900 us |  95.800 us | 197.600 us |
| 05  | 126.800 us |  14.700 us | 139.900 us | 281.400 us |
| 06  |     200 ns |   4.200 us |   6.600 us |  11.000 us |
| 07  |   1.096 ms |   1.000 us |   1.097 ms |   2.194 ms |
| 08  |  59.800 us |     100 ns | 351.600 us | 411.500 us |
| 09  |  62.100 us |     100 ns | 911.200 us | 973.400 us |
| 11  |            |            |            |            |
| 10  |  22.600 us |     100 ns |  39.500 us |  62.200 us |
| 12  |  56.300 us |   2.286 ms |   1.718 ms |   4.060 ms |
| 13  |   1.547 ms |  11.000 us |   1.605 ms |   3.163 ms |
| 14  | 209.800 us | 126.400 us |   4.031 ms |   4.367 ms |
| 15  | 593.800 us |     300 ns | 621.000 us |   1.215 ms |
| 16  | 917.900 us |  48.109 ms | 120.017 ms | 169.044 ms |
| 17  |     200 ns |  59.592 ms |  86.156 ms | 145.749 ms |
| 18  | 609.900 us | 456.100 us |   3.980 ms |   5.046 ms |
| sum |   5.496 ms | 110.636 ms | 220.895 ms | 337.029 ms |

`0.6629709s` remaining

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
AoC 2022 - Day16        time:   [180.95 ms 181.64 ms 182.37 ms]
AoC 2022 - Day17        time:   [145.28 ms 145.53 ms 145.84 ms]
AoC 2022 - Day18        time:   [4.1492 ms 4.1654 ms 4.1880 ms]

AoC 2022 - All          time:   [331.11 ms 333.00 ms 335.97 ms]
```

`0.66600 s` remaining