# Runtime Contracts

Runtime contracts connect pipeline graphs, scheduling decisions, worker
lifecycle, bounded channels, cancellation, resources, and events.

## Runtime Engine

`RuntimeEngine` defines:

- start
- stop
- state
- stats
- scheduling decision application

`RuntimeHandle` defines:

- pause
- resume
- drain
- stop
- submit task
- emit event

## Scheduling

Scheduler contracts define task priority, CPU and GPU affinity, NUMA hints,
resource limits, and scheduling policy. Full scheduling algorithms are external
runtime implementations.

## Cancellation

Cancellation contracts support job, worker, and stage shutdown. Shutdown modes
include immediate stop, graceful stop, drain-current-batch,
checkpoint-then-stop, and worker restart.

## Channels

Channel contracts require bounded capacity, backpressure state, queue pressure,
drop policy metadata, timeout send/receive semantics, and queue metrics.
