# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  81.100 us |     100 ns |  81.100 us | 162.300 us |
| 02  |  32.100 us |  14.600 us |  44.100 us |  90.800 us |
| 03  |  28.600 us |  11.600 us |  39.600 us |  79.800 us |
| 04  | 106.100 us |   6.500 us | 106.900 us | 219.500 us |
| 05  | 192.600 us |  19.300 us | 207.800 us | 419.700 us |
| 06  |       0 ns |   8.700 us |   6.500 us |  15.200 us |
| 07  |   1.023 ms |     500 ns |   1.024 ms |   2.048 ms |
| 08  |  63.400 us |       0 ns | 353.800 us | 417.200 us |
| 09  |  57.600 us |     100 ns | 871.900 us | 929.600 us |
| 10  |  16.300 us |     100 ns |  33.200 us |  49.600 us |
| 11  |  32.600 us |  30.900 us |  16.319 ms |  16.382 ms |
| 12  |  49.600 us |   2.147 ms |   1.569 ms |   3.767 ms |
| 13  |   1.634 ms |  11.500 us |   1.701 ms |   3.347 ms |
| 14  | 240.500 us | 122.900 us |   4.195 ms |   4.559 ms |
| 15  | 712.400 us |     200 ns | 741.500 us |   1.454 ms |
| 16  | 907.600 us |  47.043 ms | 118.500 ms | 166.450 ms |
| 17  |     100 ns |  59.249 ms |  86.058 ms | 145.307 ms |
| 18  | 592.500 us | 450.800 us |   4.055 ms |   5.099 ms |
| 19  | 851.500 us |  21.412 ms |  22.017 ms |  44.281 ms |
| 20  | 182.500 us |   5.657 ms |  67.562 ms |  73.402 ms |
| 21  | 888.900 us | 538.200 us |  25.027 ms |  26.454 ms |
| sum |   7.694 ms | 136.727 ms | 350.518 ms | 494.940 ms |

`0.5050596s` remaining

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
AoC 2022 - Day21        time:   [25.125 ms 25.176 ms 25.230 ms]
AoC 2022 - Day22        time:   missing
AoC 2022 - Day23        time:   missing
AoC 2022 - Day24        time:   missing
AoC 2022 - Day25        time:   missing

AoC 2022 - All          time:   [504.70 ms 505.88 ms 507.09 ms]
```

`0.49412 s` remaining