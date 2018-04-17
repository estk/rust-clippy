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
            then {
                if self.check(sym, fty) {

                span_lint_and_sugg(
                    cx,
                    EXCESSIVE_PRECISION,
                    expr.span,
                    "float has excessive precision",
                    "consider truncating it",
                    "<rounded literal>".to_string(),
                );
                }
            }
        }
    }
}

impl ExcessivePrecision {
    fn check(&mut self, sym: &Symbol, fty: &FloatTy) -> bool {
        let max = max_digits(fty);
        let digits = count_digits(sym.as_str());
        digits > max
    }
}

fn count_digits(s: InternedString) -> u32 {
    let mut count = 0;
    let mut predecimal = true;
    for ref c in s.chars() {
        // leading zeros
        if *c == '0' && count == 0 && predecimal {
            continue;
        } else if *c == '.' {
            predecimal = false;
        } else if *c == '_' {
            continue;
        // f32/f64 suffix
        } else if *c == 'f' {
            break;
        } else {
            count += 1;
        }
    }
    count
}

fn max_digits(fty: &FloatTy) -> u32 {
    match fty {
        FloatTy::F32 => f32::DIGITS,
        FloatTy::F64 => f64::DIGITS,
    }
}
