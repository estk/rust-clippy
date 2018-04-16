use rustc::lint::*;
use std::f32;
use std::f64;
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

impl LateLintPass for ExcessivePrecision {
    fn check_stmt(&mut self, cx: &LateContext, stmt: &Stmt) {
        match stmt.node {
            StmtKind::Local(ref s) => self.local_check(cx, s),
            StmtKind::Item(ref s) => self.item_check(cx, s),
            StmtKind::Expr(ref s) => self.expr_check(cx, s),
            _ => (),
        }
    }
}

impl ExcessivePrecision {
    fn check(&mut self, cx: &LateContext, sym: &Symbol, fty: &FloatTy) -> bool {
        println!("checking {} with type {}", sym, fty);
        let max = max_digits(fty);
        let digits = count_digits(sym.as_str());
        digits > max
    }

    // const foo = 0.123...
    fn item_check(&mut self, cx: &LateContext, item: &Item) {
        if_chain! {
            if let ItemKind::Const(ref ty, ref expr) = item.node;
            if let Some(ref fty) = from_ty(ty);
            if let Some((sym, _)) = extract_float_literal(expr);
            then {
                if self.check(cx, sym, fty) {
                    perform_lint(cx, expr.span, fty);
                }
            }
        }
    }

    // println!("{}", .99999999999999999999)
    fn expr_check(&mut self, cx: &LateContext, expr: &Expr) {
        if_chain! {
            if let Some((sym, Some(fty))) = extract_float_literal(expr);
            then {
                self.check(cx, sym, fty);
            }
        }
    }

    // We cant just check the expr, we need to have the type assignment
    // so that we know the float precision.
    // let foo = 0.123...
    fn local_check(&mut self, cx: &LateContext, local: &Local) {
        if_chain! {
            if let Some(ref exptr) = local.init;
            if let ExprKind::Lit(ref lit) = exptr.node;
            if let LitKind::Float(ref sym, _) | LitKind::FloatUnsuffixed(ref sym) = lit.node;
            if let Some(ref ty)  = local.ty;
            if let Some(ref fty) = from_ty(ty);
            then {
                if self.check(cx, sym, fty) {
                    perform_lint(cx, lit.span, fty);
                }
                println!();
            }
        }
    }
}

fn perform_lint(cx: &LateContext, span: Span, fty: &FloatTy) {
    match fty {
        FloatTy::F32 => {
            // TODO check can fit in f64?
            span_lint_and_sugg(
                cx,
                EXCESSIVE_PRECISION,
                span,
                "float has excessive precision",
                "consider making it a f64",
                "<expression with type f64>".to_string(),
            );
        },
        FloatTy::F64 => {
            span_lint_and_sugg(
                cx,
                EXCESSIVE_PRECISION,
                span,
                "float has excessive precision",
                "consider truncating it",
                "<truncated float>".to_string(),
            );
        },
    }
}

fn max_digits(fty: &FloatTy) -> u32 {
    match fty {
        FloatTy::F32 => f32::DIGITS,
        FloatTy::F64 => f64::DIGITS,
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

fn extract_float_literal<'a>(expr: &'a Expr) -> Option<(&'a Symbol, Option<&'a FloatTy>)> {
    if let ExprKind::Lit(ref lit) = expr.node {
        return match lit.node {
            LitKind::Float(ref sym, ref fty) => Some((sym, Some(fty))),
            LitKind::FloatUnsuffixed(ref sym) => Some((sym, None)),
            _ => None,
        };
    }
    None
}

fn from_ty(ty: &Ty) -> Option<FloatTy> {
    if_chain! {
        if let TyKind::Path(_, ref pth) = ty.node;
        let ref segs = pth.segments;
        if let ref seg = segs[0];
        then {
            let id = seg.ident;
            let name = id.name.as_str().to_lowercase();
            if name == "f32" {
                return Some(FloatTy::F32)
            } else if name == "f64" {
                return Some(FloatTy::F64)
            }
        }
    }
    None
}
