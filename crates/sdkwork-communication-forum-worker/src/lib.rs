use sdkwork_communication_forum_service::ForumService;
use sdkwork_communication_forum_service::ports::repository::ForumRepository;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForumWorkerJob {
    PublishOutbox,
    RebuildSearchProjection,
    RebuildStats,
    EvaluateModerationPolicy,
    FanoutNotifications,
}

pub struct ForumWorker<R: ForumRepository> {
    service: ForumService<R>,
}

impl<R: ForumRepository> ForumWorker<R> {
    pub fn new(service: ForumService<R>) -> Self {
        Self { service }
    }

    pub fn run_job(&self, job: ForumWorkerJob, ctx: &sdkwork_communication_forum_service::value_objects::ForumRequestContext) -> Result<(), String> {
        match job {
            ForumWorkerJob::PublishOutbox => self.publish_outbox(ctx),
            ForumWorkerJob::RebuildSearchProjection => self.rebuild_search_projection(ctx),
            ForumWorkerJob::RebuildStats => self.rebuild_stats(ctx),
            ForumWorkerJob::EvaluateModerationPolicy => self.evaluate_moderation_policy(ctx),
            ForumWorkerJob::FanoutNotifications => self.fanout_notifications(ctx),
        }
    }

    fn publish_outbox(&self, _ctx: &sdkwork_communication_forum_service::value_objects::ForumRequestContext) -> Result<(), String> {
        // Publish pending outbox events through service
        let _ = &self.service;
        // TODO: Implement outbox publishing loop when repository is connected
        Ok(())
    }

    fn rebuild_search_projection(&self, ctx: &sdkwork_communication_forum_service::value_objects::ForumRequestContext) -> Result<(), String> {
        let command = sdkwork_communication_forum_service::domain::commands::RebuildSearchProjectionCommand {
            scope: None,
            board_id: None,
        };
        self.service.rebuild_search_projection(ctx, command)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    fn rebuild_stats(&self, _ctx: &sdkwork_communication_forum_service::value_objects::ForumRequestContext) -> Result<(), String> {
        let _ = &self.service;
        // TODO: Implement stats rebuild when repository is connected
        Ok(())
    }

    fn evaluate_moderation_policy(&self, _ctx: &sdkwork_communication_forum_service::value_objects::ForumRequestContext) -> Result<(), String> {
        let _ = &self.service;
        // TODO: Implement policy evaluation when repository is connected
        Ok(())
    }

    fn fanout_notifications(&self, _ctx: &sdkwork_communication_forum_service::value_objects::ForumRequestContext) -> Result<(), String> {
        let _ = &self.service;
        // TODO: Implement notification fanout when repository is connected
        Ok(())
    }
}
