fn main() {
    let mut protos = Vec::new();
    // ???? ?????-????? ?? ???? ????????? ?????? ???????
    let search_paths = ["../chudo-core/proto", "../chudo-messenger/proto", "proto"];
    let mut include_dirs = Vec::new();

    for path in search_paths {
        if std::path::Path::new(path).exists() {
            include_dirs.push(path);
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if entry.path().extension().map_or(false, |ext| ext == "proto") {
                        protos.push(entry.path());
                    }
                }
            }
        }
    }

    if !protos.is_empty() {
        tonic_build::configure()
            .compile(&protos, &include_dirs)
            .expect("Failed to compile protos");
    }
}
