use actix_web::{web, App, HttpServer, HttpResponse};
use serde::*;

fn index_page() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(r#"
            <h1>Hello rust actix web!</h1>
            <form action="chk_cfg" method="post">
            <input type = "text" name = "req" /> <br>
            <input type = "submit" value = "send" /> 
            </form>
          "#)
}

#[derive(Deserialize)]
struct ReqParam {
    req: String
}

fn check_cfg(form: web::Form<ReqParam>) -> HttpResponse {
    let ret = format!("<h1>Hi! here's your request:<h1><br><p>{}</p>", form.req);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(ret)
}

fn main() {
    println!("Hello, world!\n ready to wait http req....");

    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(index_page))
            .route("/chk_cfg", web::post().to(check_cfg))
    });
    let svr_addr = "127.0.0.1:8999";
    let errmsg = format!("cannot bind {}", svr_addr);
    server.bind(svr_addr)
        .expect(&errmsg)
        .run()
        .expect("cannot run server");
}
