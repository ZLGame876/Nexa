// 专门测试while循环冒号语法修复的脚本
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::codegen::generate_code;

pub fn test_while_loop_fix() {
    println!("=== 测试while循环冒号语法修复 ===");
    
    // 测试用例1: 基本的while循环冒号语法
    let test1 = "var i=1;while i<5: println(i)";
    println!("\n测试用例1: {}", test1);
    
    match tokenize(test1) {
        Ok(tokens) => {
            println!("词法分析: {:?}", tokens);
            
            match parse(&tokens) {
                Ok(ast) => {
                    println!("语法分析: {:?}", ast);
                    
                    match generate_code(&ast) {
                        Ok(code) => {
                            println!("代码生成:");
                            println!("{}", code);
                        },
                        Err(e) => println!("代码生成错误: {}", e)
                    }
                },
                Err(e) => println!("语法分析错误: {}", e)
            }
        },
        Err(e) => println!("词法分析错误: {}", e)
    }
    
    // 测试用例2: 换行缩进的while循环冒号语法
    let test2 = "var i=1;while i<5:\n    println(i)";
    println!("\n测试用例2: {}", test2);
    
    match tokenize(test2) {
        Ok(tokens) => {
            println!("词法分析: {:?}", tokens);
            
            match parse(&tokens) {
                Ok(ast) => {
                    println!("语法分析: {:?}", ast);
                    
                    match generate_code(&ast) {
                        Ok(code) => {
                            println!("代码生成:");
                            println!("{}", code);
                        },
                        Err(e) => println!("代码生成错误: {}", e)
                    }
                },
                Err(e) => println!("语法分析错误: {}", e)
            }
        },
        Err(e) => println!("词法分析错误: {}", e)
    }
}