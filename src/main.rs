use bytes::Bytes;
use chrono::{Local, Timelike};
use libc::SIGTERM;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fmt, io::{Cursor, Read, Write}};
use tokio::runtime::Runtime;

#[cfg(unix)]
use std::{
    env::set_current_dir,
};

pub const URL: &str = "https://d17orwheorv96d.cloudfront.net/";

pub enum Hour {
    OnePm,
    TwoPm,
    ThreePm,
    FourPm,
    FivePm,
    SixPm,
    SevenPm,
    EightPm,
    NinePm,
    TenPm,
    TwelvePm,
    ElevenPm,
    TwelveAm,
    ElevenAm,
    SevenAm,
    EightAm,
    ThreeAm,
    NineAm,
    FourAm,
    FiveAm,
    SixAm,
    TwoAm,
    TenAm,
    OneAm,
    Morning,
    Evening,
    Night,
    Day,
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneAm => write!(f, "1am"),
            Self::TwoAm => write!(f, "2am"),
            Self::ThreeAm => write!(f, "3am"),
            Self::FourAm => write!(f, "4am"),
            Self::FiveAm => write!(f, "5am"),
            Self::SixAm => write!(f, "6am"),
            Self::SevenAm => write!(f, "7am"),
            Self::EightAm => write!(f, "8am"),
            Self::NineAm => write!(f, "9am"),
            Self::TenAm => write!(f, "10am"),
            Self::ElevenAm => write!(f, "11am"),
            Self::TwelveAm => write!(f, "12am"),
            Self::OnePm => write!(f, "1pm"),
            Self::TwoPm => write!(f, "2pm"),
            Self::ThreePm => write!(f, "3pm"),
            Self::FourPm => write!(f, "4pm"),
            Self::FivePm => write!(f, "5pm"),
            Self::SixPm => write!(f, "6pm"),
            Self::SevenPm => write!(f, "7pm"),
            Self::EightPm => write!(f, "8pm"),
            Self::NinePm => write!(f, "9pm"),
            Self::TenPm => write!(f, "10pm"),
            Self::ElevenPm => write!(f, "11pm"),
            Self::Morning => write!(f, "morning"),
            Self::Evening => write!(f, "evening"),
            Self::Night => write!(f, "night"),
            Self::Day => write!(f, "day"),
            Self::TwelvePm => write!(f, "12pm"),
        }
    }
}

impl std::str::FromStr for Hour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "01am" => Ok(Hour::OneAm),
            "02am" => Ok(Hour::TwoAm),
            "03am" => Ok(Hour::ThreeAm),
            "04am" => Ok(Hour::FourAm),
            "05am" => Ok(Hour::FiveAm),
            "06am" => Ok(Hour::SixAm),
            "07am" => Ok(Hour::SevenAm),
            "08am" => Ok(Hour::EightAm),
            "09am" => Ok(Hour::NineAm),
            "10am" => Ok(Hour::TenAm),
            "11am" => Ok(Hour::ElevenAm),
            "12pm" => Ok(Hour::TwelvePm),
            "01pm" => Ok(Hour::OnePm),
            "02pm" => Ok(Hour::TwoPm),
            "03pm" => Ok(Hour::ThreePm),
            "04pm" => Ok(Hour::FourPm),
            "05pm" => Ok(Hour::FivePm),
            "06pm" => Ok(Hour::SixPm),
            "07pm" => Ok(Hour::SevenPm),
            "08pm" => Ok(Hour::EightPm),
            "09pm" => Ok(Hour::NinePm),
            "10pm" => Ok(Hour::TenPm),
            "11pm" => Ok(Hour::ElevenPm),
            "12am" => Ok(Hour::TwelveAm),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
pub enum Climate {
    Rainy,
    Snowy,
    Cherry,
    None,
}

impl fmt::Display for Climate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "{}", ""),
            Self::Snowy => write!(f, "{}", "snowy"),
            Self::Rainy => write!(f, "{}", "rainy"),
            Self::Cherry => write!(f, "{}", "cherry"),
        }
    }
}

#[derive(PartialEq)]
pub enum Game {
    PopulationGrowing(Climate),
    NewHorizons(Climate),
    PocketCamp(Climate),
    WildWorld(Climate),
    NewLeaf(Climate),
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p = "population-growing";
        let w = "wild-world";
        let n = "new-leaf";
        let h = "new-horizons";

