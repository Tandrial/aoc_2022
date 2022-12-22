# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  62.600 us |       0 ns |  63.500 us | 126.100 us |
| 02  |  23.600 us |  14.600 us |  36.700 us |  74.900 us |
| 03  |  21.100 us |  12.400 us |  31.900 us |  65.400 us |
| 04  |  84.400 us |   6.600 us |  85.200 us | 176.200 us |
| 05  | 178.200 us |  17.100 us | 196.000 us | 391.300 us |
| 06  |     100 ns |   4.100 us |   6.600 us |  10.800 us |
| 07  | 970.200 us |     400 ns | 972.000 us |   1.942 ms |
| 08  |  58.800 us |     100 ns | 333.200 us | 392.100 us |
| 09  |  59.400 us |     100 ns | 860.900 us | 920.400 us |
| 10  |  13.100 us |     100 ns |  30.600 us |  43.800 us |
| 11  |  40.900 us |  39.000 us |  16.200 ms |  16.280 ms |
| 12  |  52.400 us |   2.062 ms |   1.557 ms |   3.671 ms |
| 13  |   1.452 ms |  15.700 us |   1.521 ms |   2.989 ms |
| 14  | 192.000 us | 109.700 us |   4.168 ms |   4.470 ms |
| 15  | 679.200 us |     200 ns | 708.700 us |   1.388 ms |
| 16  | 928.800 us |  53.209 ms | 117.596 ms | 171.734 ms |
| 17  |       0 ns |  58.477 ms |  85.270 ms | 143.747 ms |
| 18  | 724.300 us | 471.100 us |   4.515 ms |   5.710 ms |
| 19  | 659.900 us |  21.214 ms |  22.026 ms |  43.900 ms |
| 20  | 230.700 us |   6.627 ms |  84.675 ms |  91.533 ms |
| 21  | 623.700 us | 581.700 us |   1.618 ms |   2.824 ms |
| 22  |  99.200 us | 120.000 us | 200.000 us | 419.200 us |
| sum |   7.154 ms | 142.984 ms | 342.674 ms | 492.813 ms |

`0.5071867s` remaining

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
AoC 2022 - Day23        time:   missing
AoC 2022 - Day24        time:   missing
AoC 2022 - Day25        time:   missing

AoC 2022 - All          time:   [459.36 ms 460.33 ms 461.37 ms]
```

`0.53967 s` remaining