
/* use std::option;*/

enum Term<'a> {
    var(&'a Var<'a>),
    func(&'a Function<'a>), 
}

struct Function<'a> {
    name: &'a str,
    args: &'a Args<'a>,
}

enum Args<'a> {
    cons(&'a Term<'a>, &'a Args<'a>),
    nil,
}

struct Var<'a> {
    name: &'a str
}

enum Env<'a> {
    cons_eq(&'a Var<'a>,&'a Term<'a>,&'a Env<'a>),
    nil,
}

fn is_args_nil<'a>(args: &'a Args<'a>) -> bool {
    match args {
        Args::cons(_,_) => false,
        Args::nil => true
    }
}

fn arglen<'a>(args: &'a Args<'a>) -> u32 {
    match args{
        Args::cons(_,rest) => 1 + arglen(rest),
        Args::nil => 0
    }
}

fn extend<'a>(env: &'a Env<'a>, x: &'a Var<'a>, v: &'a Term<'a>) -> Env<'a> {
    Env::cons_eq(&x,&v,&env)
}

fn fun_match<'a>(f: &'a Function<'a>, g: &'a Function<'a>) -> bool {
    f.name == g.name && arglen(f.args) == arglen(g.args)
}

fn var_eq<'a>(x: &'a Var<'a>, y: &'a Var<'a>) -> bool {
    x.name == y.name
}

fn lookup<'a>(x: &'a Var<'a>, env: &'a Env<'a>) -> Option< (&'a Term<'a>)> {
    match env {
        Env::cons_eq(y,v,env2) =>
            if var_eq(x,y){
                match v {
                    /* if a var, chase its value */
                    Term::var(z) => lookup(z,env2),
                    /* otherwise return */
                    Term::func(_) => Option::Some(v)
                }
            }else{
                lookup(x,env2)
            },
        Env::nil => Option::None
    }
}

/* Makes a term more rigid at the top level if possible. */ 
fn enrich<'a>(env: &'a Env<'a>, t: &'a Term<'a>) -> &'a Term<'a> {
    match t {
        Term::var(x) =>
            match lookup(x,env) {
                Option::None => t,
                Option::Some(s) => s
            }
        Term::func(_) => t
    }
}

fn unify_args<'a>(env: &'a Env<'a>, targs: &'a Args<'a>, sargs: &'a Args<'a>) -> Option<&'a Env<'a>> {
    match targs {
        Args::cons(t,t_tail) =>
            match sargs {
                Args::cons(s,s_tail) =>
                    match unify(env,t,s) {
                        Option::None => Option::None,
                        Option::Some(env2) => unify_args(env2,t_tail,s_tail)
                    }
                Args::nil => Option::None
            }
        Args::nil =>
            match sargs {
                Args::cons(_,_) => Option::None,
                Args::nil => Some(env)
            }
    }           
}

fn unify<'a>(env: &'a Env<'a>, t: &'a Term<'a>, s: &'a Term<'a>) -> Option<&'a Env<'a>> {
    let t = enrich(env,t);
    let s = enrich(env,s);
    match t {
        Term::var(x) =>
            match s {
                Term::var(_) =>
                    /* flex - flex */
                    Option::Some(&extend(env,x,s)),
                Term::func(_) =>
                    /* flex - rigid */
                    Option::Some(&extend(env,x,s))
            },
        Term::func(f) =>
            match s {
                Term::var(y) =>
                    /* rigid - flex */
                    Some(&extend(env,y,t)),
                Term::func(g) =>
                    /* rigid- rigid */
                    if fun_match(f,g) {
                        unify_args(env,f.args,g.args)
                    } else {                        
                        Option::None
                    }
            }
    }
}


fn show_var<'a>(var: &'a Var<'a>){
    print!("{}",var.name);
}

fn show_term<'a>(term: &'a Term<'a>){
    match term {
        Term::var(v) => show_var(v),
        Term::func(f) => {
            print!("{}",f.name);
            print!("(");
            show_args(f.args);
            print!(")");
        }
    }
}

fn show_args<'a>(args: &'a Args<'a>){
    match args {
        Args::cons(t,args2) => {
            show_term(t);
            if !is_args_nil(args2){
                print!(",");
            }                
            show_args(args2)
        },
        Args::nil => ()
    }
}

fn show_env<'a>(env: &'a Env<'a>) {
    match env {
        Env::nil => (),
        Env::cons_eq(v,t,env2) => {
            show_var(v);
            print!(" = ");
            show_term(t);
            println!("");
            show_env(env2);
        }
    }
}

fn main() {
    let env = Env::nil;
    let xvar = Var {
        name: "X",
    };
    let yvar = Var {
        name: "Y",
    };
    let fun = Function {
        name: "f",
        args: &Args::cons(&Term::var(&xvar) ,&Args::nil)
    };
    let fun2 = Function {
        name: "f",
        args: &Args::cons(&Term::var(&yvar) ,&Args::nil)
    };
    let funterm = Term::func(&fun);
    let funterm2 = Term::func(&fun2);

    let res = unify(&env,&funterm,&funterm2);

    match res {
        Option::None => println!("No solution"),
        Option::Some(env) => show_env(&env)
    }
    
}
