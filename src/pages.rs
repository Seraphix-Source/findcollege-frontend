use actix_web::{ web, get, HttpResponse, Responder };
use tera::{ Tera, Context };
use serde::Serialize;


// #[get("/{name}")]
// pub async fn index(path: web::Path<String>) -> impl Responder {
//     let string = format!("<h1>{}</h1>", &path);
//     HttpResponse::Ok().body(string)
// }


#[derive(Serialize)]
struct ExamCard {
    id: u32,
    title: String,
    logo_path: String,
    date: String,
    stream: String,
}


#[get("/")]
pub async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "Prince");
    context.insert("title", "FindCollege");

    //   // exams_cards array TODO: add more
        let exams_cards = [
            ExamCard {
                id: 14,
                title: String::from("JEE Main"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("14 Dec 2023"),
                stream: String::from("Engineering"),
            },
            ExamCard {
                id: 15,
                title: String::from("NEET UG"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("2 Mar 2023"),
                stream: String::from("Medical"),
            },
            ExamCard {
                id: 15,
                title: String::from("NEET UG"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("2 Mar 2023"),
                stream: String::from("Medical"),
            },
            ExamCard {
                id: 15,
                title: String::from("NEET UG"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("2 Mar 2023"),
                stream: String::from("Medical"),
            },
             // ...
        ];
    // context.insert()
    context.insert("exams_cards", &exams_cards);




    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/hero/")]
pub async fn hero(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    let rendered = tera.render("home/1_hero.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/exams/")]
pub async fn exams(tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert("name", "Prince");
    context.insert("title", "FindCollege");

    //   // exams_cards array TODO: add more
        let exams_cards = [
            ExamCard {
                id: 14,
                title: String::from("JEE Main"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("14 Dec 2023"),
                stream: String::from("Engineering"),
            },
            ExamCard {
                id: 15,
                title: String::from("NEET UG"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("2 Mar 2023"),
                stream: String::from("Medical"),
            },
            ExamCard {
                id: 15,
                title: String::from("NEET UG"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("2 Mar 2023"),
                stream: String::from("Medical"),
            },
            ExamCard {
                id: 15,
                title: String::from("NEET UG"),
                logo_path: String::from("/static/images/exam_logo.png"),
                date: String::from("2 Mar 2023"),
                stream: String::from("Medical"),
            },
             // ...
        ];
    // context.insert()
    context.insert("exams_cards", &exams_cards);




    let rendered = tera.render("home/3_exams.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
// #[get("/json")]
// pub async fn hash_context(tera: web::Data<Tera>) -> impl Responder {
//     let mut context = Context::new();
   
//     let posts = [
//         Post {
//             title: String::from("This is the first link"),
//             link: String::from("https://example.com"),
//             author: String::from("Bob"),
//         },
//         Post {
//             title: String::from("The Second Link"),
//             link: String::from("https://example.com"),
//             author: String::from("Alice"),
//         },
//     ];

//     context.insert("title", "Find College");
//     context.insert("posts", &posts);

//     let rendered = tera.render("pages/multiple.html", &context).unwrap();
//     HttpResponse::Ok().body(rendered)
// }
