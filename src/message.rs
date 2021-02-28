use crate::{async_trait, Actor, Handler};
use tokio::sync::oneshot;

pub trait Message
where
    Self: Sized + Send + 'static,
{
}

pub struct Parcel<M, R> {
    pub(crate) message: M,
    pub(crate) reply: oneshot::Sender<R>,
}

#[async_trait]
impl<A, M, R> Parceable<A> for Parcel<M, R>
where
    M: Message,
    R: Send + 'static,
    A: Handler<M, Reply = R>,
{
    async fn unpack(self, actor: &mut A) {
        let reply = actor.handle(self.message).await;
        self.reply.send(reply);
    }
}

#[async_trait]
pub trait Parceable<A>
where
    A: Actor,
{
    async fn unpack(self, actor: &mut A);
}
