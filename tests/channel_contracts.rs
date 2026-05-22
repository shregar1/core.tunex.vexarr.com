use std::{collections::VecDeque, sync::Mutex, time::Duration};

use tunex_core::{
    channel::{
        BackpressurePolicy, BackpressureState, BoundedChannel, ChannelId, ChannelKind,
        ChannelStats, QueuePressure, ReceiveOutcome, SendOutcome,
    },
    TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

#[test]
fn pressure_state_tracks_queue_depth() {
    assert_eq!(
        BackpressureState::from_depth(1, 10).pressure,
        QueuePressure::Low
    );
    assert_eq!(
        BackpressureState::from_depth(5, 10).pressure,
        QueuePressure::Medium
    );
    assert_eq!(
        BackpressureState::from_depth(9, 10).pressure,
        QueuePressure::High
    );
    assert_eq!(
        BackpressureState::from_depth(10, 10).pressure,
        QueuePressure::Full
    );
    assert!(BackpressureState::from_depth(10, 10).producer_throttled);
}

#[test]
fn channel_stats_include_backpressure_snapshot() {
    let stats = ChannelStats::from_depth(100, 95);

    assert_eq!(stats.capacity, 100);
    assert_eq!(stats.depth, 95);
    assert_eq!(stats.backpressure.pressure, QueuePressure::High);
    assert_eq!(stats.backpressure.consumer_lag, 95);
}

#[test]
fn channel_contract_types_are_send_sync_and_object_safe() {
    assert_send_sync::<ChannelId>();
    assert_send_sync::<ChannelKind>();
    assert_send_sync::<ChannelStats>();
    assert_send_sync::<BackpressureState>();
    assert_send_sync::<BackpressurePolicy>();
    assert_send_sync::<QueuePressure>();
    assert_send_sync::<SendOutcome>();
    assert_send_sync::<ReceiveOutcome<u32>>();
    assert_object_safe::<dyn BoundedChannel<u32>>();
}

struct TestChannel<T> {
    id: ChannelId,
    kind: ChannelKind,
    capacity: usize,
    policy: BackpressurePolicy,
    queue: Mutex<VecDeque<T>>,
}

impl<T> TestChannel<T> {
    fn new(capacity: usize, policy: BackpressurePolicy) -> Self {
        Self {
            id: ChannelId::new("test"),
            kind: ChannelKind::Custom,
            capacity,
            policy,
            queue: Mutex::new(VecDeque::new()),
        }
    }
}

impl<T: Send> BoundedChannel<T> for TestChannel<T> {
    fn id(&self) -> &ChannelId {
        &self.id
    }

    fn kind(&self) -> ChannelKind {
        self.kind
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn policy(&self) -> BackpressurePolicy {
        self.policy
    }

    fn stats(&self) -> ChannelStats {
        ChannelStats::from_depth(self.capacity, self.queue.lock().unwrap().len())
    }

    fn try_send(&self, item: T) -> TunexResult<SendOutcome> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() >= self.capacity {
            return Ok(match self.policy {
                BackpressurePolicy::DropNewest => SendOutcome::Dropped {
                    reason: "drop newest".to_owned(),
                },
                BackpressurePolicy::DropOldest => {
                    let _ = queue.pop_front();
                    queue.push_back(item);
                    SendOutcome::Sent
                }
                _ => SendOutcome::Throttled {
                    retry_after_millis: 1,
                },
            });
        }
        queue.push_back(item);
        Ok(SendOutcome::Sent)
    }

    fn send_timeout(&self, item: T, _timeout: Duration) -> TunexResult<SendOutcome> {
        self.try_send(item)
    }

    fn try_receive(&self) -> TunexResult<ReceiveOutcome<T>> {
        Ok(self
            .queue
            .lock()
            .unwrap()
            .pop_front()
            .map_or(ReceiveOutcome::Empty, ReceiveOutcome::Received))
    }

    fn receive_timeout(&self, _timeout: Duration) -> TunexResult<ReceiveOutcome<T>> {
        self.try_receive()
    }
}

#[test]
fn bounded_channel_contract_supports_drop_and_timeout_methods() {
    let channel = TestChannel::new(1, BackpressurePolicy::DropOldest);

    assert_eq!(channel.try_send(1).unwrap(), SendOutcome::Sent);
    assert_eq!(
        channel.send_timeout(2, Duration::from_millis(1)).unwrap(),
        SendOutcome::Sent
    );
    assert!(matches!(
        channel.try_receive().unwrap(),
        ReceiveOutcome::Received(2)
    ));
    assert_eq!(channel.try_receive().unwrap(), ReceiveOutcome::Empty);
}
