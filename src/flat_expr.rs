use moniker::BoundTerm;
use moniker::{Binder, Ignore, Scope, Var};

use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use termcolor::{Color, ColorSpec, WriteColor};

use std::{io::Result, rc::Rc};

use crate::literals::Literal;
use crate::utils::clone_rc;

#[derive(Debug, Clone, BoundTerm)]
pub enum FExpr {
    LamOne(Scope<Binder<String>, Rc<FExpr>>),
    LamTwo(Scope<Binder<String>, Scope<Binder<String>, Rc<FExpr>>>),
    Var(Var<String>),
    Lit(Ignore<Literal>),
    CallOne(Rc<FExpr>, Rc<FExpr>),
    CallTwo(Rc<FExpr>, Rc<FExpr>, Rc<FExpr>),
}

impl FExpr {
    pub fn pretty<'a, D>(&'a self, allocator: &'a D) -> DocBuilder<'a, D, ColorSpec>
    where
        D: DocAllocator<'a, ColorSpec>,
        D::Doc: Clone,
    {
        match self {
            FExpr::LamOne(s) => {
                let Scope {
                    unsafe_pattern: pat,
                    unsafe_body: body,
                } = &s;

                let pat_pret = allocator
                    .as_string(pat)
                    .annotate(ColorSpec::new().set_fg(Some(Color::Green)).clone())
                    .parens();
                let body_pret = allocator
                    .line_()
                    .append(body.pretty(allocator))
                    .nest(1)
                    .group();

                allocator
                    .text("lambda")
                    .annotate(ColorSpec::new().set_fg(Some(Color::Magenta)).clone())
                    .append(allocator.space())
                    .append(pat_pret)
                    .append(allocator.space())
                    .append(body_pret)
                    .parens()
            }
            FExpr::LamTwo(s) => {
                let Scope {
                    unsafe_pattern: pat,
                    unsafe_body:
                        Scope {
                            unsafe_pattern: cont,
                            unsafe_body: body,
                        },
                } = &s;

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
                let body_pret = allocator
                    .line_()
                    .append(body.pretty(allocator))
                    .nest(1)
                    .group();

                allocator
                    .text("lambda")
                    .annotate(ColorSpec::new().set_fg(Some(Color::Magenta)).clone())
                    .append(allocator.space())
                    .append(args_pret)
                    .append(allocator.space())
                    .append(body_pret)
                    .parens()
            }
            FExpr::Var(s) => allocator.as_string(s),
            FExpr::Lit(Ignore(l)) => l.pretty(allocator),
            FExpr::CallOne(f, c) => {
                let f_pret = f.pretty(allocator);
                let c_pret = c.pretty(allocator);

                f_pret
                    .annotate(ColorSpec::new().set_fg(Some(Color::Blue)).clone())
                    .append(allocator.space())
                    .append(c_pret)
                    .parens()
            }
            FExpr::CallTwo(f, v, c) => {
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
        }
    }

    pub fn pretty_print(&self, out: impl WriteColor) -> Result<()> {
        let allocator = BoxAllocator;

        self.pretty(&allocator).1.render_colored(70, out)?;

        Ok(())
    }

    // I should really just functor huh
    pub fn subst<N: PartialEq<Var<String>>>(self, name: &N, rep: FExpr) -> FExpr {
        match self {
            FExpr::LamOne(s) => {
                let Scope {
                    unsafe_pattern: pat,
                    unsafe_body: body,
                } = s;

                let body = Rc::new(clone_rc(body).subst(name, rep));

                FExpr::LamOne(Scope {
                    unsafe_pattern: pat,
                    unsafe_body: body,
                })
            }
            FExpr::LamTwo(s) => {
                let Scope {
                    unsafe_pattern: pat,
                    unsafe_body:
                        Scope {
                            unsafe_pattern: cont,
                            unsafe_body: body,
                        },
                } = s;

                let body = Rc::new(clone_rc(body).subst(name, rep));

                FExpr::LamTwo(Scope {
                    unsafe_pattern: pat,
                    unsafe_body: Scope {
                        unsafe_pattern: cont,
                        unsafe_body: body,
                    },
                })
            }
            FExpr::Var(v) => {
                if name.eq(&v) {
                    rep
                } else {
                    FExpr::Var(v)
                }
            }
            l @ FExpr::Lit(_) => l,
            FExpr::CallOne(f, v) => FExpr::CallOne(
                Rc::new(clone_rc(f).subst(name, rep.clone())),
                Rc::new(clone_rc(v).subst(name, rep)),
            ),
            FExpr::CallTwo(f, v, c) => FExpr::CallTwo(
                Rc::new(clone_rc(f).subst(name, rep.clone())),
                Rc::new(clone_rc(v).subst(name, rep.clone())),
                Rc::new(clone_rc(c).subst(name, rep.clone())),
            ),
        }
    }
}
