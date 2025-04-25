use std::{collections::HashMap, env, fmt, sync::Mutex};

mod commands;
mod util;
use poise::{serenity_prelude as poise_serenity, ChoiceParameter};
use util::Character;

// Custom user data passed to all command functions
pub struct Data {
    game_settings: Mutex<Vec<GameSettings>>,
    registered_tournament: Mutex<HashMap<i32, Vec<GameSettings>>>,
    registered_players: Mutex<Vec<RegisteredPlayers>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// TODO: Change to UUIDs
#[derive(Debug)]
pub struct RegisteredPlayers {
    discord_id: String,
    naraka_id: String,
    characters_to_play: Vec<Character>,
}

// NOTE:
// - Should characters_to_play be a reference?
impl RegisteredPlayers {
    pub fn new(discord_id: String, naraka_id: String, characters_to_play: Vec<Character>) -> Self {
        Self {
            discord_id,
            naraka_id,
            characters_to_play,
        }
    }
}

impl fmt::Display for RegisteredPlayers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Discord Id: {} \nNaraka Id: {} \n",
            self.discord_id, self.naraka_id
        );
        for (i, character) in self.characters_to_play.iter().enumerate() {
            write!(f, "Character to play: {} in game {}\n", character, i + 1);
        }

        Ok(())
    }
}

// TODO:
// - Display for GameSettings to make it nicer for printing the values
#[derive(Clone, Debug)]
struct GameSettings {
    map_setting: MapRotation,
    game_number: i8,
}

impl GameSettings {
    pub fn new(map_setting: MapRotation, game_number: i8) -> Self {
        Self {
            map_setting,
            game_number,
        }
    }
}

impl fmt::Display for GameSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Map is: {} Game number is {}",
            self.map_setting, self.game_number
        )
    }
}

#[derive(ChoiceParameter, Debug, Clone)]
enum MapRotation {
    MorusIsleDay,
    MorusIsleNight,
    MorusIsleDusk,
}

impl fmt::Display for MapRotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapRotation::MorusIsleDay => write!(f, "Morus Isle Day"),
            MapRotation::MorusIsleNight => write!(f, "Morus Isle Night"),
            MapRotation::MorusIsleDusk => write!(f, "Morus Isle Dusk"),
        }
    }
}

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[actix_web::main]
async fn main() {
    // Load env files
    dotenvy::dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment variable");
    let intents = poise_serenity::GatewayIntents::non_privileged()
        | poise_serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                register(),
                commands::help(),
                commands::ultimate_bravery(),
                commands::create_tournament(),
                commands::register_for_tournament(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        // TODO: Still need to register these correctly
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    game_settings: Mutex::new(Vec::new()),
                    registered_players: Mutex::new(Vec::new()),
                    registered_tournament: Mutex::new(HashMap::new()),
                })
            })
        })
        .build();

    let client = poise_serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
