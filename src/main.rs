#![allow(unused_imports)]

use pype::{gen_python, generator, types};

use std::{
    fs,
    io::{self, Write},
    process,
};

fn argparse() -> (getopts::Options, getopts::Matches) {
    let mut opts = getopts::Options::new();

    opts.opt(
        "h",
        "help",
        "print this help menu",
        "",
        getopts::HasArg::No,
        getopts::Occur::Optional,
    );
    opts.opt(
        "v",
        "version",
        "print the version",
        "",
        getopts::HasArg::No,
        getopts::Occur::Optional,
    );
    opts.opt(
        "e",
        "",
        "one line of program",
        "command",
        getopts::HasArg::Yes,
        getopts::Occur::Multi,
    );
    opts.opt(
        "n",
        "",
        "iterate over lines",
        "",
        getopts::HasArg::No,
        getopts::Occur::Optional,
    );
    opts.opt(
        "l",
        "",
        "strip trailing newline",
        "",
        getopts::HasArg::No,
        getopts::Occur::Optional,
    );

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
    let (opts, args) = argparse();

    // no -e: stdin is python code.  Just copy it to stdout.
    if !args.opt_present("e") {
        let r = io::stdin();
        let w = io::stdout();
        let mut r = r.lock();
        let mut w = w.lock();
        let _ = io::copy(&mut r, &mut w);
        return;
    }

    let tmp_dir = tempfile::tempdir().unwrap();
    let pid = process::id();
    let fifo_path = tmp_dir.path().join(format!("pype__{pid}.fifo"));
    nix::unistd::mkfifo(&fifo_path, nix::sys::stat::Mode::S_IRWXU).unwrap();

    let fifo_path_str = fifo_path.to_str().unwrap();

    let mut arena = types::LispArena::default();
    let c11 = arena.alloc(types::LispAtom::new_symbol("with").into());
    let c12 = arena.alloc(types::LispAtom::new_symbol("call").into());
    let c13 = arena.alloc(types::LispAtom::new_symbol("open").into());
    let c14 = arena.alloc(fifo_path_str.into());
    let c15 = arena.alloc(types::LispAtom::new_symbol("f").into());

    let e = gen_python::do_e(&opts, &args, &mut arena);
    let e = gen_python::do_l(e, &opts, &args, &mut arena);
    let e = gen_python::do_n(e, &opts, &args, &mut arena);
    let e = gen_python::do_l_post(e, &opts, &args, &mut arena);
    let e = pype::alloc!(arena, [c11, [c12, c13, c14], c15, e]);

    println!("{}", generator::gen(&e));

    io::stdout().flush().unwrap();
    nix::unistd::close(1).unwrap();

    let r = io::stdin();
    let mut reader = r.lock();
    let mut w = fs::File::create(&fifo_path).unwrap();

    // TODO: receive C-c, cleanup tempdir
    _ = io::copy(&mut reader, &mut w);
}
