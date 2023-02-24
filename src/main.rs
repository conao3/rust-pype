use std::{
    fs,
    io::{self, Write},
    process,
};

fn main() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let pid = process::id();
    let fifo_path = tmp_dir.path().join(format!("pype__{pid}.fifo"));
    nix::unistd::mkfifo(&fifo_path, nix::sys::stat::Mode::S_IRWXU).unwrap();

    let fifo_path_str = fifo_path.to_str().unwrap();
    print!(
        r###"
with open("{fifo_path_str}") as f:
    for line in f:
        print(line, end="")
"###
    );
    io::stdout().flush().unwrap();
    nix::unistd::close(1).unwrap();

    let r = io::stdin();
    let mut reader = r.lock();
    let mut w = fs::File::create(&fifo_path).unwrap();

    // TODO: receive C-c, cleanup tempdir
    _ = io::copy(&mut reader, &mut w);
}
