# ext-php-rs Windows ZIP backport

The base is the published ext-php-rs 0.15.15 crate. Its crates.io index
SHA-256 is abe62f25cd053f5f95dc7bbf94e5b41818ea3cf39b36cc25de8ca6b4de68a0eb
and its publication metadata records a880fd103020c47bc9acc01f8683dcd755b7c845.

The Windows-only build dependency starts at zip 9.0.5. The file-name iterator
handling follows published ext-php-rs 0.16.3, whose crates.io index SHA-256 is
f1279870fe6585ac0ba42ed6af0c5c5477b2b7f44dc073d4a6586a5b4a9c4aa0.
Its publication metadata records 3a9df2f63dfdaf18c30883ba15f23be387015b04.
This carries the ZIP API migration while retaining the 0.15 PHP binding API
used by Liter 2.1.1. A ZIP directory parsing error propagates through the
existing build-script result before extraction.

The owner Liter workspace resolves this package and its Windows dependency
graph. Upstream's publication lock and duplicate original manifest are
available in the source tarball.

Sources

- https://static.crates.io/crates/ext-php-rs/ext-php-rs-0.15.15.crate
- https://static.crates.io/crates/ext-php-rs/ext-php-rs-0.16.3.crate
- https://github.com/ext-php-rs/ext-php-rs/blob/3a9df2f63dfdaf18c30883ba15f23be387015b04/windows_build.rs
- https://osv.dev/vulnerability/GHSA-m665-v7pm-7grj
