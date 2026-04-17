# Human Versions in Rust
Provides a versioning system, based on [Semantic Versioning](https://semver.org/), representing human age at different time scales. Major versions represent the number of decades lived, incrementing every 10 years. Minor versions represent the number of years lived within the current decade, and patch versions represent the number of months lived within the current year.

Age is represented in a `major.minor.patch` format. Unlike SemVer, labels for pre-release and build metadata are not available as such the link between these items and human age has yet to be established. Compatible with `chrono` for conversions from dates through the `Datelike` trait, and supports serialisation with `serde`.
