#[macro_use]
extern crate rustler;
extern crate iggy;
extern crate async_std;
// extern crate tokio;
// extern crate tracing;
// extern crate tracing_subscriber;

use iggy::client::{Client};
use iggy::clients::client::{IggyClient, IggyClientConfig};
use rustler::{Error, NifResult, NifTuple, ResourceArc, Term, Atom};
use async_std::task;
use std::sync::{Arc, Mutex}; // Import Mutex
// use iggy::users::defaults::*;
// use iggy::users::login_user::LoginUser;
// use std::env;
// use std::error::Error as StdError; // Rename to avoid conflict
// use tokio::time::sleep;
//much copied from https://github.com/iggy-rs/iggy/blob/master/examples/src/getting-started/consumer/main.rs

mod atoms {
    rustler::atoms! {
        // Define atoms for common responses or errors you might encounter.
        ok,
        error,
        // Add any specific atoms you might need.
    }
}

rustler::init!(
    "Elixir.IggyNif",
    [
        login_user,
        ping
    ],
    load = on_load
);

fn on_load(env: Env, _info: Term) -> bool {
    true
}


pub struct IggyResource {
    pub iggy: Mutex<IggyClient>,
}
#[derive(NifTuple)]
pub struct IggyResourceResponse {
    pub ok: Atom,
    pub resource: ResourceArc<IggyResource>,
}


// Example of a NIF function you might have for creating a new Iggy client.
#[rustler::nif]
fn ping() -> Result<Atom, Error> {
    // Logic to create a new Iggy client.
    // Return an ok tuple or error based on operation result.
}

fn login_user() -> Result<Atom, Error> {
   //See below, notionally
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
