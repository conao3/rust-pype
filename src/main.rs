#![allow(unused_imports)]

use pype::types;

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
    // let print_expr = types::expression::PyCall {
    //     function: Box::new(types::expression::PyIdent::from("print").into()),
    //     arguments: vec![
    //         types::expression::PyIdent::from("line").into(),
    //         ("end", "").into(),
    //     ],
    // };
    // let for_expr = types::statement::PyFor {
    //     targets: vec![types::expression::PyIdent::from("line")],
    //     iterator: Box::new(types::expression::PyIdent::from("f").into()),
    //     body: vec![print_expr],
    // };
    // let program = types::statement::PyProgram {
    //     statements: vec![
    //         types::statement::PyWith {
    //             expression: types::expression::PyCall {
    //                 function: types::expression::PyIdent {
    //                     ident: "open".to_string(),
    //                 },
    //                 arguments: vec![types::expression::PyString {
    //                     string: fifo_path_str.to_string(),
    //                 }],
    //             },
    //             target: types::expression::PyIdent {
    //                 ident: "f".to_string(),
    //             },
    //             body: vec![
    //                 types::statement::PyFor {
    //                     target: types::expression::PyIdent {
    //                         ident: "line".to_string(),
    //                     },
    //                     iterator: types::expression::PyCall {
    //                         function: types::expression::PyIdent {
    //                             ident: "f".to_string(),
    //                         },
    //                         arguments: vec![],
    //                     },
    //                     body: vec![types::statement::PyPrint {
    //                         expression: types::expression::PyCall {
    //                             function: types::expression::PyIdent {
    //                                 ident: "print".to_string(),
    //                             },
    //                             arguments: vec![
    //                                 types::expression::PyIdent {
    //                                     ident: "line".to_string(),
    //                                 },
    //                                 types::PyKeywordArgument {
    //                                     name: "end".to_string(),
    //                                     expression: types::expression::PyString {
    //                                         string: "".to_string(),
    //                                     },
    //                                 },
    //                             ],
    //                         },
    //                     }],
    //                 },
    //             ]
    //         }
    //     ]
    // };
    io::stdout().flush().unwrap();
    nix::unistd::close(1).unwrap();

    let r = io::stdin();
    let mut reader = r.lock();
    let mut w = fs::File::create(&fifo_path).unwrap();

    // TODO: receive C-c, cleanup tempdir
    _ = io::copy(&mut reader, &mut w);
}
