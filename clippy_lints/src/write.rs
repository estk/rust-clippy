use rustc::hir::*;
use rustc::lint::*;
use std::ops::Deref;
use syntax::ast::LitKind;
use syntax::symbol::InternedString;
use syntax_pos::Span;
use utils::{is_expn_of, match_def_path, resolve_node, span_lint, span_lint_and_sugg};
use utils::{opt_def_id, paths};

/// **What it does:** This lint warns when you use `writeln!(buf, "")` to
/// print a newline.
///
/// **Why is this bad?** You should use `writeln!(buf)`, which is simpler.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// writeln!("");
/// ```
declare_clippy_lint! {
    pub WRITELN_EMPTY_STRING,
    style,
    "using `writeln!(\"\")` with an empty string"
}

/// **What it does:** This lint warns when you use `write!()` with a format
/// string that
/// ends in a newline.
///
/// **Why is this bad?** You should use `writeln!()` instead, which appends the
/// newline.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// write!(buf, "Hello {}!\n", name);
/// ```
declare_clippy_lint! {
    pub WRITE_WITH_NEWLINE,
    style,
    "using `write!()` with a format string that ends in a newline"
}

/// **What it does:** This lint warns about the use of literals as `write!`/`writeln!` args.
///
/// **Why is this bad?** Using literals as `writeln!` args is inefficient
/// (c.f., https://github.com/matthiaskrgr/rust-str-bench) and unnecessary
/// (i.e., just put the literal in the format string)
///
/// **Known problems:** Will also warn with macro calls as arguments that expand to literals
/// -- e.g., `writeln!(buf, "{}", env!("FOO"))`.
///
/// **Example:**
/// ```rust
/// writeln!(buf, "{}", "foo");
/// ```
declare_clippy_lint! {
    pub WRITE_LITERAL,
    style,
    "writing a literal with a format string"
}

#[derive(Copy, Clone, Debug)]
pub struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(WRITE_WITH_NEWLINE, WRITELN_EMPTY_STRING, WRITE_LITERAL)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for Pass {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        if_chain! {
            if let ExprMethodCall(ref write_fun, _, ref write_args) = expr.node;
            if write_fun.name == "write_fmt";
            then {
                if let Some(span) = is_expn_of(expr.span, "write") {
                    // `writeln!` uses `write!`.
                    let (span, name) = match is_expn_of(span, "writeln") {
                        Some(span) => (span, "writeln"),
                        None => (span, "write"),
                    };
                    // println!("check for literal {:?}", write_args);

                    // Check for literals in the write!/writeln! args
                    // Also, ensure the format string is `{}` with no special options, like `{:X}`
                    check_write_args_for_literal(cx, write_args);

                    // if_chain! {
                //         // ensure we're calling Arguments::new_v1
                //         if args.len() == 1;
                //         if let ExprCall(ref args_fun, ref args_args) = args[0].node;
                //         if let ExprPath(ref qpath) = args_fun.node;
                //         if let Some(const_def_id) = opt_def_id(resolve_node(cx, qpath, args_fun.hir_id));
                //         if match_def_path(cx.tcx, const_def_id, &paths::FMT_ARGUMENTS_NEWV1);
                //         if args_args.len() == 2;
                //         if let ExprAddrOf(_, ref match_expr) = args_args[1].node;
                //         if let ExprMatch(ref args, _, _) = match_expr.node;
                //         if let ExprTup(ref args) = args.node;
                //         if let Some((fmtstr, fmtlen)) = get_argument_fmtstr_parts(&args_args[0]);
                    //     then {
                    //         match name {
                    //             "write" => check_write(cx, span, args, fmtstr, fmtlen),
                    //             "writeln" => check_writeln(cx, span, fmtstr, fmtlen),
                    //             _ => (),
                    //         }
                    //     }
                    }
            }
        }
    }
}