        match self {
            Self::PopulationGrowing(climate) => {
                if climate == &Climate::None {
                    write!(f, "{}", p)
                } else {
                    write!(f, "{}-{}", p, climate.to_string())
                }
            }
            Self::WildWorld(climate) => {
                if climate == &Climate::None {
                    write!(f, "{}", w)
                } else {
                    write!(f, "{}-{}", w, climate.to_string())
                }
            }
            Self::NewLeaf(climate) => {
                if climate == &Climate::None {
                    write!(f, "{}", n)
                } else {
                    write!(f, "{}-{}", n, climate.to_string())
                }
            }
            Self::NewHorizons(climate) => {
                if climate == &Climate::None {
                    write!(f, "{}", h)
                } else {
                    write!(f, "{}-{}", h, climate.to_string())
                }
            }
            Self::PocketCamp(_) => {
                write!(f, "{}", "pocket-camp")
            }
        }
    }
}

impl std::str::FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "population-growing" => Ok(Self::PopulationGrowing(Climate::None)),
            "population-growing-snowy" => Ok(Self::PopulationGrowing(Climate::Snowy)),
            "population-growing-cherry" => Ok(Self::PopulationGrowing(Climate::Cherry)),
            "population-growing-rainy" => Ok(Self::PopulationGrowing(Climate::Rainy)),
            "wild-world" => Ok(Self::WildWorld(Climate::None)),
            "wild-world-rainy" => Ok(Self::WildWorld(Climate::Rainy)),
            "wild-world-snowy" => Ok(Self::WildWorld(Climate::Snowy)),
            "new-leaf" => Ok(Self::NewLeaf(Climate::None)),
            "new-leaf-rainy" => Ok(Self::NewLeaf(Climate::Rainy)),
            "new-leaf-snowy" => Ok(Self::NewLeaf(Climate::Snowy)),
            "new-horizons" => Ok(Self::NewHorizons(Climate::None)),
            "new-horizons-rainy" => Ok(Self::NewHorizons(Climate::Rainy)),
            "new-horizons-snowy" => Ok(Self::NewHorizons(Climate::Snowy)),
            "pocket-camp" => Ok(Self::PocketCamp(Climate::None)),
            _ => Err(()),
        }
    }
}

impl Game {
    fn url(&self, base: &str, hour: Hour) -> String {
        format!("{}{}/{}.ogg", base, self.to_string(), hour.to_string())
    }
}

pub enum Rain {
    Game,
    Normal,
    NoThunder,
}

impl fmt::Display for Rain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Game => write!(f, "game-rain"),
            Self::Normal => write!(f, "rain"),
            Self::NoThunder => write!(f, "no-thunder-rain"),
        }
    }
}

impl std::str::FromStr for Rain {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "game" => Ok(Self::Game),
            "normal" => Ok(Self::Normal),
            "no-thunder" => Ok(Self::NoThunder),
            _ => Err(()),
        }
    }
}

impl Rain {
    fn url(&self, base: &str) -> String {
        format!("{}rain/{}.ogg", base, self.to_string())
    }
}

fn get_hour_ampm() -> Result<Hour, ()> {
    let lt = Local::now();
    let hour = lt.format("%I%P").to_string();

    match hour.parse::<Hour>() {
        Ok(h) => Ok(h),
        Err(_) => Err(()),
    }
}

async fn get_bytes(url: &str) -> anyhow::Result<Bytes, anyhow::Error> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    Ok(bytes)
}

async fn play_rain(
    rain: Rain,
    volume: Option<f32>,
    output: OutputStreamHandle,
) -> anyhow::Result<()> {
    let url = rain.url(URL);

    let bytes = get_bytes(&url).await?;
    let cursor = Cursor::new(bytes);

    loop {
        let source = Decoder::new(cursor.clone())?;
        let sink = Sink::try_new(&output)?;

        if let Some(vol) = volume {
            sink.set_volume(vol);
        }

        sink.append(source);
        sink.sleep_until_end();
    }
}

fn is_new_hour() -> bool {
    let lt = Local::now().time();
    if lt.minute() == 0 && lt.second() == 0 {
        true
    } else {
        false
    }
}

// TODO Implement this
pub async fn play_chime() {}

async fn play_song(
    game: Game,
    should_chime: bool,
    volume: Option<f32>,
    output: OutputStreamHandle,
) -> anyhow::Result<()> {
    loop {
        let hour = match get_hour_ampm() {
            Ok(h) => h,
            Err(_) => {
                eprintln!("time is broken :c");
                std::process::exit(1);
            }
        };

        let url = game.url(URL, hour);

        let bytes = get_bytes(&url).await?;
        let cursor = Cursor::new(bytes);

        let source = Decoder::new(cursor.clone())?;
        let sink = Sink::try_new(&output)?;

        if let Some(vol) = volume {
            sink.set_volume(vol);
        }

        sink.append(source);

        loop {
            match is_new_hour() {
                true => {
                    if should_chime {

                    }

                    break;
                }
                false => {
                    if sink.empty() {
                        break;
                    }
                }
            }
        }
    }
}

