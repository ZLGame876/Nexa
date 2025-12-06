// 导入解析器模块中的类型定义
use crate::parser::{DataType, Expr, Operator, Statement};

// 代码生成器结构体：将抽象语法树转换为Rust代码
pub struct CodeGenerator {
    // 存储生成的Rust代码字符串
    code: String,
    // 当前的缩进级别，用于格式化输出
    indent_level: usize,
}

// 代码生成器实现块
impl CodeGenerator {
    // 创建新的代码生成器实例
    fn new() -> Self {
        CodeGenerator {
            code: String::new(),    // 初始化空字符串用于存储代码
            indent_level: 0,        // 初始缩进级别为0
        }
    }
    
    // 添加当前缩进级别的空格
    fn indent(&mut self) {
        self.code.push_str(&"    ".repeat(self.indent_level));
    }
    
    // 增加缩进级别
    fn increment_indent(&mut self) {
        self.indent_level += 1;
    }
    
    // 减少缩进级别
    fn decrement_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    
    // 生成表达式代码
    // 根据表达式类型生成对应的Rust代码
    fn generate_expression(&mut self, expr: &Expr) {
        match expr {
            // 数字字面量：直接转换为字符串
            Expr::Number(n) => {
                self.code.push_str(&n.to_string());
            },
            // 字符串字面量：直接使用双引号
            Expr::String(s) => {
                self.code.push('"');
                self.code.push_str(s);
                self.code.push('"');
            },
            // 布尔字面量：直接生成 true 或 false
            Expr::BoolLiteral(b) => {
                self.code.push_str(if *b { "true" } else { "false" });
            },
            // 标识符：直接使用变量名
            Expr::Identifier(id) => {
                self.code.push_str(id);
            },
            // 二元运算表达式：生成 (left op right) 格式
            Expr::BinaryOp(left, op, right) => {
                self.code.push('(');
                self.generate_expression(left);
                self.code.push(' ');
                
                // 根据运算符类型生成对应的Rust运算符
                match op {
                    Operator::Add => self.code.push_str("+"),
                    Operator::Sub => self.code.push_str("-"),
                    Operator::Mul => self.code.push_str("*"),
                    Operator::Div => self.code.push_str("/"),
                    Operator::Eq => self.code.push_str("=="),
                    Operator::Lt => self.code.push_str("<"),
                    Operator::Gt => self.code.push_str(">"),
                }
                
                self.code.push(' ');
                self.generate_expression(right);
                self.code.push(')');
            },
            // 一元运算表达式：目前只支持负号
            Expr::UnaryOp(op, expr) => {
                match op {
                    Operator::Sub => self.code.push('-'),
                    _ => panic!("不支持的一元运算符: {:?}", op),
                }
                self.generate_expression(expr);
            },
        }
    }
    
    // 生成语句代码
    // 根据语句类型生成对应的Rust代码
    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            // 变量声明语句：var name [: type] = expression;
            Statement::Var(name, data_type, expr) => {
                self.indent();
                self.code.push_str("let mut ");
                self.code.push_str(name);
                if let Some(dt) = data_type {
                    self.code.push_str(": ");
                    let type_str = match dt {
                        DataType::Int => "i32",
                        DataType::Float => "f64",
                        DataType::Bool => "bool",
                        DataType::String => "String",
                    };
                    self.code.push_str(type_str);
                }
                self.code.push_str(" = ");
                
                // 根据表达式类型和目标类型进行适当的转换
                match (data_type, expr.as_ref()) {
                    // 整数赋值给Float类型时，需要添加.0使其成为浮点数字面量
                    (Some(DataType::Float), &Expr::Number(n)) => {
                        self.code.push_str(&n.to_string());
                        self.code.push_str(".0");
                    },
                    // 字符串字面量赋值给String类型时，需要添加.to_string()
                    (Some(DataType::String), &Expr::String(_)) => {
                        self.code.push_str("(");
                        self.generate_expression(expr);
                        self.code.push_str(").to_string()");
                    },
                    // 其他情况直接生成表达式
                    _ => {
                        self.generate_expression(expr);
                    }
                }
                
                self.code.push_str(";\n");
            },
            // 变量赋值语句：name = expression;
            Statement::Assign(name, expr) => {
                self.indent();
                self.code.push_str(name);
                self.code.push_str(" = ");
                self.generate_expression(expr);
                self.code.push_str(";\n");
            },
            // 打印语句：print!(...) 或 print!("{}", ...) - 不换行输出
            Statement::Print(expr) => {
                self.indent();
                self.code.push_str("print!");
                
                // 检查是否是二元操作表达式（多参数合并的结果）
                match &**expr {
                    // 如果是二元操作表达式，生成多个参数的格式
                    Expr::BinaryOp(_, _, _) => {
                        // 收集所有参数
                        fn collect_params<'a>(expr: &'a Expr, params: &mut Vec<&'a Expr>) {
                            match expr {
                                Expr::BinaryOp(left, _, right) => {
                                    collect_params(left, params);
                                    collect_params(right, params);
                                },
                                _ => params.push(expr),
                            }
                        }
                        
                        let mut params = Vec::new();
                        collect_params(expr, &mut params);
                        
                        // 生成格式字符串（用空格分隔的{}）
                        self.code.push_str("(\"");
                        for i in 0..params.len() {
                            if i > 0 {
                                self.code.push_str(" ");
                            }
                            self.code.push_str("{}");
                        }
                        self.code.push_str("\"");
                        
                        // 生成所有参数，用逗号分隔
                        for (_i, param) in params.iter().enumerate() {
                            self.code.push_str(", ");
                            self.generate_expression(param);
                        }
                        self.code.push(')');
                    },
                    // 如果是字符串字面量或字符串变量，直接使用括号形式
                    Expr::String(_) => {
                        self.code.push('(');
                        self.generate_expression(expr);
                        self.code.push(')');
                    },
                    // 数字字面量使用默认格式
                    Expr::Number(_) => {
                        self.code.push_str("(\"{}\", ");
                        self.generate_expression(expr);
                        self.code.push(')');
                    },
                    // 其他情况（主要是变量）使用默认格式
                    _ => {
                        self.code.push_str("(\"{}\", ");
                        self.generate_expression(expr);
                        self.code.push(')');
                    }
                }
                
