# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  59.500 us |       0 ns |  60.400 us | 119.900 us |
| 02  |  29.100 us |  14.800 us |  42.400 us |  86.300 us |
| 03  |  15.800 us |  11.800 us |  26.800 us |  54.400 us |
| 04  | 103.300 us |   7.300 us | 104.100 us | 214.700 us |
| 05  | 168.700 us |  16.800 us | 185.400 us | 370.900 us |
| 06  |     100 ns |   3.200 us |   7.500 us |  10.800 us |
| 07  |   1.105 ms |     600 ns |   1.106 ms |   2.212 ms |
| 08  |  57.100 us |     100 ns | 339.300 us | 396.500 us |
| 09  |  58.600 us |     100 ns | 874.900 us | 933.600 us |
| 10  |  12.800 us |     100 ns |  29.400 us |  42.300 us |
| 11  |            |            |            |            |
| 12  |  46.700 us |   2.321 ms |   1.904 ms |   4.272 ms |
| 13  |   1.611 ms |  11.700 us |   1.671 ms |   3.294 ms |
| 14  | 204.500 us | 116.700 us |   3.912 ms |   4.234 ms |
| 15  | 446.300 us |     100 ns | 460.600 us | 907.000 us |
| 16  |   1.055 ms |  47.700 ms | 122.064 ms | 170.820 ms |
| 17  |     200 ns |  59.246 ms |  85.927 ms | 145.174 ms |
| 18  | 605.600 us | 465.600 us |   4.117 ms |   5.188 ms |
| sum |   5.580 ms | 109.916 ms | 222.836 ms | 338.332 ms |

`0.6616674s` remaining

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
AoC 2022 - Day18        time:   [4.4548 ms 4.5072 ms 4.5787 ms]

AoC 2022 - All          time:   [332.49 ms 333.22 ms 334.00 ms]
```

`0.66678 s` remaining