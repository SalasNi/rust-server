use poem::{get, handler, listener::TcpListener, post, web::Path, Route, Server};

mod post_handlers;

struct Person {
    name: String
}

#[handler]
fn hello(Path(name): Path<String>) -> String {
    let my_person = Person {
        name : String::from(name),
    };
    
    format!("hello: {}", my_person.name)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello)) 
        .at("/hello/", post(post_handlers::create_post));
       
    Server::new(TcpListener::bind("127.0.0.1:3000"))
      .run(app)
      .await
}