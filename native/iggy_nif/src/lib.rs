use iggy::client::{Client, SystemClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::system::ping::Ping;
use iggy::users::login_user::LoginUser;
use lazy_static::lazy_static;
use rustler::{Env, Error, Term};
use rustler::{Encoder, Error as RustlerError};
use tokio::runtime::{Builder, Runtime};

lazy_static! {
    static ref IGGY_CLIENT: IggyResource = IggyResource::new();
}

mod atoms {
    rustler::atoms! {
        ok,
        error,
        unknown
    }
}

rustler::init!("Elixir.IggyEx", [ping, login_user, connect], load = on_load);

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(IGGY_CLIENT, env);
    true
}

pub struct IggyResource {
    pub inner: IggyClient,
    pub runtime: Runtime,
}

impl IggyResource {
    fn new() -> Self {
        let runtime = Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();
        IggyResource {
            inner: IggyClient::default(),
            runtime,
        }
    }
}

#[rustler::nif]
fn connect(env: Env) ->  Result<Term, Error> {
    let resource = &IGGY_CLIENT;

    let connect_future = resource.inner.connect();
    let result = resource.runtime.block_on(connect_future);

    match result {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(err) => Err(RustlerError::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
fn ping(env: Env) -> Result<Term, Error> {
    let resource = &IGGY_CLIENT;

    let ping_command = Ping {};
    let ping_future = resource.inner.ping(&ping_command);
    let result = resource.runtime.block_on(ping_future);

    match result {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(err) => Err(RustlerError::Term(Box::new(err.to_string()))),
    }
}

#[rustler::nif]
fn login_user(env: Env, username: String, password: String) -> Result<Term, RustlerError> {
    let resource = &IGGY_CLIENT;

    let login_user = LoginUser {
        username,
        password,
    };
    let login_user_future = resource.inner.login_user(&login_user);
    let result = resource.runtime.block_on(login_user_future);

    match result {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(err) => Err(RustlerError::Term(Box::new(err.to_string()))),
    }
}