# `whatsys` -- What system is this?

What kernel version is running?

# Example

```
let kernel = whatsys::kernel_version(); // E.g. Some("20.3.0")
```

# Supported operating systems

We support the following operating systems:

* Windows
* macOS
* Linux
* Android

# License

MIT. See [LICENSE](LICENSE).

Based on:

* [sys-info](https://crates.io/crates/sys-info), [Repository](https://github.com/FillZpp/sys-info-rs), [MIT LICENSE][sys-info-mit]
* [sysinfo](https://crates.io/crates/sysinfo), [Repository](https://github.com/GuillaumeGomez/sysinfo), [MIT LICENSE][sysinfo-mit]

[sys-info-mit]: https://github.com/FillZpp/sys-info-rs/blob/master/LICENSE
[sysinfo-mit]: https://github.com/GuillaumeGomez/sysinfo/blob/master/LICENSE
