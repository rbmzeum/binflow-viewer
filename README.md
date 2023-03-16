### VS BinFlow Viewer

You can install the schema by executing the following commands on a Linux or macOS machine:

```
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp ./src/schemas/net.vs-binflow.viewer.gschema.xml $HOME/.local/share/glib-2.0/schemas/
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
```

On Windows run:

```
mkdir C:\ProgramData\glib-2.0\schemas\
cp .\src\schemas\net.vs-binflow.viewer.gschema.xml C:\ProgramData\glib-2.0\schemas\
glib-compile-schemas C:\ProgramData\glib-2.0\schemas\
```

Run:

```
cargo run --bin vs-binflow-viewer
```

Example file:

```
./data/prices/btcusd
```

Example of creating a file form a ``Vec<f64>``:
```rust
use std::fs::OpenOptions;
use std::io::{prelude::*, Seek, SeekFrom};
use std::path::Path;

pub fn save_values(filename: String, values: Vec<f64>) {
    let oo = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename);
    // let oo = OpenOptions::new().write(true).open(filename)
    match oo {
        Ok(mut file) => {
            file.seek(SeekFrom::Start(0)).unwrap();

            for value in values {
                let bytes: [u8; 8] = value.to_be_bytes();
                file.write_all(&bytes);
            }
        },
        Err(_e) => {},
    }
}
```