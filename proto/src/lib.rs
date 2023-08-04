

pub mod helloworld_pb {
    include!("./helloworld/helloworld.rs");
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("./helloworld/helloworld_descriptor.bin");
}

pub mod cachekv_pb {
    include!("./cachekv/cachekv.rs");
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("./cachekv/cachekv_descriptor.bin");
}

