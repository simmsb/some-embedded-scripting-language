use moniker::BoundTerm;
use moniker::{Var, Scope, Binder};

use pretty::{BoxAllocator, DocAllocator, DocBuilder};

use std::rc::Rc;


#[derive(Debug, Clone, BoundTerm)]
pub enum Expr {
    Var(Var<String>),
    Lam(Scope<Binder<String>, Rc<Expr>>),
    App(Rc<Expr>, Rc<Expr>),
}

impl Expr {
    pub fn pretty<'a, D, A>(&'a self, allocator: &'a D) -> DocBuilder<'a, D, A>
    where D: DocAllocator<'a, A>,
          D::Doc: Clone,
          A: Clone,
    {
        match self {
            Expr::Var(s) => {
                allocator.as_string(s)
            }
            Expr::Lam(s) => {
                let Scope { unsafe_pattern: pat, unsafe_body: body } = &s;
                let pat_pret = allocator.as_string(pat)
                                        .parens();
                let body_pret = body.pretty(allocator);

                allocator.text("lambda")
                         .append(allocator.space())
                         .append(pat_pret)
                         .append(allocator.space())
                         .append(body_pret)
                         .parens()

                // allocator.intersperse(
                //     &[allocator.text("lambda"), pat_pret, expr_pret],
                //     allocator.space()
                // ).parens()
            }
            Expr::App(f, v) => {
                let f_pret = f.pretty(allocator);
                let v_pret = v.pretty(allocator);

                f_pret
                    .append(allocator.space())
                    .append(v_pret)
                    .parens()

                // allocator.intersperse(
                //     &[f_pret, v_pret],
                //     allocator.space()
                // ).parens()
            }
        }
    }

    pub fn pretty_print(&self) -> String {
        let allocator = BoxAllocator;

        let res = self.pretty::<_, ()>(&allocator).1.pretty(50).to_string();

        res
    }
}
