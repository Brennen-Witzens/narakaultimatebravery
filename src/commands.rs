use poise::serenity_prelude;

use crate::{
    util::{Character, MeleeWeapon, RangeWeapon, Skill, Ultimate},
    Context, Error, GameSettings, MapRotation,
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

// Tourney Commands
// TODO:
// 1. Start Register - Discord ID, Naraka UUID/Name, Optional Character?
// 2. (Get) Return Results

// Register for Tourney command
// !register -- (user)author - discord id, Naraka UUID, list characters -> unqiue characters
// - Backend/Server needs to have some value for number of games
// - Make a Register Tourney command -> Sindbads, Myself, Erq -> or Discord Guild Admin registers a
// toruney. With games required, and other info.
// - Determine number of games and Map rotation + weather

// TODO:
// Create tournament
// - Can't do multiple choice in single command. Need to track edits to properly set up "game"
// - Not sure if re-run on edit or re-run after done is a thing for slash commands
// - Add descriotions to everything
// - Struct for return
// - Hashmap -> 'Tourney Id' (internal thing) + Settings struct (Map + Game)
#[poise::command(slash_command)]
pub async fn create_tournament(
    ctx: Context<'_>,
    map: MapRotation,
    #[min = 1]
    #[max = 6]
    game_number: i8,
    is_final_game: bool,
) -> Result<(), Error> {
    if is_final_game {
        // Add final map and game to set
        // Lock set/value for game
        // Return with full overall struct
        // Create New GameSetting object
        let game_setting = GameSettings::new(map, game_number);

        // TODO: Need to get a uuid for touranment value - dont hard code
        let _ = {
            let mut game_settings = ctx.data().game_settings.lock().unwrap();
            let mut tournament = ctx.data().registered_tournament.lock().unwrap();
            game_settings.push(game_setting);
            tournament.insert(0, game_settings.to_vec());
        };

        let mut response = String::new();
        let _ = {
            let settings = ctx.data().registered_tournament.lock().unwrap();

            if let Some(settings) = settings.get(&0) {
                for val in settings.iter() {
                    response += &format!("{:?}\n", val);
                }
            } else {
                response += &format!("There is no tournament with that id");
            }
        };
        ctx.say(response).await?;
        Ok(())
    } else {
        // Create New GameSetting object
        let game_setting = GameSettings::new(map, game_number);

        let _ = {
            let mut game_settings = ctx.data().game_settings.lock().unwrap();
            let mut tournament = ctx.data().registered_tournament.lock().unwrap();
            game_settings.push(game_setting);
            tournament.insert(0, game_settings.to_vec());
        };

        let response = format!("Successfully added to the tournament. Please run the command again to add more values for the tournament.");
        // return response saying success, and to run command again
        ctx.say(response).await?;
        Ok(())
    }
}

// TODO:
// - Check permissions/add permissions
#[poise::command(prefix_command, slash_command)]
pub async fn register(ctx: Context<'_>, user: serenity_prelude::User) -> Result<(), Error> {
    Ok(())
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

            // TODO: is there a way to do this better
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

            // TODO: is there a way to do this better
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
