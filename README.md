# udd

Rust access to the [Ultimate Debian Database](https://wiki.debian.org/UltimateDebianDatabase)
(UDD), built on [sqlx](https://crates.io/crates/sqlx).

It provides helpers for connecting to the public mirror at
`udd-mirror.debian.net`, and `sqlx::FromRow` structs for the UDD tables.
`debversion` columns are decoded as `debversion::Version`.

```rust
let pool = udd::connect_public_mirror().await?;
let sources: Vec<udd::Source> =
    sqlx::query_as("SELECT * FROM sources WHERE source = $1 AND release = 'sid'")
        .bind("dulwich")
        .fetch_all(&pool)
        .await?;
```

Tables with the same layout as a Debian table (for example `archived_bugs`,
`ubuntu_sources` or `ports_packages`) share its struct.

The structs in `src/tables/` are generated from the schema of the public
mirror by `scripts/generate-tables.py`:

```sh
python3 scripts/generate-tables.py src/tables && cargo fmt
```

The tests that talk to the public mirror are ignored by default. Run them with
`cargo test -- --ignored`.
