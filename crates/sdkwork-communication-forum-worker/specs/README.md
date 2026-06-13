# Worker Specs

Component spec for `sdkwork-communication-forum-worker`.

- **Crate type**: Background job worker
- **Domain**: communication
- **Capability**: forum
- **Jobs**: PublishOutbox, RebuildSearchProjection, RebuildStats, EvaluateModerationPolicy, FanoutNotifications
- **Queues**: `jobs/queues/forum-worker.queues.yaml`
- **Schedules**: `jobs/schedules/forum-maintenance.schedule.yaml`
- **Implementation**: ForumWorker wraps ForumService. Jobs return Ok(()) pending SQLx pool connection.
