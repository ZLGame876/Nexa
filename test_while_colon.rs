use std::process::Command;
use std::io::Write;
use std::fs;

fn main() {
    println!("=== 测试while循环冒号语法修复 ===");
    
    // 测试用例1: 同一行的while循环
    let test1 = "var i = 1; while i < 5: println(i); i += 1;";
    
    // 测试用例2: 换行缩进的while循环
    let test2 = "var i = 1; while i < 3:\n    println(i);\n    i += 1;";
    
    let test_cases = vec![
        ("同一行while循环", test1),
        ("换行缩进while循环", test2),
    ];
    
    for (name, code) in test_cases {
        println!("\n测试: {}", name);
        println!("输入: {}", code);
        
        // 创建临时文件
        let mut file = fs::File::create("test.nexa").expect("Failed to create test file");
        file.write_all(code.as_bytes()).expect("Failed to write test file");
        
        // 运行程序
        let output = Command::new("./target/debug/nexa-lang")
            .arg("test.nexa")
            .output()
            .expect("Failed to execute command");
        
        // 打印输出
        println!("输出:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        
        if !output.stderr.is_empty() {
            println!("错误:");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
    
    // 清理
    fs::remove_file("test.nexa").expect("Failed to remove test file");
}