fn main() {
    let style = match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("windows") => "fluent-dark",
        Ok("macos") => "cupertino-dark",
        _ => "native",
    };

    let config = slint_build::CompilerConfiguration::new().with_style(style.into());
    slint_build::compile_with_config("./ui/wgpu_shader_app.slint", config)
        .expect("Failed to compile slint ui");
}
