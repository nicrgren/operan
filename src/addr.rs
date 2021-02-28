use crate::{Actor, Handler, Message, Parceable, Parcel, Receiver, Sender};
use tokio::sync::{mpsc, oneshot};

/// Receives messages from an Actor.
pub struct Addr<A>
where
    A: Actor,
{
    cmd_tx: mpsc::Sender<Box<dyn Parceable<A>>>,
    _marker: std::marker::PhantomData<A>,
}

impl<A> Addr<A>
where
    A: Actor,
{
    pub async fn send<M>(&self, message: M) -> <A as Handler<M>>::Reply
    where
        M: Message,
        A: Handler<M>,
    {
        let (reply, res) = oneshot::channel();
        let parcel = Parcel { message, reply };
        self.cmd_tx.send(Box::new(parcel)).await;
        res.await
    }
}
