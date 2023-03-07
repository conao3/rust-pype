use crate::types;

pub fn do_e(_opts: getopts::Options, args: getopts::Matches, arena: &mut types::LispArena) -> types::LispExpRef {
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
        },
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
