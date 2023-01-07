# uniswap-rs

[![Crates.io][crates-badge]][crates-url]
[![MIT License][mit-badge]][mit-url]
[![CI Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/uniswap-rs.svg
[crates-url]: https://crates.io/crates/uniswap-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/danipopes/uniswap-rs/blob/master/LICENSE
[actions-badge]: https://github.com/danipopes/uniswap-rs/workflows/CI/badge.svg
[actions-url]: https://github.com/danipopes/uniswap-rs/actions?query=workflow%3ACI+branch%3Amaster

Unofficial Rust SDK library for Uniswap smart contracts.

## Quickstart

Add this to your Cargo.toml:

```toml
[dependencies]
uniswap-rs = { git = "https://github.com/danipopes/uniswap-rs" }
```

## Examples

Examples can be found [here][examples].

[examples]: https://github.com/danipopes/uniswap-rs/tree/master/examples

## Roadmap

-   [ ] UniswapV2 protocol
    -   [x] Implementation
    -   [x] Documentation
    -   [ ] Tests
-   [ ] UniswapV3 protocol
    -   [ ] Implementation
    -   [ ] Documentation
    -   [ ] Tests
-   [x] [UniversalRouter](https://github.com/Uniswap/universal-router)
    -   [x] Implementation
    -   [x] Documentation
    -   [x] Tests
-   [x] Contract addresses
    -   [x] Uniswap
    -   [x] Sushiswap
    -   [x] Pancakeswap
    -   [x] Quickswap
    -   [x] Spookyswap
    -   [x] Traderjoe
-   [ ] Features
    -   [x] Serde, Addressbook and `new_with_chain`
    -   [ ] V2 and V3, separated

## License

This project is licensed under the [MIT license](https://github.com/danipopes/uniswap-rs/blob/master/LICENSE).
