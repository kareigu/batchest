use dotenv;
use std::env;
use serenity::{
  async_trait, 
  model::prelude::Activity, 
  client::bridge::gateway::GatewayIntents,
  model::{
    gateway::Ready,
    interactions::{
      application_command::{
        ApplicationCommand,
        ApplicationCommandInteractionDataOptionValue,
        ApplicationCommandOptionType,
        ApplicationCommandType,
      },
      Interaction,
      InteractionResponseType,
    },
  },
  prelude::*
};

use tracing::{info, error};


const BATCHEST_EMOTE: &str = "<:BatChest:893122511225634816>";

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
      let content = match command.data.name.as_str() {
        "batchest" => {
          let options = command
            .data
            .options
            .get(0)
            .expect("Expected option")
            .resolved
            .as_ref()
            .expect("Expected option object");
          
          if let ApplicationCommandInteractionDataOptionValue::String(s) = options {
            format!("{emote} {msg} {emote}", msg = s, emote = BATCHEST_EMOTE)
          } else {
            "BatChest".to_string()
          }
        },
        "BatChest" => {
          let msg = command
            .data
            .resolved
            .messages
            .iter()
            .next()
            .expect("Expected message");

            format!("{emote} {msg} {emote}", msg = msg.1.content, emote = BATCHEST_EMOTE)
        },
        _ => "not implemented".to_string(),
      };

      if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
          response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(content))
        })
        .await
        {
          info!("Couldn't respond to command: {}", e)
        }
    }
  }


  async fn ready(&self, ctx: Context, ready: Ready) {
    let activity = Activity::watching("Asmongold");
    ctx.set_activity(activity).await;
    info!("{}#{} running", ready.user.name, ready.user.discriminator);

    let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
      commands
        .create_application_command(|command| {
          command
            .name("batchest")
            .description("BatChest")
            .create_option(|option| {
              option
                .name("message")
                .description("message to BatChest")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
            })
        })
        .create_application_command(|command| {
          command
            .name("BatChest")
            .kind(ApplicationCommandType::Message)
        })
    })
    .await;

    info!("Added commands: {:#?}", commands);
  }
}

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt::init();

  let token = match env::var("TOKEN") {
    Ok(t) => t,
    Err(e) => { 
      error!("Bot token missing {:?}", e);
      panic!("Missing token");
    },
  };

  let application_id: u64 = match env::var("ID") {
    Ok(t) => t.parse().expect("Invalid ID"),
    Err(e) => { 
      error!("Application ID missing {:?}", e);
      panic!("Missing ID");
    },
  };

  let mut client = Client::builder(&token)
    .event_handler(Handler{})
    .application_id(application_id)
    .intents(
        GatewayIntents::GUILDS
      | GatewayIntents::GUILD_MESSAGES
    )
    .await
    .expect("Error creating client");

  if let Err(err) = client.start().await {
    error!("Client error: {:?}", err);
  }
}
