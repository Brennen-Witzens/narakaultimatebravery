use crate::{
    util::{Character, MeleeWeapon, RangeWeapon, Skill, Ultimate},
    Context, Error,
};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

enum GameMode {
    Customs,
    Solos,
    Duos,
    Trios,
}

#[derive(Debug)]
struct CharacterSet {
    character: Character,
    skill: Skill,
    ultimate: Ultimate,
    main_weapon: MeleeWeapon,
    sub_weapon: MeleeWeapon,
    range_weapon: RangeWeapon,
}

/// Ultimate Bravery for Naraka
/// /ultimatebravery
#[poise::command(slash_command)]
pub async fn ultimatebravery(
    ctx: Context<'_>,
    #[description = "Which mode to choose from"]
    #[choices("Customs", "Solos", "Duos", "Trios")]
    mode: &'static str,
    #[description = "If trios wants to be a serious comp"] _serious: Option<bool>,
) -> Result<(), Error> {
    let game_mode = get_game_mode(&mode.to_lowercase()).await?;
    match game_mode {
        GameMode::Customs | GameMode::Solos => {
            let response = create_character_set().await;
            let format = format!(
                "The set is - Character: {}, Skill: {}, Ultimate: {}, Main Melee Priority: {}, Sub Melee Priority: {}, Range: {}",
                response.character,
                response.skill,
                response.ultimate,
                response.main_weapon,
                response.sub_weapon,
                response.range_weapon,
            );
            ctx.say(format).await?;
        }
        GameMode::Duos => {
            let character_one = create_character_set().await;
            let mut character_two = create_character_set().await;

            if character_one.character.to_string() == character_two.character.to_string() {
                character_two.character = rand::random();
            }

            let format_one = format_character_set(character_one).await;
            let format_two = format_character_set(character_two).await;

            let response = format!("{} \n\n{}", format_one, format_two);
            ctx.say(response).await?;
        }
        GameMode::Trios => {
            let mut character_one = create_character_set().await;
            let mut character_two = create_character_set().await;
            let mut character_three = create_character_set().await;

            if character_one.character.to_string() == character_two.character.to_string() {
                character_one.character = rand::random();
            } else if character_one.character.to_string() == character_three.character.to_string() {
                character_three.character = rand::random();
            } else if character_two.character.to_string() == character_three.character.to_string() {
                character_two.character = rand::random();
            }

            let format_one = format_character_set(character_one).await;
            let format_two = format_character_set(character_two).await;
            let format_three = format_character_set(character_three).await;

            let response = format!("{} \n\n{} \n\n{}", format_one, format_two, format_three);
            ctx.say(response).await?;
        }
    }

    Ok(())
}

async fn get_game_mode(game_mode: &str) -> Result<GameMode, Error> {
    match game_mode {
        "customs" => Ok(GameMode::Customs),
        "solos" => Ok(GameMode::Solos),
        "duos" => Ok(GameMode::Duos),
        "trios" => Ok(GameMode::Trios),
        _ => Err("Not a valid game mode".into()),
    }
}

async fn create_character_set() -> CharacterSet {
    let character: Character = rand::random();
    let skill: Skill = rand::random();
    let ultimate: Ultimate = rand::random();
    let main_weapon: MeleeWeapon = rand::random();
    let sub_weapon: MeleeWeapon = rand::random();
    let range_weapon: RangeWeapon = rand::random();
    CharacterSet {
        character,
        skill,
        ultimate,
        main_weapon,
        sub_weapon,
        range_weapon,
    }
}

async fn format_character_set(character_set: CharacterSet) -> String {
    format!("The Character set is - Character: {}, Skill: {}, Ultimate: {}, Main Melee Priority: {}, Sub Melee Priority: {}, Range: {}", character_set.character,
character_set.skill, character_set.ultimate, character_set.main_weapon, character_set.sub_weapon, character_set.range_weapon)
}
