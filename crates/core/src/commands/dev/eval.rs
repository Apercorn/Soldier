use crate::{CommandResult, Context};
use rhai::Engine;

fn test_console(msg: &str) {
  println!("send from rhai: {}", msg);
}

#[poise::command(
  slash_command,
  description_localized("en-US", "Dynamically parses and executes ")
)]
pub async fn eval(
  ctx: Context<'_>,
  #[description = "Rhai code to evaluate"] code: String,
) -> CommandResult {
  let output = {
    let mut engine = Engine::new();
    engine.register_fn("send", test_console);

    match engine.eval::<rhai::Dynamic>(&code) {
      Ok(result) => format!("Result: {:?}", result),
      Err(e) => format!("Error: {}", e),
    }
  };

  ctx.say(output).await?;

  Ok(())
}
