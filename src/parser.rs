// 导入词法分析器模块中的Token枚举，用于解析词法单元
use crate::lexer::Token;

// 表达式枚举：表示程序中所有可能的表达式类型
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
    // 数字字面量：i64类型整数
    Number(i64),
    // 字符串字面量：String类型
    String(String),
    // 布尔字面量：true或false
    BoolLiteral(bool),
    // 标识符：变量名
    Identifier(String),
    // 二元运算表达式，包含左操作数、运算符和右操作数
    // 使用Box智能指针避免递归类型导致的无限大小问题
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
    // 一元运算表达式，包含运算符和操作数
    UnaryOp(Operator, Box<Expr>),
    // 范围表达式：表示从start到end的范围
    Range(Box<Expr>, Box<Expr>),
}

// 运算符枚举：表示程序中所有支持的运算符类型
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,    // 加法运算符：+ 或 add
    Sub,    // 减法运算符：- 或 sub
    Mul,    // 乘法运算符：* 或 mul
    Div,    // 除法运算符：/ 或 div
    Eq,     // 相等运算符：= 或 eq
    Lt,     // 小于运算符：< 或 lt
    Le,     // 小于等于运算符：<= 或 le
    Gt,     // 大于运算符：> 或 gt
    Ge,     // 大于等于运算符：>= 或 ge
}

// 数据类型枚举：表示支持的数据类型
#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
}

// 语句枚举：表示程序中所有可能的语句类型
#[derive(Debug, PartialEq)]
pub enum Statement {
    // 变量声明语句（Nexa使用var），包含变量名、可选类型和初始值表达式
    Var(String, Option<DataType>, Box<Expr>),
    // 变量赋值语句，包含变量名和赋值表达式
    Assign(String, Box<Expr>),
    // 打印语句，包含要打印的表达式
    Print(Box<Expr>),
    // 换行打印语句，包含多个要打印的表达式
    Println(Vec<Expr>),
    // 条件语句，包含条件表达式、then分支语句列表和可选的else分支语句列表
    If(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>),
    // For循环语句：for 变量 in 表达式 { 语句列表 }
    For(String, Box<Expr>, Vec<Statement>),
    // While循环语句：while 条件表达式 { 语句列表 }
    While(Box<Expr>, Vec<Statement>),
    // 表达式语句：独立的表达式（如函数调用、算术运算等）
    Expression(Box<Expr>),

}

// 解析器结构体：用于将词法单元序列转换为抽象语法树
pub struct Parser {
    // 存储待解析的词法单元序列
    tokens: Vec<Token>,
    // 当前解析位置的索引，用于跟踪解析进度
    position: usize,
}

