// Token枚举：定义了Nexa语言中所有可能的词法单元（token）类型
// Debug: 支持调试打印  PartialEq: 支持相等比较  Clone: 支持克隆
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // ===== 关键字部分 =====
    // 变量声明关键字（Nexa只支持var）
    Var,
    // 输出打印关键字
    Print,
    // 换行打印关键字（多参数）
    Println,
    // 条件语句关键字
    If,
    Then,
    Else,
    End,
    // 循环语句关键字
    For,
    In,
    
    // ===== 数据类型关键字 =====
    // 整数类型
    Int,
    // 浮点数类型
    Float,
    // 布尔类型
    Bool,
    // 字符串类型关键字
    StringType,
    
    // ===== 运算符部分 =====
    // 加法运算符：+
    Plus,      // +
    // 减法运算符：-
    Minus,     // -
    // 乘法运算符：*
    Star,      // *
    // 除法运算符：/
    Slash,     // /
    // 赋值/相等运算符：=
    Equal,     // =
    // 小于比较运算符：<
    LessThan,  // <
    // 大于比较运算符：>
    GreaterThan, // >
    
    // ===== 关键字形式的运算符 =====
    // 加法运算（关键字形式）：add
    Add,       // add
    // 减法运算（关键字形式）：sub
    Sub,       // sub
    // 乘法运算（关键字形式）：mul
    Mul,       // mul
    // 除法运算（关键字形式）：div
    Div,       // div
    // 相等比较（关键字形式）：eq
    Eq,        // eq
    // 小于比较（关键字形式）：lt
    Lt,        // lt
    // 大于比较（关键字形式）：gt
    Gt,        // gt
    
    // ===== 标识符部分 =====
    // 用户定义的标识符（变量名、函数名等）
    // 包含字符串数据：标识符的名称
    Identifier(String),
    
    // ===== 字面量部分 =====
    // 整数字面量：64位有符号整数
    Number(i64),
    // 字符串字面量：文本内容
    StringLiteral(String),
    // 布尔字面量：true/false
    BoolLiteral(bool),
    
    // ===== 特殊符号部分 =====
    // 左括号：(
    LParen,    // (
    // 右括号：)
    RParen,    // )
    // 左花括号：{
    LBrace,    // {
    // 右花括号：}
    RBrace,    // }
    // 逗号：参数分隔符
    Comma,     // ,
    // 冒号：用于类型注解
    Colon,     // :
    // 分号：语句结束符
    Semicolon, // ;
    // 缩进开始（Python风格）
    Indent,    // 缩进
    // 缩进结束（Python风格）
    Dedent,    // 取消缩进
    // 换行符
    Newline,   // 换行
}

// Token实现块：为Token枚举添加方法
impl Token {
    // 判断当前token是否为关键字
    // 返回true如果是Var, Print, Println, If, Else, Int, Float, Bool, StringType, For, In中的任意一个
    // 注意：Then和End不再是关键字（移除then-end语法支持）
    pub fn is_keyword(&self) -> bool {
        matches!(self, Token::Var | Token::Print | Token::Println | Token::If | Token::Else | Token::Int | Token::Float | Token::Bool | Token::StringType | Token::For | Token::In)
    }
    
    // 返回true如果是BoolLiteral
    pub fn is_bool_literal(&self) -> bool {
        matches!(self, Token::BoolLiteral(_))
    }
    
    // 判断当前token是否为运算符
    // 返回true如果是Plus, Minus, Star, Slash, Equal, LessThan, GreaterThan中的任意一个
    pub fn is_operator(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Equal | Token::LessThan | Token::GreaterThan)
    }
    
    // 判断当前token是否为关键字形式的运算符
    // 返回true如果是Add, Sub, Mul, Div, Eq, Lt, Gt中的任意一个
    pub fn is_keyword_operator(&self) -> bool {
        matches!(self, Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Eq | Token::Lt | Token::Gt)
    }
}

// 测试模块：包含单元测试函数
// 仅在测试模式下编译（#[cfg(test)]）
#[cfg(test)]
mod tests {
    // 导入父模块的所有内容
    use super::*;
    
