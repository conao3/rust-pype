use crate::types;

pub fn do_inpt(
    cur: types::LispExpRef,
    fifo_path_str: &str,
    _opts: &getopts::Options,
    _args: &getopts::Matches,
    arena: &mut types::LispArena,
) -> types::LispExpRef {
    let s_with = arena.alloc(types::LispAtom::new_symbol("with").into());
    let s_call = arena.alloc(types::LispAtom::new_symbol("call").into());
    let s_open = arena.alloc(types::LispAtom::new_symbol("open").into());
    let s_f = arena.alloc(types::LispAtom::new_symbol("f").into());
    let v_fifo_path = arena.alloc(fifo_path_str.into());

    crate::alloc!(arena, [s_with, [s_call, s_open, v_fifo_path], s_f, cur])
}

pub fn do_e(
    _opts: &getopts::Options,
    args: &getopts::Matches,
    arena: &mut types::LispArena,
) -> types::LispExpRef {
    let commands = args.opt_strs("e");
    let s_progn = arena.alloc(types::LispAtom::new_symbol("progn").into());

    let mut cur = crate::alloc!(arena, []);
    for cmd in commands.iter().rev() {
        let v = arena.alloc(types::LispAtom::new_raw_text(cmd).into());
        cur = crate::alloc!(arena, [v; cur]);
    }
    crate::alloc!(arena, [s_progn; cur])
}

pub fn do_n(
    cur: types::LispExpRef,
    _opts: &getopts::Options,
    args: &getopts::Matches,
    arena: &mut types::LispArena,
) -> types::LispExpRef {
    match args.opt_present("n") {
        true => {
            let s_for = arena.alloc(types::LispAtom::new_symbol("for").into());
            let s_line = arena.alloc(types::LispAtom::new_symbol("line").into());
            let s_f = arena.alloc(types::LispAtom::new_symbol("f").into());
            crate::alloc!(arena, [s_for, s_line, s_f, cur])
        }
        false => cur,
    }
}

pub fn do_l(
    cur: types::LispExpRef,
    _opts: &getopts::Options,
    args: &getopts::Matches,
    arena: &mut types::LispArena,
) -> types::LispExpRef {
    match args.opt_present("l") {
        true => {
            let s_progn = arena.alloc(types::LispAtom::new_symbol("progn").into());
            let s_assign = arena.alloc(types::LispAtom::new_symbol("assign").into());
            let s_line = arena.alloc(types::LispAtom::new_symbol("line").into());
            let s_call = arena.alloc(types::LispAtom::new_symbol("call").into());
            let s_attr = arena.alloc(types::LispAtom::new_symbol("attr").into());
            let s_rstrip = arena.alloc(types::LispAtom::new_symbol("rstrip").into());
            crate::alloc!(
                arena,
                [
                    s_progn,
                    [s_assign, s_line, [s_call, [s_attr, s_line, s_rstrip]]],
                    cur
                ]
            )
        }
        false => cur,
    }
}

pub fn do_l_post(
    cur: types::LispExpRef,
    _opts: &getopts::Options,
    args: &getopts::Matches,
    arena: &mut types::LispArena,
) -> types::LispExpRef {
    match args.opt_present("l") {
        true => cur,
        false => {
            let s_progn = arena.alloc(types::LispAtom::new_symbol("progn").into());
            let s_import = arena.alloc(types::LispAtom::new_symbol("import").into());
            let s_assign = arena.alloc(types::LispAtom::new_symbol("assign").into());
            let s_star = arena.alloc(types::LispAtom::new_symbol("*").into());
            let s_dstar = arena.alloc(types::LispAtom::new_symbol("**").into());
            let s_kw = arena.alloc(types::LispAtom::new_symbol("kw").into());
            let s_attr = arena.alloc(types::LispAtom::new_symbol("attr").into());
            let s_call = arena.alloc(types::LispAtom::new_symbol("call").into());

            let s_builtins = arena.alloc(types::LispAtom::new_symbol("builtins").into());
            let s_print = arena.alloc(types::LispAtom::new_symbol("print").into());
            let s_lambda = arena.alloc(types::LispAtom::new_symbol("lambda").into());
            let s_args = arena.alloc(types::LispAtom::new_symbol("args").into());
            let s_kwargs = arena.alloc(types::LispAtom::new_symbol("kwargs").into());
            let s_end = arena.alloc(types::LispAtom::new_symbol("end").into());
            let v_empty = arena.alloc("".into());

            let stmt_import = crate::alloc!(arena, [s_import, s_builtins]);
            let exp_print = crate::alloc!(
                arena,
                [
                    s_call,
                    [s_attr, s_builtins, s_print],
                    [s_star, s_args],
                    [s_dstar, s_kwargs],
                    [s_kw, s_end, v_empty]
                ]
            );
            let exp_lambda = crate::alloc!(
                arena,
                [s_lambda, [[s_star, s_args], [s_dstar, s_kwargs]], exp_print]
            );
            let stmt_assign = crate::alloc!(arena, [s_assign, s_print, exp_lambda]);
            crate::alloc!(arena, [s_progn, stmt_import, stmt_assign, cur])
        }
    }
}
