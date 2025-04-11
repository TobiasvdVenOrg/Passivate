use crate::delegation::ActorApi;

use super::channel_fakes::stub_sender;


pub fn stub_actor_api<T: Send + Clone + 'static>() -> ActorApi<T> {
    ActorApi::new(stub_sender())
}