    // 测试完整的var语句词法分析
    #[test]
    fn test_tokenize_var() {
        // 定义测试输入：变量声明语句
        let input = "var x=10";
        // 调用词法分析函数
        let result = tokenize(input);
        // 打印输入和结果，用于调试
        println!("Input: {:?}, Result: {:?}", input, result);
        // 断言结果必须是Ok（成功）
        assert!(result.is_ok());
    }
    
    // 测试简单词法分析功能
    #[test]
    fn test_tokenize_simple() {
        // 测试单个关键字"var"
        let result = tokenize("var");
        println!("测试 'var': {:?}", result);
        // 断言必须成功
        assert!(result.is_ok());
        
        // 测试关键字+标识符"var x"
        let result = tokenize("var x");
        println!("测试 'var x': {:?}", result);
        // 断言必须成功
        assert!(result.is_ok());
        
        // 测试完整的变量声明语句"var x=10"
        let result = tokenize("var x=10");
        println!("测试 'var x=10': {:?}", result);
        // 断言必须成功
        assert!(result.is_ok());
    }
}

// 词法分析器主函数：将输入字符串转换为token序列
// 参数：input - 输入的源代码字符串
// 返回：成功时返回token向量，失败时返回错误字符串
// 版本：2024-12-19 更新 - 支持缩进风格语法
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // 添加调试输出 - 打印原始输入信息
    println!("DEBUG: 输入字符串: {:?}", input);
    // 打印输入字符串的长度（字符数）
    println!("DEBUG: 输入长度: {}", input.len());
    // 将输入字符串分解为字符向量，便于调试分析
    println!("DEBUG: 输入字符: {:?}", input.chars().collect::<Vec<_>>());
    
    // 创建空的token向量，用于存储分析结果
    let mut tokens = Vec::new();
    // 创建字符迭代器，支持peek操作（查看下一个字符但不消耗）
    let mut chars = input.chars().peekable();
    // 当前位置计数器，用于调试和错误报告
    let mut position = 0;
    // 缩进栈，用于跟踪缩进级别
    let mut indent_stack = vec![0]; // 基础缩进级别为0
    // 当前行缩进级别
    let mut current_indent = 0;
    // 是否在行首
    let mut at_line_start = true;
    
    // 主循环：遍历输入字符串的每个字符
    while let Some(c) = chars.peek() {
        // 打印当前位置和字符，用于调试
        println!("DEBUG: 当前位置: {}, 当前字符: {:?}", position, c);
        
        // 根据当前字符的类型进行匹配处理
        match c {
            // 空白字符匹配分支：处理空格、制表符、换行符等空白字符
            ' ' | '\t' | '\n' | '\r' => {
                // 打印调试信息，确认正在处理空白字符
                println!("DEBUG: 处理空白字符: {:?}", c);
                
                // 处理换行符
                if *c == '\n' {
                    // 生成换行token
                    tokens.push(Token::Newline);
                    println!("DEBUG: 生成换行token");
                    
                    // 重置状态
                    at_line_start = true;
                    current_indent = 0;
                } 
                // 处理回车符（Windows风格换行符的一部分）
                else if *c == '\r' {
                    // 只处理回车符，不生成token
                    println!("DEBUG: 处理回车符");
                }
                // 处理行首的空格和制表符（用于缩进）
                else if at_line_start {
                    if *c == ' ' {
                        // 每遇到一个空格，当前缩进级别加1
                        current_indent += 1;
                        println!("DEBUG: 空格缩进，当前缩进: {}", current_indent);
                    } else if *c == '\t' {
                        // 计算下一个制表位位置（向上取整到4的倍数）
                        current_indent = (current_indent + 4) / 4 * 4;
                        println!("DEBUG: 制表符缩进，当前缩进: {}", current_indent);
                    }
                }
                // 处理非行首的空格和制表符
                else {
                    println!("DEBUG: 跳过非行首空白字符: {:?}", c);
                }

                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            
            // ===== 运算符和特殊符号匹配分支 =====
            // 加法运算符匹配：遇到'+'字符
            '+' => {
                // 打印调试信息，确认匹配到加号
                println!("DEBUG: 匹配加号");
                // 创建Plus token并添加到结果向量
                tokens.push(Token::Plus);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 减法运算符匹配：遇到'-'字符
            '-' => {
                // 打印调试信息，确认匹配到减号
                println!("DEBUG: 匹配减号");
                // 创建Minus token并添加到结果向量
                tokens.push(Token::Minus);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 乘法运算符匹配：遇到'*'字符
            '*' => {
                // 打印调试信息，确认匹配到乘号
                println!("DEBUG: 匹配乘号");
                // 创建Star token并添加到结果向量
                tokens.push(Token::Star);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 除法运算符匹配：遇到'/'字符
            '/' => {
                // 打印调试信息，确认匹配到除号
                println!("DEBUG: 匹配除号");
                // 创建Slash token并添加到结果向量
                tokens.push(Token::Slash);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 赋值运算符匹配：遇到'='字符
            '=' => {
                // 打印调试信息，确认匹配到等号
                println!("DEBUG: 匹配等号");
                // 创建Equal token并添加到结果向量
                tokens.push(Token::Equal);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 小于运算符匹配：遇到'<'字符
            '<' => {
                // 打印调试信息，确认匹配到小于号
                println!("DEBUG: 匹配小于号");
                // 创建LessThan token并添加到结果向量
                tokens.push(Token::LessThan);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 大于运算符匹配：遇到'>'字符
            '>' => {
                // 打印调试信息，确认匹配到大于号
                println!("DEBUG: 匹配大于号");
                // 创建GreaterThan token并添加到结果向量
                tokens.push(Token::GreaterThan);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 左括号匹配：遇到'('字符
            '(' => {
                // 打印调试信息，确认匹配到左括号
                println!("DEBUG: 匹配左括号");
                // 创建LParen token并添加到结果向量
                tokens.push(Token::LParen);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 右括号匹配：遇到')'字符
            ')' => {
                // 打印调试信息，确认匹配到右括号
                println!("DEBUG: 匹配右括号");
                // 创建RParen token并添加到结果向量
                tokens.push(Token::RParen);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 逗号匹配：遇到','字符
            ',' => {
                // 打印调试信息，确认匹配到逗号
                println!("DEBUG: 匹配逗号");
                // 创建Comma token并添加到结果向量
                tokens.push(Token::Comma);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 冒号匹配：遇到':'字符（用于类型注解或Python风格语法）
            ':' => {
                // 首先生成冒号token
                tokens.push(Token::Colon);
                chars.next();
                position += 1;
                
                // 检查是否是Python风格语法（冒号后跟空白字符和换行符）
                // 我们需要预检查后面的字符，但不能消耗它们
                let mut temp_chars = chars.clone();
                let mut found_newline = false;
                
                // 跳过可能的空白字符
                while let Some(&next_c) = temp_chars.peek() {
                    if next_c == ' ' || next_c == '\t' {
                        temp_chars.next();
                    } else if next_c == '\n' {
                        // 找到换行符，这是Python风格语法
                        found_newline = true;
                        break;
                    } else {
                        // 遇到非空白非换行字符，不是Python风格语法
                        break;
                    }
                }
                
                if found_newline {
                    println!("DEBUG: 匹配Python风格冒号语法（冒号后有换行）");
                    // Python风格冒号语法，我们需要生成换行token
                    // 但我们需要先消耗掉冒号后的空白字符和换行符
                    
                    // 消耗冒号后的空白字符
                    while let Some(&next_c) = chars.peek() {
                        if next_c == ' ' || next_c == '\t' {
                            chars.next();
                            position += 1;
                        } else {
                            break;
                        }
                    }
                    
                    // 消耗换行符
                    if let Some('\n') = chars.peek() {
                        chars.next();
                        position += 1;
                    }
                    
                    // 生成换行token
                    tokens.push(Token::Newline);
                    
                    // 重置状态
                    at_line_start = true;
                    current_indent = 0;
                    
                    println!("DEBUG: 生成换行token（Python风格冒号后）");
                } else {
                    println!("DEBUG: 匹配普通冒号（类型注解或同一行代码块）");
                }
            }
            // 分号匹配：遇到';'字符
            ';' => {
                // 打印调试信息，确认匹配到分号
                println!("DEBUG: 匹配分号");
                // 创建Semicolon token并添加到结果向量
                tokens.push(Token::Semicolon);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 左花括号匹配：遇到'{'字符
            '{' => {
                // 打印调试信息，确认匹配到左花括号
                println!("DEBUG: 匹配左花括号");
                // 创建LBrace token并添加到结果向量
                tokens.push(Token::LBrace);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 右花括号匹配：遇到'}'字符
            '}' => {
                // 打印调试信息，确认匹配到右花括号
                println!("DEBUG: 匹配右花括号");
                // 创建RBrace token并添加到结果向量
                tokens.push(Token::RBrace);
                // 消耗当前字符（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
            }
            // 字符串字面量匹配：遇到'"'字符（双引号）
            '"' => {
                // 打印调试信息，确认开始匹配字符串
                println!("DEBUG: 匹配字符串字面量");
                // 消耗开头的双引号（移动到下一个字符）
                chars.next();
                // 位置计数器加1
                position += 1;
                // 创建空字符串用于存储字符串内容
                let mut content = String::new();
                
                // 循环读取字符串内容，直到遇到结束的双引号
                while let Some(c) = chars.peek() {
                    // 检查是否遇到结束的双引号
                    if *c == '"' {
                        // 消耗结束的双引号
                        chars.next();
                        // 位置计数器加1
                        position += 1;
                        // 跳出内层循环，字符串解析完成
                        break;
                    } else {
                        // 将当前字符添加到字符串内容中
                        content.push(*c);
                        // 消耗当前字符（移动到下一个字符）
                        chars.next();
                        // 位置计数器加1
                        position += 1;
                    }
                }
                
                // 将字符串token添加到结果列表
                tokens.push(Token::StringLiteral(content.clone()));
                // 打印调试信息，显示解析到的字符串内容
                println!("DEBUG: 字符串内容: {:?}", content);
            }
            
            // 数字字面量匹配：遇到数字字符（0-9）
            '0'..='9' => {
                // 打印调试信息，确认开始匹配数字
                println!("DEBUG: 匹配数字");
                // 创建空字符串用于存储数字文本
                let mut num_str = String::new();
                
                // 循环读取连续的数字字符
                while let Some(c) = chars.peek() {
                    // 检查当前字符是否为ASCII数字
                    if c.is_ascii_digit() {
                        // 将数字字符添加到字符串中
                        num_str.push(*c);
                        // 消耗当前字符（移动到下一个字符）
                        chars.next();
                        // 位置计数器加1
                        position += 1;
                    } else {
                        // 遇到非数字字符，结束数字解析
                        break;
                    }
                }
                
                // 检查是否遇到小数点，支持浮点数
                if let Some(&'.') = chars.peek() {
                    // 消耗小数点
                    chars.next();
                    position += 1;
                    num_str.push('.');
                    
                    // 读取小数部分
                    while let Some(c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num_str.push(*c);
                            chars.next();
                            position += 1;
                        } else {
                            break;
                        }
                    }
                    
                    // 尝试解析为f64浮点数
                    match num_str.parse::<f64>() {
                        Ok(num) => {
                            // 将浮点数转换为i64（简化处理，后续可扩展Token支持浮点类型）
                            tokens.push(Token::Number(num as i64));
                            println!("DEBUG: 浮点数值: {}", num);
                        },
                        Err(_) => {
                            return Err(format!("无效的浮点数: {}", num_str));
                        }
                    }
                } else {
                    // 普通整数解析
                    match num_str.parse::<i64>() {
                        Ok(num) => {
                            // 解析成功，创建Number token并添加到结果向量
                            tokens.push(Token::Number(num));
                            // 打印调试信息，显示解析到的数字值
                            println!("DEBUG: 数字值: {}", num);
                        },
                        Err(_) => {
                            // 解析失败，返回错误信息
                            return Err(format!("无效的数字: {}", num_str));
                        }
                    }
                }
            }
            
            // 标识符或关键字匹配：遇到字母或下划线
            'a'..='z' | 'A'..='Z' | '_' => {
                // 打印调试信息，确认开始匹配标识符或关键字
                println!("DEBUG: 匹配标识符/关键字");
                // 创建空字符串用于存储标识符/关键字文本
                let mut ident = String::new();
                
                // 循环读取连续的字母、数字或下划线字符
                while let Some(c) = chars.peek() {
                    // 检查是否为字母、数字或下划线（标识符的有效字符）
                    if c.is_ascii_alphanumeric() || *c == '_' {
                        // 将字符添加到标识符字符串中
                        ident.push(*c);
                        // 消耗当前字符（移动到下一个字符）
                        chars.next();
                        // 位置计数器加1
                        position += 1;
                    } else {
                        // 遇到无效字符，结束标识符解析
                        break;
                    }
                }
                
                // 检查解析到的文本是否为Nexa语言的关键字
                // 使用match表达式进行字符串匹配
                let token = match ident.as_str() {
                    // 变量声明关键字（Nexa只支持var）
                    "var" => {
                        // 打印调试信息，确认匹配到var关键字
                        println!("DEBUG: 匹配关键字 'var'");
                        // 返回Var token
                        Token::Var
                    },
                    // 输出打印关键字
                    "print" => {
                        // 打印调试信息，确认匹配到print关键字
                        println!("DEBUG: 匹配关键字 'print'");
                        // 返回Print token
                        Token::Print
                    },
                    // 换行打印关键字
                    "println" => {
                        // 打印调试信息，确认匹配到println关键字
                        println!("DEBUG: 匹配关键字 'println'");
                        // 返回Println token
                        Token::Println
                    },
                    // 条件语句关键字
                    "if" => {
                        // 打印调试信息，确认匹配到if关键字
                        println!("DEBUG: 匹配关键字 'if'");
                        // 返回If token
                        Token::If
                    },
                    // then关键字
                    "then" => {
                        // 打印调试信息，确认匹配到then关键字
                        println!("DEBUG: 匹配关键字 'then'");
                        // 返回Then token
                        Token::Then
                    },
                    // else关键字
                    "else" => {
                        // 打印调试信息，确认匹配到else关键字
                        println!("DEBUG: 匹配关键字 'else'");
                        // 返回Else token
                        Token::Else
                    },
                    // end关键字
                    "end" => {
                        // 打印调试信息，确认匹配到end关键字
                        println!("DEBUG: 匹配关键字 'end'");
                        // 返回End token
                        Token::End
                    },
                    // 加法运算关键字
                    "add" => {
                        // 打印调试信息，确认匹配到add关键字
                        println!("DEBUG: 匹配关键字 'add'");
                        // 返回Add token
                        Token::Add
                    },
                    // 减法运算关键字
                    "sub" => {
                        // 打印调试信息，确认匹配到sub关键字
                        println!("DEBUG: 匹配关键字 'sub'");
                        // 返回Sub token
                        Token::Sub
                    },
                    // 乘法运算关键字
                    "mul" => {
                        // 打印调试信息，确认匹配到mul关键字
                        println!("DEBUG: 匹配关键字 'mul'");
                        // 返回Mul token
                        Token::Mul
                    },
                    // 除法运算关键字
                    "div" => {
                        // 打印调试信息，确认匹配到div关键字
                        println!("DEBUG: 匹配关键字 'div'");
                        // 返回Div token
                        Token::Div
                    },
                    // 相等比较关键字
                    "eq" => {
                        // 打印调试信息，确认匹配到eq关键字
                        println!("DEBUG: 匹配关键字 'eq'");
                        // 返回Eq token
                        Token::Eq
                    },
                    // 小于比较关键字
                    "lt" => {
                        // 打印调试信息，确认匹配到lt关键字
                        println!("DEBUG: 匹配关键字 'lt'");
                        // 返回Lt token
                        Token::Lt
                    },
                    // 大于比较关键字
                    "gt" => {
                        // 打印调试信息，确认匹配到gt关键字
                        println!("DEBUG: 匹配关键字 'gt'");
                        // 返回Gt token
                        Token::Gt
                    },
                    // 整数类型关键字
                    "int" => {
                        // 打印调试信息，确认匹配到int关键字
                        println!("DEBUG: 匹配关键字 'int'");
                        // 返回Int token
                        Token::Int
                    },
                    // 浮点数类型关键字
                    "float" => {
                        // 打印调试信息，确认匹配到float关键字
                        println!("DEBUG: 匹配关键字 'float'");
                        // 返回Float token
                        Token::Float
                    },
                    // 布尔类型关键字
                    "bool" => {
                        // 打印调试信息，确认匹配到bool关键字
                        println!("DEBUG: 匹配关键字 'bool'");
                        // 返回Bool token
                        Token::Bool
                    },
                    // 布尔字面量 true
                    "true" => {
                        // 打印调试信息，确认匹配到true字面量
                        println!("DEBUG: 匹配布尔字面量 'true'");
                        // 返回BoolLiteral token，值为true
                        Token::BoolLiteral(true)
                    },
                    // 布尔字面量 false
                    "false" => {
                        // 打印调试信息，确认匹配到false字面量
                        println!("DEBUG: 匹配布尔字面量 'false'");
                        // 返回BoolLiteral token，值为false
                        Token::BoolLiteral(false)
                    },
                    // 字符串类型关键字
                    "string" => {
                        // 打印调试信息，确认匹配到string关键字
                        println!("DEBUG: 匹配关键字 'string'");
                        // 返回StringType token
                        Token::StringType
                    },
                    // for循环关键字
                    "for" => {
                        // 打印调试信息，确认匹配到for关键字
                        println!("DEBUG: 匹配关键字 'for'");
                        // 返回For token
                        Token::For
                    },
                    // in关键字（用于for循环）
                    "in" => {
                        // 打印调试信息，确认匹配到in关键字
                        println!("DEBUG: 匹配关键字 'in'");
                        // 返回In token
                        Token::In
                    },
                    // 默认情况：不是关键字，作为标识符处理
                    _ => {
                        // 打印调试信息，确认匹配到用户定义的标识符
                        println!("DEBUG: 匹配标识符: {:?}", ident);
                        // 创建Identifier token，包含标识符名称
                        Token::Identifier(ident)
                    },
                };
                
                // 将创建好的token添加到结果向量中
                tokens.push(token);
                
                // 如果在行首，处理缩进逻辑
                if at_line_start {
                    // 获取当前的基础缩进级别
                    let base_indent = *indent_stack.last().unwrap_or(&0);
                    
                    // 根据缩进级别生成缩进token
                    if current_indent > base_indent {
                        // 增加缩进
                        indent_stack.push(current_indent);
                        tokens.push(Token::Indent);
                        println!("DEBUG: 生成缩进token，级别: {}", current_indent);
                    } else if current_indent < base_indent {
                        // 减少缩进 - 可能需要生成多个dedent
                        while current_indent < *indent_stack.last().unwrap_or(&0) {
                            indent_stack.pop();
                            tokens.push(Token::Dedent);
                            println!("DEBUG: 生成取消缩进token，级别: {}", current_indent);
                        }
                        
                        // 检查缩进是否对齐
                        if current_indent != *indent_stack.last().unwrap_or(&0) {
                            return Err(format!("缩进错误: 期望缩进级别 {}, 但实际是 {}", 
                                             indent_stack.last().unwrap_or(&0), current_indent));
                        }
                    }
                    
                    // 标记不再在行首
                    at_line_start = false;
                }
            }
            
            // 无法识别的字符处理分支
            // 当遇到不属于Nexa语言字符集的字符时执行
            _ => {
                // 打印调试信息，显示遇到的未知字符
                println!("DEBUG: 无法识别的字符: {:?}", c);
                // 返回错误信息，包含字符内容和位置信息
                return Err(format!("[UPDATED LEXER v2] 无法识别的字符: '{}' 在位置 {}", c, position));
            }
        }
    }
    
    // 打印词法分析完成信息，显示生成的token数量
    println!("DEBUG: 词法分析完成，共生成 {} 个token", tokens.len());
    // 打印完整的token列表，用于调试分析
    println!("DEBUG: Token列表: {:?}", tokens);
    
    // 返回成功结果，包含所有解析到的token
    Ok(tokens)
}