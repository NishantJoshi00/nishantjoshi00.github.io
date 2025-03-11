const FS: &str = "../dist/fs";
fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed={}", FS);
    let out_dir = std::env::var("OUT_DIR")?;

    let packed_fs = PackedFs::new();
    let packed_fs = packed_fs.pack()?;

    let dst_path = std::path::Path::new(&out_dir).join("fs.bin");

    std::fs::write(&dst_path, packed_fs)?;

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PackedFs {
    files: Vec<(String, Vec<u8>)>,
}

impl PackedFs {
    fn new() -> Self {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        dirs.push(std::path::PathBuf::from(FS));
        while let Some(dir) = dirs.pop() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    let name = path.strip_prefix(FS).unwrap().to_str().unwrap().to_string();
                    let extension = path.extension().unwrap().to_str().unwrap();
                    let data = match extension {
                        "png" | "jpg" | "jpeg" => {
                            vec![]
                        }
                        _ => std::fs::read(path).unwrap(),
                    };
                    files.push((name, data));
                }
            }
        }
        Self { files }
    }

    fn pack(&self) -> anyhow::Result<Vec<u8>> {
        Ok(bincode::serde::encode_to_vec(
            self,
            bincode::config::standard(),
        )?)
    }
}
