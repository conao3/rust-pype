use crate::types;

fn indent(s: &str) -> String {
    s.lines()
        .map(|x| format!("    {}", x))
        .collect::<Vec<String>>()
        .join("\n")
}

fn gen_atom(atom: &types::LispAtom) -> String {
    match atom {
        types::LispAtom::Symbol(s) => match &**s {
            "t" => "True".to_string(),
            "nil" => "None".to_string(),
            "true" => "True".to_string(),
            "false" => "False".to_string(),
            _ => s.to_string(),
        },
        _ => atom.to_string(),
    }
}

fn gen_cons(car: &types::LispExpRef, cdr: &types::LispExpRef) -> String {
    let car_ptr = car.upgrade().unwrap();
    let fn_ = match &*car_ptr.borrow() {
        types::LispExp::Atom(types::LispAtom::Symbol(s)) => s.clone(),
        _ => panic!("WrongTypeArgument: atom; car"),
    };

    match &*fn_ {
        "kw" => gen_cons_kw(cdr),
        "call" => gen_cons_call(cdr),
        "for" => gen_cons_for(cdr),
        "with" => gen_cons_with(cdr),
        "progn" => gen_cons_progn(cdr),
        "attr" => gen_cons_attr(cdr),
        "assign" => gen_cons_assign(cdr),
        _ => panic!("Unknown function: {}", fn_),
    }
}

fn gen_cons_kw(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let mut arg_iter = args_ptr.borrow().iter();
    let v1 = arg_iter.next().unwrap();
    let v2 = arg_iter.next().unwrap();

    format!("{}={}", gen(&v1), gen(&v2))
}

fn gen_cons_call(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let mut arg_iter = args_ptr.borrow().iter();
    let fn_ = arg_iter.next().unwrap();
    let fn_args_str = arg_iter.map(|x| gen(&x)).collect::<Vec<_>>().join(", ");

    format!("{}({})", gen(&fn_), fn_args_str)
}

fn gen_cons_for(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let mut arg_iter = args_ptr.borrow().iter();
    let v1 = arg_iter.next().unwrap();
    let v2 = arg_iter.next().unwrap();
    let v3 = arg_iter.next().unwrap();

    format!("for {} in {}:\n{}", gen(&v1), gen(&v2), indent(&gen(&v3)))
}

fn gen_cons_with(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let mut arg_iter = args_ptr.borrow().iter();
    let v1 = arg_iter.next().unwrap();
    let v2 = arg_iter.next().unwrap();
    let v3 = arg_iter.next().unwrap();

    format!("with {} as {}:\n{}", gen(&v1), gen(&v2), indent(&gen(&v3)))
}

fn gen_cons_progn(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let arg_iter = args_ptr.borrow().iter();
    arg_iter.map(|x| gen(&x)).collect::<Vec<_>>().join("\n")
}

fn gen_cons_attr(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let mut arg_iter = args_ptr.borrow().iter();
    let v1 = arg_iter.next().unwrap();
    let v2 = arg_iter.next().unwrap();

    format!("{}.{}", gen(&v1), gen(&v2))
}

fn gen_cons_assign(args: &types::LispExpRef) -> String {
    let args_ptr = args.upgrade().unwrap();

    let mut arg_iter = args_ptr.borrow().iter();
    let v1 = arg_iter.next().unwrap();
    let v2 = arg_iter.next().unwrap();

    format!("{} = {}", gen(&v1), gen(&v2))
}