// 解析器实现块：为Parser结构体添加方法
impl Parser {
    // 创建新的解析器实例
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,      // 初始化词法单元序列
            position: 0,  // 初始化位置为0，从第一个token开始
        }
    }
    
    // 获取当前位置的词法单元
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    // 前进到下一个词法单元
    fn advance(&mut self) {
        self.position += 1;
    }
    
    // 消耗指定的词法单元
    // 如果当前token与期望的token匹配，则前进位置并返回成功
    // 如果不匹配，则返回错误信息
    fn consume(&mut self, expected: Token) -> Result<(), String> {
        if let Some(token) = self.current() {
            if token == &expected {
                // 匹配成功，前进位置
                self.advance();
                Ok(())
            } else {
                // 匹配失败，返回错误信息
                Err(format!("期望 {:?}, 但得到 {:?}", expected, token))
            }
        } else {
            // 已到达末尾，返回错误信息
            Err(format!("期望 {:?}, 但已到达文件末尾", expected))
        }
    }
    
    // 解析基本表达式（原子表达式）
    // 包括数字、字符串、布尔、标识符和括号表达式
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current().cloned() {
            // 匹配数字字面量
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expr::Number(n))
            },
            // 匹配字符串字面量
            Some(Token::StringLiteral(s)) => {
                self.advance();
                Ok(Expr::String(s))
            },
            // 匹配布尔字面量
            Some(Token::BoolLiteral(b)) => {
                self.advance();
                Ok(Expr::BoolLiteral(b))
            },
            // 匹配标识符（变量名）
            Some(Token::Identifier(id)) => {
                let name = id;
                self.advance();
                Ok(Expr::Identifier(name))
            },
            // 匹配括号表达式
            Some(Token::LParen) => {
                self.advance();
                let first_expr = self.parse_expression()?;
                
                // 检查是否是范围表达式 (start, end)
                if let Some(Token::Comma) = self.current() {
                    self.advance();
                    let second_expr = self.parse_expression()?;
                    self.consume(Token::RParen)?;
                    Ok(Expr::Range(Box::new(first_expr), Box::new(second_expr)))
                } else {
                    // 普通括号表达式
                    self.consume(Token::RParen)?;
                    Ok(first_expr)
                }
            },
            // 匹配到不期望的token
            Some(token) => {
                Err(format!("期望表达式，但得到 {:?}", token))
            },
            // 已到达末尾
            None => {
                Err("期望表达式，但已到达文件末尾".to_string())
            },
        }
    }
    
    // 将词法单元转换为运算符
    // 支持符号形式和关键字形式的运算符
    fn get_operator(&self, token: &Token) -> Option<Operator> {
        match token {
            // 加法运算符：+ 或 add
            Token::Plus | Token::Add => Some(Operator::Add),
            // 减法运算符：- 或 sub
            Token::Minus | Token::Sub => Some(Operator::Sub),
            // 乘法运算符：* 或 mul
            Token::Star | Token::Mul => Some(Operator::Mul),
            // 除法运算符：/ 或 div
            Token::Slash | Token::Div => Some(Operator::Div),
            // 相等运算符：= 或 eq
            Token::Equal | Token::Eq => Some(Operator::Eq),
            // 小于运算符：< 或 lt
            Token::LessThan | Token::Lt => Some(Operator::Lt),
            // 小于等于运算符：<= 或 le
            Token::LessThanOrEqual => Some(Operator::Le),
            // 大于运算符：> 或 gt
            Token::GreaterThan | Token::Gt => Some(Operator::Gt),
            // 大于等于运算符：>= 或 ge
            Token::GreaterThanOrEqual => Some(Operator::Ge),
            // 不是运算符
            _ => None,
        }
    }
    
    // 解析表达式
    // 调用二元表达式解析函数，初始优先级为0
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_binary_expression(0)
    }
    
    // 获取运算符的优先级
    // 数值越大优先级越高，用于正确处理运算符优先级
    fn precedence(&self, op: &Operator) -> u8 {
        match op {
            // 加减法优先级为1
            Operator::Add | Operator::Sub => 1,
            // 乘除法优先级为2，高于加减法
            Operator::Mul | Operator::Div => 2,
            // 比较运算符优先级最低，为0
            Operator::Eq | Operator::Lt | Operator::Le | Operator::Gt | Operator::Ge => 0,
        }
    }
    
    // 解析二元表达式
    // 使用递归下降算法处理运算符优先级
    fn parse_binary_expression(&mut self, precedence: u8) -> Result<Expr, String> {
        // 首先解析左操作数（基本表达式）
        let mut left = self.parse_primary()?;
        
        // 循环处理后续运算符和右操作数
        while let Some(token) = self.current() {
            if let Some(op) = self.get_operator(token) {
                let current_precedence = self.precedence(&op);
                // 如果当前运算符优先级低于要求的最小优先级，停止解析
                if current_precedence < precedence {
                    break;
                }
                
                // 消耗运算符token
                self.advance();
                // 递归解析右操作数，优先级+1确保左结合性
                let right = self.parse_binary_expression(current_precedence + 1)?;
                // 构建二元运算表达式节点
                left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
            } else {
                // 不是运算符，结束解析
                break;
            }
        }
        
        Ok(left)
    }
    
    // 解析语句
    // 根据当前token的类型选择相应的解析方法
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current() {
            // 处理表达式语句：直接解析表达式作为语句
            Some(Token::Number(_)) | Some(Token::StringLiteral(_)) | Some(Token::BoolLiteral(_)) | 
            Some(Token::LParen) | Some(Token::Plus) | Some(Token::Minus) => {
                // 解析整个表达式
                let expr = self.parse_expression()?;
                Ok(Statement::Expression(Box::new(expr)))
            },
            // 解析变量声明语句：var variable [: type] = expression
            Some(Token::Var) => {
                // 消耗var关键字
                self.advance();
                
                // 期望标识符作为变量名
                if let Some(Token::Identifier(id)) = self.current() {
                    let name = id.clone();
                    self.advance();
                    
                    // 检查是否有类型注解
                    let data_type = if let Some(Token::Colon) = self.current() {
                        self.advance(); // 消耗冒号
                        match self.current() {
                            Some(Token::Int) => {
                                self.advance();
                                Some(DataType::Int)
                            },
                            Some(Token::Float) => {
                                self.advance();
                                Some(DataType::Float)
                            },
                            Some(Token::Bool) => {
                                self.advance();
                                Some(DataType::Bool)
                            },
                            Some(Token::StringType) => {
                                self.advance();
                                Some(DataType::String)
                            },
                            _ => return Err("期望数据类型 (int, float, bool, string)".to_string())
                        }
                    } else {
                        None // 没有类型注解
                    };
                    
                    // 期望等号
                    self.consume(Token::Equal)?;
                    // 解析初始值表达式
                    let expr = self.parse_expression()?;
                    
                    Ok(Statement::Var(name, data_type, Box::new(expr)))
                } else {
                    Err("期望标识符作为变量名".to_string())
                }
            },
            // 解析标识符相关语句：可能是赋值语句或表达式语句
            Some(Token::Identifier(id)) => {
                let name = id.clone();
                self.advance();
                
                // 跳过任何换行token
                while let Some(Token::Newline) = self.current() {
                    self.advance();
                }
                
                // 检查是否是复合赋值语句 (+=, -=, *=, /= 等)
                if let Some(op_token) = self.current() {
                    // 检查是否是可能的复合赋值运算符前缀
                    if matches!(op_token, Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::LessThan | Token::GreaterThan) {
                        // 保存当前运算符
                        let op = self.get_operator(op_token).ok_or(format!("无效的运算符: {:?}", op_token))?;
                        self.advance();
                        
                        // 检查下一个token是否是等号
                        if let Some(Token::Equal) = self.current() {
                            self.advance();
                            let rhs = self.parse_expression()?;
                            
                            // 构建复合赋值的表达式: 例如 i += 1 转换为 i = i + 1
                            let lhs = Expr::Identifier(name.clone());
                            let combined_expr = Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs));
                            
                            return Ok(Statement::Assign(name, Box::new(combined_expr)));
                        } else {
                            // 不是复合赋值，回退并解析为普通二元表达式
                            // 构建二元运算表达式
                            let lhs = Expr::Identifier(name);
                            let rhs = self.parse_binary_expression(self.precedence(&op) + 1)?;
                            let expr = Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs));
                            return Ok(Statement::Expression(Box::new(expr)));
                        }
                    }
                }
                
                // 检查是否是简单赋值语句
                if let Some(Token::Equal) = self.current() {
                    self.advance();
                    let expr = self.parse_expression()?;
                    Ok(Statement::Assign(name, Box::new(expr)))
                } else {
                    // 不是赋值语句，解析为表达式语句
                    let expr = Expr::Identifier(name);
                    Ok(Statement::Expression(Box::new(expr)))
                }
            },
            // 解析打印语句：print expression 或 print(expr1, expr2, ...)
            Some(Token::Print) => {
                // 消耗print关键字
                self.advance();
                
                // 检查是否有左括号（多参数语法）
                if let Some(Token::LParen) = self.current() {
                    // 多参数语法：print(expr1, expr2, ...)
                    self.consume(Token::LParen)?;
                    
                    let mut expressions = Vec::new();
                    expressions.push(self.parse_expression()?);
                    
                    // 解析逗号分隔的额外表达式
                    while let Some(Token::Comma) = self.current() {
                        self.advance(); // 消耗逗号
                        expressions.push(self.parse_expression()?);
                    }
                    
                    self.consume(Token::RParen)?;
                    
                    // 将多个表达式用二元操作符连接成单个表达式
                    let combined_expr = if expressions.len() == 1 {
                        expressions.into_iter().next().unwrap()
                    } else {
                        // 用加法操作符连接所有表达式（实际生成时会用空格分隔）
                        expressions.into_iter().reduce(|acc, expr| {
                            Expr::BinaryOp(Box::new(acc), Operator::Add, Box::new(expr))
                        }).unwrap()
                    };
                    
                    Ok(Statement::Print(Box::new(combined_expr)))
                } else {
                    // 单参数语法：print expression
                    let expr = self.parse_expression()?;
                    Ok(Statement::Print(Box::new(expr)))
                }
            },
            // 解析换行打印语句：println expression 或 println(expr1, expr2, ...)
            Some(Token::Println) => {
                // 消耗println关键字
                self.advance();
                
                // 检查是否有左括号（多参数语法）
                if let Some(Token::LParen) = self.current() {
                    // 多参数语法：println(expr1, expr2, ...)
                    self.consume(Token::LParen)?;
                    
                    let mut expressions = Vec::new();
                    expressions.push(self.parse_expression()?);
                    
                    // 解析逗号分隔的额外表达式
                    while let Some(Token::Comma) = self.current() {
                        self.advance(); // 消耗逗号
                        expressions.push(self.parse_expression()?);
                    }
                    
                    self.consume(Token::RParen)?;
                    
                    Ok(Statement::Println(expressions))
                } else {
                    // 单参数语法：println expression
                    let expr = self.parse_expression()?;
                    Ok(Statement::Println(vec![expr]))
                }
            },
            // 解析条件语句：if condition { statements } [else { statements }] 或 if condition\n    statements（缩进风格）
            Some(Token::If) => {
                // 消耗if关键字
                self.advance();
                // 解析条件表达式
                let condition = self.parse_expression()?;
                
                // 检查代码块开始方式：只支持花括号
                let mut then_branch = Vec::new();

                // 只支持花括号语法
                if let Some(Token::LBrace) = self.current() {
                    // 消耗左花括号
                    self.advance();
                    
                    // 解析花括号内的语句
                    while let Some(token) = self.current() {
                        if let Token::RBrace = token {
                            break;
                        }
                        
                        // 跳过换行符
                        while let Some(Token::Newline) = self.current() {
                            self.advance();
                        }
                        
                        if let Some(token) = self.current() {
                            if let Token::RBrace = token {
                                break;
                            }
                            then_branch.push(self.parse_statement()?);
                            
                            // 如果下一个token是分号，消耗它（可选的分号分隔符）
                            while let Some(Token::Semicolon) = self.current() {
                                self.advance();
                            }
                        }
                    }
                    
                    // 期望右花括号结束then分支
                    self.consume(Token::RBrace)?;
                } else {
                    // 不支持其他语法，返回错误
                    return Err(format!("期望 '{{' 开始then分支，实际找到: {:?}", self.current()));
                }
                
                // 解析可选的else分支
                let else_branch = if let Some(Token::Else) = self.current() {
                    self.advance();
                    let mut else_stmts = Vec::new();
                    
                    // 检查else分支的代码块开始方式
                    match self.current() {
                        // 只支持花括号语法
                        Some(Token::LBrace) => {
                            // 消耗左花括号
                            self.advance();
                            
                            // 解析花括号内的语句
                            while let Some(token) = self.current() {
                                if let Token::RBrace = token {
                                    break;
                                }
                                
                                // 跳过换行符
                                while let Some(Token::Newline) = self.current() {
                                    self.advance();
                                }
                                
                                if let Some(token) = self.current() {
                                    if let Token::RBrace = token {
                                        break;
                                    }
                                    else_stmts.push(self.parse_statement()?);
                                    
                                    // 如果下一个token是分号，消耗它（可选的分号分隔符）
                                    while let Some(Token::Semicolon) = self.current() {
                                        self.advance();
                                    }
                                }
                            }
                            
                            // 期望右花括号结束else分支
                            self.consume(Token::RBrace)?;
                        },
                        _ => return Err(format!("期望 '{{' 开始else分支，实际找到: {:?}", self.current())),
                    }
                    
                    Some(else_stmts)
                } else {
                    None
                };
                
                Ok(Statement::If(Box::new(condition), then_branch, else_branch))
            },
            // 解析for循环：for variable in expression { statements } 或 for variable in expression\n    statements（缩进风格）
            Some(Token::For) => {
                // 消耗for关键字
                self.advance();
                
                // 期望标识符作为循环变量
                let var_name = if let Some(Token::Identifier(id)) = self.current() {
                    let name = id.clone();
                    self.advance();
                    name
                } else {
                    return Err("期望标识符作为循环变量".to_string());
                };
                
                // 期望in关键字
                self.consume(Token::In)?;
                
                // 解析要遍历的表达式（通常是字符串）
                let iterable = self.parse_expression()?;
                
                // 检查代码块开始方式：只支持花括号
                let body = match self.current() {
                    // 只支持花括号语法
                    Some(Token::LBrace) => {
                        // 消耗左花括号
                        self.advance();
                        
                        // 解析花括号内的语句
                        let mut statements = Vec::new();
                        while let Some(token) = self.current() {
                            if let Token::RBrace = token {
                                break;
                            }
                            
                            // 跳过换行符
                            while let Some(Token::Newline) = self.current() {
                                self.advance();
                            }
                            
                            if let Some(token) = self.current() {
                                if let Token::RBrace = token {
                                    break;
                                }
                                statements.push(self.parse_statement()?);
                                
                                // 如果下一个token是分号，消耗它（可选的分号分隔符）
                                while let Some(Token::Semicolon) = self.current() {
                                    self.advance();
                                }
                            }
                        }
                        
                        // 期望右花括号结束循环体
                        self.consume(Token::RBrace)?;
                        statements
                    },
                    // 不支持其他语法
                    _ => return Err("期望'{'开始循环体".to_string()),
                };
                
                Ok(Statement::For(var_name, Box::new(iterable), body))
            },
            // 解析while循环：while condition { statements } 或 while condition\n    statements（缩进风格）
            Some(Token::While) => {
                // 消耗while关键字
                self.advance();
                
                // 解析循环条件表达式
                let condition = self.parse_expression()?;
                
                // 检查代码块开始方式：只支持花括号
                let body = match self.current() {
                    // 只支持花括号语法
                    Some(Token::LBrace) => {
                        // 消耗左花括号
                        self.advance();
                        
                        // 解析花括号内的语句
                        let mut statements = Vec::new();
                        while let Some(token) = self.current() {
                            if let Token::RBrace = token {
                                break;
                            }
                            
                            // 跳过换行符
                            while let Some(Token::Newline) = self.current() {
                                self.advance();
                            }
                            
                            if let Some(token) = self.current() {
                                if let Token::RBrace = token {
                                    break;
                                }
                                statements.push(self.parse_statement()?);
                                
                                // 如果下一个token是分号，消耗它（可选的分号分隔符）
                                while let Some(Token::Semicolon) = self.current() {
                                    self.advance();
                                }
                            }
                        }
                        
                        // 期望右花括号结束循环体
                        self.consume(Token::RBrace)?;
                        statements
                    },
                    _ => return Err("期望'{'开始循环体".to_string()),
                };
                
                Ok(Statement::While(Box::new(condition), body))
            },
            // 无法识别的语句类型
            Some(token) => {
                // 如果是then或end关键字，提示这些关键字不再使用
                match token {
                    Token::Then => return Err("语法错误: 'then'关键字不再使用。请使用花括号{}或缩进风格代替。".to_string()),
                    Token::End => return Err("语法错误: 'end'关键字不再使用。请使用花括号{}或缩进风格代替。".to_string()),
                    _ => return Err(format!("无法识别的语句: {:?}", token))
                }
            },
            // 已到达文件末尾
            None => {
                Err("期望语句，但已到达文件末尾".to_string())
            }
        }
    }
    
    // 解析整个程序
    // 循环解析所有语句直到结束，支持可选的分号分隔
    fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        // 循环解析语句，直到所有token都被处理
        while self.position < self.tokens.len() {
            // 跳过换行符
            while let Some(Token::Newline) = self.current() {
                self.advance();
            }
            
            // 如果还有token，解析语句
            if self.position < self.tokens.len() {
                statements.push(self.parse_statement()?);
                
                // 如果下一个token是分号，消耗它（可选的分号分隔符）
                if let Some(Token::Semicolon) = self.current() {
                    self.advance();
                }
            }
        }

        Ok(statements)
    }
}

// 公开的解析函数
// 接收词法单元序列，返回解析后的语句列表或错误信息
pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Statement>, String> {
    // 创建解析器实例，克隆tokens以避免所有权问题
    let mut parser = Parser::new(tokens.clone());
    // 执行解析
    parser.parse()
}