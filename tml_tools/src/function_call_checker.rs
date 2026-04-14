use tml_parser::tml_actions::*;
use crate::symbol_table::{Scope, SimpleTypeKind, SymbolTable, SymbolType};
use crate::type_inference::infer_type;
use crate::visitor::AstVisitor;

// ───────────────────────── Entry functions ─────────────────────────

const ENTRY_FUNCTIONS: &[&str] = &["init_fnc", "output_fnc", "update_fnc"];

pub fn is_entry_function(name: &str) -> bool {
    ENTRY_FUNCTIONS.contains(&name)
}

// ───────────────────────── Built-ins ─────────────────────────

pub struct BuiltinSignature {
    name: &'static str,
    arg_count: usize,
}

const BUILTINS: &[BuiltinSignature] = &[
    // 1 argument
    BuiltinSignature { name: "min",   arg_count: 1 },
    BuiltinSignature { name: "min!",  arg_count: 1 },
    BuiltinSignature { name: "max",   arg_count: 1 },
    BuiltinSignature { name: "max!",  arg_count: 1 },
    BuiltinSignature { name: "all",   arg_count: 1 },
    BuiltinSignature { name: "all!",  arg_count: 1 },
    BuiltinSignature { name: "any",   arg_count: 1 },
    BuiltinSignature { name: "any!",  arg_count: 1 },
    BuiltinSignature { name: "isnan", arg_count: 1 },
    BuiltinSignature { name: "sin",   arg_count: 1 },
    BuiltinSignature { name: "cos",   arg_count: 1 },
    BuiltinSignature { name: "tan",   arg_count: 1 },
    BuiltinSignature { name: "asin",  arg_count: 1 },
    BuiltinSignature { name: "acos",  arg_count: 1 },
    BuiltinSignature { name: "atan",  arg_count: 1 },
    BuiltinSignature { name: "floor", arg_count: 1 },
    BuiltinSignature { name: "ceil",  arg_count: 1 },
    BuiltinSignature { name: "round", arg_count: 1 },
    BuiltinSignature { name: "trunc", arg_count: 1 },
    BuiltinSignature { name: "abs",   arg_count: 1 },
    BuiltinSignature { name: "log",   arg_count: 1 },
    BuiltinSignature { name: "log10", arg_count: 1 },
    BuiltinSignature { name: "sqrt",  arg_count: 1 },
    // 2 arguments
    BuiltinSignature { name: "atan2",  arg_count: 2 },
    BuiltinSignature { name: "getbit", arg_count: 2 },
    // 3 arguments
    BuiltinSignature { name: "setbit", arg_count: 3 },
];

pub fn lookup_builtin(name: &str) -> Option<&'static BuiltinSignature> {
    BUILTINS.iter().find(|b| b.name == name)
}

/// Infers the return type of built-in function based on its arguments.
/// Built-in functions are polymorphic — return type matches the input type.
pub fn infer_builtin_return_type(
    name: &str,
    args: &[&Argument],
    table: &SymbolTable,
    scope: &Scope,
) -> Option<SymbolType> {
    let base = name.trim_end_matches('!');

    match base {
        // Math functions — return type matches argument type
        "min" | "max" | "abs" | "sin" | "cos" | "tan"
        | "asin" | "acos" | "atan" | "floor" | "ceil"
        | "round" | "trunc" | "log" | "log10" | "sqrt" => {
            let arg_expr = match args.first()? {
                Argument::C1(a) => &a.value,
                Argument::C2(a) => &a.value,
            };
            infer_type(arg_expr, table, scope)
        }
        // atan2 — promote both arguments
        "atan2" => {
            if args.len() < 2 { return None; }
            let a1 = match args[0] {
                Argument::C1(a) => infer_type(&a.value, table, scope),
                Argument::C2(a) => infer_type(&a.value, table, scope),
            }?;
            let a2 = match args[1] {
                Argument::C1(a) => infer_type(&a.value, table, scope),
                Argument::C2(a) => infer_type(&a.value, table, scope),
            }?;
            Some(crate::type_inference::promote(&a1, &a2))
        }
        // Boolean reductions
        "all" | "any" | "isnan" => Some(SymbolType::Simple(SimpleTypeKind::Bool)),
        // Bit operations — always return uint
        "getbit" | "setbit" => Some(SymbolType::Simple(SimpleTypeKind::Uint)),
        _ => None,
    }
}

// ───────────────────────── Errors ─────────────────────────

