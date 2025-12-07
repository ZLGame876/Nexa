use nexa_lang::lexer::tokenize;
use nexa_lang::parser::parse;
use nexa_lang::codegen::generate_code;
use nexa_lang::interpreter::interpret;

fn test_println_syntax() {
    println!("=== 测试println函数语法修复 ===");
    
    // 测试用例1: 带括号的println语法
    let test1 = "var i=1; println(i)";
    println!("\n测试用例1: {}\n", test1);
    
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
                            
                            // 执行生成的代码
                            match interpret(&code) {
                                Ok(result) => println!("执行结果: {}", result),
                                Err(e) => println!("执行错误: {}", e)
                            }
                        },
                        Err(e) => println!("代码生成错误: {}", e)
                    }
                },
                Err(e) => println!("语法分析错误: {}", e)
            }
        },
        Err(e) => println!("词法分析错误: {}", e)
    }
    
    // 测试用例2: 不带括号的println语法
    let test2 = "var i=1; println i";
    println!("\n测试用例2: {}\n", test2);
    
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
                            
                            // 执行生成的代码
                            match interpret(&code) {
                                Ok(result) => println!("执行结果: {}", result),
                                Err(e) => println!("执行错误: {}", e)
                            }
                        },
                        Err(e) => println!("代码生成错误: {}", e)
                    }
                },
                Err(e) => println!("语法分析错误: {}", e)
            }
        },
        Err(e) => println!("词法分析错误: {}", e)
    }
    
    println!("=== println函数语法修复测试完成 ===");
}

fn main() {
    test_println_syntax();
}