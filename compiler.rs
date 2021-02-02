use std::{cell::RefCell, rc::Rc};

use crate::ast::{BlockStmt, Expr, Ident, Literal, Number, Program, Stmt};
use crate::env::Env;
use crate::macros::Macro;

/// A compiler will hold the AST of the input source.
pub struct Compiler {
    ast: Program,
    scope: Rc<RefCell<Env>>,
}

/// Compiler implementation
impl Compiler {
    /// Create a new compiler instance.
    pub fn new(ast: Program) -> Self {
        Self {
            ast,
            scope: Rc::new(RefCell::new(Env::new())),
        }
    }

    /// Execute AST to JS codegen. Returns javascript code.
    pub fn compile(&self) -> String {
        let mut source = "// Generated by uwu compiler v0.0\n".to_string();
        for stmt in &self.ast {
            if stmt == &Stmt::Blank {
                // PERF: swith to push() instead?
                source.push_str("\n");
                break;
            }

            let block: Option<String> = match stmt {
                Stmt::Expr(expr) => self.compile_expr(expr),
                _ => None,
            };

            if block.is_some() {
                source.push_str(&block.unwrap());
            }
        }
        source
    }

    fn compile_block(&self, ast: &BlockStmt) -> Option<String> {
        let mut source = String::new();
        for stmt in ast {
            if stmt == &Stmt::Blank {
                // PERF: swith to push() instead?
                source.push_str("\n");
                break;
            }

            let block: String = match stmt {
                Stmt::Expr(expr) => self.compile_expr(expr)?,
                Stmt::Return(expr) => {
                    source.push_str("return ");
                    self.compile_expr(expr)?
                }
                _ => return None,
            };

            source.push_str(&block);
        }
        Some(source)
    }

