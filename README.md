# Vemto (Rust Edition) ༄

Vemto is a fast, lightweight native desktop application written in **Rust** focused on generating **Laravel** code (migrations, models, factories, controllers/CRUD, and data schemas).

By replacing Electron, Node.js, and embedded PHP runtimes with a native Rust implementation using **egui / eframe**, Vemto achieves instant startup, minimal memory consumption (~30-50 MB vs hundreds of MBs in Electron), and single-binary native performance while still generating 100% standard Laravel PHP code.

## Features

- **Native Rust Desktop UI (eframe/egui)**: No Electron, no browser engine bloat. Fast and responsive.
- **Schema Editor**: Interactive editing of tables, columns, indexes, and Eloquent relationships.
- **Laravel Migrations Generator**: Produces clean, idiomatic Laravel migrations (`create_..._table.php`).
- **Eloquent Models Generator**: Generates Laravel Model classes with fillable fields, casts, traits, and relationship methods (`hasOne`, `hasMany`, `belongsTo`, `belongsToMany`).
- **Factories Generator**: Generates database factories with appropriate `fake()` data mappings for each column type.
- **Controllers & CRUD Generator**: Generates resource controllers with validation rules and CRUD actions.
- **Instant Code Preview**: Live preview of generated PHP code before writing to disk.
- **Export & File Generation**: Direct export to any directory in your Laravel project.
- **Project Serialization**: Save and load projects in standard JSON format.

## Building and Running

Ensure you have Rust and Cargo installed (1.70+):

```bash
cargo build --release
```

Run Vemto:

```bash
cargo run --release
```

## Running Tests

Run all unit and integration tests:

```bash
cargo test
```

## License

Vemto Rust source code is licensed under the [MIT license](LICENSE).
