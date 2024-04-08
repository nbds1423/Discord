use serenity::all::CreateMessage;

use crate::components::embed::embed::embed;
use crate::components::select_menu::select_roles::select_roles;
use crate::handler::Command;

pub async fn roles(content: Command) {
    let Command { message, ctx, .. } = content;

    message
        .delete(&ctx)
        .await
        .expect("[Command - Roles (Delete)] -> ");

    message
        .channel_id
        .send_message(
            &ctx.http,
            CreateMessage::default()
                .embed(embed())
                .components(vec![select_roles()]),
        )
        .await
        .expect("[Command - Roles] -> ");
}
