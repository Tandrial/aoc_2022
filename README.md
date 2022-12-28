# AoC 2022 - Rust

Trying to stay under `1s` for the whole thing.


## Manual Timings

| Day |      Parse |      Part1 |      Part2 |      Total |
| :-: | ---------: | ---------: | ---------: | ---------: |
| 01  |  69.400 us |       0 ns |  71.600 us | 141.000 us |
| 02  |  37.700 us |  14.500 us |  52.700 us | 104.900 us |
| 03  |  30.500 us |  11.800 us |  41.400 us |  83.700 us |
| 04  | 115.500 us |   6.500 us | 116.300 us | 238.300 us |
| 05  | 154.000 us |  19.800 us | 167.200 us | 341.000 us |
| 06  |       0 ns |   4.000 us |   8.300 us |  12.300 us |
| 07  |   1.121 ms |     500 ns |   1.122 ms |   2.244 ms |
| 08  |  19.200 us |       0 ns | 284.900 us | 304.100 us |
| 09  | 101.300 us |       0 ns | 692.900 us | 794.200 us |
| 10  |  37.000 us |       0 ns |  62.900 us |  99.900 us |
| 11  |  32.000 us |  27.500 us |  16.413 ms |  16.472 ms |
| 12  |  25.500 us |   1.569 ms |   1.172 ms |   2.768 ms |
| 13  |   1.598 ms |  11.200 us |   1.656 ms |   3.266 ms |
| 14  | 268.900 us | 116.700 us |   3.995 ms |   4.380 ms |
| 15  | 779.500 us |     200 ns | 801.000 us |   1.580 ms |
| 16  | 964.500 us |  48.549 ms | 117.939 ms | 167.453 ms |
| 17  |  57.900 us |  15.667 ms |  15.974 ms |  31.699 ms |
| 18  | 487.700 us | 177.000 us |   1.701 ms |   2.366 ms |
| 19  | 915.300 us |  16.574 ms |  12.834 ms |  30.324 ms |
| 20  | 172.300 us |   5.486 ms |  69.568 ms |  75.227 ms |
| 21  | 656.900 us | 506.500 us |   1.791 ms |   2.955 ms |
| 22  |  96.500 us | 134.900 us | 183.600 us | 415.000 us |
| 23  |  92.900 us |       0 ns | 322.810 ms | 322.902 ms |
| 24  |  46.200 us |       0 ns |  60.248 ms |  60.294 ms |
| 25  |   8.400 us |  34.500 us |       0 ns |  42.900 us |
| sum |   7.889 ms |  88.913 ms | 629.711 ms | 726.514 ms |

`0.27348542s` remaining

## Criterion benches

```
AoC 2022 - Day01        time:   [46.091 µs 46.215 µs 46.348 µs]
AoC 2022 - Day02        time:   [14.074 µs 14.134 µs 14.207 µs]
AoC 2022 - Day03        time:   [27.604 µs 27.689 µs 27.775 µs]
AoC 2022 - Day04        time:   [77.619 µs 77.795 µs 77.997 µs]
AoC 2022 - Day05        time:   [124.54 µs 124.86 µs 125.22 µs]
AoC 2022 - Day06        time:   [10.396 µs 10.448 µs 10.499 µs]
AoC 2022 - Day07        time:   [703.00 µs 705.10 µs 707.21 µs]
AoC 2022 - Day08        time:   [261.56 µs 262.31 µs 263.25 µs]
AoC 2022 - Day09        time:   [541.35 µs 544.75 µs 548.07 µs]
AoC 2022 - Day10        time:   [21.214 µs 21.285 µs 21.364 µs]
AoC 2022 - Day11        time:   [21.494 µs 21.636 µs 21.792 µs]
AoC 2022 - Day12        time:   [2.6000 ms 2.6085 ms 2.6172 ms]
AoC 2022 - Day13        time:   [1.5047 ms 1.5073 ms 1.5100 ms]
AoC 2022 - Day14        time:   [4.0346 ms 4.0451 ms 4.0566 ms]
AoC 2022 - Day15        time:   [42.836 µs 42.977 µs 43.123 µs]
AoC 2022 - Day16        time:   [136.98 ms 137.44 ms 137.90 ms]
AoC 2022 - Day17        time:   [30.201 ms 30.247 ms 30.298 ms]
AoC 2022 - Day18        time:   [1.6799 ms 1.6814 ms 1.6829 ms]
AoC 2022 - Day19        time:   [22.724 ms 22.958 ms 23.194 ms]
AoC 2022 - Day20        time:   [72.267 ms 72.399 ms 72.554 ms]
AoC 2022 - Day21        time:   [1.4894 ms 1.4957 ms 1.5024 ms]
AoC 2022 - Day22        time:   [227.04 µs 227.66 µs 228.34 µs]
AoC 2022 - Day23        time:   [317.63 ms 318.39 ms 319.32 ms]
AoC 2022 - Day24        time:   [50.205 ms 50.403 ms 50.605 ms]
AoC 2022 - Day25        time:   [34.175 µs 34.219 µs 34.275 µs]

AoC 2022 - All          time:   [678.46 ms 679.19 ms 679.97 ms]
```

`0.23164 s` remaining