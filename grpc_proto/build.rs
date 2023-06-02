

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true) // Generates client
        .build_server(true) // Generates server
        .out_dir("src/helloworld")
        .compile(&["proto/helloworld.proto"], &["proto"])?;
    Ok(())
}