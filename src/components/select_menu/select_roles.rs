use serenity::all::{
    CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, ReactionType,
};

pub fn select_roles() -> CreateActionRow {
    let sm: CreateSelectMenu = CreateSelectMenu::new(
        "roles",
        CreateSelectMenuKind::String {
            options: vec![
                CreateSelectMenuOption::new("Espadachim", "espadachim")
                    .emoji(ReactionType::Unicode("⚔️".to_string())),
                CreateSelectMenuOption::new("Arqueiro", "arqueiro").emoji(ReactionType::from('🏹')),
                CreateSelectMenuOption::new("Mercador", "mercador").emoji(ReactionType::from('🛒')),
                CreateSelectMenuOption::new("Noviço", "noviço").emoji(ReactionType::from('🙏')),
                CreateSelectMenuOption::new("Gatuno", "gatuno")
                    .emoji(ReactionType::Unicode("🗡️".to_string())),
                CreateSelectMenuOption::new("Mago", "mago").emoji(ReactionType::from('🧙')),
            ],
        },
    )
    .placeholder("Seleciona um cargo baseado na sua classe");

    return CreateActionRow::SelectMenu(sm);
}
