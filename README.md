# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  61.100 us |     100 ns |  62.000 us | 123.200 us |
| 02  |  24.000 us |  14.200 us |  36.400 us |  74.600 us |
| 03  |  20.800 us |  11.800 us |  31.700 us |  64.300 us |
| 04  |  86.000 us |   6.300 us |  86.800 us | 179.100 us |
| 05  | 172.200 us |  18.100 us | 187.000 us | 377.300 us |
| 06  |     100 ns |   4.000 us |   7.300 us |  11.400 us |
| 07  | 994.700 us |     400 ns | 995.700 us |   1.990 ms |
| 08  |  58.000 us |     100 ns | 336.200 us | 394.300 us |
| 09  |  58.900 us |     100 ns | 860.100 us | 919.100 us |
| 10  |  13.300 us |     100 ns |  31.300 us |  44.700 us |
| 11  |            |            |            |            |
| 12  |  50.400 us |   1.982 ms |   1.525 ms |   3.558 ms |
| 13  |   1.418 ms |  10.600 us |   1.473 ms |   2.903 ms |
| 14  | 223.000 us | 120.500 us |   3.979 ms |   4.323 ms |
| 15  | 481.700 us |     200 ns | 501.800 us | 983.700 us |
| 16  | 939.000 us |  46.334 ms | 116.775 ms | 164.049 ms |
| 17  |     300 ns |  59.320 ms |  85.536 ms | 144.858 ms |
| 18  | 607.100 us | 456.700 us |   3.887 ms |   4.951 ms |
| 19  | 693.200 us |  21.827 ms |  21.360 ms |  43.881 ms |
| sum |   5.902 ms | 130.108 ms | 237.676 ms | 373.687 ms |

`0.6263125s` remaining
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
AoC 2022 - Day19        time:   [37.325 ms 37.621 ms 37.921 ms]

AoC 2022 - All          time:   [366.92 ms 367.75 ms 368.75 ms]
```

`0.63225 s` remaining