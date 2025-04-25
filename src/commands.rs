use poise::serenity_prelude;

use crate::{
    util::{Character, MeleeWeapon, RangeWeapon, Skill, Ultimate},
    Context, Error, GameSettings, MapRotation, RegisteredPlayers,
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
// - Better saving of data
// - Add tournament id for more consistent 'retrevial' of data (can be internal and just return
// value)
#[poise::command(slash_command)]
pub async fn create_tournament(
    ctx: Context<'_>,
    map: MapRotation,
    #[min = 1]
    #[max = 6]
    game_number: i8,
    is_final_game: bool,
) -> Result<(), Error> {
    let mut response = String::new();
    if is_final_game {
        // Create game setting object
        let game_setting = GameSettings::new(map, game_number);

        // TODO:
        // - Need to get a uuid for touranment value - dont hard code
        // - Possible integration into database for 'easier' retrevial
        let _ = {
            let mut game_settings = ctx.data().game_settings.lock().unwrap();
            let mut tournament = ctx.data().registered_tournament.lock().unwrap();
            game_settings.push(game_setting);
            tournament.insert(0, game_settings.to_vec());
        };

        let _ = {
            let settings = ctx.data().registered_tournament.lock().unwrap();

            if let Some(settings) = settings.get(&0) {
                for val in settings.iter() {
                    response += &format!("{:?}\n", val);
                }
            } else {
                response = format!("There is no tournament with that id");
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

        // TODO: Add tournament id to 'easily' get back the correct tournament
        // return response saying success, and to run command again
        let response = format!("Successfully added to the tournament. Please run the command again to add more values for the tournament.");
        ctx.say(response).await?;
        Ok(())
    }
}

// TODO:
// - naraka_id should be a uuid (number)
// - The other way to do characters is several optional values would allow for easier ways to get
// characters, and people would just need to know how many games there are and the order.
// - Have a tournament id to make things "easier" to link up -> could also have it be part of the
// selection choices. Have it be an internal value that's gotten but shown as a content value
// (though not sure if that's possible)
// !register_for_tournament or /register_for_tournament
#[poise::command(prefix_command, slash_command)]
pub async fn register_for_tournament(
    ctx: Context<'_>,
    user: serenity_prelude::User,
    naraka_id: String,
    character_one: Option<Character>,
    character_two: Option<Character>,
    character_three: Option<Character>,
    character_four: Option<Character>,
    character_five: Option<Character>,
    character_six: Option<Character>,
) -> Result<(), Error> {
    let mut response = String::new();
    let mut character_vec = Vec::<Character>::new();

    // Don't really like these if let somes here for the character(s)... >_<
    if let Some(one) = character_one {
        character_vec.push(one);
    };

    if let Some(two) = character_two {
        character_vec.push(two);
    };

    if let Some(three) = character_three {
        character_vec.push(three);
    };
    if let Some(four) = character_four {
        character_vec.push(four);
    };
    if let Some(five) = character_five {
        character_vec.push(five);
    };
    if let Some(six) = character_six {
        character_vec.push(six);
    };

    let _ = {
        let mut players = ctx.data().registered_players.lock().unwrap();
        let registered_players =
            RegisteredPlayers::new(user.id.to_string(), naraka_id, character_vec);
        players.push(registered_players);

        for player in players.iter() {
            response += &format!("Playing in the tournament: {player:?}\n");
        }
    };

    // TODO: Add a way to save info to a database
    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

// Only for tournament organizers
// Admin or whatever role
#[poise::command(prefix_command)]
pub async fn get_registered_players(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Ultimate Bravery for Naraka
/// /ultimatebravery
#[poise::command(slash_command)]
pub async fn ultimate_bravery(
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
