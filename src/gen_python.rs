use crate::types;

pub fn do_e(
    _opts: &getopts::Options,
    args: &getopts::Matches,
    arena: &mut types::LispArena,
) -> types::LispExpRef {
    let commands = args.opt_strs("e");

    match commands.len() {
        0 => {
            let s_call = arena.alloc(types::LispAtom::new_symbol("call").into());
            let s_print = arena.alloc(types::LispAtom::new_symbol("print").into());
            let s_line = arena.alloc(types::LispAtom::new_symbol("line").into());
            let s_kw = arena.alloc(types::LispAtom::new_symbol("kw").into());
            let s_end = arena.alloc(types::LispAtom::new_symbol("end").into());
            let v_empty = arena.alloc("".into());

            crate::alloc!(arena, [s_call, s_print, s_line, [s_kw, s_end, v_empty]])
        }
        _ => {
            let s_progn = arena.alloc(types::LispAtom::new_symbol("progn").into());
            let mut cur = crate::alloc!(arena, []);
            for cmd in commands.iter().rev() {
                let v = arena.alloc(types::LispAtom::new_raw_text(cmd).into());
                cur = crate::alloc!(arena, [v; cur]);
            }
            crate::alloc!(arena, [s_progn; cur])
        }
    }
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
