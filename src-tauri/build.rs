fn main() {
    // Compile Cap'n Proto schemas
    capnpc::CompilerCommand::new()
        .src_prefix("../schemas")
        .file("../schemas/market_data.capnp")
        .run()
        .expect("Failed to compile Cap'n Proto schema");

    tauri_build::build()
}
