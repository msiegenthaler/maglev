use crate::aggregator::Aggregator;
use concread::CowCell;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Measurer<E, A: Aggregator<E>> {
    join_handle: JoinHandle<()>,
    aggregator: Arc<CowCell<A>>,
    last_value: Arc<CowCell<Option<E>>>,
}

impl<E: Clone + Copy + Send + Sync + 'static, A: Aggregator<E> + Clone + Send + Sync + 'static>
    Measurer<E, A>
{
    pub fn start<F: Fn() -> Option<E>>(
        read_fun: F,
        aggregator: A,
        sleep: Duration,
    ) -> Measurer<E, A>
    where
        F: Send + 'static,
    {
        let last_value = Arc::new(CowCell::new(None));
        let value_writer = last_value.clone();
        let aggregator_arc = Arc::new(CowCell::new(aggregator.reset()));
        let aggregator_writer = aggregator_arc.clone();
        let join_handle = thread::spawn(move || loop {
            let value = read_fun();

            let mut writer = value_writer.write();
            *writer = value;
            writer.commit();

            let mut writer = aggregator_writer.write();
            *writer = writer.clone().push(value);
            writer.commit();

            thread::sleep(sleep);
        });
        Measurer {
            join_handle,
            aggregator: aggregator_arc.clone(),
            last_value,
        }
    }

    pub fn value(&self) -> Option<E> {
        self.aggregator.read().value()
    }

    pub fn last_value(&self) -> Option<E> {
        *self.last_value.read()
    }

    pub fn join(self) -> thread::Result<()> {
        self.join_handle.join()
    }
}
