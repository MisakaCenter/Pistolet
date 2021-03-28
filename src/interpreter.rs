use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
enum ValueBind<'a> {
    Vb(&'a str, VarValue), /* type, value*/
}

impl<'a> ValueBind<'a> {
    pub fn get_type(&self) -> &'a str {
        match self {
            ValueBind::Vb(t, _) => t,
        }
    }
    pub fn get_value(&self) -> VarValue {
        match self {
            ValueBind::Vb(_, v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
struct ProgList<'a> {
    var_list: HashMap<&'a str, ValueBind<'a>>,
    func_list: HashMap<&'a str, PistoletAST<'a>>,
}

#[derive(Debug, Clone)]
struct ProgState<'a>(Rc<RefCell<ProgList<'a>>>);

impl<'a> ProgState<'a> {
    pub fn insert(&self, var_name: &'a str, var_value: ValueBind<'a>) {
        self.0.borrow_mut().var_list.insert(var_name, var_value);
    }
    pub fn get(&self, var_name: &'a str) -> Option<ValueBind> {
        match self.0.borrow().var_list.get(var_name) {
            Some(n) => Some(*n),
            None => None,
        }
    }
    pub fn print(&self) {
        println!("------ PROGRAM STATE ------");
        for var in &(self.0.borrow().var_list) {
            let (var_name, ValueBind::Vb(type_name, var_value)) = var;
            println!(
                "Var: {}    Type: {}    Value: {}",
                var_name, type_name, var_value
            )
        }
        println!("------ PROGRAM STATE ------");
    }
}

#[derive(Debug, Copy, Clone)]
enum VarValue {
    Int(i128),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for VarValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarValue::Int(i) => write!(f, "{}", i),
            VarValue::Float(i) => write!(f, "{}", i),
            VarValue::Bool(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug)]
enum RuntimeErr {
    TypeMismatch,
    Unknown,
    VarUsedBeforeDefine,
    DivideByZero,
}

impl RuntimeErr {
    pub fn print(&self) {
        println!("------ Runtime Error ------");
        match self {
            RuntimeErr::TypeMismatch => println!("[Error] Type mismatch in an expression"),
            RuntimeErr::VarUsedBeforeDefine => println!("[Error] Var used before defined"),
            RuntimeErr::Unknown => println!("[Error] An exception has occurred"),
            RuntimeErr::DivideByZero => println!("[Error] Attempt to divide by zero "),
        }
        println!("------ Runtime Error ------");
    }
}

fn type_dec(v1: VarValue, v2: VarValue) -> bool {
    match v1 {
        VarValue::Int(_) => match v2 {
            VarValue::Int(_) => true,
            _ => false,
        },
        VarValue::Float(_) => match v2 {
            VarValue::Float(_) => true,
            _ => false,
        },
        VarValue::Bool(_) => match v2 {
            VarValue::Bool(_) => true,
            _ => false,
        },
    }
}

fn var_eval<'a>(
    name: &'a str,
    global_state: &'a ProgState<'a>,
) -> Result<ValueBind<'a>, RuntimeErr> {
    match global_state.get(name) {
        Some(result) => Ok(result),
        None => Err(RuntimeErr::VarUsedBeforeDefine),
    }
}

fn expr_eval<'a>(
    expr: PistoletExpr<'a>,
    state: &'a ProgState<'a>,
) -> Result<ValueBind<'a>, RuntimeErr> {
    match expr {
        PistoletExpr::Val(value) => match value {
            PistoletValue::Integer(n) => Ok(ValueBind::Vb("int", VarValue::Int(n))),
            PistoletValue::Float(n) => Ok(ValueBind::Vb("float", VarValue::Float(n))),
            PistoletValue::Boolean(n) => Ok(ValueBind::Vb("bool", VarValue::Bool(n))),
            PistoletValue::Var(n) => var_eval(n, state),
            _ => unimplemented!(),
        },
        PistoletExpr::Add(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => Ok(ValueBind::Vb("int", VarValue::Int(n + m))),
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => Ok(ValueBind::Vb("float", VarValue::Float(n + m))),
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Sub(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => Ok(ValueBind::Vb("int", VarValue::Int(n - m))),
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => Ok(ValueBind::Vb("float", VarValue::Float(n - m))),
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Mul(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => Ok(ValueBind::Vb("int", VarValue::Int(n * m))),
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => Ok(ValueBind::Vb("float", VarValue::Float(n * m))),
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Div(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            if m == 0 {
                                Err(RuntimeErr::DivideByZero)
                            } else {
                                Ok(ValueBind::Vb("int", VarValue::Int(n / m)))
                            }
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            let r = n / m;
                            if r.is_infinite() {
                                Err(RuntimeErr::DivideByZero)
                            } else {
                                Ok(ValueBind::Vb("float", VarValue::Float(r)))
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::And(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Bool(n) => match v2 {
                        VarValue::Bool(m) => Ok(ValueBind::Vb("bool", VarValue::Bool(n && m))),
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Orb(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Bool(n) => match v2 {
                        VarValue::Bool(m) => Ok(ValueBind::Vb("bool", VarValue::Bool(n || m))),
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Nand(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Bool(n) => match v2 {
                        VarValue::Bool(m) => Ok(ValueBind::Vb("bool", VarValue::Bool(!(n && m)))),
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Eq(e1, e2) => {
            let v1 = expr_eval(*e1, state)?;
            let v2 = expr_eval(*e2, state)?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => Ok(ValueBind::Vb("bool", VarValue::Bool(n == m))),
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => Ok(ValueBind::Vb("bool", VarValue::Bool(n == m))),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
    }
}

fn seq_eval<'a>(ast: PistoletAST<'a>, state: &'a ProgState<'a>) -> Option<RuntimeErr> {
    let mut error: RuntimeErr = RuntimeErr::Unknown;
    let mut error_state = false;
    match ast {
        PistoletAST::Seq(term_list) => {
            for term in term_list {
                match ast_eval(term, state) {
                    Ok(_) => continue,
                    Err(err) => {
                        error = err;
                        error_state = true;
                        break;
                    }
                }
            }
        }
        _ => unreachable!(),
    }
    if error_state {
        Some(error)
    } else {
        None
    }
}

fn ast_eval<'a>(
    ast: PistoletAST<'a>,
    state: &'a ProgState<'a>,
) -> Result<&'a ProgState<'a>, RuntimeErr> {
    match ast {
        PistoletAST::Seq(term_list) => match seq_eval(PistoletAST::Seq(term_list), state) {
            Some(err) => Err(err),
            None => Ok(state),
        },
        PistoletAST::Let(var_name, var_type, var_expr) => {
            let var_value = expr_eval(var_expr, state)?;
            if var_value.get_type().eq_ignore_ascii_case(var_type) {
                state.insert(var_name, var_value);
                Ok(state)
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletAST::EOI => Ok(state),
        _ => Err(RuntimeErr::Unknown),
    }
}
