// 导入colored库用于终端彩色输出
use colored::*;
// 导入标准库中的IO模块，用于读写操作
use std::io::{self, Write};
// 导入临时文件和进程执行相关库
use std::fs;
use std::process::Command;

// 声明词法分析器模块
mod lexer;
// 声明语法分析器模块
mod parser;
// 声明代码生成器模块
mod codegen;

// 直接测试词法分析器的函数
// 用于快速验证词法分析器的基本功能
fn test_lexer_direct() {
    // 打印测试标题，空行用于分隔输出
    println!("\n=== 直接词法分析器测试 ===");
    // 打印测试时间标识，用于确认代码版本
    println!("测试时间: {}", "2024-12-19 更新版本");
    
    // 测试基本的token识别
    let test_input = "var x = 10";
    println!("测试输入: {}", test_input);
    
    match lexer::tokenize(test_input) {
        Ok(tokens) => {
            println!("词法分析成功!");
            for (i, token) in tokens.iter().enumerate() {
                println!("Token[{}]: {:?}", i, token);
            }
        }
        Err(e) => {
            println!("❌ 词法分析错误: {}", e);
        }
    }
    
    // 测试冒号识别（类型注解）
    println!("\n=== 冒号识别测试 ===");
    let colon_test = "var x: int = 10";
    println!("测试输入: {}", colon_test);
    
    match lexer::tokenize(colon_test) {
        Ok(tokens) => {
            println!("词法分析成功!");
            for (i, token) in tokens.iter().enumerate() {
                println!("Token[{}]: {:?}", i, token);
            }
        }
        Err(e) => {
            println!("❌ 词法分析错误: {}", e);
        }
    }
    
    // 测试字符分解
    // 将输入字符串分解为单个字符，用于调试分析
    let test_input = "var x=10";
    // 使用chars()方法获取字符迭代器，collect转换为Vec向量
    println!("测试输入分解: {:?}", test_input.chars().collect::<Vec<_>>());
    
    // 测试用户提供的用例
    println!("\n=== 用户测试用例 ===");
    let user_test = "for i in \"hello\" :\n    println(i)";
    println!("测试输入: {}", user_test);
    
    match lexer::tokenize(user_test) {
        Ok(tokens) => {
            println!("词法分析成功!");
            for (i, token) in tokens.iter().enumerate() {
                println!("Token[{}]: {:?}", i, token);
            }
        }
        Err(e) => {
            println!("❌ 词法分析错误: {}", e);
        }
    }
}

