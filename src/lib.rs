mod addr;
mod message;

pub use async_trait::async_trait;

use message::{Message, Parceable, Parcel};
use tokio::sync::{mpsc, oneshot};

type Sender<A> = mpsc::Sender<Box<dyn Parceable<A>>>;
type Receiver<A> = mpsc::Receiver<Box<dyn Parceable<A>>>;

/// The Context that Runs the Actor.
/// Routes the incoming messages in streams.
pub struct ActorContext<A>
where
    A: Actor,
{
    cmd_tx: Sender<A>,
    cmd_rx: Receiver<A>,

    _marker: std::marker::PhantomData<A>,
}

pub trait Actor
where
    Self: Send,
{
}

#[async_trait]
pub trait Handler<M>
where
    Self: Actor,
    M: Message,
{
    type Reply: Send + 'static;

    async fn handle(&mut self, m: M) -> Self::Reply;
}
