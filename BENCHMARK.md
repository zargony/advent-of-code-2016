# Runtime comparison

- Machine: Mac mini (mid 2011), 2.7 GHz Intel Core i7, 16 GB RAM, macOS Sierra 10.12.3
- Toolchain:
  - Rust 1.15.1
  - Emscripten 1.37.1
  - Node 7.7.2
- Variants:
  - **Rust native**: Code from this repo
  - **Rust JS**: Code from this repo
  - **JS native**: Code from [olbura/advent-of-code-2016](https://github.com/olbura/advent-of-code-2016) (unfair comparison since code was developed independently and runs different approaches)
- Measurement: CPU total time as printed by zsh `time` command. The lowest result of three measurement was taken.

## Results

|       |  Rust native |      Rust JS |    JS native |
|-------|-------------:|-------------:|-------------:|
| day01 |       0.004s |       0.301s |       0.108s |
| day02 |       0.008s |       0.292s |       0.092s |
| day03 |       0.007s |       0.325s |       0.104s |
| day04 |       0.012s |       0.467s |       0.137s |
| day05 |      10.795s |      28.852s |     153.720s |
| day06 |       0.004s |       0.267s |       0.085s |
| day07 |       0.001s |            - |       0.222s |
| day08 |       0.003s |       0.301s |       0.081s |
| day09 |       0.004s |       0.305s |       0.094s |
| day10 |       0.005s |       0.554s |       0.089s |
| day11 |     589.970s |          oom |            - |
| day12 |       0.087s |       0.547s |      24.401s |
| day13 |       0.011s |       0.489s |            - |
| day14 |       0.001s |            - |     150.690s |
| day15 |       0.021s |       0.333s |       0.933s |
| day16 |       0.496s |          oom |      44.613s |
| day17 |       0.058s |       1.430s |       0.430s |
| day18 |       0.582s |       2.064s |      18.519s |
| day19 |       0.067s |       0.270s |            - |
| day20 |       0.005s |       0.261s |       0.095s |
| day21 |       0.004s |       0.529s |       0.095s |
| day22 |       0.019s |       7.688s |       0.128s |
| day23 |       0.003s |       0.250s |            - |
| day24 |       0.117s |       4.285s |            - |
| day25 |       0.066s |       0.971s |            - |