// 主函数 - 程序的入口点
fn main() {
    // 打印程序启动标题
    println!("=== NEXA LANG 启动 ===");
    // 打印版本信息，用于确认运行的代码版本
    println!("版本: 2024-12-19 测试版");
    
    // 立即测试最简单的词法分析
    // 这是最关键的快速测试，验证基本功能
    println!("\n=== 紧急测试 ===");
    // 调用词法分析器分析"var"关键字
    let simple_test = lexer::tokenize("var");
    // 打印测试结果
    println!("'var' 测试结果: {:?}", simple_test);
    
    // 先直接测试词法分析器
    // 调用详细测试函数
    test_lexer_direct();
    
    // 再测试完整的词法分析器
    // 调用标准测试函数
    println!("\n=== 词法分析器测试 ===");
    test_lexer();
    
    // 测试显式类型定义功能
    test_explicit_types();
    
    // 打印程序标题，使用绿色显示
    println!("🌟 {} v0.1.0 🌟", "Nexa Programming Language".green());
    // 打印分隔线，使用青色显示
    println!("{}", "=====================================".cyan());
    // 打印REPL模式说明，使用蓝色显示
    println!("🔄 {}", "REPL模式 - 输入代码按回车执行".blue());
    // 打印退出指令说明
    println!("输入 'exit' 退出");
    
    // 主循环 - REPL的核心
    loop {
        // 打印提示符，使用黄色显示
        print!("{}", "nexa>".yellow());
        // 刷新标准输出，确保提示符立即显示
        io::stdout().flush().unwrap();
        
        // 创建字符串变量用于存储用户输入
        let mut input = String::new();
        // 从标准输入读取一行，unwrap()处理可能的错误
        io::stdin().read_line(&mut input).unwrap();
        
        // 去除输入字符串两端的空白字符（包括换行符）
        let input = input.trim();
        
        // 检查是否需要多行输入（以冒号结尾，类似Python）
        let full_input = if input.ends_with(':') || (input.contains("for ") && !input.contains('{')) || (input.contains("if ") && !input.contains('{')) {
            let mut full_input = input.to_string();
            loop {
                print!("{}", "... ".yellow());
                io::stdout().flush().unwrap();
                
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                
                // 保留所有空白字符，仅在处理空行时进行特殊处理
                if line.trim_end().is_empty() {
                    // 如果是空行，且已经有内容，则添加换行符后终止
                    if !full_input.is_empty() {
                        full_input.push('\n');
                    }
                    break;
                }
                
                // 直接添加原始行内容（保留缩进）
                full_input.push_str(&line);
            }
            full_input
        } else {
            input.to_string()
        };
        
        // 使用完整的输入
        let input = full_input.as_str();
        // 检查用户是否输入了退出命令
        if input == "exit" {
            // 打印告别信息，使用紫色显示
            println!("{}", "👋 再见！".purple());
            // 跳出循环，结束程序
            break;
        }
        
        // 如果输入为空（用户只按了回车），跳过本次循环
        if input.is_empty() {
            continue;
        }
        
        // 添加调试信息 - 用于分析输入内容
        println!("=== 调试信息 ===");
        // 打印原始输入字符串（带引号，便于查看边界）
        println!("输入字符串: {:?}", input);
        // 打印输入字符串的长度（字符数）
        println!("输入长度: {}", input.len());
        // 将输入字符串分解为单个字符的向量，便于分析
        println!("输入字符: {:?}", input.chars().collect::<Vec<_>>());
        
        // 词法分析 - 将输入字符串转换为token序列
        match lexer::tokenize(input) {
            Ok(tokens) => {
                // 词法分析成功，打印结果，使用笔记本emoji
                println!("📝 词法分析结果: {:?}", tokens);
                
                // 语法分析 - 将token序列转换为抽象语法树(AST)
                match parser::parse(tokens) {
                    Ok(ast) => {
                        // 语法分析成功，打印AST，使用图表emoji
                        println!("📊 语法分析结果: {:?}", ast);
                        
                        // 代码生成 - 将AST转换为目标代码
                        match codegen::generate(ast) {
                            Ok(output) => {
                                // 代码生成成功，打印结果，使用火箭emoji
                                println!("🚀 代码生成结果:");
                                // 使用绿色显示生成的代码
                                println!("{}", output.green());
                                
                                // 执行生成的Rust代码
                                match execute_rust_code(&output) {
                                    Ok(result) => {
                                        println!("✅ 执行结果:");
                                        println!("--- 开始 ---");
                                        print!("{}", result);
                                        println!("--- 结束 ---");
                                        println!("结果长度: {}", result.len());
                                        println!("包含换行符: {}", result.contains('\n'));
                                    },
                                    Err(e) => {
                                        println!("❌ 执行错误: {}", e.red());
                                    }
                                }
                            },
                            Err(e) => {
                                // 代码生成失败，打印错误信息，使用红色显示
                                println!("❌ 代码生成错误: {}", e.red());
                            }
                        }
                    },
                    Err(e) => {
                        // 语法分析失败，打印错误信息，使用红色显示
                        println!("❌ 语法分析错误: {}", e.red());
                    }
                }
            },
            Err(e) => {
                // 词法分析失败，打印错误信息，使用红色显示
                println!("❌ 词法分析错误: {}", e.red());
                // 标记调试信息结束
                println!("=== 调试信息结束 ===");
            }
        }
    }
}

