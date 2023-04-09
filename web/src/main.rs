use rocket::shield::{Hsts, Shield};
use rocket::time::Duration;

#[rocket::get("/")]
pub async fn page() -> &'static str {
    "o/"
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![page])
        .attach(Shield::default().enable(Hsts::IncludeSubDomains(Duration::new(31536000, 0))))
}
