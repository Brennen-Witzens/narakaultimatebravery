use crate::{Context, Error};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
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

/// Vote for something
///
/// Enter `~vote pumpkin` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    ctx.say(response).await?;
    Ok(())
}

/// Retrieve number of votes
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// ~getvotes
/// ~getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn getvotes(
    ctx: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {} yet", choice),
            _ => format!("{} people have voted for {}", num_votes, choice),
        };
        ctx.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
            response += &format!("{}: {} votes", choice, num_votes);
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        ctx.say(response).await?;
    };

    Ok(())
}

/// Pick a random number
///
/// ~randomNum
#[poise::command(prefix_command)]
pub async fn getRandomNum(
    ctx: Context<'_>,
    #[description = "Add a value to the random number"] value_to_add: Option<i32>,
) -> Result<(), Error> {
    if let Some(val) = value_to_add {
        let ran = val + 5;
        ctx.say(ran.to_string()).await?;
    } else {
        ctx.say("5").await?;
    }
    Ok(())
}

enum GameMode {
    Customs,
    Solos,
    Duos,
    Trios,
}

#[derive(Debug)]
enum Character {
    Unspecified,
    Viper,
    Feria,
    Tianhai,
    Ziping,
    Temulch,
    Tarka,
    Kurumi,
    Yoto,
    Valda,
    Yueshan,
    Wuchen,
    Justina,
    Takeda,
    Matari,
    Akos,
    Zai,
    Tessa,
    Hadi,
    Shayol,
    Lyam,
    Kylin,
    Cyra,
    Lannie,
}

impl Distribution<Character> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        match rng.random_range(0..=22) {
            0 => Character::Viper,
            1 => Character::Feria,
            2 => Character::Tianhai,
            3 => Character::Ziping,
            4 => Character::Temulch,
            5 => Character::Tarka,
            6 => Character::Kurumi,
            7 => Character::Yoto,
            8 => Character::Valda,
            9 => Character::Yueshan,
            10 => Character::Wuchen,
            11 => Character::Justina,
            12 => Character::Takeda,
            13 => Character::Matari,
            14 => Character::Akos,
            15 => Character::Zai,
            16 => Character::Tessa,
            17 => Character::Hadi,
            18 => Character::Shayol,
            19 => Character::Lyam,
            20 => Character::Kylin,
            21 => Character::Cyra,
            22 => Character::Lannie,
            _ => Character::Unspecified,
        }
    }
}

enum MeleeWeapon {
    Longsword,
    Katana,
    HengSword,
    GreatSword,
    PoleSword,
    Spear,
    Staff,
    DualBlades,
    DualHalberds,
    Dagger,
    Fan,
    Nunchucks,
    FistBlades,
}

enum RangeWeapon {
    Bow,
    Cannon,
    Musket,
    Pistol,
    RepeatingCrossbow,
}

enum Skill {
    F1,
    F2,
    F3,
}

enum Ultimate {
    V1,
    V2,
    V3,
}

// TODO: Setup the skill and ultimate as enums as well
struct CharacterSet {
    character: Character,
    skill: Skill,
    ultimate: Ultimate,
    melee_weapon: MeleeWeapon,
    range_weapon: RangeWeapon,
}

/// Ultimate Bravery for Naraka
/// Define between Customs, solos, duos and trios
/// Trios can have actual comps if flagged
/// Everything else is randomized
/// Character, skill, ultimate and weapons to prioritize
/// !ultimatebravery
#[poise::command(prefix_command)]
pub async fn ultimatebravery(
    ctx: Context<'_>,
    // TODO: Mode is required
    // TODO: Figure out serious flag
    // Serious flag is just a string that is there for if someone wants an actual
    // build or comp, value of what they put in doesn't matter. We just want to know
    // they want a serious value returned
    #[description = "Which mode to choose from"] mode: String,
    #[description = "If trios wants to be a serious comp"] serious: Option<String>,
) -> Result<(), Error> {
    let game_mode = get_game_mode(&mode)?;
    match game_mode {
        GameMode::Customs => {
            let character: Character = rand::random();
            println!("Character is {:?}", character);
        }
        GameMode::Solos => todo!(),
        GameMode::Duos => todo!(),
        GameMode::Trios => {
            //match serious {
            //    Some(_) => {
            //        // Do serious comp randomization here
            //        ctx.say("Sand Siphon".to_string()).await?
            //    }
            //    None => {
            //        // Do ultimate bravery randomization here
            //        ctx.say("Some crazy comp".to_string()).await?
            //    }
            //}
            if serious.is_some() {
                // Do serious comp randomization here
                ctx.say("Sand Siphon".to_string()).await?;
            } else {
                // Do ultimate bravery randomization here
                ctx.say("Some crazy comp".to_string()).await?;
            }
        }
    }

    Ok(())
}

fn get_game_mode(game_mode: &str) -> Result<GameMode, Error> {
    match game_mode {
        "customs" => return Ok(GameMode::Customs),
        "solos" => return Ok(GameMode::Solos),
        "duos" => return Ok(GameMode::Duos),
        "trios" => return Ok(GameMode::Trios),
        _ => return Err("Not a valid game mode".into()),
    }
}
