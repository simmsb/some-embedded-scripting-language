use moniker::{Var, Scope, Binder, FreeVar, Ignore};
use termcolor::{ColorChoice, StandardStream};
use std::{io::Result, rc::Rc};

use some_embedded_scripting_language::{expr::Expr, cont_expr::{KExpr, self}, literals::Literal};

pub fn main() -> Result<()> {
    expr_test()?;
    cexpr_test()?;

    Ok(())
}

pub fn cexpr_test() -> Result<()> {
    let f = FreeVar::fresh(Some("f".to_string()));
    let x = FreeVar::fresh(Some("x".to_string()));

    // \f => \x => f x
    let expr = Rc::new(Expr::Lam(Scope::new(
        Binder(f.clone()),
        Rc::new(Expr::Lam(Scope::new(
            Binder(x.clone()),
            Rc::new(Expr::App(
                Rc::new(Expr::Var(Var::Free(f))),
                Rc::new(Expr::Lit(Ignore(Literal::String("lmao".to_owned())))),
            )),
        )))
    )));


    let g = FreeVar::fresh(Some("g".to_string()));
    let x = FreeVar::fresh(Some("x".to_string()));
    let expr = Expr::Lam(Scope::new(
        Binder(g.clone()),
        Rc::new(Expr::Lam(Scope::new(
            Binder(x),
            Rc::new(Expr::App(
                Rc::new(Expr::Var(Var::Free(g))),
                expr,
            )),
        )))
    ));

    let k = Rc::new(KExpr::Var(Var::Free(FreeVar::fresh_named("exit"))));

    let kexpr = cont_expr::t_k(expr, k);
    let fexpr = kexpr.into_fexpr();


    fexpr.pretty_print(StandardStream::stdout(ColorChoice::Auto))?;

    println!();

    Ok(())
}

pub fn expr_test() -> Result<()> {
    let f = FreeVar::fresh(Some("f".to_string()));
    let x = FreeVar::fresh(Some("x".to_string()));

    // \f => \x => f x
    let expr = Rc::new(Expr::Lam(Scope::new(
        Binder(f.clone()),
        Rc::new(Expr::Lam(Scope::new(
            Binder(x.clone()),
            Rc::new(Expr::App(
                Rc::new(Expr::Var(Var::Free(f))),
                Rc::new(Expr::Var(Var::Free(x))),
            )),
        )))
    )));


    let g = FreeVar::fresh(Some("g".to_string()));
    let x = FreeVar::fresh(Some("x".to_string()));
    let expr = Expr::Lam(Scope::new(
        Binder(g.clone()),
        Rc::new(Expr::Lam(Scope::new(
            Binder(x),
            Rc::new(Expr::App(
                Rc::new(Expr::Var(Var::Free(g))),
                expr,
            )),
        )))
    ));

    expr.pretty_print(StandardStream::stdout(ColorChoice::Auto))?;

    println!();

    Ok(())
}
