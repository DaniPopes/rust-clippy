#![allow(unused_imports)]

use arrayvec::ArrayVec;
use clippy_config::msrvs::Msrv;
use clippy_config::DisallowedFormatSpec;
use clippy_utils::diagnostics::{span_lint_and_sugg, span_lint_and_then};
use clippy_utils::is_diag_trait_item;
use clippy_utils::macros::{
    find_format_arg_expr, find_format_args, format_arg_removal_span, format_placeholder_format_span, is_assert_macro,
    is_format_macro, is_panic, macro_backtrace, root_macro_call, root_macro_call_first_node, FormatParamUsage,
};
use clippy_utils::source::snippet_opt;
use clippy_utils::ty::{implements_trait, is_type_lang_item};
use rustc_ast::{
    Attribute, FormatArgPosition, FormatArgPositionKind, FormatArgsPiece, FormatArgumentKind, FormatCount,
    FormatOptions, FormatPlaceholder, FormatTrait,
};
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_hir::def::Res;
use rustc_hir::def_id::{DefId, DefIdMap};
use rustc_hir::{Expr, ExprKind, ForeignItem, HirId, ImplItem, Item, Pat, Path, Stmt, TraitItem, Ty, *};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint, impl_lint_pass};
use rustc_span::{ExpnId, Span, Symbol};

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.74.0"]
    pub DISALLOWED_FORMAT_SPECS,
    style,
    "default lint description"
}

#[derive(Debug)]
pub struct DisallowedFormatSpecs {
    conf_disallowed: Vec<DisallowedFormatSpec>,
    disallowed_paths: DefIdMap<usize>,
}

// impl Default for DisallowedFormatSpecs {
//     fn default() -> Self {
//         Self::new(vec![conf::DisallowedFormatSpec {
//             path: "disallowed_format_specs::Address".into(),
//             specs: vec![FormatTrait::Debug, FormatTrait::LowerHex, FormatTrait::UpperHex],
//             reason: Some("Use `Display` instead".into()),
//         }])
//     }
// }

impl DisallowedFormatSpecs {
    pub fn new(conf_disallowed: Vec<DisallowedFormatSpec>) -> Self {
        Self {
            conf_disallowed,
            disallowed_paths: DefIdMap::default(),
        }
    }
}

impl_lint_pass!(DisallowedFormatSpecs => [DISALLOWED_FORMAT_SPECS]);

impl<'tcx> LateLintPass<'tcx> for DisallowedFormatSpecs {
    fn check_crate(&mut self, cx: &LateContext<'_>) {
        for (index, conf) in self.conf_disallowed.iter().enumerate() {
            let segs: Vec<_> = conf.path.split("::").collect();
            for id in clippy_utils::def_path_def_ids(cx, &segs) {
                self.disallowed_paths.insert(id, index);
            }
        }
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let Some(macro_call) = root_macro_call_first_node(cx, expr)
            && is_format_macro(cx, macro_call.def_id)
            && let Some(format_args) = find_format_args(cx, expr, macro_call.expn)
        {
            for piece in &format_args.template {
                if let FormatArgsPiece::Placeholder(placeholder) = piece
                    && let Ok(index) = placeholder.argument.index
                    && let Some(arg) = format_args.arguments.all_args().get(index)
                    && let Ok(arg_expr) = find_format_arg_expr(expr, arg)
                    && let arg_ty = cx.typeck_results().expr_ty_adjusted(arg_expr).peel_refs()
                    && let Some(adt) = arg_ty.ty_adt_def()
                    && let def_id = adt.did()
                    && let Some(index) = self.disallowed_paths.get(&def_id)
                    && let conf = &self.conf_disallowed[*index]
                    && conf.specs.contains(&placeholder.format_trait)
                {
                    let tr = placeholder.format_trait;
                    let ty = cx.tcx.def_path_str(def_id);
                    span_lint_and_then(
                        cx,
                        DISALLOWED_FORMAT_SPECS,
                        arg.expr.span,
                        &format!("format trait `{tr:?}` is not allowed for type `{ty}` according to config"),
                        |diag| {
                            if let Some(reason) = &conf.reason {
                                diag.note(reason.clone());
                            }
                        },
                    );
                }
            }
        }
    }
}
