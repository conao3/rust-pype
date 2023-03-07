#![allow(unused_imports)]

use pype::{generator, types};

use std::{
    fs,
    io::{self, Write},
    process,
};

fn argparse() -> (getopts::Options, getopts::Matches) {
    let mut opts = getopts::Options::new();

    opts.opt("h", "help", "print this help menu", "", getopts::HasArg::No, getopts::Occur::Optional);
    opts.opt("v", "version", "print the version", "", getopts::HasArg::No, getopts::Occur::Optional);
    opts.opt("e", "", "one line of program", "command", getopts::HasArg::Yes, getopts::Occur::Multi);

    let args = match opts.parse(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            eprint!("{}", opts.usage("Usage: pype [options]"));
            process::exit(1);
        }
    };

    if args.opt_present("help") {
        print!("{}", opts.usage("Usage: pype [options]"));
        process::exit(0);
    }

    if args.opt_present("version") {
        println!("pype {}", env!("CARGO_PKG_VERSION"));
        print!("{}", opts.usage("Usage: pype [options]"));
        process::exit(0);
    }

    (opts, args)
}

fn main() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let pid = process::id();
    let fifo_path = tmp_dir.path().join(format!("pype__{pid}.fifo"));
    nix::unistd::mkfifo(&fifo_path, nix::sys::stat::Mode::S_IRWXU).unwrap();

    let fifo_path_str = fifo_path.to_str().unwrap();

    let (_opts, args) = argparse();

    let mut arena = types::LispArena::default();
    let c11 = arena.alloc(types::LispAtom::new_symbol("with").into());
    let c12 = arena.alloc(types::LispAtom::new_symbol("call").into());
    let c13 = arena.alloc(types::LispAtom::new_symbol("open").into());
    let c14 = arena.alloc(fifo_path_str.into());
    let c15 = arena.alloc(types::LispAtom::new_symbol("f").into());

    let c21 = arena.alloc(types::LispAtom::new_symbol("for").into());
    let c22 = arena.alloc(types::LispAtom::new_symbol("line").into());
    let c23 = arena.alloc(types::LispAtom::new_symbol("f").into());


    let commands = args.opt_strs("e");
    let e2 = match commands.len() {
        0 => {
            let c31 = arena.alloc(types::LispAtom::new_symbol("call").into());
            let c32 = arena.alloc(types::LispAtom::new_symbol("print").into());
            let c33 = arena.alloc(types::LispAtom::new_symbol("line").into());

            let c41 = arena.alloc(types::LispAtom::new_symbol("kw").into());
            let c42 = arena.alloc(types::LispAtom::new_symbol("end").into());
            let c43 = arena.alloc("".into());

            let e1 = pype::alloc!(arena, [c41, c42, c43]);
            let e2 = pype::alloc!(arena, [c31, c32, c33, e1]);
            e2
        },
        _ => {
            let c51 = arena.alloc(types::LispAtom::new_symbol("progn").into());
            let mut e5 = pype::alloc!(arena, []);
            for cmd in commands.iter().rev() {
                let c52 = arena.alloc(types::LispAtom::new_raw_text(cmd).into());
                e5 = pype::alloc!(arena, [c52; e5]);
            }
            let e6 = pype::alloc!(arena, [c51; e5]);
            e6
        }
    };

    let e3 = pype::alloc!(arena, [c21, c22, c23, e2]);
    let e4 = pype::alloc!(arena, [c11, [c12, c13, c14], c15, e3]);

    println!("{}", generator::gen(&e4));

    io::stdout().flush().unwrap();
    nix::unistd::close(1).unwrap();

    let r = io::stdin();
    let mut reader = r.lock();
    let mut w = fs::File::create(&fifo_path).unwrap();

    // TODO: receive C-c, cleanup tempdir
    _ = io::copy(&mut reader, &mut w);
}
