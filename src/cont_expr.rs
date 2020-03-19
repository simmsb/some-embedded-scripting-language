use moniker::BoundTerm;
use moniker::{Var, Scope, Binder, FreeVar};

use termcolor::{WriteColor, ColorSpec, Color};
use pretty::{BoxAllocator, DocAllocator, DocBuilder};

use std::{io::Result, rc::Rc};

use crate::expr::Expr;

#[derive(Debug, Clone, BoundTerm)]
pub enum UExpr {
    Lam(Scope<Binder<String>, Scope<Binder<String>, Rc<CCall>>>),
    Var(Var<String>),
}

impl UExpr {
    pub fn pretty<'a, D>(&'a self, allocator: &'a D) -> DocBuilder<'a, D, ColorSpec>
    where D: DocAllocator<'a, ColorSpec>,
          D::Doc: Clone,
    {
        match self {
            UExpr::Lam(s) => {
                let Scope { unsafe_pattern: pat,
                            unsafe_body: Scope { unsafe_pattern: cont,
                                                 unsafe_body: body } } = &s;

                let pat_pret = allocator
                    .as_string(pat)
                    .annotate(ColorSpec::new().set_fg(Some(Color::Green)).clone());
                let cont_pret = allocator
                    .as_string(cont)
                    .annotate(ColorSpec::new().set_fg(Some(Color::Red)).clone());
                let args_pret = pat_pret
                    .append(allocator.space())
                    .append(cont_pret)
                    .parens();
                let body_pret =
                    allocator.line_()
                             .append(body.pretty(allocator))
                             .nest(1)
                             .group();

                allocator.text("lambda")
                         .annotate(ColorSpec::new().set_fg(Some(Color::Magenta)).clone())
                         .append(allocator.space())
                         .append(args_pret)
                         .append(allocator.space())
                         .append(body_pret)
                         .parens()
            }
            UExpr::Var(s) => {
                allocator.as_string(s)
            }
        }
    }
}

#[derive(Debug, Clone, BoundTerm)]
pub enum KExpr {
    Lam(Scope<Binder<String>, Rc<CCall>>),
    Var(Var<String>),
}

impl KExpr {
    pub fn pretty<'a, D>(&'a self, allocator: &'a D) -> DocBuilder<'a, D, ColorSpec>
    where D: DocAllocator<'a, ColorSpec>,
          D::Doc: Clone,
    {
        match self {
            KExpr::Lam(s) => {
                let Scope { unsafe_pattern: pat,
                            unsafe_body: body } = &s;

                let pat_pret = allocator
                    .as_string(pat)
                    .annotate(ColorSpec::new().set_fg(Some(Color::Green)).clone())
                    .parens();
                let body_pret =
                    allocator.line_()
                             .append(body.pretty(allocator))
                             .nest(1)
                             .group();

                allocator.text("lambda")
                         .annotate(ColorSpec::new().set_fg(Some(Color::Magenta)).clone())
                         .append(allocator.space())
                         .append(pat_pret)
                         .append(allocator.space())
                         .append(body_pret)
                         .parens()
            }
            KExpr::Var(s) => {
                allocator.as_string(s)
            }
        }
    }
}

#[derive(Debug, Clone, BoundTerm)]
pub enum CCall {
    UCall(Rc<UExpr>, Rc<UExpr>, Rc<KExpr>),
    KCall(Rc<KExpr>, Rc<UExpr>),
}

impl CCall {
    pub fn pretty<'a, D>(&'a self, allocator: &'a D) -> DocBuilder<'a, D, ColorSpec>
    where D: DocAllocator<'a, ColorSpec>,
          D::Doc: Clone,
    {
        match self {
            CCall::UCall(f, v, c) => {
                let f_pret = f.pretty(allocator);
                let v_pret = v.pretty(allocator);
                let c_pret = c.pretty(allocator);

                f_pret
                    .annotate(ColorSpec::new().set_fg(Some(Color::Blue)).clone())
                    .append(allocator.space())
                    .append(v_pret)
                    .append(allocator.space())
                    .append(c_pret)
                    .parens()
            }


            CCall::KCall(f, c) => {
                let f_pret = f.pretty(allocator);
                let c_pret = c.pretty(allocator);

                f_pret
                    .annotate(ColorSpec::new().set_fg(Some(Color::Blue)).clone())
                    .append(allocator.space())
                    .append(c_pret)
                    .parens()
            }
        }
    }

    pub fn pretty_print(&self, out: impl WriteColor) -> Result<()> {
        let allocator = BoxAllocator;

        self.pretty(&allocator).1.render_colored(70, out)?;

        Ok(())
    }
}

pub fn t_k(expr: Expr, k: Rc<KExpr>) -> CCall {
    match expr {
        e@(Expr::Lam(_) | Expr::Var(_)) => CCall::KCall(k, Rc::new(m(e))),
        Expr::App(f, e) => {
            let rv_v = FreeVar::fresh_named("rv");
            let cont = Rc::new(KExpr::Lam(
                Scope::new(Binder(rv_v.clone()),
                           Rc::new(CCall::KCall(k,
                                                Rc::new(UExpr::Var(Var::Free(rv_v))))))));

            let f_v = FreeVar::fresh_named("f");
            let e_v = FreeVar::fresh_named("e");

            t_k(clone_rc(f),
                Rc::new(KExpr::Lam(
                    Scope::new(Binder(f_v.clone()),
                               Rc::new(t_k(clone_rc(e),
                                           Rc::new(KExpr::Lam(Scope::new(Binder(e_v.clone()),
                                                                         Rc::new(CCall::UCall(
                                                                             Rc::new(UExpr::Var(Var::Free(f_v))),
                                                                             Rc::new(UExpr::Var(Var::Free(e_v))),
                                                                             cont)))))))))))
        }
    }
}

fn clone_rc<T: Clone>(r: Rc<T>) -> T {
    Rc::try_unwrap(r).unwrap_or_else(|t| t.as_ref().clone())
}

fn t_c(expr: Expr, c: FreeVar<String>) -> CCall {
    let c_v = Rc::new(KExpr::Var(Var::Free(c)));
    match expr {
        e@(Expr::Lam(_) | Expr::Var(_)) =>
            CCall::KCall(c_v, Rc::new(m(e))),
        Expr::App(f, e) => {
            let f_v = FreeVar::fresh_named("f");
            let e_v = FreeVar::fresh_named("e");

            t_k(clone_rc(f),
                Rc::new(KExpr::Lam(
                    Scope::new(Binder(f_v.clone()),
                               Rc::new(t_k(clone_rc(e),
                                           Rc::new(KExpr::Lam(Scope::new(Binder(e_v.clone()),
                                                                         Rc::new(CCall::UCall(
                                                                             Rc::new(UExpr::Var(Var::Free(f_v))),
                                                                             Rc::new(UExpr::Var(Var::Free(e_v))),
                                                                             c_v)))))))))))
        }
    }
}

fn m(expr: Expr) -> UExpr {
    match expr {
        Expr::Lam(s) => {
            let (p, t) = s.unbind();
            let k = FreeVar::fresh_named("k");
            let body = t_c(clone_rc(t), k.clone());
            UExpr::Lam(
                Scope::new(p, Scope::new(
                    Binder(k),
                    Rc::new(body),
                )),
            )
        }
        Expr::Var(v) => UExpr::Var(v),
        _ => unreachable!()
    }
}


