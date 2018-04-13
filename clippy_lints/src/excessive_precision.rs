use rustc::lint::*;
use syntax::ast::*;
use syntax::ptr::P;
use syntax_pos::symbol::Symbol;
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

// This will check consts
impl EarlyLintPass for ExcessivePrecision {
    // println!("{}", .99999999999999999999)
    fn check_expr(&mut self, cx: &EarlyContext, expr: &Expr) {
        println!("check_expr {:?}", expr);
        if_chain! {
            if let ExprKind::Lit(ref lit) = expr.node;
            if let LitKind::Float(ref sym, ref fty) = lit.node;
            then {
                self.check(sym, fty);
            }

        }
    }
    // fn check_stmt(&mut self, cx: &EarlyContext, stmt: &stmt) {
    //     match stmt.node {
    //         StmtKind::Local(ref loc) => self.local_check(loc),
    //         StmtKind::Item(ref loc) => self.item_check(loc),
    //     }
    // }
}

impl ExcessivePrecision {
    fn check(&mut self, sym: &Symbol, fty: &FloatTy) {
        println!("checking {} with type {}", sym, fty);
    }
    // We cant just check the expr, we need to have the type assignment
    // so that we know the float precision.
    // let foo = 0.123...
    fn local_check(&mut self, cx: &EarlyContext, local: &Local) {
        if_chain! {
            if let Some(ref exptr) = local.init;
            if let ExprKind::Lit(ref lit) = exptr.node;
            if let LitKind::Float(ref sym, ref fty) = lit.node;

            if let Some(ref ty)  = local.ty;
            if let TyKind::Path(_, ref pth) = ty.node;
            let ref segs = pth.segments;
            if let ref seg = segs[0];
            then {
                let id = seg.ident;
                let name = id.name.as_str().to_lowercase();
                if name == "f32" {
                    self.check(sym, &FloatTy::F32);
                } else if name == "f64" {
                    self.check(sym, &FloatTy::F64);
                }
            }
        }
    }
    // const foo = 0.123...
    // fn item_check(&mut self, cx: &EarlyContext, item: &Item) {
    //     if let ItemKind::Const(ref ty, ref expr) = item.node {
    //         // check the expr with ty
    //     }
    // }
}
