//! Functions for generating test data
use fake::fake;

pub enum With<T> {
    Value(T),
    Random,
}

pub fn article_content() -> routeguide_domain::ArticleContent {
    routeguide_domain::ArticleContent {
        title: fake!(Lorem.sentence(4, 10)).to_string(),
        description: fake!(Lorem.paragraph(3, 10)),
        body: fake!(Lorem.paragraph(10, 5)),
        tag_list: vec![fake!(Lorem.word).to_string()],
    }
}

pub fn new_user() -> (routeguide_domain::SignUp, String) {
    let password = fake!(Lorem.word).to_string();
    let sign_up = routeguide_domain::SignUp {
        username: fake!(Internet.user_name).to_string(),
        email: fake!(Internet.free_email).to_string(),
        password: routeguide_domain::Password::from_clear_text(password.clone())
            .expect("Failed to hash password"),
    };
    (sign_up, password)
}
