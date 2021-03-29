use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum ValueBind {
    Vb(String, VarValue), /* type, value*/
}

impl ValueBind {
    pub fn get_type(&self) -> String {
        match self {
            ValueBind::Vb(t, _) => t.to_string(),
        }
    }
    pub fn get_value(&self) -> VarValue {
        match self {
            ValueBind::Vb(_, v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
struct ProgList {
    var_list: HashMap<String, ValueBind>,
    func_list: HashMap<String, (PistoletAST, String, PistoletAST)>,
}

#[derive(Debug, Clone)]
struct ProgState(Rc<RefCell<ProgList>>);

#[derive(Debug, Clone)]
struct StateVec {
    states: VecDeque<ProgState>,
}

#[derive(Debug, Clone)]
struct ProgStates(Rc<RefCell<StateVec>>);

impl ProgStates {
    pub fn new() -> ProgStates {
        let main_state = ProgState(Rc::new(RefCell::new(ProgList {
            var_list: HashMap::new(),
            func_list: HashMap::new(),
        })));

        let state = ProgStates(Rc::new(RefCell::new(StateVec {
            states: VecDeque::new(),
        })));

        state.push_back(main_state);
        return state;
    }
    pub fn push_front(&self, state: ProgState) {
        self.0.borrow_mut().states.push_front(state)
    }
    pub fn push_back(&self, state: ProgState) {
        self.0.borrow_mut().states.push_back(state)
    }
    pub fn pop_front(&self) {
        self.0.borrow_mut().states.pop_front();
    }
    pub fn find_var(&self, name: String) -> Result<ValueBind, RuntimeErr> {
        let mut r: ValueBind = ValueBind::Vb("foo".to_string(), VarValue::Bool(true));
        let mut find_var: bool = false;
        for state in self.0.borrow().states.iter() {
            match state.get(name.clone()) {
                Some(result) => {
                    r = result.clone();
                    find_var = true;
                    break;
                }
                None => continue,
            }
        }
        if find_var {
            Ok(r)
        } else {
            Err(RuntimeErr::VarUsedBeforeDefine)
        }
    }
    pub fn insert(&self, var_name: String, var_value: ValueBind) {
        self.0
            .borrow_mut()
            .states
            .get(0)
            .unwrap()
            .insert(var_name, var_value);
    }
    pub fn func_insert(
        &self,
        func_name: String,
        para_list: PistoletAST,
        return_type: String,
        func_body: PistoletAST,
    ) {
        self.0.borrow_mut().states.get(0).unwrap().func_insert(
            func_name,
            para_list,
            return_type,
            func_body,
        );
    }
    pub fn print(&self) {
        self.0.borrow_mut().states.get(0).unwrap().print();
    }
}

impl ProgState {
    pub fn insert(&self, var_name: String, var_value: ValueBind) {
        self.0.borrow_mut().var_list.insert(var_name, var_value);
    }
    pub fn func_insert(
        &self,
        func_name: String,
        para_list: PistoletAST,
        return_type: String,
        func_body: PistoletAST,
    ) {
        self.0
            .borrow_mut()
            .func_list
            .insert(func_name, (para_list, return_type, func_body));
    }
    pub fn get(&self, var_name: String) -> Option<ValueBind> {
        match self.0.borrow().var_list.get(&var_name) {
            Some(n) => Some(n.clone()),
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
    ReturnValue(ValueBind),
}

impl RuntimeErr {
    pub fn print(&self) {
        println!("------ Runtime Error ------");
        match self {
            RuntimeErr::TypeMismatch => println!("[Error] Type mismatch in an expression"),
            RuntimeErr::VarUsedBeforeDefine => println!("[Error] Var used before defined"),
            RuntimeErr::Unknown => println!("[Error] An exception has occurred"),
            RuntimeErr::DivideByZero => println!("[Error] Attempt to divide by zero "),
            _ => unreachable!(),
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

fn var_eval(name: String, states: ProgStates) -> Result<ValueBind, RuntimeErr> {
    states.find_var(name)
}

fn expr_eval(expr: PistoletExpr, state: ProgStates) -> Result<ValueBind, RuntimeErr> {
    match expr {
        PistoletExpr::Val(value) => match value {
            PistoletValue::Integer(n) => Ok(ValueBind::Vb("int".to_string(), VarValue::Int(n))),
            PistoletValue::Float(n) => Ok(ValueBind::Vb("float".to_string(), VarValue::Float(n))),
            PistoletValue::Boolean(n) => Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n))),
            PistoletValue::Var(n) => var_eval(n, state),
            _ => unimplemented!(),
        },
        PistoletExpr::Add(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("int".to_string(), VarValue::Int(n + m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("float".to_string(), VarValue::Float(n + m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Sub(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("int".to_string(), VarValue::Int(n - m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("float".to_string(), VarValue::Float(n - m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Mul(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("int".to_string(), VarValue::Int(n * m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("float".to_string(), VarValue::Float(n * m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Div(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            if m == 0 {
                                Err(RuntimeErr::DivideByZero)
                            } else {
                                Ok(ValueBind::Vb("int".to_string(), VarValue::Int(n / m)))
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
                                Ok(ValueBind::Vb("float".to_string(), VarValue::Float(r)))
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
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Bool(n) => match v2 {
                        VarValue::Bool(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n && m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Orb(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Bool(n) => match v2 {
                        VarValue::Bool(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n || m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Nand(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Bool(n) => match v2 {
                        VarValue::Bool(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(!(n && m))))
                        }
                        _ => unreachable!(),
                    },
                    _ => Err(RuntimeErr::TypeMismatch),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Eq(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone())?;
            let v2 = expr_eval(*e2, state.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n == m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n == m)))
                        }
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

fn seq_eval(ast: PistoletAST, state: ProgStates) -> Option<RuntimeErr> {
    let mut error: RuntimeErr = RuntimeErr::Unknown;
    let mut error_state = false;
    match ast {
        PistoletAST::Seq(term_list) => {
            for term in term_list {
                match ast_eval(term, state.clone()) {
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

fn ast_eval(ast: PistoletAST, state: ProgStates) -> Result<ProgStates, RuntimeErr> {
    match ast {
        PistoletAST::Seq(term_list) => match seq_eval(PistoletAST::Seq(term_list), state.clone()) {
            Some(err) => Err(err),
            None => Ok(state.clone()),
        },
        PistoletAST::Let(var_name, var_type, var_expr) => {
            let var_value = expr_eval(var_expr, state.clone())?;
            if var_value.get_type().eq_ignore_ascii_case(&var_type) {
                state.insert(var_name, var_value);
                Ok(state)
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletAST::If(expr, branch_true, branch_false) => {
            let expr_value = expr_eval(expr, state.clone())?;
            let sub_state = ProgState(Rc::new(RefCell::new(ProgList {
                var_list: HashMap::new(),
                func_list: HashMap::new(),
            })));
            state.push_front(sub_state);
            match expr_value.get_value() {
                VarValue::Bool(true) => match seq_eval(*branch_true, state.clone()) {
                    Some(err) => {
                        state.pop_front();
                        Err(err)
                    }
                    None => {
                        state.pop_front();
                        Ok(state)
                    }
                },
                VarValue::Bool(false) => match seq_eval(*branch_false, state.clone()) {
                    Some(err) => {
                        state.pop_front();
                        Err(err)
                    }
                    None => {
                        state.pop_front();
                        Ok(state)
                    }
                },
                _ => unreachable!(),
            }
        }
        PistoletAST::While(seq, expr) => {
            let info: Result<ProgStates, RuntimeErr>;
            let sub_state = ProgState(Rc::new(RefCell::new(ProgList {
                var_list: HashMap::new(),
                func_list: HashMap::new(),
            })));
            state.push_front(sub_state);
            loop {
                match seq_eval(*seq.clone(), state.clone()) {
                    Some(err) => {
                        info = Err(err);
                        break;
                    } //Here can process break. in future...
                    None => {
                        let expr_value = expr_eval(expr.clone(), state.clone())?.get_value();
                        match expr_value {
                            VarValue::Bool(true) => {
                                info = Ok(state.clone());
                                break;
                            }
                            VarValue::Bool(false) => {
                                continue;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            state.pop_front();
            info
        }
        PistoletAST::Fun(func_name, para_list, return_type, fun_body) => {
            state.func_insert(func_name, *para_list, return_type, *fun_body);
            Ok(state)
        }
        PistoletAST::Return(expr) => {
            let expr_value = expr_eval(expr, state.clone())?;
            Err(RuntimeErr::ReturnValue(expr_value))
        }
        PistoletAST::PrintLine(expr) => {
            let expr_value = expr_eval(expr, state.clone())?;
            println!("{} : {}", expr_value.get_value(), expr_value.get_type());
            Ok(state)
        }
        PistoletAST::EOI => Ok(state),
        _ => Err(RuntimeErr::Unknown),
    }
}
