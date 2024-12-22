fn main () { fn quot(q: &str) -> String { format!("\"{}\"", q.escape_default()) }fn out(q: &str) { println!("fn main () {{ {} }}", q); }(|quine_closure: &str, quot_def: &str, out_def: &str| {
        let ls = [
            quot_def,
            out_def,
            "(",
            quine_closure,
            ")",
            "(",
            &quot(quine_closure),
            ",",
            &quot(quot_def),
            ",",
            &quot(out_def),
            ",",
            ")",
        ];

        let mut s = String::new();
        for st in ls {
            s += st
        }
        out(&s);
    })("|quine_closure: &str, quot_def: &str, out_def: &str| {\n        let ls = [\n            quot_def,\n            out_def,\n            \"(\",\n            quine_closure,\n            \")\",\n            \"(\",\n            &quot(quine_closure),\n            \",\",\n            &quot(quot_def),\n            \",\",\n            &quot(out_def),\n            \",\",\n            \")\",\n        ];\n\n        let mut s = String::new();\n        for st in ls {\n            s += st\n        }\n        out(&s);\n    }","fn quot(q: &str) -> String { format!(\"\\\"{}\\\"\", q.escape_default()) }","fn out(q: &str) { println!(\"fn main () {{ {} }}\", q); }",) }
