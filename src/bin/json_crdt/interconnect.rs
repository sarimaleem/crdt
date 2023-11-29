use crate::op::Op;
use async_trait::async_trait;
/**



*/
#[async_trait]
pub trait InterconnectInterface<T> {
    fn register(num_participants: u32);
    async fn multicast(payload: T, sender_id: u32);
    async fn recv() -> T;
}

// pub struct ChannelInterconnect<T> {
//
// }
//
// impl<T> ChannelInterconnect<T> {
//
// }
//
// impl<T> InterconnectInterface<T> for ChannelInterconnect<T> {
//
// }