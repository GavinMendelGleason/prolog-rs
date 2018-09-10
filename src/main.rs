
/* use std::option;*/

enum Term {
    var(Box<Var>),
    func(Box<Function>), 
}

struct Function {
    name: Box<String>,
    args: Box<Args>,
}

enum Args {
    cons(Box<Term>, Box<Args>),
    nil,
}

struct Var {
    name: Box<String>
}

enum Env {
    cons_eq(Box<Var>,Box<Term>,Box<Env>),
    nil,
}

fn is_args_nil(args: &Args) -> bool {
    match args {
        Args::cons(_,_) => false,
        Args::nil => true
    }
}

fn arglen(args: &Args) -> u32 {
    match args {
        Args::cons(_,rest) => 1 + arglen(rest),
        Args::nil => 0
    }
}

fn extend(env: Box<Env>, x: Box<Var>, v: Box<Term>) -> Box<Env> {
    Box::new(Env::cons_eq(x,v,env))        
}
/*
fn fun_match(f: Box<Function>, g: Box<Function>) -> bool {
    f.name == g.name && arglen(&f.args) == arglen(&g.args)
}

fn var_eq<'a>(x: &'a Var, y: &'a Var) -> bool {
    x.name == y.name
}

fn lookup(x: Box<Var>, env: Box<Env>) -> Option<Box<Term>> {
    let new_env = *env;
    match new_env {
        Env::cons_eq(y,v,env2) => {
            let newx = *x;
            let newy = *y;
            if var_eq(&newx,&newy){
                let newv = *v;
                match newv {
                    /* if a var, chase its value */
                    Term::var(z) => lookup(z,env2),
                    /* otherwise return */
                    Term::func(_) => Option::Some(Box::new(newv))
                }
            }else{
                lookup(Box::new(newx),env2)
            }
        },
        Env::nil => Option::None
    }
}

/* Makes a term more rigid at the top level if possible. */ 
fn enrich(env: Box<Env>, t: Box<Term>) -> Box<Term> {
    let newt = *t;
    match newt {
        Term::var(x) => {
            let newx = *x;
            let newname = newx.name.clone();                    
            match lookup(Box::new(newx),env) {
                Option::None => {
                    let newvar = Var {name: newname};
                    Box::new(Term::var(Box::new(newvar)))
                },
                Option::Some(s) => s
            }
        },
        Term::func(_) => Box::new(newt)
    }
}

fn unify_args(envbox: Box<Env>, targsbox: Box<Args>, sargsbox: Box<Args>) -> Option<Box<Env>> {
    let targs = *targsbox;
    match targs {
        Args::cons(t,t_tail) => {
            let sargs = *sargsbox;
            match sargs {
                Args::cons(s,s_tail) => {
                    let result = unify(envbox,t,s);
                    match result {
                        Option::None => Option::None,
                        Option::Some(env2) => unify_args(env2,t_tail,s_tail)
                    }
                },
                Args::nil => Option::None
            }
        },
        Args::nil => {
            let sargs = *sargsbox;
            match sargs {
                Args::cons(_,_) => Option::None,
                Args::nil => Some(envbox)
            }
        }
    }           
}

fn unify(env: Box<Env>, t: Box<Term>, s: Box<Term>) -> Option<Box<Env>> {
    let t = *enrich(env,t);
    let s = *enrich(env,s);
    match t {
        Term::var(x) =>
            match s {
                Term::var(_) =>
                    /* flex - flex */
                    Option::Some(extend(env,x,Box::new(s))),
                Term::func(_) =>
                    /* flex - rigid */
                    Option::Some(extend(env,x,Box::new(s)))
            },
        Term::func(f) =>
            match s {
                Term::var(y) =>
                    /* rigid - flex */
                    Some(extend(env,y,Box::new(t))),
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
*/

fn show_var(var: &Var){
    print!("{}",var.name);
}

fn show_term(term: &Term){
    match term {
        Term::var(v) => show_var(v),
        Term::func(f) => {
            print!("{}",f.name);
            print!("(");
            show_args(&*f.args);
            print!(")");
        }
    }
}

fn show_args(args: &Args){
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

fn show_env(env: &Env) {
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
    let argnila = Box::new(Args::nil);
    let argnilb = Box::new(Args::nil);
    let envnil = Box::new(Env::nil);
    let xvar = Var {
        name: Box::new("X".to_string()),
    };
    let xbox = Box::new(xvar);
    let xterm = Box::new(Term::var(xbox));
    let yvar = Var {
        name: Box::new("Y".to_string()),
    };
    let ybox = Box::new(yvar);
    let yterm = Box::new(Term::var(ybox));
    let fun = Function {
        name: Box::new("f".to_string()),        
        args: Box::new(Args::cons(xterm,argnila))
    };
    let fun2 = Function {
        name: Box::new("f".to_string()),
        args: Box::new(Args::cons(yterm,argnilb))
    };
    let funterm = Term::func(Box::new(fun));
    let funterm2 = Term::func(Box::new(fun2));

    show_term(&funterm);
    println!("");
    // let res = unify(&env,&funterm,&funterm2);
    /* 
    match res {
        Option::None => println!("No solution"),
        Option::Some(env) => show_env(&env)
    }*/
    
}
