# Avamain
A domain availabilty checker.

### Installation
Avamain is available on the crates.io registry, and can be install with cargo.
```sh
cargo install avamain
```

### Usage
Specify 3 environment variables; `TLD`, `CHARSET`, and `LENGTH`.
TLD is the `TLD` you want to check availability under and `CHARSET` specifies the characters used in domain generation alongside the `LENGTH` variable, which specifies the length of the domain (not including TLD).

Example
```sh
TLD=moe CHARSET=su LENGTH=3 avamain
```

Output is in CSV format, so they can be easily analyzed.
```sh
TLD=moe CHARSET=su LENGTH=3 avamain > availability.csv
column -s, -t < availability.csv
```
