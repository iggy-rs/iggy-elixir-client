use iggy::users::login_user::LoginUser;
use tokio::runtime::{Builder, Runtime};
use iggy::client::{Client, SystemClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::system::ping::Ping;
use lazy_static::lazy_static;
use rustler::{Encoder, Error as RustlerError};
use rustler::{Env, Term};
use crate::atom;

lazy_static! {
    static ref IGGY_CLIENT: IggyResource = IggyResource::new();
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
fn connect(env: Env) ->  Result<Term, RustlerError> {
    let resource = &IGGY_CLIENT;

    let connect_future = resource.inner.connect();
    let result = resource.runtime.block_on(connect_future);

    match result {
        Ok(_) => Ok(atom::ok().encode(env)),
        Err(err) => Err(RustlerError::Term(Box::new(err.to_string()))),
    }
}
// for slower functions remember to use dirty scheduler
//#[rustler::nif(schedule = "DirtyCpu")]
#[rustler::nif]
fn ping(env: Env) -> Result<Term, RustlerError> {
    let resource = &IGGY_CLIENT;

    let ping_command = Ping {};
    let ping_future = resource.inner.ping(&ping_command);
    let result = resource.runtime.block_on(ping_future);

    match result {
        Ok(_) => Ok(atom::ok().encode(env)),
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
        Ok(_) => Ok(atom::ok().encode(env)),
        Err(err) => Err(RustlerError::Term(Box::new(err.to_string()))),
    }
}
// TODO:
// Port remaining functions from Python client
//
// impl IggyClient {
//     /// Constructs a new IggyClient.
//     ///
//     /// This initializes a new runtime for asynchronous operations.
//     /// Future versions might utilize asyncio for more Pythonic async.
//     #[new]
//     fn new() -> Self {
//         // TODO: use asyncio
//         let runtime = Builder::new_multi_thread()
//             .worker_threads(4) // number of worker threads
//             .enable_all() // enables all available Tokio features
//             .build()
//             .unwrap();
//         IggyClient {
//             inner: RustIggyClient::default(),
//             runtime,
//         }
//     }

//     /// Creates a new stream with the provided ID and name.
//     ///
//     /// Returns Ok(()) on successful stream creation or a PyRuntimeError on failure.
//     fn create_stream(&self, stream_id: u32, name: String) -> PyResult<()> {
//         let create_stream = CreateStream { stream_id, name };
//         let create_stream_future = self.inner.create_stream(&create_stream);
//         let _create_stream = self
//             .runtime
//             .block_on(async move { create_stream_future.await })
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
//         PyResult::Ok(())
//     }

//     /// Creates a new topic with the given parameters.
//     ///
//     /// Returns Ok(()) on successful topic creation or a PyRuntimeError on failure.
//     fn create_topic(
//         &self,
//         stream_id: u32,
//         topic_id: u32,
//         partitions_count: u32,
//         name: String,
//     ) -> PyResult<()> {
//         let create_topic = CreateTopic {
//             stream_id: Identifier::numeric(stream_id).map_err(|e| {
//                 PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e))
//             })?,
//             topic_id,
//             name,
//             partitions_count,
//             message_expiry: None,
//         };
//         let create_topic_future = self.inner.create_topic(&create_topic);
//         let _create_topic = self
//             .runtime
//             .block_on(async move { create_topic_future.await })
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
//         PyResult::Ok(())
//     }

//     /// Sends a list of messages to the specified topic.
//     ///
//     /// Returns Ok(()) on successful sending or a PyRuntimeError on failure.
//     fn send_messages(
//         &self,
//         stream_id: u32,
//         topic_id: u32,

//         partitioning: u32,
//         messages: &PyList,
//     ) -> PyResult<()> {
//         let messages: Vec<SendMessage> = messages
//             .iter()
//             .map(|item| item.extract::<SendMessage>())
//             .collect::<Result<Vec<_>, _>>()?;
//         let messages: Vec<RustMessage> = messages
//             .into_iter()
//             .map(|message| message.inner)
//             .collect::<Vec<_>>();

//         let mut messages = SendMessages {
//             stream_id: Identifier::numeric(stream_id).unwrap(),
//             topic_id: Identifier::numeric(topic_id).unwrap(),
//             partitioning: Partitioning::partition_id(partitioning),
//             messages,
//         };

//         let send_message_future = self.inner.send_messages(&mut messages);
//         self.runtime
//             .block_on(async move { send_message_future.await })
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;
//         PyResult::Ok(())
//     }

//     /// Polls for messages from the specified topic and partition.
//     ///
//     /// Returns a list of received messages or a PyRuntimeError on failure.
//     fn poll_messages(
//         &self,
//         stream_id: u32,
//         topic_id: u32,
//         partition_id: u32,
//         count: u32,
//         auto_commit: bool,
//     ) -> PyResult<Vec<ReceiveMessage>> {
//         let poll_message_cmd = PollMessages {
//             consumer: RustConsumer::default(),
//             stream_id: Identifier::numeric(stream_id).unwrap(),
//             topic_id: Identifier::numeric(topic_id).unwrap(),
//             partition_id: Some(partition_id),
//             strategy: PollingStrategy::next(),
//             count,
//             auto_commit,
//         };
//         let poll_messages = self.inner.poll_messages(&poll_message_cmd);

//         let polled_messages = self
//             .runtime
//             .block_on(async move { poll_messages.await })
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{:?}", e)))?;

//         let messages = polled_messages
//             .messages
//             .into_iter()
//             .map(|message| ReceiveMessage::from_rust_message(message))
//             .collect::<Vec<_>>();
//         PyResult::Ok(messages)
//     }
// }
// }