// 执行生成的Rust代码
fn execute_rust_code(code: &str) -> Result<String, String> {
    // 创建临时文件存储Rust代码
    let temp_file = "/tmp/nexa_temp.rs";
    
    // 写入代码到临时文件
    if let Err(e) = fs::write(temp_file, code) {
        return Err(format!("无法写入临时文件: {}", e));
    }
    
    // 使用rustc编译并运行
    let output = Command::new("rustc")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/nexa_temp")
        .output()
        .map_err(|e| format!("编译失败: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("编译错误:\nSTDERR: {}\nSTDOUT: {}\n生成的代码:\n{}", error, stdout, code));
    }
    
    // 运行编译后的程序
    let run_output = Command::new("/tmp/nexa_temp")
        .output()
        .map_err(|e| format!("运行失败: {}", e))?;
    
    if !run_output.status.success() {
        let error = String::from_utf8_lossy(&run_output.stderr);
        return Err(format!("运行错误: {}", error));
    }
    
    // 获取标准输出并转换为字符串
    let stdout_str = String::from_utf8_lossy(&run_output.stdout).to_string();
    
    // 调试信息：检查原始输出
    eprintln!("调试 - 原始输出长度: {}", stdout_str.len());
    eprintln!("调试 - 原始输出字节: {:?}", stdout_str.as_bytes());
    
    // 返回标准输出
    Ok(stdout_str)
}

// 测试显式类型定义功能
fn test_explicit_types() {
    println!("\n=== 显式类型定义测试 ===");
    
    let test_cases = [
        "var y: String = \"ww\"",
        "var z: Float = 42",
        "var x: Int = 10",
        "var b: Bool = true",
    ];
    
    for input in &test_cases {
        println!("\n测试输入: {}", input);
        
        match lexer::tokenize(input) {
            Ok(tokens) => {
                println!("词法分析: {:?}", tokens);
                
                match parser::parse(tokens) {
                    Ok(ast) => {
                        println!("语法分析: {:?}", ast);
                        
                        match codegen::generate(ast) {
                            Ok(code) => {
                                println!("生成的代码:");
                                println!("{}", code);
                                
                                // 尝试编译生成的代码
                                match execute_rust_code(&code) {
                                    Ok(result) => {
                                        println!("执行结果: {}", result);
                                    },
                                    Err(e) => {
                                        println!("执行错误: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                println!("代码生成错误: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("语法分析错误: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("词法分析错误: {}", e);
            }
        }
    }
}

// 测试词法分析器的函数
// 包含多种测试用例，验证词法分析器的各项功能
fn test_lexer() {
    // 定义测试输入数组，包含各种Nexa语言语句
    let test_inputs = [
        "var",           // 单独的关键字
        "var x",         // 关键字+标识符
        "var x=10",      // 变量声明（无空格）
        "var x = 10",    // 变量声明（有空格）
        "print 5",       // 打印语句
        "print (2 + 3)", // 打印表达式
        "print (10 + 5 * 2)", // 复杂表达式
        "var x = 10; var y = 20; print (x + y)", // 多语句测试
        "println(x, y, x + y)", // 多参数println测试
        "var y: String = \"ww\"", // 显式类型定义 - 字符串字面量赋值给String类型
        "var z: Float = 42",     // 显式类型定义 - 整数赋值给Float类型
    ];
    
    // 遍历所有测试输入
    for input in &test_inputs {
        // 打印空行分隔不同测试
        println!("\n测试输入: {:?}", input);
        // 调用词法分析器进行测试
        match lexer::tokenize(input) {
            Ok(tokens) => {
                // 测试成功，打印成功标志和结果
                println!("✅ 成功: {:?}", tokens);
            },
            Err(e) => {
                // 测试失败，打印错误标志和错误信息
                println!("❌ 错误: {}", e);
            }
        }
    }
}