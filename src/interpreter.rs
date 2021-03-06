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
struct FuncDic {
    func_list: HashMap<String, (PistoletAST, String, PistoletAST)>
}

impl FuncDic {
    pub fn new() -> FuncDic {
        let func_list = FuncDic{
            func_list: HashMap::new()
        };
        return func_list;
    }

    pub fn find_func(
        &self,
        name: String,
    ) -> Result<(PistoletAST, String, Vec<(String, String)>), RuntimeErr> {
        let fun = self.func_list.get(&name);
            match fun {
                Some(result) => {
                    match result {
                        (PistoletAST::Paralist(paralist), func_type, func_body) => {
                            let para_vec = para_to_vec(paralist.clone());
                            Ok((func_body.clone(), func_type.clone(), para_vec))
                        }
                        _ => unreachable!(),
                    }
                }
                None => Err(RuntimeErr::FuncUsedBeforeDefine)
            }
    }

    pub fn func_insert(
        &mut self,
        func_name: String,
        para_list: PistoletAST,
        return_type: String,
        func_body: PistoletAST,
    ) {
        self.func_list
            .insert(func_name, (para_list, return_type, func_body));
    }
}

#[derive(Debug)]
struct ProgList {
    var_list: HashMap<String, ValueBind>
}

#[derive(Debug)]
struct ProgState(Rc<RefCell<ProgList>>);

#[derive(Debug)]
struct StateVec {
    states: VecDeque<ProgState>,
}

#[derive(Debug, Clone)]
struct ProgStates(Rc<RefCell<StateVec>>);

