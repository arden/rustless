use url::Url;
use rustless::server::method::Method::{Get};
use rustless::server::status::StatusCode;
use rustless::errors::{Error, ErrorRefExt};
use rustless::{
    Application, Api, Client, Valico, Nesting, SimpleRequest, Response
};

#[deriving(Show)]
pub struct UnauthorizedError;

impl Error for UnauthorizedError {
    fn name(&self) -> &'static str {
        return "Unauthorized";
    }
}

#[test]
fn it_allows_to_create_namespace() {

    let app = app!(|api| {
        api.prefix("api");

        format_error!(api, UnauthorizedError, |_err, _media| {
            Some(Response::from_string(StatusCode::Unauthorized, "Please provide correct `token` parameter".to_string()))
        });

        api.namespace("admin", |admin_ns| {

            admin_ns.params(|params| {
                params.req_typed("token", Valico::string())
            });

            // Using after_validation callback to check token
            admin_ns.after_validation(callback!(|_client, params| {
                match params.get(&"token".to_string()) {
                    // We can unwrap() safely because token in validated already
                    Some(token) => if token.as_string().unwrap().as_slice() == "password1" { return Ok(()) },
                    None => ()
                }

                // Fire error from callback is token is wrong
                return Err(box UnauthorizedError as Box<Error>)
            }));

            // This `/api/admin/server_status` endpoint is secure now
            admin_ns.get("server_status", |endpoint| {

                edp_handler!(endpoint, |client, _params| {
                    client.text("Everything is OK".to_string())  
                })
            });
        })
    });

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/admin/server_status").unwrap();
    assert_eq!(response.status, StatusCode::BadRequest);

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/admin/server_status?token=wrong%20token").unwrap();
    assert_eq!(response.status, StatusCode::Unauthorized);

    let response = call_app!(app, Get, "http://127.0.0.1:3000/api/admin/server_status?token=password1").unwrap();
    assert_eq!(response.status, StatusCode::Ok);

}
