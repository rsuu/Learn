use fields_derive::{ShowSize, ToUrl};

#[derive(ShowSize, ToUrl)]
pub struct Request<'a> {
    response_type: &'a str,
    client_id: &'a str,
    scope: Vec<&'a str>,
    redirect_uri: &'a str,
    state: String,
    nonce: String,
}

fn main() {
    let dummy_req = Request {
        response_type: "code",
        client_id: "1234andSomeText",
        scope: vec!["openid", "email", "profile"],
        redirect_uri: "http://dummy-redirect.com",
        state: "security_token0815".to_string(),
        nonce: "80085-3531".to_string(),
    };

    // Call `to_url` on the insance, passing in a base-url
    let url = dummy_req.show_size();
    println!("{}", url);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_url_from_fields_and_values() {
        // Create an instance of the annotated sruct
        let dummy_req = Request {
            response_type: "code",
            client_id: "1234andSomeText",
            scope: vec!["openid", "email", "profile"],
            redirect_uri: "http://dummy-redirect.com",
            state: "security_token0815".to_string(),
            nonce: "80085-3531".to_string(),
        };

        // Call `to_url` on the insance, passing in a base-url
        let url = dummy_req.to_url("https://dummy-base-url".to_string());

        // All fields are collected into one url with query string
        assert_eq!(
            url,
            "https://dummy-base-url?\
                response_type=code&\
                client_id=1234andSomeText&\
                scope=openid%20email%20profile&\
                redirect_uri=http://dummy-redirect.com&\
                state=security_token0815&\
                nonce=80085-3531"
                .to_string()
        );
    }
}