impl ProgStates {
    pub fn new() -> ProgStates {
        let main_state = ProgState(Rc::new(RefCell::new(ProgList {
            var_list: HashMap::new()
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
        let mut r: ValueBind = ValueBind::Vb("foo".to_string(), VarValue::Bool(true)); // dummy useless initial value
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
    pub fn print(&self) {
        self.0.borrow_mut().states.get(0).unwrap().print();
    }
}

impl ProgState {
    pub fn insert(&self, var_name: String, var_value: ValueBind) {
        self.0.borrow_mut().var_list.insert(var_name, var_value);
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
    FuncUsedBeforeDefine,
    DivideByZero,
    FuncallParaNum,
    FunctionNoReturn,
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
            RuntimeErr::FuncUsedBeforeDefine => println!("[Error] Function used before defined"),
            RuntimeErr::FuncallParaNum => println!("[Error] wrong number function call"),
            RuntimeErr::FunctionNoReturn => println!("[Error] function no return"),
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

fn para_to_vec(paralist: Vec<PistoletAST>) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    for bind in paralist.iter() {
        match bind {
            PistoletAST::Varbind(v, t) => {
                result.push((v.to_string(), t.to_string()));
            }
            _ => unreachable!(),
        }
    }
    result
}

fn var_eval(name: String, states: ProgStates) -> Result<ValueBind, RuntimeErr> {
    states.find_var(name)
}

fn func_eval(
    name: String,
    expr_list: Vec<PistoletExpr>,
    states: ProgStates,
    func_list: FuncDic
    
) -> Result<ValueBind, RuntimeErr> {
    let (func_body, func_type, para_list) = func_list.find_func(name)?;
    let mut val_list: Vec<ValueBind> = Vec::new();
    for expr in expr_list.iter() {
        let expr_val = expr_eval(expr.clone(), states.clone(), func_list.clone()).unwrap();
        val_list.push(expr_val);
    }
    if val_list.len() == para_list.len() {
        let sub_state = ProgState(Rc::new(RefCell::new(ProgList {
            var_list: HashMap::new()
        })));
        states.push_front(sub_state);
        for (index, val) in val_list.iter().enumerate() {
            let (para_name, para_type) = para_list.get(index).unwrap();
            if val.get_type().eq_ignore_ascii_case(&para_type) {
                states.insert(para_name.clone(), val.clone());
            } else {
                states.pop_front();
                return Err(RuntimeErr::TypeMismatch);
            }
        }
        let result = ast_eval(func_body, states.clone(), &mut func_list.clone());
        let func_result: Result<ValueBind, RuntimeErr>;
        match result {
            Err(some_err) => match some_err {
                RuntimeErr::ReturnValue(expr_value) => {
                    if expr_value.get_type().eq_ignore_ascii_case(&func_type) {
                        func_result = Ok(expr_value);
                    } else {
                        func_result = Err(RuntimeErr::TypeMismatch);
                    }
                }
                _ => func_result = Err(some_err),
            },
            Ok(_) => func_result = Err(RuntimeErr::FunctionNoReturn),
        }
        states.pop_front();
        return func_result;
    } else {
        return Err(RuntimeErr::FuncallParaNum);
    }
}

fn expr_eval(expr: PistoletExpr, state: ProgStates, func_list: FuncDic) -> Result<ValueBind, RuntimeErr> {
    match expr {
        PistoletExpr::Val(value) => match value {
            PistoletValue::Integer(n) => Ok(ValueBind::Vb("int".to_string(), VarValue::Int(n))),
            PistoletValue::Float(n) => Ok(ValueBind::Vb("float".to_string(), VarValue::Float(n))),
            PistoletValue::Boolean(n) => Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n))),
            PistoletValue::Var(n) => var_eval(n, state),
            PistoletValue::Funcall(func_name, expr_list) => func_eval(func_name, expr_list, state, func_list.clone()),
        },
        PistoletExpr::Add(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
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
        PistoletExpr::Leq(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n <= m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n <= m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Req(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n >= m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n >= m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Left(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n < m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n < m)))
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletExpr::Right(e1, e2) => {
            let v1 = expr_eval(*e1, state.clone(), func_list.clone())?;
            let v2 = expr_eval(*e2, state.clone(), func_list.clone())?;
            let v1 = v1.get_value();
            let v2 = v2.get_value();

            if type_dec(v1, v2) {
                match v1 {
                    VarValue::Int(n) => match v2 {
                        VarValue::Int(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n > m)))
                        }
                        _ => unreachable!(),
                    },
                    VarValue::Float(n) => match v2 {
                        VarValue::Float(m) => {
                            Ok(ValueBind::Vb("bool".to_string(), VarValue::Bool(n > m)))
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

fn seq_eval(ast: PistoletAST, state: ProgStates, mut func_list: FuncDic) -> Option<RuntimeErr> {
    let mut error: RuntimeErr = RuntimeErr::Unknown;
    let mut error_state = false;
    match ast {
        PistoletAST::Seq(term_list) => {
            for term in term_list {
                match ast_eval(term, state.clone(), &mut func_list) {
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

fn ast_eval(ast: PistoletAST, state: ProgStates, func_list: &mut FuncDic) -> Result<(ProgStates, FuncDic), RuntimeErr> {
    match ast {
        PistoletAST::Seq(term_list) => match seq_eval(PistoletAST::Seq(term_list), state.clone(), func_list.clone()) {
            Some(err) => Err(err),
            None => Ok((state.clone(), func_list.clone())),
        },
        PistoletAST::Let(var_name, var_type, var_expr) => {
            let var_value = expr_eval(var_expr, state.clone(), func_list.clone())?;
            if var_value.get_type().eq_ignore_ascii_case(&var_type) {
                state.insert(var_name, var_value);
                Ok((state.clone(), func_list.clone()))
            } else {
                Err(RuntimeErr::TypeMismatch)
            }
        }
        PistoletAST::If(expr, branch_true, branch_false) => {
            let expr_value = expr_eval(expr, state.clone(), func_list.clone())?;
            let sub_state = ProgState(Rc::new(RefCell::new(ProgList {
                var_list: HashMap::new()
            })));
            state.push_front(sub_state);
            match expr_value.get_value() {
                VarValue::Bool(true) => match seq_eval(*branch_true, state.clone(), func_list.clone()) {
                    Some(err) => {
                        state.pop_front();
                        Err(err)
                    }
                    None => {
                        state.pop_front();
                        Ok((state.clone(), func_list.clone()))
                    }
                },
                VarValue::Bool(false) => match seq_eval(*branch_false, state.clone(), func_list.clone()) {
                    Some(err) => {
                        state.pop_front();
                        Err(err)
                    }
                    None => {
                        state.pop_front();
                        Ok((state.clone(), func_list.clone()))
                    }
                },
                _ => unreachable!(),
            }
        }
        PistoletAST::While(seq, expr) => {
            let info: Result<(ProgStates, FuncDic), RuntimeErr>;
            let sub_state = ProgState(Rc::new(RefCell::new(ProgList {
                var_list: HashMap::new()
            })));
            state.push_front(sub_state);
            loop {
                match seq_eval(*seq.clone(), state.clone(), func_list.clone()) {
                    Some(err) => {
                        info = Err(err);
                        break;
                    } //Here can process break. in future...
                    None => {
                        let expr_value = expr_eval(expr.clone(), state.clone(), func_list.clone())?.get_value();
                        match expr_value {
                            VarValue::Bool(true) => {
                                info = Ok((state.clone(), func_list.clone()));
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
            func_list.func_insert(func_name, *para_list, return_type, *fun_body);
            Ok((state.clone(), func_list.clone()))
        }
        PistoletAST::Return(expr) => {
            let expr_value = expr_eval(expr, state, func_list.clone())?;
            Err(RuntimeErr::ReturnValue(expr_value))
        }
        PistoletAST::PrintLine(expr) => {
            let expr_value = expr_eval(expr, state.clone(), func_list.clone())?;
            println!("{} : {}", expr_value.get_value(), expr_value.get_type());
            Ok((state.clone(), func_list.clone()))
        }
        PistoletAST::EOI => Ok((state.clone(), func_list.clone())),
        _ => Err(RuntimeErr::Unknown),
    }
}
