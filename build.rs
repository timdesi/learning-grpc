fn main() {
    let proto_file = "./proto/hello.proto"; 
    let proto_file2 = "./proto/downloader.proto"; 

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file, proto_file2], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
  
        println!("cargo:rerun-if-changed={}", proto_file);
}