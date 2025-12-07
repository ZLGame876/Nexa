use actix_web::{web, App, HttpResponse, HttpServer};
use std::fs;
use std::io::Write;
use std::path::Path;

// 处理编辑器页面请求
async fn editor() -> HttpResponse {
    let html = fs::read_to_string(Path::new("editor.html")).unwrap_or_else(|_| {
        // 如果文件不存在，返回默认的编辑器HTML
        String::from(include_str!("../editor.html"))
    });
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

// 处理代码保存请求
async fn save_code(data: web::Json<SaveRequest>) -> HttpResponse {
    let content = &data.content;
    
    // 保存到临时文件
    if let Ok(mut file) = fs::File::create("temp.nexa") {
        if file.write_all(content.as_bytes()).is_ok() {
            return HttpResponse::Ok().body("Code saved successfully");
        }
    }
    
    HttpResponse::InternalServerError().body("Failed to save code")
}

// 定义保存请求的数据结构
#[derive(serde::Deserialize)]
struct SaveRequest {
    content: String,
}

// 处理代码加载请求
async fn load_code() -> HttpResponse {
    // 读取保存的代码
    let content = match fs::read_to_string("temp.nexa") {
        Ok(content) => content,
        Err(_) => String::from("// Nexa语言示例代码\nvar i = 1;\nwhile i < 5:\n    println(i);\n    i += 1;\n"),
    };
    
    HttpResponse::Ok().json(LoadResponse {
        content,
    })
}

// 定义加载响应的数据结构
#[derive(serde::Serialize)]
struct LoadResponse {
    content: String,
}

// 处理代码执行请求
async fn run_code() -> HttpResponse {
    // 读取保存的代码
    if let Ok(content) = fs::read_to_string("temp.nexa") {
        // 执行词法分析
        match crate::lexer::tokenize(&content) {
            Ok(tokens) => {
                // 执行语法分析
                match crate::parser::parse(&tokens) {
                    Ok(ast) => {
                        // 执行代码生成
                                match crate::codegen::generate_code(&ast) {
                                    Ok(rust_code) => {
                                // 保存生成的Rust代码
                                if let Ok(mut file) = fs::File::create("temp.rs") {
                                    if file.write_all(rust_code.as_bytes()).is_ok() {
                                        return HttpResponse::Ok().json(Response {
                                            success: true,
                                            output: rust_code,
                                            error: None,
                                        });
                                    }
                                }
                                return HttpResponse::InternalServerError().json(Response {
                                    success: false,
                                    output: String::new(),
                                    error: Some("Failed to save Rust code".to_string()),
                                });
                            },
                            Err(e) => {
                                return HttpResponse::Ok().json(Response {
                                    success: false,
                                    output: String::new(),
                                    error: Some(format!("Code generation error: {}", e)),
                                });
                            },
                        }
                    },
                    Err(e) => {
                        return HttpResponse::Ok().json(Response {
                            success: false,
                            output: String::new(),
                            error: Some(format!("Parsing error: {}", e)),
                        });
                    },
                }
            },
            Err(e) => {
                return HttpResponse::Ok().json(Response {
                    success: false,
                    output: String::new(),
                    error: Some(format!("Lexical error: {}", e)),
                });
            },
        }
    }
    
    HttpResponse::InternalServerError().json(Response {
        success: false,
        output: String::new(),
        error: Some("Failed to read code".to_string()),
    })
}

// 定义响应的数据结构
#[derive(serde::Serialize)]
struct Response {
    success: bool,
    output: String,
    error: Option<String>,
}

// 启动Web服务器
pub async fn run() -> std::io::Result<()> {
    println!("Starting web editor server at http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(editor))
            .route("/save", web::post().to(save_code))
            .route("/load", web::get().to(load_code))
            .route("/run", web::post().to(run_code))
            .service(actix_files::Files::new("/static", "./static"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}