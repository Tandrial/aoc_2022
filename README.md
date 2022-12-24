# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  72.600 us |     100 ns |  74.900 us | 147.600 us |
| 02  |  41.100 us |  14.700 us |  52.800 us | 108.600 us |
| 03  |  23.600 us |  12.200 us |  37.400 us |  73.200 us |
| 04  | 110.400 us |   6.600 us | 111.200 us | 228.200 us |
| 05  | 215.500 us |  20.400 us | 231.500 us | 467.400 us |
| 06  |       0 ns |   6.300 us |   9.300 us |  15.600 us |
| 07  |   1.589 ms |     800 ns |   1.590 ms |   3.181 ms |
| 08  |  61.400 us |       0 ns | 363.900 us | 425.300 us |
| 09  |  67.000 us |       0 ns | 919.700 us | 986.700 us |
| 10  |  14.400 us |       0 ns |  32.000 us |  46.400 us |
| 11  |  37.000 us |  31.500 us |  16.329 ms |  16.397 ms |
| 12  |  76.200 us |   2.421 ms |   1.879 ms |   4.377 ms |
| 13  |   1.893 ms |  11.900 us |   1.954 ms |   3.860 ms |
| 14  | 251.700 us | 161.000 us |   4.432 ms |   4.844 ms |
| 15  | 723.800 us |     200 ns | 758.500 us |   1.482 ms |
| 16  |   1.001 ms |  52.812 ms | 132.070 ms | 185.883 ms |
| 17  |       0 ns |  61.397 ms |  88.581 ms | 149.979 ms |
| 18  | 672.300 us | 464.800 us |   4.187 ms |   5.324 ms |
| 19  | 771.500 us |  20.842 ms |  23.757 ms |  45.371 ms |
| 20  | 231.300 us |   5.716 ms |  70.357 ms |  76.305 ms |
| 21  | 616.800 us | 482.900 us |   1.762 ms |   2.862 ms |
| 22  | 124.000 us | 108.300 us | 205.100 us | 437.400 us |
| 23  | 145.100 us |       0 ns | 598.081 ms | 598.226 ms |
| 24  | 340.400 us |       0 ns |    6.575 s |    6.575 s |
| sum |   9.080 ms | 144.511 ms |    7.523 s |    7.676 s |

`-6.523s` remaining

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
AoC 2022 - Day23        time:   [587.06 ms 587.61 ms 588.15 ms]
AoC 2022 - Day24        time:   missing
AoC 2022 - Day25        time:   missing

AoC 2022 - All          time:   [1.0325 s 1.0339 s 1.0354 s]
```

`-0.0339 s` remaining