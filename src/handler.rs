use serenity::{
    all::{
        ComponentInteraction, ComponentInteractionDataKind, CreateInteractionResponse,
        CreateInteractionResponseMessage, Interaction, Member, Ready, RoleId,
    },
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    prelude::SerenityError,
};

use crate::commands::execute;

const PREFIX: &'static str = "!";
pub struct Handler;
pub struct Command {
    pub command_name: String,
    pub ctx: Context,
    pub message: Message,
    pub args: Vec<String>,
}

struct RoleHandler<'a> {
    user_has_role: Result<bool, SerenityError>,
    interaction: ComponentInteraction,
    ctx: Context,
    member: Member,
    role_id: &'a RoleId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("[Bot] -> {} is Online!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(PREFIX) || msg.author.bot {
            return;
        };

        let args: Vec<String> = msg
            .content
            .clone()
            .split(" ")
            .map(|s| s.trim().to_string())
            .collect();

        let command_name = args[0].to_lowercase().replace(PREFIX, "").clone();

        execute(Command {
            command_name,
            ctx: ctx.clone(),
            message: msg.clone(),
            args,
        })
        .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(interaction) = interaction.message_component() {
            let select = interaction.data.kind.clone();

            let role = match &select {
                ComponentInteractionDataKind::StringSelect { values } => &values[0],
                _ => panic!("unexpected interaction data kind"),
            };

            let guild_id = interaction
                .guild_id
                .expect("[Interaction Create (Guild)] ->");

            let guild = guild_id
                .to_partial_guild(&ctx.http)
                .await
                .expect("[Interaction Create (Guild)] ->");

            let roles = guild
                .roles
                .iter()
                .find(|i| i.1.name.to_lowercase() == role.to_string());

            let role_id = match roles {
                Some((role_id, _)) => role_id,
                None => {
                    println!("[Interaction Create (Role)] -> Role not found.");
                    return;
                }
            };

            let user_has_role = interaction
                .user
                .has_role(&ctx.http, guild_id, role_id)
                .await;

            let member = guild.member(&ctx.http, interaction.user.id).await.unwrap();

            add_role(RoleHandler {
                user_has_role,
                interaction,
                ctx,
                member,
                role_id,
            })
            .await;
        }
    }
}

async fn add_role<'a>(handler: RoleHandler<'a>) {
    
    let RoleHandler {
        user_has_role,
        interaction,
        ctx,
        member,
        role_id,
    } = handler;

    match user_has_role {
        Ok(true) => {
            response(interaction, ctx.clone(), "Cargo removido!").await;
            set_role(false, member, ctx, role_id).await;
        }
        Ok(false) => {
            response(interaction, ctx.clone(), "Cargo Adicionado!").await;
            set_role(true, member, ctx, role_id).await;
        }
        Err(e) => {
            println!("{:?}", e);
            response(
                interaction,
                ctx.clone(),
                "Ocorreu um erro ao adicionar o cargo.",
            )
            .await;
        }
    };
}

async fn response(interaction: ComponentInteraction, ctx: Context, message: &str) {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(message)
                    .ephemeral(true),
            ),
        )
        .await
        .expect("[Role (Message)] -> ");
}

async fn set_role(check: bool, member: Member, ctx: Context, role_id: &RoleId) {
    let result = if check {
        member.add_role(&ctx.http, role_id).await
    } else {
        member.remove_role(&ctx.http, role_id).await
    };

    result.expect("[Role (Set)] ->");
}
