use core::fmt;

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

impl fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Character::Unspecified => write!(f, "Unspecified"),
            Character::Viper => write!(f, "Viper"),
            Character::Feria => write!(f, "Feria"),
            Character::Tianhai => write!(f, "Tianhai"),
            Character::Ziping => write!(f, "Ziping"),
            Character::Temulch => write!(f, "Temulch"),
            Character::Tarka => write!(f, "Tarka"),
            Character::Kurumi => write!(f, "Kurumi"),
            Character::Yoto => write!(f, "Yoto"),
            Character::Valda => write!(f, "Valda"),
            Character::Yueshan => write!(f, "Yueshan"),
            Character::Wuchen => write!(f, "Wuchen"),
            Character::Justina => write!(f, "Justina"),
            Character::Takeda => write!(f, "Takeda"),
            Character::Matari => write!(f, "Matari"),
            Character::Akos => write!(f, "Akos"),
            Character::Zai => write!(f, "Zai"),
            Character::Tessa => write!(f, "Tessa"),
            Character::Hadi => write!(f, "Hadi"),
            Character::Shayol => write!(f, "Shayol"),
            Character::Lyam => write!(f, "Lyam"),
            Character::Kylin => write!(f, "Kylin"),
            Character::Cyra => write!(f, "Cyra"),
            Character::Lannie => write!(f, "Lannie"),
        }
    }
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

// TODO: Serde these
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

impl fmt::Display for MeleeWeapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeleeWeapon::Longsword => write!(f, "Longsword"),
            MeleeWeapon::Katana => write!(f, "Katana"),
            MeleeWeapon::HengSword => write!(f, "Heng Sword"),
            MeleeWeapon::GreatSword => write!(f, "Greatsword"),
            MeleeWeapon::PoleSword => write!(f, "Polesword"),
            MeleeWeapon::Spear => write!(f, "Spear"),
            MeleeWeapon::Staff => write!(f, "Staff"),
            MeleeWeapon::DualBlades => write!(f, "Dual Blades"),
            MeleeWeapon::DualHalberds => write!(f, "Dual Halberds"),
            MeleeWeapon::Dagger => write!(f, "Dagger"),
            MeleeWeapon::Fan => write!(f, "Fan"),
            MeleeWeapon::Nunchucks => write!(f, "Nunchucks"),
            MeleeWeapon::FistBlades => write!(f, "Fistblades"),
        }
    }
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

// TODO: Serde
#[derive(Debug)]
enum RangeWeapon {
    Bow,
    Cannon,
    Musket,
    Pistol,
    RepeatingCrossbow,
}

impl fmt::Display for RangeWeapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeWeapon::Bow => write!(f, "Bow"),
            RangeWeapon::Cannon => write!(f, "Cannon"),
            RangeWeapon::Musket => write!(f, "Musket"),
            RangeWeapon::Pistol => write!(f, "Pistol"),
            RangeWeapon::RepeatingCrossbow => write!(f, "Repeating Crossbow"),
        }
    }
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

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Skill::F1 => write!(f, "F1"),
            Skill::F2 => write!(f, "F2"),
            Skill::F3 => write!(f, "F3"),
        }
    }
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

impl fmt::Display for Ultimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ultimate::V1 => write!(f, "V1"),
            Ultimate::V2 => write!(f, "V2"),
            Ultimate::V3 => write!(f, "V3"),
        }
    }
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
        GameMode::Customs => {
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
        GameMode::Solos => todo!(),
        GameMode::Duos => todo!(),
        GameMode::Trios => todo!(), //{
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
                                    //},
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
