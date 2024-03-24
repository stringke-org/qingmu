use deno_core::{extension, OpState};
use deno_core::error::AnyError;
use deno_core::op2;

use crate::channel::EventSender;
use crate::message::{PluginMessage, SentToUiMessage};

#[op2(fast)]
fn op_send_ui(
    state: &mut OpState,
    #[string] event: String,
    #[string] content: String,
) -> Result<(), AnyError> {
    let mut sender = { state.borrow_mut::<EventSender>().clone() };

    let package = { state.borrow::<String>().clone() };

    sender
        .send(PluginMessage::SentToUi(SentToUiMessage {
            id: "".to_string(),
            package,
            event: event.to_string(),
            content: content.to_string(),
        }))
        .unwrap();

    Ok(())
}

extension!(
  plugin_runtime,
  ops = [ op_send_ui ],
  options = {
    package: String,
    sender: EventSender,
  },
  state = |state, options| {
        state.put(options.package);
        state.put(options.sender);
    },
);
