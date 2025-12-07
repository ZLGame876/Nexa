// 直接测试Nexa语言的while循环冒号语法修复
use nexa_lang::lexer::tokenize;
use nexa_lang::parser::parse;
use nexa_lang::codegen::generate;

fn main() {
    println!("=== 直接测试Nexa while循环冒号语法 ===");
    
    // 测试用例1: 同一行的while循环冒号语法
    let test1 = "var i = 1; while i < 5: println(i); i += 1;";
    println!("\n测试1 - 同一行while循环:");
    println!("输入: {}", test1);
    
    // 执行词法分析
    match tokenize(test1) {
        Ok(tokens) => {
            println!("✅ 词法分析成功");
            
            // 执行语法分析
            match parse(tokens) {
                Ok(ast) => {
                    println!("✅ 语法分析成功");
                    println!("AST: {:?}", ast);
                    
                    // 执行代码生成
                    match generate(ast) {
                        Ok(rust_code) => {
                            println!("✅ 代码生成成功");
                            println!("生成的Rust代码:");
                            println!("{}", rust_code);
                            
                            // 验证生成的代码
                            if rust_code.contains("while i < 5 {") {
                                println!("✅ 验证通过: while循环结构正确");
                            } else {
                                println!("❌ 验证失败: while循环结构不正确");
                            }
                            
                            if rust_code.contains("println!(i);") {
                                println!("✅ 验证通过: println语句正确");
                            } else {
                                println!("❌ 验证失败: println语句不正确");
                            }
                            
                            if rust_code.contains("i += 1;") {
                                println!("✅ 验证通过: i += 1语句正确");
                            } else {
                                println!("❌ 验证失败: i += 1语句不正确");
                            }
                        },
                        Err(e) => {
                            println!("❌ 代码生成错误: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("❌ 语法分析错误: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ 词法分析错误: {}", e);
        }
    }
    
    // 测试用例2: 换行缩进的while循环冒号语法
    let test2 = "var i = 1; while i < 3:\n    println(i);\n    i += 1;";
    println!("\n测试2 - 换行缩进while循环:");
    println!("输入: {}", test2);
    
    // 执行词法分析
    match tokenize(test2) {
        Ok(tokens) => {
            println!("✅ 词法分析成功");
            
            // 执行语法分析
            match parse(tokens) {
                Ok(ast) => {
                    println!("✅ 语法分析成功");
                    println!("AST: {:?}", ast);
                    
                    // 执行代码生成
                    match generate(ast) {
                        Ok(rust_code) => {
                            println!("✅ 代码生成成功");
                            println!("生成的Rust代码:");
                            println!("{}", rust_code);
                            
                            // 验证生成的代码
                            if rust_code.contains("while i < 3 {") {
                                println!("✅ 验证通过: while循环结构正确");
                            } else {
                                println!("❌ 验证失败: while循环结构不正确");
                            }
                        },
                        Err(e) => {
                            println!("❌ 代码生成错误: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("❌ 语法分析错误: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ 词法分析错误: {}", e);
        }
    }
}