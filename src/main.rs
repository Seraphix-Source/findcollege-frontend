use actix_files as fs;
use actix_web::{ web, HttpServer, App };
use tera::{ Tera };
mod pages;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
 
    return HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap(); // html

        App::new()
        .app_data(web::Data::new(tera))
        .service(
            fs::Files::new("/static", "./static") // css, js, image etc...
                .show_files_listing()
                .use_last_modified(true),
        )
        .service(pages::index)
        .service(pages::hero)
        .service(pages::exams)
     })

        // .bind(("127.0.0.1", 8000))?
        .bind(("0.0.0.0", 8000))?
        .bind("[::1]:9000")?
        .run().await;
}
