use poem::handler;


#[handler]
pub fn create_post() -> String{
    String::from("hola")
}