#[derive(Debug, Clone)]
pub enum CallError {
    UndefinedFunction { name: String, scope: Scope },
    ArgumentCountMismatch { name: String, expected: usize, got: usize, scope: Scope },
    NamedArgumentNotAllowed { function_name: String, arg_name: String, scope: Scope },
    EntryFunctionCall { name: String, scope: Scope },
}

impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallError::UndefinedFunction { name, scope } => match scope {
                Scope::Global => write!(f, "Undefined function '{}'", name),
                Scope::Function(fn_name) => {
                    write!(f, "Undefined function '{}' called from '{}'", name, fn_name)
                }
            },
            CallError::ArgumentCountMismatch { name, expected, got, .. } => {
                write!(f, "Function '{}' expects {} argument(s), got {}", name, expected, got)
            }
            CallError::NamedArgumentNotAllowed { function_name, arg_name, .. } => {
                write!(
                    f,
                    "Named argument '{}' not allowed in call to '{}', use positional arguments",
                    arg_name, function_name
                )
            }
            CallError::EntryFunctionCall { name, scope } => match scope {
                Scope::Global => write!(f, "Entry function '{}' cannot be called from user code", name),
                Scope::Function(fn_name) => write!(
                    f,
                    "Entry function '{}' cannot be called from '{}'",
                    name, fn_name
                ),
            },
        }
    }
}

// ───────────────────────── Checker ─────────────────────────

pub struct FunctionCallChecker<'a> {
    table: &'a SymbolTable,
    errors: Vec<CallError>,
}

impl<'a> FunctionCallChecker<'a> {
    pub fn new(table: &'a SymbolTable) -> Self {
        FunctionCallChecker { table, errors: vec![] }
    }

    pub fn check(mut self, unit: &TranslationUnit) -> Vec<CallError> {
        for decl in &unit.ext_decls {
            match decl {
                ExternalDeclaration::FunctionDefinition(f) => {
                    let scope = Scope::Function(f.id.value.clone());
                    self.visit_statement_block(&f.statement_block, &scope);
                }
                ExternalDeclaration::DeclarationStatement(d) => {
                    self.visit_expression(&d.rvalue, &Scope::Global);
                }
                ExternalDeclaration::AssignmentStatement(s) => {
                    self.visit_assignment(s, &Scope::Global);
                }
                ExternalDeclaration::IoWriteStatement(s) => {
                    self.visit_io_write(s, &Scope::Global);
                }
                ExternalDeclaration::MacroFor(m) => {
                    self.visit_for(&m.body, &Scope::Global);
                }
                ExternalDeclaration::MacroIf(m) => {
                    self.visit_selection(&m.body, &Scope::Global);
                }
                ExternalDeclaration::IoDeclarationStatement(_) => {}
            }
        }
        self.errors
    }
}

// ───────────────────────── AstVisitor impl ─────────────────────────

impl<'a> AstVisitor for FunctionCallChecker<'a> {
    fn visit_function_call(&mut self, call: &FunctionCall, scope: &Scope) {
        let name = &call.id.value;

        let args: Vec<&Argument> = match &call.arguments_list {
            None => vec![],
            Some(args) => args.iter().collect(),
        };

        // Check named arguments and visit argument expressions
        for arg in &args {
            if let Argument::C1(a) = arg {
                self.errors.push(CallError::NamedArgumentNotAllowed {
                    function_name: name.clone(),
                    arg_name: a.id.value.clone(),
                    scope: scope.clone(),
                });
            }
            let val = match arg {
                Argument::C1(a) => &a.value,
                Argument::C2(a) => &a.value,
            };
            self.visit_expression(val, scope);
        }

        // Entry functions cannot be called from user code
        if is_entry_function(name) {
            self.errors.push(CallError::EntryFunctionCall {
                name: name.clone(),
                scope: scope.clone(),
            });
            return;
        }

        // Check built-in argument count
        if let Some(builtin) = lookup_builtin(name) {
            let got = args.len();
            if builtin.arg_count != got {
                self.errors.push(CallError::ArgumentCountMismatch {
                    name: name.clone(),
                    expected: builtin.arg_count,
                    got,
                    scope: scope.clone(),
                });
            }
            return;
        }

        // Check user-defined function
        match self.table.lookup_function(name) {
            None => {
                self.errors.push(CallError::UndefinedFunction {
                    name: name.clone(),
                    scope: scope.clone(),
                });
            }
            Some(sig) => {
                let expected = sig.params.len();
                let got = args.len();
                if expected != got {
                    self.errors.push(CallError::ArgumentCountMismatch {
                        name: name.clone(),
                        expected,
                        got,
                        scope: scope.clone(),
                    });
                }
            }
        }
    }
}