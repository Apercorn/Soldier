use poise::serenity_prelude as serenity;
use tracing::info;

pub async fn on_ready(ctx: &serenity::Context, ready: &serenity::Ready) {
  info!(
    "{} [Shard: {}] is connected!",
    ready.user.name, ctx.shard_id
  );
}
