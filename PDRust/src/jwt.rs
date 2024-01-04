
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    file_ID: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct JWT {

}


pub fn create_jwt(id: u32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let claims = Claims {
        subject_id: id,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();


    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}