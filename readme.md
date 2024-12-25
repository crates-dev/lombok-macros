## lombok-macros

[![](https://img.shields.io/crates/v/lombok-macros.svg)](https://crates.io/crates/lombok-macros)
[![](https://docs.rs/lombok-macros/badge.svg)](https://docs.rs/lombok-macros)
[![](https://img.shields.io/crates/l/lombok-macros.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/lombok-macros/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/lombok-macros/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/lombok-macros/)

[Api Docs](https://docs.rs/lombok-macros/latest/lombok_macros/)

> A collection of procedural macros for Lombok-like functionality in Rust.

## Features

## Installation

To use this crate, you can run cmd:

```shell
cargo add lombok-macros
```

## Use

```rust
use lombok_macros::*;

#[derive(Data)]
struct LombokMacros {
    list: Vec<String>,
}

fn main() {
    let mut data: LombokMacros = LombokMacros { list: vec![] };
    let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    data.set_list(list.clone());
    assert_eq!(*data.get_list(), list);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
