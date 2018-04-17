use rustc::hir;
use rustc::lint::*;
use rustc::ty::TypeVariants;
use std::f32;
use std::f64;
use std::iter::Enumerate;
use std::str::Chars;
use syntax::ast::*;
use syntax::symbol::InternedString;
use syntax_pos::symbol::Symbol;
use syntax_pos::Span;
use utils::span_lint_and_sugg;

/// **What it does:** Checks for float literals with a precision greater
/// than that supported by the underlying type
///
/// **Why is this bad?** Rust will truncate the literal silently.
///
/// **Known problems:** None.
///
/// **Example:**
///
/// ```rust
/// // Bad
/// Insert a short example of code that triggers the lint
///    let v: f32 = 0.123_456_789_9;
///    println!("{}", v); //  0.123_456_789
///
/// // Good
/// Insert a short example of improved code that doesn't trigger the lint
///    let v: f64 = 0.123_456_789_9;
///    println!("{}", v); //  0.123_456_789_9
/// ```
declare_lint! {
    pub EXCESSIVE_PRECISION,
    Warn,
    "excessive precision for float literal"
}

pub struct ExcessivePrecision;

impl LintPass for ExcessivePrecision {
    fn get_lints(&self) -> LintArray {
        lint_array!(EXCESSIVE_PRECISION)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for ExcessivePrecision {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx hir::Expr) {
        if_chain! {
            let ty = cx.tables.expr_ty(expr);
            if let TypeVariants::TyFloat(ref fty) = ty.sty;
            if let hir::ExprLit(ref lit) = expr.node;
            if let LitKind::Float(ref sym, _) | LitKind::FloatUnsuffixed(ref sym) = lit.node;
            if let Some(sugg) = self.check(sym, fty);
            then {
                span_lint_and_sugg(
                    cx,
                    EXCESSIVE_PRECISION,
                    expr.span,
                    "float has excessive precision",
                    "consider truncating it",
                    sugg,
                );
            }
        }
    }
}

impl ExcessivePrecision {
    fn check(&self, sym: &Symbol, fty: &FloatTy) -> Option<String> {
        let max = max_digits(fty);
        let sym_str = sym.as_str();
        let (l, r) = split_at_digit(&sym_str, max);

        Some(l.to_string())
    }
}

fn round(x: &str, y: &str) -> String {
    if should_round_up(y) {
        round_up(x)
    } else {
        x.to_string()
    }
}
fn should_round_up(s: &str) -> bool {
    for c in s.chars() {
        if c == '5' || c == '_' || c == '.' {
            continue;
        }
        if c == 'f' {
            // TODO fine to just trunc?
            return false;
        }

        match c.to_digit(10) {
            None => return false,
            Some(x) => {
                if x > 5 {
                    return true;
                } else {
                    return false;
                }
            },
        }
    }
    false
}

fn round_up(s: &str) -> String {
    let res = s.get(..s.len()).unwrap().to_string();
    let last = s.chars().last().unwrap();
    let mut digit = last.to_digit(10).unwrap();
    digit += 1;
    s.to_string()
}

fn split_at_digit<'a>(s: &'a InternedString, n: u32) -> (&'a str, &'a str) {
    let ds = Digits::new(s);
    for (di, (i, ref c)) in ds.enumerate() {
        if di > n as usize {
            return s.split_at(i);
        }
    }
    (&s, "")
}

fn max_digits(fty: &FloatTy) -> u32 {
    match fty {
        FloatTy::F32 => f32::DIGITS,
        FloatTy::F64 => f64::DIGITS,
    }
}

struct Digits<'a> {
    index: u32,
    predecimal: bool,
    chars: Enumerate<Chars<'a>>,
}

impl<'a> Digits<'a> {
    fn new(s: &'a str) -> Self {
        Digits {
            index: 0,
            predecimal: true,
            chars: s.chars().enumerate(),
        }
    }
}

impl<'a> Iterator for Digits<'a> {
    type Item = (usize, char);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((i, c)) => {
                    if c == '0' && self.index == 0 && self.predecimal || c == '_' {
                        continue;
                    } else if c == '.' {
                        self.predecimal = false;
                        continue;
                    } else if c == 'f' {
                        // f32/f64 suffix
                        return None;
                    } else {
                        self.index += 1;
                        return Some((i, c));
                    }
                },
                None => return None,
            }
        }
    }
}
