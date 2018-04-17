use rustc::hir;
use rustc::lint::*;
use rustc::ty::TypeVariants;
use std::f32;
use std::f64;
use std::iter::Enumerate;
use std::str::Chars;
use syntax::ast::*;
use syntax_pos::symbol::Symbol;
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
                    "consider changing the type or truncating it to",
                    sugg,
                );
            }
        }
    }
}

impl ExcessivePrecision {
    // TODO: need to check case where digits > Max but still ok
    // None if nothing to lint, Some(suggestion) if lint neccessary
    fn check(&self, sym: &Symbol, fty: &FloatTy) -> Option<String> {
        let max = max_digits(fty);
        let sym_str = sym.as_str();
        let digits = count_digits(&sym_str);
        // Try to bail out if the float is for sure fine.
        // If its within the 2 decimal digits of overflow we
        // check if the parsed representation is the same as the string
        // since we'll need the truncated string anyway.
        if digits > max as usize {
            let s = match *fty {
                FloatTy::F32 => {
                    let f = sym_str.parse::<f32>().unwrap();
                    f.to_string()
                },
                FloatTy::F64 => {
                    let f = sym_str.parse::<f64>().unwrap();
                    f.to_string()
                },
            };
            println!("got {}, have {}", sym_str, s);

            if sym_str == s {
                None
            } else {
                Some(s)
            }
        } else {
            None
        }
    }
}

fn max_digits(fty: &FloatTy) -> u32 {
    match fty {
        FloatTy::F32 => f32::DIGITS,
        FloatTy::F64 => f64::DIGITS,
    }
}

fn count_digits(s: &str) -> usize {
    Digits::new(s).collect::<Vec<_>>().len()
}

struct Digits<'a> {
    index: u32,
    chars: Enumerate<Chars<'a>>,
}
impl<'a> Digits<'a> {
    fn new(s: &'a str) -> Self {
        Digits {
            index: 0,
            chars: s.chars().enumerate(),
        }
    }
}
impl<'a> Iterator for Digits<'a> {
    type Item = (usize, char);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (i, c) = self.chars.next()?;
            // point char or leading zero
            if c == '.' || c == '0' && self.index == 0 {
                continue;
            } else {
                self.index += 1;
                return Some((i, c));
            }
        }
    }
}
