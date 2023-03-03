use pype::types::*;

#[test]
fn test() {
    let mut arena = LispArena::default();
    let c1 = arena.alloc(1.into());
    let c2 = arena.alloc("foo".into());
    let c3 = arena.alloc(LispExp::new_symbol("bar"));

    let e1 = pype::alloc!(arena, [c1, c2, c3]);
}
