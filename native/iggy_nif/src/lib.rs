use rustler::{resource, Env, Term};

pub mod atom;
pub mod client;

rustler::init!(
    "Elixir.IggyEx",
    [
        client::ping,
        client::login_user,
        client::connect,
        client::create_stream
    ],
    load = on_load
);

fn on_load(env: Env, _info: Term) -> bool {
    resource!(client::IggyResource, env);
    true
}
