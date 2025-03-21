// Add these imports at the top
use actix_web::{get, HttpResponse, Responder};
use crate::templates::basic::{render_login_page, render_signup_page, render_profile_page};

// Add these services to your App in the main function (around line 63)
// Add these handler functions at the end of the file
#[get("/login-page")]
pub async fn login_page() -> HttpResponse {
    render_login_page()
}

#[get("/signup-page")]
pub async fn signup_page() -> impl Responder{
    render_signup_page()
}

#[get("/profile-page")]
pub async fn profile_page() -> HttpResponse {
    render_profile_page()
}
