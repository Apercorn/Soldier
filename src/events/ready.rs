use poise::serenity_prelude::Ready;
use tracing::info;

pub async fn on_ready(ctx: &poise::serenity_prelude::Context, ready: &Ready) {
  info!("{} [Shard: {}] is connected!", ready.user.name, ctx.shard_id);
}
