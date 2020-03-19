use moniker::{Var, Scope, Binder, FreeVar};
use termcolor::{ColorChoice, StandardStream};
use std::{io::Result, rc::Rc};

use some_embedded_scripting_language::expr::Expr;

pub fn main() -> Result<()> {
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


    let g = FreeVar::fresh(Some("g".to_string()));
    let x = FreeVar::fresh(Some("x".to_string()));
    let expr = Rc::new(Expr::Lam(Scope::new(
        Binder(g.clone()),
        Rc::new(Expr::Lam(Scope::new(
            Binder(x.clone()),
            Rc::new(Expr::App(
                Rc::new(Expr::Var(Var::Free(g.clone()))),
                expr,
            )),
        )))
    )));

    expr.pretty_print(StandardStream::stdout(ColorChoice::Auto))?;

    println!();

    Ok(())
}
