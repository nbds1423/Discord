use serenity::all::{CreateEmbed, CreateEmbedFooter};

pub fn embed() -> CreateEmbed {
    return CreateEmbed::default()
        .title("Ragnarok - Classe")
        .description("Selecione um cargo baseado na classe do seu personagem principal.")
        .footer(CreateEmbedFooter::new(
            "Selecione o Cargo novamente para remover.",
        ));
}