// Check for literals in write!/writeln! args
// ensuring the format string for the literal is `DISPLAY_FMT_METHOD`
// e.g., `writeln!(buf, "... {} ...", "foo")`
//                                ^ literal in `writeln!`
fn check_write_args_for_literal<'a, 'tcx>(cx: &LateContext<'a, 'tcx>, args: &HirVec<Expr>) {
    if_chain! {
        if args.len() == 2;
        if let ExprCall(_, ref args_args) = args[1].node;
        if args_args.len() > 1;
        if let ExprAddrOf(_, ref match_expr) = args_args[1].node;
        if let ExprMatch(ref matchee, ref arms, _) = match_expr.node;
        if let ExprTup(ref tup) = matchee.node;
        if arms.len() == 1;
        if let ExprArray(ref arm_body_exprs) = arms[0].body.node;
        then {
            // it doesn't matter how many args there are in the `write!`/`writeln!`,
            // if there's one literal, we should warn the user
            for (idx, tup_arg) in tup.iter().enumerate() {
                if_chain! {
                    // first, make sure we're dealing with a literal (i.e., an ExprLit)
                    if let ExprAddrOf(_, ref tup_val) = tup_arg.node;
                    if let ExprLit(_) = tup_val.node;

                    // next, check the corresponding match arm body to ensure
                    // this is "{}", or DISPLAY_FMT_METHOD
                    if let ExprCall(_, ref body_args) = arm_body_exprs[idx].node;
                    if body_args.len() == 2;
                    if let ExprPath(ref body_qpath) = body_args[1].node;
                    if let Some(fun_def_id) = opt_def_id(resolve_node(cx, body_qpath, body_args[1].hir_id));
                    if match_def_path(cx.tcx, fun_def_id, &paths::DISPLAY_FMT_METHOD) ||
                       match_def_path(cx.tcx, fun_def_id, &paths::DEBUG_FMT_METHOD);
                    then {
                        span_lint(cx, WRITE_LITERAL, tup_val.span, "writing a literal with an empty format string");
                    }
                }
            }
        }
    }
}

// Check for write!(..., "... \n", ...).
fn check_write<'a, 'tcx>(
    cx: &LateContext<'a, 'tcx>,
    span: Span,
    args: &HirVec<Expr>,
    fmtstr: InternedString,
    fmtlen: usize,
) {
    if_chain! {
        // check the final format string part
        if let Some('\n') = fmtstr.chars().last();

        // "foo{}bar" is made into two strings + one argument,
        // if the format string starts with `{}` (eg. "{}foo"),
        // the string array is prepended an empty string "".
        // We only want to check the last string after any `{}`:
        if args.len() < fmtlen;
        then {
            span_lint(cx, WRITE_WITH_NEWLINE, span,
                      "using `write!()` with a format string that ends in a \
                       newline, consider using `writeln!()` instead");
        }
    }
}

/// Check for writeln!("")
fn check_writeln<'a, 'tcx>(cx: &LateContext<'a, 'tcx>, span: Span, fmtstr: InternedString, fmtlen: usize) {
    if_chain! {
        // check that the string is empty
        if fmtlen == 1;
        if fmtstr.deref() == "\n";

        // check the presence of that string
        if let Ok(snippet) = cx.sess().codemap().span_to_snippet(span);
        if snippet.contains("\"\"");
        then {
            span_lint_and_sugg(
                cx,
                WRITE_WITH_NEWLINE,
                span,
                "using `writeln!(v, \"\")`",
                "replace it with",
                "writeln!(v)".to_string(),
            );
         }
    }
}

/// Returns the slice of format string parts in an `Arguments::new_v1` call.
fn get_argument_fmtstr_parts(expr: &Expr) -> Option<(InternedString, usize)> {
    if_chain! {
        if let ExprAddrOf(_, ref expr) = expr.node; // &["…", "…", …]
        if let ExprArray(ref exprs) = expr.node;
        if let Some(expr) = exprs.last();
        if let ExprLit(ref lit) = expr.node;
        if let LitKind::Str(ref lit, _) = lit.node;
        then {
            return Some((lit.as_str(), exprs.len()));
        }
    }
    None
}
