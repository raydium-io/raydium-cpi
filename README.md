Raydium CPI adapters are suitable for different Anchor versions. For the stability of the AMM contract, it cannot be updated to the latest versions of Anchor and Solana in a timely manner. If you want to use the latest version of Anchor, please rely on this repository. The master branch maintains the latest version, if relying on an older Anchor, please refer to other branches.

## Environmental requirements
```
solana-cli 2.1.0
anchor-cli 0.31.0
```

## Usage
Call CLMM through CPI.
```
[dependencies]
anchor-lang = "=0.31.0"
anchor-spl = "=0.31.0"
raydium-clmm-cpi = { git = "https://github.com/raydium-io/raydium-cpi", package = "raydium-clmm-cpi", branch = "anchor-0.31.0" }
```

Call CPMM through CPI.
```
[dependencies]
anchor-lang = "=0.31.0"
anchor-spl = "=0.31.0"
raydium-cpmm-cpi = { git = "https://github.com/raydium-io/raydium-cpi", package = "raydium-cpmm-cpi", branch = "anchor-0.31.0" }
```

Call AMM through CPI.
```
[dependencies]
anchor-lang = "=0.31.0"
anchor-spl = "=0.31.0"
raydium-amm-cpi = { git = "https://github.com/raydium-io/raydium-cpi", package = "raydium-amm-cpi", branch = "anchor-0.31.0" }
```

You can find usage examples in this [repository](https://github.com/raydium-io/raydium-cpi-example/tree/anchor-0.31.0).