async fn nookd(args: Args) {
    let (_s, handle) = OutputStream::try_default().expect("Failed to create output stream");

    if args.rain != "none" {
        if let Ok(rain) = args.rain.parse::<Rain>() {
            tokio::spawn(play_rain(rain, args.rain_volume, handle.clone()));
        }
    }

    if let Ok(game) = args.game.parse::<Game>() {
        tokio::spawn(play_song(
            game,
            args.chime,
            args.game_volume,
            handle.clone(),
        ));
    } else {
        eprintln!("Stupid theres no game called '{}'", args.game);
        std::process::exit(1);
    }

    loop {}
}

#[cfg(target_os = "linux")]
fn child(args: Args) {
    if unsafe { libc::setsid() } == -1 {
        panic!("Faile to setsid");
    }

    if let Err(err) = set_current_dir("/") {
        eprintln!("{err}");
    }

    unsafe {
        libc::signal(libc::SIGTERM, handle as usize);
        libc::signal(libc::SIGINT, handle as usize);
    }

    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => {
            std::process::exit(1);
        }
    };

    rt.block_on(nookd(args));
}

use clap::Parser;

const GAME_HELP: &str = r#"
Possible game names, I guess:
    - new-horizons
    - new-horizons-rainy
    - new-horizons-snowy
    - wild-world
    - wild-world-rainy
    - wild-world-snowy
    - new-leaf
    - new-leaf-rainy
    - new-leaf-snowy
    - pocket-camp
    - population-growing
    - population-growing-rainy
    - population-growing-snowy
    - population-growing-cherry
"#;

const RAIN_HELP: &str = r#"
Possible rain types:
    - normal
    - none
    - no-thunder
    - game
"#;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The game you want to play the music from
    #[arg(short, help = GAME_HELP)]
    game: String,

    /// Game volume
    #[arg(long, help = "goes 1 - 100")]
    game_volume: Option<f32>,

    /// The type of rain you want, if any
    #[arg(short, help = RAIN_HELP, default_value_t = String::from("normal"))]
    rain: String,

    /// Rain volume
    #[arg(long, help = "goes 1 - 100")]
    rain_volume: Option<f32>,

    /// If you want to play chimes
    #[arg(short, long, default_value_t = true)]
    chime: bool,

    /// Specifies if you don't want to run as a daemon
    #[arg(long)]
    no_daemon: bool,
}

const LOCKFILE_PATH: &str = "/tmp/sub.lock";

extern "C" fn handle(_: libc::c_int) {
    let fs = std::fs::remove_file(LOCKFILE_PATH);
    if fs.is_err() {
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn handle_shutdown(sid: i32) {
    match unsafe { libc::kill(sid, SIGTERM) } {
        0 => {}
        _ => std::process::exit(1),
    }
}

fn file_exists(path: &str) -> bool {
    if let Ok(m) = std::fs::metadata(path) {
        m.is_file()
    } else {
        false
    }
}

fn lockfile(pid: u32) {
    if file_exists(LOCKFILE_PATH) {
        let mut f = std::fs::File::open(LOCKFILE_PATH)
            .unwrap();

        let mut prev = String::new();
        
        if let Err(_) = f.read_to_string(&mut prev) {
            std::process::exit(1);
        }

        match prev.parse::<u32>() {
            Ok(p) => handle_shutdown(p as i32),
            Err(_) => std::process::exit(1),

        }

        if let Ok(mut f) = std::fs::File::create(LOCKFILE_PATH) {
            let rs = f.write(pid.to_string().as_bytes());
            if rs.is_err() {
                panic!("failure to write pid");
            }
        } else {
            std::process::exit(1);
        }
    } else {
        match std::fs::File::create(LOCKFILE_PATH) {
            Ok(mut f) => {
                if let Err(_) = f.write(pid.to_string().as_bytes()) {
                    std::process::exit(1);
                }
            },
            Err(_) => std::process::exit(1),
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.no_daemon || !cfg!(target_os = "linux") {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(_) => {
                std::process::exit(1);
            }
        };

        rt.block_on(nookd(args));

        return;
    }

    #[cfg(target_os = "linux")]

    lockfile(std::process::id());

    match unsafe { libc::fork() } {
        -1 => panic!("Failed to fork"),
        0 => child(args),
        _ => std::process::exit(0),
    }
}
