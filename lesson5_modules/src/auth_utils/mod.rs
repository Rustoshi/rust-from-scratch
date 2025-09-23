pub fn login(creds: models::Credentials) {
    // authenticate
    super::database::get_user();
}

fn logout(){}   

pub mod models;