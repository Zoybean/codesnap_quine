fn main() {
    fn quot(q: &str) -> String {
        format!("\"{}\"", q.escape_default())
    }
    fn out(q: &str) {
        println!("fn main() {{\n    {}\n}}", q);
    }
    (|quine_closure: &str, quot_def: &str, out_def: &str| {
    out(&[
        quot_def, "\n    ",
        out_def, "\n    ",
        "(", quine_closure, ") (\n        ",
            &quot(quine_closure), ",\n        ",
            &quot(quot_def), ",\n        ",
            &quot(out_def), "\n    ",
        ")"
        ].concat());
    }) (
        "|quine_closure: &str, quot_def: &str, out_def: &str| {\n    out(&[\n        quot_def, \"\\n    \",\n        out_def, \"\\n    \",\n        \"(\", quine_closure, \") (\\n        \",\n            &quot(quine_closure), \",\\n        \",\n            &quot(quot_def), \",\\n        \",\n            &quot(out_def), \"\\n    \",\n        \")\"\n        ].concat());\n    }",
        "fn quot(q: &str) -> String {\n        format!(\"\\\"{}\\\"\", q.escape_default())\n    }",
        "fn out(q: &str) {\n        println!(\"fn main() {{\\n    {}\\n}}\", q);\n    }"
    )
}
