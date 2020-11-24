extern crate protoc_rust;

fn main() -> std::io::Result<()> {
    let mut protos = vec![];
    for file in std::fs::read_dir("protos")? {
        protos.push(file?.path());
    }

    protoc_rust::Codegen::new()
        .out_dir("src/generated")
        .inputs(&protos)
        .include("protos")
        .run()
        .expect("protoc");

    Ok(())
}
