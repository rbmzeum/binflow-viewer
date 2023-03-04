fn main() {
    glib_build_tools::compile_resources(
        &["./"],
        "data/resources/binflow.gresource.xml",
        "resources.gresource",
    );
}