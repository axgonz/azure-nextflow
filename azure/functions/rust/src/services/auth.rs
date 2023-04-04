use actix_web::{
    dev::ServiceRequest,
    Error,
    http::header,
};

use serde::Deserialize;
use base64_url::decode;

// #[derive(Debug, Deserialize)]
// struct Header {
//     typ: String,
//     alg: String,
//     x5t: String,
//     kid: String
// }

#[derive(Debug, Deserialize)]
struct Payload {
    // aud: String,
    // iss: String,
    // appid: String,
    // deviceid: String,
    // family_name: String,
    // given_name: String,
    // name: String,
    // oid: String,
    // scp: String,
    // sub: String,
    // unique_name: String,
    // upn: String,
    roles: Option<Vec<String>>
}

pub async fn extract_roles(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    /*
        Azure will be performing the authentication before we even see the
         request so there is no need to validate token.
    */
    let map = req.headers();

    // If there is no authorization header than it is assumed that auth is not required.
    if !map.contains_key(header::AUTHORIZATION) {
        println!("[auth::extractor] No Authorization header in request.");
        return Ok(vec!["admin".to_string()]);
    }

    let authorization = match map.get(header::AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Ok(value) => value,
            Err(error) => {
                println!("[auth::extractor] Invalid Authorization header.");
                println!("{:#?}", error);
                return Ok(vec![])
            }
        }
        None => {
            println!("[auth::extractor] No Authorization header in request.");
            return Ok(vec![])
        }
    };

    if !authorization.contains(" ") {
        println!("[auth::extractor] Invalid Authorization header.");
        return Ok(vec![])
    }

    if !authorization.contains(".") {
        println!("[auth::extractor] Invalid Authorization header.");
        return Ok(vec![])
    }

    let bearer = authorization
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .split(".")
        .collect::<Vec<&str>>();

    // // Header
    // let header_encoded = bearer[0];
    // let header_decoded = decode(&header_encoded).unwrap();
    // let header_string = std::str::from_utf8(&header_decoded).unwrap();
    // let header: Header = serde_json::from_str(&header_string).unwrap();
    // println!("Header decode successful!");

    // Payload
    let payload_encoded = bearer[1];
    let payload_decoded = match decode(&payload_encoded) {
        Ok(value) => value,
        Err(error) => {
            println!("[auth::extractor] Invalid Authorization header.");
            println!("[auth::extractor] Unable to decode payload.");
            println!("{:#?}", error);
            return Ok(vec![])
        }
    };

    let payload_string = match std::str::from_utf8(&payload_decoded) {
        Ok(value) => value,
        Err(error) => {
            println!("[auth::extractor] Invalid Authorization header.");
            println!("[auth::extractor] Unable to cast payload bytes to string.");
            println!("{:#?}", error);
            return Ok(vec![])
        }
    };

    let payload: Payload = match serde_json::from_str(&payload_string) {
        Ok(value) => value,
        Err(error) => {
            println!("[auth::extractor] Invalid Authorization header.");
            println!("[auth::extractor] Unable to deserialize payload string.");
            println!("{:#?}", error);
            return Ok(vec![])
        }
    };

    if payload.roles.is_some() {
        return Ok(payload.roles.unwrap())
    }
    else {
        return Ok(vec![])
    }
}