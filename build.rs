fn main() {
    slint_build::compile_with_config(
        "examples/ui/app.slint",
        slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer)
    )
    .expect("Slint build failed");
}