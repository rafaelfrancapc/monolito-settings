fn main() {
    println!("cargo:rerun-if-changed=resources");
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.xml",
        "monolito.gresource",
    );
}
