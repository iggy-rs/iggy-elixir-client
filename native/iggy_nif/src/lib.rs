// #[macro_use]
extern crate rustler;
extern crate iggy;
// extern crate async_std;
// extern crate tokio;
// extern crate tracing;
// extern crate tracing_subscriber;

    use iggy::client::{ UserClient};
use rustler::{Error, NifResult, NifTuple, ResourceArc, Term, Atom};
use iggy::clients::client::{IggyClient};

use client::IggyClient;
use iggy::system::ping::Ping;
use rustler::{Env, NifResult, Term, Atom};
use tokio::runtime::{Builder, Runtime};
use iggy::users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME};
pub mod atom;

// pub mod client;

// use async_std::task;
// use std::sync::{Arc, Mutex}; // Import Mutex
// use iggy::users::defaults::*;
use iggy::users::login_user::LoginUser;
// use std::env;
// use std::error::Error as StdError; // Rename to avoid conflict
// use tokio::time::sleep;
//much copied from https://github.com/iggy-rs/iggy/blob/master/examples/src/getting-started/consumer/main.rs
// use crate::atom;

rustler::init!(
    "Elixir.IggyEx",
    [
        login_user,
        ping
    ],
    load = on_load
);

fn on_load(_env: Env, _info: Term) -> bool {
    rustler::resource!(IggyClient, env);
    true
}

pub struct IggyResource {
    inner: RustIggyClient,
    runtime: Runtime,
}

impl IggyResource {
    /// Constructs a new IggyClient.
    ///
    /// This initializes a new runtime for asynchronous operations.
    /// Future versions might utilize asyncio for more Pythonic async.
    #[new]
    fn new() -> Self {
        // TODO: use asyncio
        let runtime = Builder::new_multi_thread()
            .worker_threads(4) // number of worker threads
            .enable_all() // enables all available Tokio features
            .build()
            .unwrap();
        IggyResource {
            inner: RustIggyClient::default(),
            runtime,
        }
    }

    /// Logs in the user with the given credentials.
    /// 
    /// Returns `Ok(())` on success, or a PyRuntimeError on failure.
    fn login_user(&self, username: String, password: String) -> PyResult<()> {
        let login_command = LoginUser { username, password };

        let login_future = self.inner.login_user(&login_command);
        self.runtime
            .block_on(async move { login_future.await })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
        PyResult::Ok(())
    }


#[derive(NifTuple)]
pub struct IggyResourceResponse {
    pub ok: Atom,
    pub resource: ResourceArc<IggyResource>,
}


    // Example of a NIF function you might have for creating a new Iggy client.
    #[rustler::nif]
    fn ping() -> NifResult<Atom> {
        // Logic to create a new Iggy client.
        // Return an ok tuple or error based on operation result.
        let client = IggyClient::default();
        // let client = IggyClient::create(client, IggyClientConfig::default(), None, None, None);
        


        // Ok(atom::ok())
    }

    #[rustler::nif]
    fn login_user(_username: String, _password: String) -> NifResult<Atom> {
    //See below, notionally
    let _login_user = LoginUser {
            username: DEFAULT_ROOT_USERNAME.to_string(),
            password: DEFAULT_ROOT_PASSWORD.to_string(),
        };
    Ok(atom::ok())
    }

}

// #[rustler::nif]
// fn login_user() -> Result<Term, Error> {
//     // Logic to create a new Iggy client.
//     tracing_subscriber::fmt::init();
//     let tcp_client_config = TcpClientConfig {
//         server_address: get_tcp_server_addr(),
//         ..TcpClientConfig::default()
//     };
//     let tcp_client = Box::new(TcpClient::create(Arc::new(tcp_client_config)).unwrap());
//     let client = IggyClient::create(tcp_client, IggyClientConfig::default(), None, None, None);

//     // Or, instead of above lines, you can just use below code, which will create a Iggy
//     // TCP client with default config (default server address for TCP is 127.0.0.1:8090):
//     // let client = IggyClient::default();

//     match task::block_on(client.connect()) {
//         Ok(()) => 

//             match task::block_on(client
//             .login_user(&LoginUser {
//                 username: DEFAULT_ROOT_USERNAME.to_string(),
//                 password: DEFAULT_ROOT_PASSWORD.to_string()
//             })) {
//                 Ok(_identityInfo) => NifResult::<IggyResourceResponse>{
//                     ok: atom::ok(),
//                     client: ResourceArc::new(client)
//                 },
                
//                 Err(err) => Err(Error::Term(Box::new(err.to_string())))
//             },

//         Err(err) => Err(Error::Term(Box::new(err.to_string())))
//     }

// }

// #[rustler::nif]
// fn login_user() -> NifResult<FluvioResourceResponse> {
//     match task::block_on(Fluvio::connect()) {
//         Ok(fluvio) => Ok(FluvioResourceResponse {
//             ok: atom::ok(),
//             resource: ResourceArc::new(FluvioResource {
//                 fluvio: Mutex::new(fluvio),
//             }),
//         }),
//         Err(err) => Err(Error::Term(Box::new(err.to_string())))
//     }
// }
// Continue implementing other necessary NIFs...
// Each function should interact with the Iggy crate and return results or errors to Elixir.
