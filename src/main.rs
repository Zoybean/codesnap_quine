
use codesnap::config::{CodeBuilder, CodeSnap};
use std::error::Error;

fn quote(s: &str) -> String {
    format!("r#{0}{s}{0}#", '"')
}
fn embed(defs: &str, body: &str) -> String {
    format!("{defs}fn main() -> Result<(), Box<dyn Error>> {{{body}}}")
}
fn print(s: &str) -> Result<(), Box<dyn Error>> {
    println!("{s}");
    Ok(())
}
fn code_snap(s: &str) -> Result<(), Box<dyn Error>> {
    CodeSnap::default()
        .code(CodeBuilder::default().language("rust").content(s).build()?)
        .build()?
        .create_snapshot()?
        .png_data()?
        .save("./snap.png")?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    (|quine: &str, def: &str| {
        code_snap(&embed(
            def,
            &[quine, "(", &quote(quine), ", ", &quote(def), ")"].concat(),
        ))
    })(r#"
    (|quine: &str, def: &str| {
        code_snap(&embed(
            def,
            &[quine, "(", &quote(quine), ", ", &quote(def), ")"].concat(),
        ))
    })"#, r#"
use codesnap::config::{CodeBuilder, CodeSnap};
use std::error::Error;

fn quote(s: &str) -> String {
    format!("r#{0}{s}{0}#", '"')
}
fn embed(defs: &str, body: &str) -> String {
    format!("{defs}fn main() -> Result<(), Box<dyn Error>> {{{body}}}")
}
fn print(s: &str) -> Result<(), Box<dyn Error>> {
    println!("{s}");
    Ok(())
}
fn code_snap(s: &str) -> Result<(), Box<dyn Error>> {
    CodeSnap::default()
        .code(CodeBuilder::default().language("rust").content(s).build()?)
        .build()?
        .create_snapshot()?
        .png_data()?
        .save("./snap.png")?;
    Ok(())
}
"#)}
