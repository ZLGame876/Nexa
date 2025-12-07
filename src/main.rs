// 导入colored库用于终端彩色输出
use colored::*;
// 导入标准库中的IO模块，用于读写操作
use std::io::{self, Write};
// 导入标准库中的文件系统模块
use std::fs;
// 导入标准库中的Command模块，用于执行外部命令
use std::process::Command;
// 导入tokio运行时
use tokio;

// 声明词法分析器模块
mod lexer;
// 声明语法分析器模块
mod parser;
// 声明代码生成器模块
mod codegen;
// 声明Web编辑器模块
mod web_editor;

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

// 测试println函数修复的函数
// 验证println函数的两种语法（带括号和不带括号）是否都能正确解析
fn test_println_fix() {
    // 打印测试标题，空行用于分隔输出
    println!("\n=== println函数修复测试 ===");
    
    // 测试用例1: 带括号的println语法
    let test1 = "var i=1; println(i)";
    println!("\n测试用例1: {}", test1);
    
    match lexer::tokenize(test1) {
        Ok(tokens) => {
            println!("词法分析: {:?}", tokens);
            
            match parser::parse(&tokens) {
                Ok(ast) => {
                    println!("语法分析: {:?}", ast);
                    
                    match codegen::generate_code(&ast) {
                        Ok(code) => {
                            println!("代码生成:");
                            println!("{}", code);
                            
                            // 执行生成的代码
                            match execute_rust_code(&code) {
                                Ok(result) => println!("执行结果: {}", result),
                                Err(e) => println!("执行错误: {}", e)
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
    
    // 测试用例2: 不带括号的println语法
    let test2 = "var i=1; println i";
    println!("\n测试用例2: {}", test2);
    
    match lexer::tokenize(test2) {
        Ok(tokens) => {
            println!("词法分析: {:?}", tokens);
            
            match parser::parse(&tokens) {
                Ok(ast) => {
                    println!("语法分析: {:?}", ast);
                    
                    match codegen::generate_code(&ast) {
                        Ok(code) => {
                            println!("代码生成:");
                            println!("{}", code);
                            
                            // 执行生成的代码
                            match execute_rust_code(&code) {
                                Ok(result) => println!("执行结果: {}", result),
                                Err(e) => println!("执行错误: {}", e)
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

// 测试while循环修复的函数
// 验证修复后的while循环语法是否能正确解析
fn test_while_loop_fix() {
    // 打印测试标题，空行用于分隔输出
    println!("\n=== While循环修复测试 ===");
    // 定义包含修复后语法的测试输入
    let test_input = "var x = 0\nwhile x < 5 :\n    print x\n    x = x + 1";
    println!("测试输入:\n{}", test_input);
    
    // 调用词法分析器进行测试
    match lexer::tokenize(test_input) {
        Ok(tokens) => {
            // 词法分析成功
            println!("词法分析成功: {:?}", tokens);
            // 调用语法分析器
            match parser::parse(&tokens) {
                Ok(ast) => {
                    // 语法分析成功
                    println!("语法分析成功: {:?}", ast);
                    // 调用代码生成器
                    match codegen::generate_code(&ast) {
                        Ok(code) => {
                            // 代码生成成功
                            println!("代码生成成功:");
                            println!("{}", code);
                            // 执行生成的代码
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

// 测试for循环修复的函数
// 验证修复后的for循环语法是否能正确解析
fn test_for_loop_fix() {
    // 打印测试标题，空行用于分隔输出
    println!("\n=== For循环修复测试 ===");
    // 定义包含修复后语法的测试输入
    let test_input = "for i in 0 to 5 :\n    print i";
    println!("测试输入:\n{}", test_input);
    
    // 调用词法分析器进行测试
    match lexer::tokenize(test_input) {
        Ok(tokens) => {
            // 词法分析成功
            println!("词法分析成功: {:?}", tokens);
            // 调用语法分析器
            match parser::parse(&tokens) {
                Ok(ast) => {
                    // 语法分析成功
                    println!("语法分析成功: {:?}", ast);
                    // 调用代码生成器
                    match codegen::generate_code(&ast) {
                        Ok(code) => {
                            // 代码生成成功
                            println!("代码生成成功:");
                            println!("{}", code);
                            // 执行生成的代码
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

// 运行Nexa文件的函数
// 接收文件路径作为参数，执行文件中的Nexa代码
fn run_file(file_path: &str) {
    // 读取文件内容
    match fs::read_to_string(file_path) {
        Ok(content) => {
            // 打印读取的文件内容
            println!("读取文件内容:\n{}", content);
            
            // 调用词法分析器进行测试
            match lexer::tokenize(&content) {
                Ok(tokens) => {
                    // 词法分析成功
                    println!("词法分析成功: {:?}", tokens);
                    // 调用语法分析器
                    match parser::parse(&tokens) {
                        Ok(ast) => {
                            // 语法分析成功
                            println!("语法分析成功: {:?}", ast);
                            // 调用代码生成器
                            match codegen::generate_code(&ast) {
                                Ok(code) => {
                                    // 代码生成成功
                                    println!("代码生成成功:");
                                    println!("{}", code);
                                    // 执行生成的代码
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
        },
        Err(e) => {
            println!("读取文件错误: {}", e);
        }
    }
}

// 主函数
// 程序的入口点
#[tokio::main]
async fn main() {
    // 打印欢迎信息
    println!("{}", "欢迎使用Nexa语言解释器 v0.1".bold().cyan());
    println!("{}", "类型 'help' 查看可用命令，'exit' 退出程序".italic());
    
    // 主循环，持续接收用户输入
    loop {
        // 打印命令提示符
        print!("{}", "> ".bold().green());
        // 刷新标准输出，确保提示符立即显示
        io::stdout().flush().unwrap();
        
        // 读取用户输入
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // 去除输入字符串末尾的换行符
                let input = input.trim_end();
                // 匹配用户输入的命令
                match input {
                    // 退出命令
                    "exit" => {
                        println!("{}", "感谢使用Nexa语言解释器！".bold().cyan());
                        break;
                    },
                    // 帮助命令
                    "help" => {
                        println!("{}", "可用命令:".bold().cyan());
                        println!("  help     - 显示此帮助信息");
                        println!("  exit     - 退出程序");
                        println!("  test     - 运行内置测试");
                        println!("  run <文件路径> - 运行指定的Nexa文件");
                        println!("  web      - 启动Web编辑器");
                        println!("  lexer    - 测试词法分析器");
                        println!("  while    - 测试while循环修复");
                        println!("  for      - 测试for循环修复");
                        println!("  println  - 测试println函数修复");
                    },
                    // 测试命令
                    "test" => {
                        // 调用所有测试函数
                        test_lexer_direct();
                        test_while_loop_fix();
                        test_for_loop_fix();
                        test_explicit_types();
                        test_lexer();
                        test_println_fix();
                    },
                    // 词法分析器测试命令
                    "lexer" => {
                        test_lexer_direct();
                    },
                    // while循环修复测试命令
                    "while" => {
                        test_while_loop_fix();
                    },
                    // for循环修复测试命令
                    "for" => {
                        test_for_loop_fix();
                    },
                    // println函数修复测试命令
                    "println" => {
                        test_println_fix();
                    },
                    // Web编辑器命令
                    "web" => {
                        println!("启动Web编辑器...");
                        // 调用Web编辑器模块的run函数，并处理结果
                        match web_editor::run().await {
                            Ok(_) => println!("Web编辑器已关闭"),
                            Err(e) => println!("{} {}", "Web编辑器启动错误:".red(), e),
                        }
                    },
                    // 运行文件命令
                    cmd if cmd.starts_with("run ") => {
                        // 提取文件路径
                        let file_path = &cmd[4..];
                        run_file(file_path);
                    },
                    // 空输入
                    "" => continue,
                    // 其他输入
                    _ => {
                        println!("{}", "未知命令，请输入 'help' 查看可用命令".red());
                    }
                }
            },
            Err(e) => {
                println!("{}", format!("读取输入错误: {}", e).red());
            }
        }
    }
}

// 执行Rust代码的函数
// 将生成的Rust代码写入临时文件并执行
fn execute_rust_code(code: &str) -> Result<String, String> {
    // 创建临时文件
    let temp_file = std::env::temp_dir().join("temp.rs");
    // 写入代码到临时文件
    match fs::write(&temp_file, code) {
        Ok(_) => {},
        Err(e) => {
            return Err(format!("写入临时文件错误: {}", e));
        }
    }
    
    // 编译临时文件
    let output = Command::new("rustc")
        .arg(&temp_file)
        .arg("-o")
        .arg(temp_file.with_extension(""))
        .output()
        .map_err(|e| format!("编译命令执行错误: {}", e))?;
    
    // 检查编译是否成功
    if !output.status.success() {
        // 编译失败，返回错误信息
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("编译错误: {}", stderr));
    }
    
    // 执行编译后的程序
    let output = Command::new(temp_file.with_extension(""))
        .output()
        .map_err(|e| format!("执行命令错误: {}", e))?;
    
    // 检查执行是否成功
    if output.status.success() {
        // 执行成功，返回标准输出
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        // 执行失败，返回错误信息
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("执行错误: {}", stderr))
    }
}

// 测试显式类型定义的函数
// 验证显式类型定义是否能正确解析和生成代码
fn test_explicit_types() {
    // 打印测试标题，空行用于分隔输出
    println!("\n=== 显式类型定义测试 ===");
    // 定义多种类型的测试输入
    let test_inputs = [
        "var x: int = 10",            // 整数类型
        "var y: Float = 3.14",        // 浮点数类型
        "var z: bool = true",         // 布尔类型
        "var s: String = \"hello\"",    // 字符串类型
        "var a: Array = [1, 2, 3]",    // 数组类型
        "var x: int = 10; print x",   // 变量声明+打印语句
    ];
    
    // 遍历所有测试输入
    for input in &test_inputs {
        println!("测试输入: {}", input);
        
        // 调用词法分析器进行测试
        match lexer::tokenize(input) {
            Ok(tokens) => {
                // 词法分析成功
                println!("词法分析成功: {:?}", tokens);
                // 调用语法分析器
                match parser::parse(&tokens) {
                    Ok(ast) => {
                        // 语法分析成功
                        println!("语法分析成功: {:?}", ast);
                        // 调用代码生成器
                        match codegen::generate_code(&ast) {
                            Ok(code) => {
                                // 代码生成成功
                                println!("代码生成成功:");
                                println!("{}", code);
                                // 执行生成的代码
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