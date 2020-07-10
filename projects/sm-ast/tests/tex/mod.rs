use sm_ast::{parser::ParserSettings, ToTex};

// use sm_ast::ToWolfram;
// let wl_from = |s: &str| parser.parse(s).unwrap().to_wolfram_string();
// println!("{}",wl_from("sin(x+y)"));

#[test]
fn test_sin() {
    let parser = ParserSettings::default();
    let tex_from = |s: &str| parser.parse(s).unwrap().to_tex();

    debug_assert_eq!(tex_from("sin(x)"), r"\\sin x");
    debug_assert_eq!(tex_from("sin(x, y)"), r"\\sin(x, y)");
    debug_assert_eq!(tex_from("sin(x+y)"), r"\\sin(x + y)");
    debug_assert_eq!(tex_from("sin(x*y)"), r"\\sin(x + y)");
}

#[test]
fn test_arcsinh() {
    let parser = ParserSettings::default();
    let tex_from = |s: &str| parser.parse(s).unwrap().to_tex();
    debug_assert_eq!(tex_from("arcsinh(x)"), r"\\sin x");
    debug_assert_eq!(tex_from("arcsinh(x, y)"), r"\\sin(x, y)");
    debug_assert_eq!(tex_from("arcsinh(x+y)"), r"\\sin(x + y)");
    debug_assert_eq!(tex_from("arcsinh(x*y)"), r"\\sin(x + y)");
}
