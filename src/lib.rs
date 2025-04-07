use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use dashmap::DashMap;
use tokio::sync::broadcast::{self, Sender};

pub trait Subscriber<T>
where
    T: Any + Send + Sync,
{
    fn subscribe_handler(&self, data: &T);
}

pub struct Broker {
    channels: DashMap<TypeId, Sender<Arc<dyn Any + Send + Sync>>>,
}

impl Default for Broker {
    fn default() -> Self {
        Self {
            channels: DashMap::new(),
        }
    }
}

impl Broker {
    pub fn publish<T>(&self, data: T)
    where
        T: Any + Send + Sync,
    {
        let type_id = TypeId::of::<T>();

        // Get or create a channel if not exists
        let channel = self.channels.entry(type_id).or_insert_with(|| {
            let (sdr, _) = broadcast::channel(10);

            sdr
        });

        // Then send
        let arc_data = Arc::new(data);

        channel
            .send(arc_data)
            .inspect_err(|err| {
                log::error!("Failed to send event: {:?}", err);
            })
            .ok();
    }

    pub fn subcribe<T, S>(&self, subscriber: S)
    where
        T: Any + Send + Sync,
        S: Subscriber<T> + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        // Get or create a channel if not exists
        let channel = self.channels.entry(type_id).or_insert_with(|| {
            let (sdr, _) = broadcast::channel(10);

            sdr
        });
        let mut rcv = channel.subscribe();

        tokio::spawn(async move {
            loop {
                match rcv.recv().await {
                    Ok(data) => subscriber.subscribe_handler(data.downcast_ref::<T>().unwrap()),
                    Err(err) => log::error!("Error when receiving an message: {:?}", err),
                }
            }
        });
    }
}