    fn compile_expr(&self, expr: &Expr) -> Option<String> {
        let mut source = String::new();
        match expr {
            Expr::Let(ident, expr) => {
                let value = match self.compile_expr(expr) {
                    Some(value) => value,
                    None => return None,
                };
                let Ident(name) = ident;
                source.push_str(&format!("let {} = {}; \n", name, value));
            }
            Expr::Assign(v, expr) => {
                let value = self.compile_expr(expr)?;
                let name = self.compile_expr(v)?;
                source.push_str(&format!("{} = {}; \n", name, value));
            }
            Expr::Prefix(prefix, expr) => {
                let value = self.compile_expr(expr)?;
                let pre = format!("{}", prefix);
                source.push_str(&pre);
                source.push_str(&value);
            }
            Expr::Infix(infix, e1, e2) => {
                let e1 = self.compile_expr(e1)?;
                let e2 = self.compile_expr(e2)?;
                let infix = format!("{}", infix);

                source.push_str(&e1);
                source.push_str(&infix);
                source.push_str(&e2);
            }
            Expr::Index(exp1, exp2) => {
                let e1 = self.compile_expr(exp1)?;
                let e2 = self.compile_expr(exp2)?;
                source.push_str(&e1);
                source.push_str("[");
                source.push_str(&e2);
                source.push_str("]");
            }
            Expr::Accessor(expr1, expr2) => {
                let e1 = self.compile_expr(expr1)?;
                let Ident(name) = &expr2[0];
                source.push_str(&e1);
                source.push_str(".");
                source.push_str(&name);
            }
            Expr::Literal(lit) => {
                source.push_str(&self.compile_literal(lit)?);
            }
            Expr::Ident(Ident(expr)) => {
                source.push_str(expr);
            }
            Expr::Func { params, body, name } => {
                if let Some(Ident(ident)) = name {
                    source.push_str("function ");
                    source.push_str(ident);
                    source.push_str("(");
                    self.scope.borrow_mut().add(ident.into());
                    for (i, param) in params.iter().enumerate() {
                        let Ident(n) = param;
                        source.push_str(n);
                        if i == params.len() - 1 {
                            break;
                        }
                        source.push_str(",");
                    }
                    source.push_str("){");
                    source.push_str(&self.compile_block(body)?);
                    source.push_str("}");
                }
            }
            Expr::Call { func, args } => {
                let function = &self.compile_expr(func)?;
                if !self.scope.borrow_mut().has(function.into()) {
                    return None;
                };
                source.push_str(function);
                source.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    source.push_str(&self.compile_expr(arg)?);
                    if i == args.len() - 1 {
                        break;
                    }
                    source.push_str(",");
                }
                source.push_str(");");
            }
            Expr::Macro { name, args } => {
                let function = &self.compile_expr(name)?;
                let m = Macro::from_name(function)?;
                source.push_str(&m.expand(args)?);
            }
            Expr::Regexp { pattern, flags } => {
                source.push_str("/");
                source.push_str(&self.compile_expr(pattern)?);
                source.push_str("/");
                if let Some(Ident(fl)) = flags {
                    source.push_str(&fl);
                }
            }
            Expr::While { cond, consequence } => {
                let cnd = &self.compile_expr(cond)?;
                let consq = &self.compile_block(consequence)?;
                source.push_str("while(");
                source.push_str(cnd);
                source.push_str("){");
                source.push_str(consq);
                source.push_str("}");
            }
            Expr::If {
                cond,
                consequence,
                alternative,
            } => {
                let cnd = &self.compile_expr(cond)?;
                let consq = &self.compile_block(consequence)?;
                source.push_str("if(");
                source.push_str(cnd);
                source.push_str("){");
                source.push_str(consq);
                source.push_str("}");
                if alternative.is_some() {
                    let alt = &self.compile_block(alternative.as_ref().unwrap())?;
                    source.push_str("else{");
                    source.push_str(alt);
                    source.push_str("}");
                }
            }
        }
        Some(source)
    }

    fn compile_literal(&self, literal: &Literal) -> Option<String> {
        match literal {
            // Format a number from it's display trait.
            Literal::Number(val) => Some(self.compile_number(val)),
            Literal::String(val) => Some(val.to_string()),
            Literal::Bool(val) => Some(val.to_string()),
            Literal::Array(val) => {
                let mut arr = "[".to_string();
                for (i, exp) in val.iter().enumerate() {
                    if let Some(e) = self.compile_expr(exp) {
                        // Technically, [1, 1,] is valid javascript and does not throw syntax errors.
                        arr.push_str(&e);
                        if i == val.len() - 1 {
                            break;
                        }
                        arr.push_str(",");
                    }
                }
                arr.push_str("]");
                Some(arr)
            }
            Literal::Hash(val) => {
                let mut obj = "{".to_string();
                for (i, (k, v)) in val.iter().enumerate() {
                    if let Some(e) = self.compile_expr(v) {
                        obj.push_str(&self.compile_expr(k)?);
                        obj.push_str(":");
                        obj.push_str(&e);
                        if i == val.len() - 1 {
                            break;
                        }
                        obj.push_str(",");
                    }
                }
                obj.push_str("}");
                Some(obj)
            }
        }
    }

    fn compile_number(&self, val: &Number) -> String {
        match val {
            Number::Int(int) => format!("{}", int),
            Number::Float(flt) => format!("{}", flt),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::*;
    use crate::parser::Parser;
    use crate::tokenizer::Lexer;

    fn c(source: &str) -> String {
        let mut parser = Parser::new(Lexer::new(source));
        let ast = parser.parse();
        let compiler = Compiler::new(ast);
        compiler.compile()
    }

    #[test]
    fn compile_decl() {
        let source =
            "let a = 1; let b = \"hello world\"; let c = true; let d = [1, 2, 2]; let e = { x: 1 }";
        assert_eq!(c(source), "// Generated by uwu compiler v0.0\nlet a = 1; \nlet b = \"hello world\"; \nlet c = true; \nlet d = [1,2,2]; \nlet e = {x:1}; \n");
    }

    #[test]
    fn compile_mut_decl() {
        let source = "let a = 1; a = a + 1 / 2";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nlet a = 1; \na = a+1/2; \n"
        );
    }

    #[test]
    fn compile_fn() {
        let source = "fn add(x, y): end";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nfunction add(x,y){}"
        );
    }

    #[test]
    fn compile_fn_return() {
        let source = "fn add(x, y): return 0 end";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nfunction add(x,y){return 0}"
        );
    }

    #[test]
    fn compile_call() {
        let source = "fn add(x, y): return x + y end add(1, 2)";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nfunction add(x,y){return x+y}add(1,2);"
        );
    }

    #[test]
    fn compile_while() {
        let source = "fn print(): end while(true): print(1) end";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nfunction print(){}while(true){print(1);}"
        );
    }

    #[test]
    fn compile_if() {
        let source = "fn print(): end if(true): print(1); end";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nfunction print(){}if(true){print(1);}"
        );
    }

    #[test]
    fn compile_if_else() {
        let source = "if(x): else: end";
        assert_eq!(
            c(source),
            "// Generated by uwu compiler v0.0\nif(x){}else{}"
        );
    }

    #[test]
    fn compile_prefix_expr() {
        let source = "!x";
        assert_eq!(c(source), "// Generated by uwu compiler v0.0\n!x");
    }

    #[test]
    fn compile_infix_expr() {
        let source = "x + 2 - 1";
        assert_eq!(c(source), "// Generated by uwu compiler v0.0\nx+2-1");
    }

    #[test]
    fn compile_index_expr() {
        let source = "x[1]";
        assert_eq!(c(source), "// Generated by uwu compiler v0.0\nx[1]");
    }
}