pub fn gen(exp: &types::LispExpRef) -> String {
    let exp_ptr = exp.upgrade().unwrap();
    let x = match &*exp_ptr.borrow() {
        types::LispExp::Atom(atom) => gen_atom(atom),
        types::LispExp::Cons { car, cdr } => gen_cons(car, cdr),
    };
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_atom() {
        assert_eq!(
            gen_atom(&types::LispAtom::new_symbol("t")),
            "True".to_string()
        );
        assert_eq!(
            gen_atom(&types::LispAtom::new_symbol("nil")),
            "None".to_string()
        );
        assert_eq!(
            gen_atom(&types::LispAtom::new_symbol("true")),
            "True".to_string()
        );
        assert_eq!(
            gen_atom(&types::LispAtom::new_symbol("false")),
            "False".to_string()
        );
        assert_eq!(
            gen_atom(&types::LispAtom::new_symbol("foo")),
            "foo".to_string()
        );
        assert_eq!(gen_atom(&types::LispAtom::Int(1)), "1".to_string());
        assert_eq!(gen_atom(&types::LispAtom::Float(1.1)), "1.1".to_string());
        assert_eq!(
            gen_atom(&types::LispAtom::String("foo".to_string())),
            "\"foo\"".to_string()
        );
    }

    #[test]
    fn test_gen_cons_kw() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("kw").into());
        let c2 = arena.alloc(types::LispAtom::new_symbol("end").into());
        let c3 = arena.alloc("".into());

        let e1 = crate::alloc!(arena, [c1, c2, c3]);
        assert_eq!(gen(&e1), "end=\"\"".to_string());
    }

    #[test]
    fn test_gen_cons_call() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("kw").into());
        let c2 = arena.alloc(types::LispAtom::new_symbol("end").into());
        let c3 = arena.alloc("".into());
        let c4 = arena.alloc(types::LispAtom::new_symbol("call").into());
        let c5 = arena.alloc(types::LispAtom::new_symbol("print").into());
        let c6 = arena.alloc(types::LispAtom::new_symbol("line").into());

        let e1 = crate::alloc!(arena, [c1, c2, c3]);
        let e2 = crate::alloc!(arena, [c4, c5, c6, e1]);

        assert_eq!(gen(&e2), "print(line, end=\"\")".to_string());
    }

    #[test]
    fn test_gen_cons_for() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("kw").into());
        let c2 = arena.alloc(types::LispAtom::new_symbol("end").into());
        let c3 = arena.alloc("".into());
        let c4 = arena.alloc(types::LispAtom::new_symbol("call").into());
        let c5 = arena.alloc(types::LispAtom::new_symbol("print").into());
        let c6 = arena.alloc(types::LispAtom::new_symbol("line").into());
        let c7 = arena.alloc(types::LispAtom::new_symbol("for").into());
        let c8 = arena.alloc(types::LispAtom::new_symbol("line").into());
        let c9 = arena.alloc(types::LispAtom::new_symbol("f").into());

        let e1 = crate::alloc!(arena, [c1, c2, c3]);
        let e2 = crate::alloc!(arena, [c4, c5, c6, e1]);
        let e3 = crate::alloc!(arena, [c7, c8, c9, e2]);

        let expect = "\
for line in f:
    print(line, end=\"\")";
        assert_eq!(gen(&e3), expect.to_string());
    }

    #[test]
    fn test_gen_cons_with() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("kw").into());
        let c2 = arena.alloc(types::LispAtom::new_symbol("end").into());
        let c3 = arena.alloc("".into());
        let c4 = arena.alloc(types::LispAtom::new_symbol("call").into());
        let c5 = arena.alloc(types::LispAtom::new_symbol("print").into());
        let c6 = arena.alloc(types::LispAtom::new_symbol("line").into());
        let c7 = arena.alloc(types::LispAtom::new_symbol("for").into());
        let c8 = arena.alloc(types::LispAtom::new_symbol("line").into());
        let c9 = arena.alloc(types::LispAtom::new_symbol("f").into());
        let c10 = arena.alloc(types::LispAtom::new_symbol("with").into());
        let c11 = arena.alloc(types::LispAtom::new_symbol("open").into());
        let c12 = arena.alloc("./temp".into());

        let e1 = crate::alloc!(arena, [c1, c2, c3]);
        let e2 = crate::alloc!(arena, [c4, c5, c6, e1]);
        let e3 = crate::alloc!(arena, [c7, c8, c9, e2]);
        let e4 = crate::alloc!(arena, [c10, [c4, c11, c12], c9, e3]);

        let expect = "\
with open(\"./temp\") as f:
    for line in f:
        print(line, end=\"\")";
        assert_eq!(gen(&e4), expect.to_string());
    }

    #[test]
    fn test_gen_progn() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("progn").into());

        let args = ["print(\"hello\")", "print(\"world\")"];
        let mut cur = crate::alloc!(arena, []);
        for arg in args.iter().rev() {
            let c2 = arena.alloc(types::LispAtom::new_raw_text(*arg).into());
            cur = crate::alloc!(arena, [c2; cur]);
        }

        let e1 = crate::alloc!(arena, [c1; cur]);
        let expect = "\
print(\"hello\")
print(\"world\")";
        assert_eq!(gen(&e1), expect.to_string());
    }

    #[test]
    fn test_gen_attr() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("attr").into());
        let c2 = arena.alloc(types::LispAtom::new_symbol("foo").into());
        let c3 = arena.alloc(types::LispAtom::new_symbol("bar").into());

        let e1 = crate::alloc!(arena, [c1, c2, c3]);
        assert_eq!(gen(&e1), "foo.bar".to_string());
    }

    #[test]
    fn test_gen_attr_call() {
        let mut arena = types::LispArena::default();
        let c1 = arena.alloc(types::LispAtom::new_symbol("attr").into());
        let c2 = arena.alloc(types::LispAtom::new_symbol("foo").into());
        let c3 = arena.alloc(types::LispAtom::new_symbol("bar").into());
        let c4 = arena.alloc(types::LispAtom::new_symbol("call").into());

        let e1 = crate::alloc!(arena, [c1, c2, c3]);
        let e2 = crate::alloc!(arena, [c4, e1]);

        assert_eq!(gen(&e2), "foo.bar()".to_string());
    }
}
