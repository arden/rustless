use serialize::json::{Object};

use valico::Builder as ValicoBuilder;

use backend::{Request, Response};
use backend::{HandleResult, HandleSuccessResult};
use errors::{Error};

pub use self::api::{Application, Api, Versioning};
pub use self::endpoint::{Endpoint, EndpointBuilder};
pub use self::client::Client;
pub use self::nesting::Nesting;
pub use self::namespace::{Namespace};
pub use self::media::Media;

mod nesting;
mod api;
mod endpoint;
mod namespace;
mod client;
mod media;
mod path;
mod formatters;

pub type ValicoBuildHandler<'a> = |&mut ValicoBuilder|:'a;

pub trait ApiHandler {
    fn api_call(&self, &str, &mut Object, &mut Request, &mut CallInfo) -> HandleResult<Response>;
}

pub type ApiHandlers = Vec<Box<ApiHandler + Send + Sync>>;

pub type Callback = for<'a> fn(&'a mut Client, &Object) -> HandleSuccessResult;
pub type ErrorFormatter = fn(&Box<Error>, &Media) -> Option<Response>;

pub type Callbacks = Vec<Callback>;
pub type ErrorFormatters = Vec<ErrorFormatter>;

pub struct CallInfo<'a> {
    pub media: Media,
    pub before: Callbacks,
    pub before_validation: Callbacks,
    pub after_validation: Callbacks,
    pub after: Callbacks,
    pub app: &'a Application
}

impl<'a> CallInfo<'a> {
    pub fn new(app: &'a Application) -> CallInfo<'a> {
        CallInfo {
            media: Media::default(),
            before: vec![],
            before_validation: vec![],
            after_validation: vec![],
            after: vec![],
            app: app
        }
    }
}