                self.code.push_str(";\n");
            },
            // 换行打印语句：println!(expr1, expr2, ...)
            // 每个参数单独一行输出
            Statement::Println(exprs) => {
                for expr in exprs {
                    self.indent();
                    self.code.push_str("println!(\"{}\", ");
                    self.generate_expression(expr);
                    self.code.push_str(");\n");
                }
            },
            // 条件语句：if condition { statements } [else { statements }]
            Statement::If(condition, then_branch, else_branch) => {
                // 生成if条件
                self.indent();
                self.code.push_str("if ");
                self.generate_expression(condition);
                self.code.push_str(" {\n");
                
                // 生成then分支
                self.increment_indent();
                for stmt in then_branch {
                    self.generate_statement(stmt);
                }
                self.decrement_indent();
                
                // 生成可选的else分支
                if let Some(else_branch) = else_branch {
                    self.indent();
                    self.code.push_str("} else {\n");
                    
                    self.increment_indent();
                    for stmt in else_branch {
                        self.generate_statement(stmt);
                    }
                    self.decrement_indent();
                }
                
                // 结束if语句
                self.indent();
                self.code.push_str("}\n");
            },
            // For循环：for 变量 in 表达式 then statements end
            Statement::For(var_name, iterable, body) => {
                // 生成for循环，将字符串转换为字符迭代器
                self.indent();
                self.code.push_str(&format!("for {} in ({}).chars() {{\n", var_name, self.expr_to_string(iterable)));
                
                // 生成循环体
                self.increment_indent();
                for stmt in body {
                    self.generate_statement(stmt);
                }
                self.decrement_indent();
                
                // 结束for循环
                self.indent();
                self.code.push_str("}\n");
            },
        }
    }
    
    // 将表达式转换为字符串表示（用于for循环等场景）
    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::BoolLiteral(b) => b.to_string(),
            Expr::Identifier(id) => id.clone(),
            _ => "unknown".to_string(), // 对于复杂表达式，简化处理
        }
    }
    
    // 生成完整的Rust程序
    // 接收语句列表，生成包含main函数的完整Rust代码
    fn generate(&mut self, statements: &[Statement]) -> String {
        // 生成main函数头部
        self.code.push_str("fn main() {\n");
        self.increment_indent();
        
        // 生成所有语句
        for stmt in statements {
            self.generate_statement(stmt);
        }
        
        // 结束main函数
        self.decrement_indent();
        self.indent();
        self.code.push_str("}");
        // 返回生成的代码
        self.code.clone()
    }
}

// 公开的代码生成函数
// 接收语句列表，返回生成的Rust代码或错误
pub fn generate(statements: Vec<Statement>) -> Result<String, String> {
    // 创建代码生成器实例
    let mut generator = CodeGenerator::new();
    // 执行代码生成
    Ok(generator.generate(&statements))
}