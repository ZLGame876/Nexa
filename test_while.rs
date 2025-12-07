// 这是一个测试while循环冒号语法的简单程序
fn main() {
    // 测试基本的while循环冒号语法
    println!("测试1: 基本while循环");
    let code1 = "var i=1;while i<5: println(i)";
    println!("输入: {}", code1);
    
    // 测试换行缩进的while循环冒号语法
    println!("\n测试2: 换行缩进的while循环");
    let code2 = "var i=1;while i<5:\n    println(i)";
    println!("输入: {}", code2);
}