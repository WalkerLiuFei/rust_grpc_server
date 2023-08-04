use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {



    for proto_file in fs::read_dir("protobuf")? {
        let file_name_with_suffix = proto_file?.file_name().to_string_lossy().to_string();
        let file_name =  file_name_with_suffix.split(".").collect::<Vec<&str>>()[0];
        // Ensure the directory exists
        let out_dir = format!("src/{}", file_name);
        fs::create_dir_all(&out_dir)?;
        tonic_build::configure()
            .build_client(true) // Generates gateway
            .build_server(true)
            .out_dir( format!("src/{}",file_name))
            .file_descriptor_set_path(format!("src/{}/{}_descriptor.bin",file_name,file_name))
            .compile(&[format!("protobuf/{}.proto",file_name)], &["protobuf"])?;
    }
    Ok(())
}