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