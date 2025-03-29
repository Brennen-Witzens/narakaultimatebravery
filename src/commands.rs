use crate::{Context, Error};
use rand::{
    distr::{Distribution, StandardUniform},
    random, Rng,
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

#[derive(Debug)]
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

impl Distribution<MeleeWeapon> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MeleeWeapon {
        match rng.random_range(0..=13) {
            0 => MeleeWeapon::Longsword,
            1 => MeleeWeapon::Katana,
            2 => MeleeWeapon::HengSword,
            3 => MeleeWeapon::GreatSword,
            4 => MeleeWeapon::PoleSword,
            5 => MeleeWeapon::Spear,
            6 => MeleeWeapon::Staff,
            7 => MeleeWeapon::DualBlades,
            8 => MeleeWeapon::DualHalberds,
            9 => MeleeWeapon::Dagger,
            10 => MeleeWeapon::Fan,
            11 => MeleeWeapon::Nunchucks,
            _ => MeleeWeapon::FistBlades,
        }
    }
}

#[derive(Debug)]
enum RangeWeapon {
    Bow,
    Cannon,
    Musket,
    Pistol,
    RepeatingCrossbow,
}

impl Distribution<RangeWeapon> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RangeWeapon {
        match rng.random_range(0..=4) {
            0 => RangeWeapon::Bow,
            1 => RangeWeapon::Cannon,
            2 => RangeWeapon::Musket,
            3 => RangeWeapon::Pistol,
            _ => RangeWeapon::RepeatingCrossbow,
        }
    }
}

#[derive(Debug)]
enum Skill {
    F1,
    F2,
    F3,
}

impl Distribution<Skill> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Skill {
        match rng.random_range(0..=3) {
            0 => Skill::F1,
            1 => Skill::F2,
            _ => Skill::F3,
        }
    }
}

#[derive(Debug)]
enum Ultimate {
    V1,
    V2,
    V3,
}

impl Distribution<Ultimate> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ultimate {
        match rng.random_range(0..=3) {
            0 => Ultimate::V1,
            1 => Ultimate::V2,
            _ => Ultimate::V3,
        }
    }
}

// TODO: Setup the skill and ultimate as enums as well
#[derive(Debug)]
struct CharacterSet {
    character: Character,
    skill: Skill,
    ultimate: Ultimate,
    melee_weapon: MeleeWeapon,
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
    let game_mode = get_game_mode(&mode.to_lowercase())?;
    match game_mode {
        GameMode::Customs => {
            let response = create_character_set().await;
            let format = format!("The character set is: {:?}", response);
            ctx.say(format).await?;
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

async fn create_character_set() -> CharacterSet {
    let character: Character = rand::random();
    let skill: Skill = rand::random();
    let ultimate: Ultimate = rand::random();
    let melee_weapon: MeleeWeapon = rand::random();
    let range_weapon: RangeWeapon = rand::random();
    CharacterSet {
        character,
        skill,
        ultimate,
        melee_weapon,
        range_weapon,
    }
}
