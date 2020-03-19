use moniker::{Var, Scope, Binder, FreeVar};
use some_embedded_scripting_language::expr::Expr;
use std::rc::Rc;

pub fn main() {
    let f = FreeVar::fresh(Some("f".to_string()));
    let x = FreeVar::fresh(Some("x".to_string()));

    // \f => \x => f x
    let expr = Rc::new(Expr::Lam(Scope::new(
        Binder(f.clone()),
        Rc::new(Expr::Lam(Scope::new(
            Binder(x.clone()),
            Rc::new(Expr::App(
                Rc::new(Expr::Var(Var::Free(f.clone()))),
                Rc::new(Expr::Var(Var::Free(x.clone()))),
            )),
        )))
    )));

    println!("{}", expr.pretty_print());
}
