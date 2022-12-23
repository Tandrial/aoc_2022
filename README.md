# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  62.200 us |       0 ns |  63.200 us | 125.400 us |
| 02  |  23.600 us |  15.000 us |  36.300 us |  74.900 us |
| 03  |  22.900 us |  12.400 us |  33.900 us |  69.200 us |
| 04  | 101.300 us |   6.700 us | 102.100 us | 210.100 us |
| 05  | 178.700 us |  17.200 us | 193.500 us | 389.400 us |
| 06  |     100 ns |   5.800 us |   6.400 us |  12.300 us |
| 07  | 991.600 us |     500 ns | 993.400 us |   1.985 ms |
| 08  |  68.100 us |     100 ns | 382.600 us | 450.800 us |
| 09  |  74.900 us |     100 ns | 925.200 us |   1.000 ms |
| 10  |  15.800 us |       0 ns |  47.000 us |  62.800 us |
| 11  |  28.200 us |  28.000 us |  16.526 ms |  16.582 ms |
| 12  |  49.000 us |   2.012 ms |   1.524 ms |   3.585 ms |
| 13  |   1.528 ms |  11.000 us |   1.592 ms |   3.131 ms |
| 14  | 199.000 us | 133.200 us |   3.917 ms |   4.250 ms |
| 15  | 532.500 us |     200 ns | 552.500 us |   1.085 ms |
| 16  | 804.600 us |  48.181 ms | 120.656 ms | 169.642 ms |
| 17  |     100 ns |  59.541 ms |  85.164 ms | 144.706 ms |
| 18  | 716.100 us | 458.700 us |   4.215 ms |   5.389 ms |
| 19  | 885.600 us |  21.434 ms |  21.804 ms |  44.124 ms |
| 20  | 155.400 us |   5.450 ms |  66.930 ms |  72.537 ms |
| 21  | 635.600 us | 526.600 us |   1.749 ms |   2.911 ms |
| 22  |  86.300 us |  95.400 us | 165.000 us | 346.700 us |
| 23  | 187.000 us |     100 ns | 750.550 ms | 750.737 ms |
| sum |   7.347 ms | 137.931 ms |    1.078 s |    1.223 s |

`-0.223s` remaining

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
AoC 2022 - Day11        time:   [19.433 µs 19.478 µs 19.535 µs]
AoC 2022 - Day12        time:   [1.8280 ms 1.8317 ms 1.8356 ms]
AoC 2022 - Day13        time:   [1.4609 ms 1.4626 ms 1.4645 ms]
AoC 2022 - Day14        time:   [3.9756 ms 3.9793 ms 3.9829 ms]
AoC 2022 - Day15        time:   [49.464 µs 49.524 µs 49.586 µs]
AoC 2022 - Day16        time:   [168.37 ms 168.90 ms 169.46 ms]
AoC 2022 - Day17        time:   [143.80 ms 144.07 ms 144.35 ms]
AoC 2022 - Day18        time:   [4.1492 ms 4.1654 ms 4.1880 ms]
AoC 2022 - Day20        time:   [72.320 ms 72.483 ms 72.682 ms]
AoC 2022 - Day21        time:   [1.7750 ms 1.7797 ms 1.7850 ms]
AoC 2022 - Day22        time:   [210.07 µs 210.50 µs 210.98 µs]
AoC 2022 - Day23        time:   [785.09 ms 786.45 ms 788.35 ms]
AoC 2022 - Day24        time:   missing
AoC 2022 - Day25        time:   missing

AoC 2022 - All          time:   [1.2120 s 1.2134 s 1.2153 s]
```

`-0.2134 s` remaining