use pype::types::*;

#[test]
fn test() {
    let mut arena = LispArena::default();
    let v1 = arena.alloc(LispExp::new_symbol("call"));
    let v2 = arena.alloc(LispExp::new_symbol("print"));
    let v3 = arena.alloc(LispExp::new_symbol("line"));
    let v4 = arena.alloc(LispExp::new_symbol("kw"));
    let v5 = arena.alloc(LispExp::new_symbol("end"));
    let v6 = arena.alloc("".into());

    let e1 = pype::alloc!(arena, [v1, v2, v3, [v4, v5, v6]]);
    assert_eq!(
        e1.upgrade().unwrap().borrow().to_string(),
        "(call print line (kw end \"\"))"
    );
}

#[test]
fn test_stmt1() {
    let mut arena = LispArena::default();
    let v1 = arena.alloc(LispExp::new_symbol("call"));
    let v2 = arena.alloc(LispExp::new_symbol("print"));
    let v3 = arena.alloc(LispExp::new_symbol("line"));
    let v4 = arena.alloc(LispExp::new_symbol("kw"));
    let v5 = arena.alloc(LispExp::new_symbol("end"));
    let v6 = arena.alloc("".into());
    let v7 = arena.alloc(LispExp::new_symbol("for"));
    let v8 = arena.alloc(LispExp::new_symbol("line"));
    let v9 = arena.alloc(LispExp::new_symbol("f"));

    let e1 = pype::alloc!(arena, [v1, v2, v3, [v4, v5, v6]]);
    let e2 = pype::alloc!(arena, [v7, v8, v9, e1]);
    assert_eq!(
        e1.upgrade().unwrap().borrow().to_string(),
        "(call print line (kw end \"\"))"
    );
    assert_eq!(
        e2.upgrade().unwrap().borrow().to_string(),
        "(for line f (call print line (kw end \"\")))"
    );
}
