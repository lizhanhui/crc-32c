# Introduction
This repository is to provide a Rust crate to compute CRC-32C of data buffer on modern X86/ARM64 chips.

Algorithms, accelerated by SIMD intrinsics and mathematics manipulations, are described in reference [1](https://www.corsix.org/content/fast-crc32c-4k) and [2](https://github.com/komrad36/CRC)

# Goal

Ultimate benchmark performance should be something as below:

|                  |Broadwell @ 3.00GHz| Skylake @ 3.20GHz  | Ice Lake @ 2.40GHz|
|------------------|-------------------|--------------------|-------------------|
|crc32_4k          |  10.19 GB/s 21.3 b/c|  12.64 GB/s 21.3 b/c|  11.59 GB/s 21.3 b/c|
|crc32_4k_pclmulqdq|  23.32 GB/s 48.8 b/c|  28.12 GB/s 47.5 b/c|  27.45 GB/s 50.5 b/c|
|crc32_4k_three_way|  26.99 GB/s 56.5 b/c|  33.26 GB/s 56.1 b/c|  29.18 GB/s 53.7 b/c|
|crc32_4k_fusion   | 44.44 GB/s 93.0 b/c | 55.65 GB/s 93.9 b/c | 50.36 GB/s 92.7 b/c|


# Alternatives
| Crate  | Throughput 4K | Throughtput 1M|
| -------| --------------| --------------|
|  crc   | 286.60 MiB/s | 286.02 MiB/s   |
| crc32c |3.3124 GiB/s|3.5175 GiB/s |

# Reference
1. https://www.corsix.org/content/fast-crc32c-4k
2. https://github.com/komrad36/CRC
3. https://en.wikipedia.org/wiki/Cyclic_redundancy_check
4. https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html