use serenity::all::{
	Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, Interaction, Ready,
};

use crate::commands;

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		let Interaction::Command(mut command) = interaction else {
			return;
		};

		let result = match command.data.name.as_str() {
			"blog" => match command.data.options.first().map_or("", |option| &option.name) {
				"create" => commands::blog::create(&ctx, &command).await,
				"nick" => commands::blog::nick(&ctx, &command).await,
				"delete" => commands::blog::delete(&ctx, &command).await,
				"webhook" => commands::blog::webhook(&ctx, &command).await,
				name => Err(anyhow::anyhow!("Invalid blog subcommand: '{name}'")),
			},
			"timeoutme" => commands::timeoutme::timeoutme(&ctx, &mut command).await,
			name => Err(anyhow::anyhow!("Invalid command: '{name}'")),
		};

		if let Err(error) = result {
			let message = CreateInteractionResponseMessage::new().content(error.to_string());
			let response = CreateInteractionResponse::Message(message);

			command.create_response(&ctx, response).await.unwrap();
		}
	}

	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is running!", ready.user.name);
	}
}
