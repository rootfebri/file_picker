use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickStrings {
    pub dots: Dots,
    pub dots2: Dots2,
    pub dots3: Dots3,
    pub dots4: Dots4,
    pub dots5: Dots5,
    pub dots6: Dots6,
    pub dots7: Dots7,
    pub dots8: Dots8,
    pub dots9: Dots9,
    pub dots10: Dots10,
    pub dots11: Dots11,
    pub dots12: Dots12,
    pub dots13: Dots13,
    pub dots14: Dots14,
    pub dots8bit: Dots8bit,
    pub dots_circle: DotsCircle,
    pub sand: Sand,
    pub line: Line,
    pub line2: Line2,
    pub pipe: Pipe,
    pub simple_dots: SimpleDots,
    pub simple_dots_scrolling: SimpleDotsScrolling,
    pub star: Star,
    pub star2: Star2,
    pub flip: Flip,
    pub hamburger: Hamburger,
    pub grow_vertical: GrowVertical,
    pub grow_horizontal: GrowHorizontal,
    pub balloon: Balloon,
    pub balloon2: Balloon2,
    pub noise: Noise,
    pub bounce: Bounce,
    pub box_bounce: BoxBounce,
    pub box_bounce2: BoxBounce2,
    pub triangle: Triangle,
    pub binary: Binary,
    pub arc: Arc,
    pub circle: Circle,
    pub square_corners: SquareCorners,
    pub circle_quarters: CircleQuarters,
    pub circle_halves: CircleHalves,
    pub squish: Squish,
    pub toggle: Toggle,
    pub toggle2: Toggle2,
    pub toggle3: Toggle3,
    pub toggle4: Toggle4,
    pub toggle5: Toggle5,
    pub toggle6: Toggle6,
    pub toggle7: Toggle7,
    pub toggle8: Toggle8,
    pub toggle9: Toggle9,
    pub toggle10: Toggle10,
    pub toggle11: Toggle11,
    pub toggle12: Toggle12,
    pub toggle13: Toggle13,
    pub arrow: Arrow,
    pub arrow2: Arrow2,
    pub arrow3: Arrow3,
    pub bouncing_bar: BouncingBar,
    pub bouncing_ball: BouncingBall,
    pub smiley: Smiley,
    pub monkey: Monkey,
    pub hearts: Hearts,
    pub clock: Clock,
    pub earth: Earth,
    pub material: Material,
    pub moon: Moon,
    pub runner: Runner,
    pub pong: Pong,
    pub shark: Shark,
    pub dqpb: Dqpb,
    pub weather: Weather,
    pub christmas: Christmas,
    pub grenade: Grenade,
    pub point: Point,
    pub layer: Layer,
    pub beta_wave: BetaWave,
    pub finger_dance: FingerDance,
    pub fist_bump: FistBump,
    pub soccer_header: SoccerHeader,
    pub mindblown: Mindblown,
    pub speaker: Speaker,
    pub orange_pulse: OrangePulse,
    pub blue_pulse: BluePulse,
    pub orange_blue_pulse: OrangeBluePulse,
    pub time_travel: TimeTravel,
    pub aesthetic: Aesthetic,
    pub dwarf_fortress: DwarfFortress,
}

impl TickStrings {
    pub fn new() -> Self {
        serde_json::from_str(_JSON).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots3 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots4 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots5 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots6 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots7 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots8 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots9 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots10 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots11 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots12 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots13 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots14 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dots8bit {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DotsCircle {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sand {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipe {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDots {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDotsScrolling {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Star {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Star2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flip {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hamburger {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowVertical {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowHorizontal {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balloon {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balloon2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Noise {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bounce {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxBounce {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxBounce2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Triangle {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binary {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arc {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circle {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SquareCorners {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircleQuarters {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircleHalves {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Squish {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle3 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle4 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle5 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle6 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle7 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle8 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle9 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle10 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle11 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle12 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle13 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrow {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrow2 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrow3 {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BouncingBar {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BouncingBall {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Smiley {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monkey {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hearts {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Earth {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Moon {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runner {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pong {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shark {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dqpb {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Christmas {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grenade {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetaWave {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerDance {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FistBump {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoccerHeader {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mindblown {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speaker {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrangePulse {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BluePulse {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrangeBluePulse {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeTravel {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aesthetic {
    pub interval: i64,
    pub frames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DwarfFortress {
    pub interval: i64,
    pub frames: Vec<String>,
}

const _JSON: &str = r#"{
    "dots": {
        "interval": 80,
        "frames": [
            "⠋",
            "⠙",
            "⠹",
            "⠸",
            "⠼",
            "⠴",
            "⠦",
            "⠧",
            "⠇",
            "⠏"
        ]
    },
    "dots2": {
        "interval": 80,
        "frames": [
            "⣾",
            "⣽",
            "⣻",
            "⢿",
            "⡿",
            "⣟",
            "⣯",
            "⣷"
        ]
    },
    "dots3": {
        "interval": 80,
        "frames": [
            "⠋",
            "⠙",
            "⠚",
            "⠞",
            "⠖",
            "⠦",
            "⠴",
            "⠲",
            "⠳",
            "⠓"
        ]
    },
    "dots4": {
        "interval": 80,
        "frames": [
            "⠄",
            "⠆",
            "⠇",
            "⠋",
            "⠙",
            "⠸",
            "⠰",
            "⠠",
            "⠰",
            "⠸",
            "⠙",
            "⠋",
            "⠇",
            "⠆"
        ]
    },
    "dots5": {
        "interval": 80,
        "frames": [
            "⠋",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋"
        ]
    },
    "dots6": {
        "interval": 80,
        "frames": [
            "⠁",
            "⠉",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠤",
            "⠄",
            "⠄",
            "⠤",
            "⠴",
            "⠲",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠚",
            "⠙",
            "⠉",
            "⠁"
        ]
    },
    "dots7": {
        "interval": 80,
        "frames": [
            "⠈",
            "⠉",
            "⠋",
            "⠓",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠖",
            "⠦",
            "⠤",
            "⠠",
            "⠠",
            "⠤",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            "⠉",
            "⠈"
        ]
    },
    "dots8": {
        "interval": 80,
        "frames": [
            "⠁",
            "⠁",
            "⠉",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠤",
            "⠄",
            "⠄",
            "⠤",
            "⠠",
            "⠠",
            "⠤",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            "⠉",
            "⠈",
            "⠈"
        ]
    },
    "dots9": {
        "interval": 80,
        "frames": [
            "⢹",
            "⢺",
            "⢼",
            "⣸",
            "⣇",
            "⡧",
            "⡗",
            "⡏"
        ]
    },
    "dots10": {
        "interval": 80,
        "frames": [
            "⢄",
            "⢂",
            "⢁",
            "⡁",
            "⡈",
            "⡐",
            "⡠"
        ]
    },
    "dots11": {
        "interval": 100,
        "frames": [
            "⠁",
            "⠂",
            "⠄",
            "⡀",
            "⢀",
            "⠠",
            "⠐",
            "⠈"
        ]
    },
    "dots12": {
        "interval": 80,
        "frames": [
            "⢀⠀",
            "⡀⠀",
            "⠄⠀",
            "⢂⠀",
            "⡂⠀",
            "⠅⠀",
            "⢃⠀",
            "⡃⠀",
            "⠍⠀",
            "⢋⠀",
            "⡋⠀",
            "⠍⠁",
            "⢋⠁",
            "⡋⠁",
            "⠍⠉",
            "⠋⠉",
            "⠋⠉",
            "⠉⠙",
            "⠉⠙",
            "⠉⠩",
            "⠈⢙",
            "⠈⡙",
            "⢈⠩",
            "⡀⢙",
            "⠄⡙",
            "⢂⠩",
            "⡂⢘",
            "⠅⡘",
            "⢃⠨",
            "⡃⢐",
            "⠍⡐",
            "⢋⠠",
            "⡋⢀",
            "⠍⡁",
            "⢋⠁",
            "⡋⠁",
            "⠍⠉",
            "⠋⠉",
            "⠋⠉",
            "⠉⠙",
            "⠉⠙",
            "⠉⠩",
            "⠈⢙",
            "⠈⡙",
            "⠈⠩",
            "⠀⢙",
            "⠀⡙",
            "⠀⠩",
            "⠀⢘",
            "⠀⡘",
            "⠀⠨",
            "⠀⢐",
            "⠀⡐",
            "⠀⠠",
            "⠀⢀",
            "⠀⡀"
        ]
    },
    "dots13": {
        "interval": 80,
        "frames": [
            "⣼",
            "⣹",
            "⢻",
            "⠿",
            "⡟",
            "⣏",
            "⣧",
            "⣶"
        ]
    },
    "dots14": {
        "interval": 80,
        "frames": [
            "⠉⠉",
            "⠈⠙",
            "⠀⠹",
            "⠀⢸",
            "⠀⣰",
            "⢀⣠",
            "⣀⣀",
            "⣄⡀",
            "⣆⠀",
            "⡇⠀",
            "⠏⠀",
            "⠋⠁"
        ]
    },
    "dots8bit": {
        "interval": 80,
        "frames": [
            "⠀",
            "⠁",
            "⠂",
            "⠃",
            "⠄",
            "⠅",
            "⠆",
            "⠇",
            "⡀",
            "⡁",
            "⡂",
            "⡃",
            "⡄",
            "⡅",
            "⡆",
            "⡇",
            "⠈",
            "⠉",
            "⠊",
            "⠋",
            "⠌",
            "⠍",
            "⠎",
            "⠏",
            "⡈",
            "⡉",
            "⡊",
            "⡋",
            "⡌",
            "⡍",
            "⡎",
            "⡏",
            "⠐",
            "⠑",
            "⠒",
            "⠓",
            "⠔",
            "⠕",
            "⠖",
            "⠗",
            "⡐",
            "⡑",
            "⡒",
            "⡓",
            "⡔",
            "⡕",
            "⡖",
            "⡗",
            "⠘",
            "⠙",
            "⠚",
            "⠛",
            "⠜",
            "⠝",
            "⠞",
            "⠟",
            "⡘",
            "⡙",
            "⡚",
            "⡛",
            "⡜",
            "⡝",
            "⡞",
            "⡟",
            "⠠",
            "⠡",
            "⠢",
            "⠣",
            "⠤",
            "⠥",
            "⠦",
            "⠧",
            "⡠",
            "⡡",
            "⡢",
            "⡣",
            "⡤",
            "⡥",
            "⡦",
            "⡧",
            "⠨",
            "⠩",
            "⠪",
            "⠫",
            "⠬",
            "⠭",
            "⠮",
            "⠯",
            "⡨",
            "⡩",
            "⡪",
            "⡫",
            "⡬",
            "⡭",
            "⡮",
            "⡯",
            "⠰",
            "⠱",
            "⠲",
            "⠳",
            "⠴",
            "⠵",
            "⠶",
            "⠷",
            "⡰",
            "⡱",
            "⡲",
            "⡳",
            "⡴",
            "⡵",
            "⡶",
            "⡷",
            "⠸",
            "⠹",
            "⠺",
            "⠻",
            "⠼",
            "⠽",
            "⠾",
            "⠿",
            "⡸",
            "⡹",
            "⡺",
            "⡻",
            "⡼",
            "⡽",
            "⡾",
            "⡿",
            "⢀",
            "⢁",
            "⢂",
            "⢃",
            "⢄",
            "⢅",
            "⢆",
            "⢇",
            "⣀",
            "⣁",
            "⣂",
            "⣃",
            "⣄",
            "⣅",
            "⣆",
            "⣇",
            "⢈",
            "⢉",
            "⢊",
            "⢋",
            "⢌",
            "⢍",
            "⢎",
            "⢏",
            "⣈",
            "⣉",
            "⣊",
            "⣋",
            "⣌",
            "⣍",
            "⣎",
            "⣏",
            "⢐",
            "⢑",
            "⢒",
            "⢓",
            "⢔",
            "⢕",
            "⢖",
            "⢗",
            "⣐",
            "⣑",
            "⣒",
            "⣓",
            "⣔",
            "⣕",
            "⣖",
            "⣗",
            "⢘",
            "⢙",
            "⢚",
            "⢛",
            "⢜",
            "⢝",
            "⢞",
            "⢟",
            "⣘",
            "⣙",
            "⣚",
            "⣛",
            "⣜",
            "⣝",
            "⣞",
            "⣟",
            "⢠",
            "⢡",
            "⢢",
            "⢣",
            "⢤",
            "⢥",
            "⢦",
            "⢧",
            "⣠",
            "⣡",
            "⣢",
            "⣣",
            "⣤",
            "⣥",
            "⣦",
            "⣧",
            "⢨",
            "⢩",
            "⢪",
            "⢫",
            "⢬",
            "⢭",
            "⢮",
            "⢯",
            "⣨",
            "⣩",
            "⣪",
            "⣫",
            "⣬",
            "⣭",
            "⣮",
            "⣯",
            "⢰",
            "⢱",
            "⢲",
            "⢳",
            "⢴",
            "⢵",
            "⢶",
            "⢷",
            "⣰",
            "⣱",
            "⣲",
            "⣳",
            "⣴",
            "⣵",
            "⣶",
            "⣷",
            "⢸",
            "⢹",
            "⢺",
            "⢻",
            "⢼",
            "⢽",
            "⢾",
            "⢿",
            "⣸",
            "⣹",
            "⣺",
            "⣻",
            "⣼",
            "⣽",
            "⣾",
            "⣿"
        ]
    },
    "dotsCircle": {
        "interval": 80,
        "frames": [
            "⢎ ",
            "⠎⠁",
            "⠊⠑",
            "⠈⠱",
            " ⡱",
            "⢀⡰",
            "⢄⡠",
            "⢆⡀"
        ]
    },
    "sand": {
        "interval": 80,
        "frames": [
            "⠁",
            "⠂",
            "⠄",
            "⡀",
            "⡈",
            "⡐",
            "⡠",
            "⣀",
            "⣁",
            "⣂",
            "⣄",
            "⣌",
            "⣔",
            "⣤",
            "⣥",
            "⣦",
            "⣮",
            "⣶",
            "⣷",
            "⣿",
            "⡿",
            "⠿",
            "⢟",
            "⠟",
            "⡛",
            "⠛",
            "⠫",
            "⢋",
            "⠋",
            "⠍",
            "⡉",
            "⠉",
            "⠑",
            "⠡",
            "⢁"
        ]
    },
    "line": {
        "interval": 130,
        "frames": [
            "-",
            "\\",
            "|",
            "/"
        ]
    },
    "line2": {
        "interval": 100,
        "frames": [
            "⠂",
            "-",
            "–",
            "—",
            "–",
            "-"
        ]
    },
    "pipe": {
        "interval": 100,
        "frames": [
            "┤",
            "┘",
            "┴",
            "└",
            "├",
            "┌",
            "┬",
            "┐"
        ]
    },
    "simpleDots": {
        "interval": 400,
        "frames": [
            ".  ",
            ".. ",
            "...",
            "   "
        ]
    },
    "simpleDotsScrolling": {
        "interval": 200,
        "frames": [
            ".  ",
            ".. ",
            "...",
            " ..",
            "  .",
            "   "
        ]
    },
    "star": {
        "interval": 70,
        "frames": [
            "✶",
            "✸",
            "✹",
            "✺",
            "✹",
            "✷"
        ]
    },
    "star2": {
        "interval": 80,
        "frames": [
            "+",
            "x",
            "*"
        ]
    },
    "flip": {
        "interval": 70,
        "frames": [
            "_",
            "_",
            "_",
            "-",
            "`",
            "`",
            "'",
            "´",
            "-",
            "_",
            "_",
            "_"
        ]
    },
    "hamburger": {
        "interval": 100,
        "frames": [
            "☱",
            "☲",
            "☴"
        ]
    },
    "growVertical": {
        "interval": 120,
        "frames": [
            "▁",
            "▃",
            "▄",
            "▅",
            "▆",
            "▇",
            "▆",
            "▅",
            "▄",
            "▃"
        ]
    },
    "growHorizontal": {
        "interval": 120,
        "frames": [
            "▏",
            "▎",
            "▍",
            "▌",
            "▋",
            "▊",
            "▉",
            "▊",
            "▋",
            "▌",
            "▍",
            "▎"
        ]
    },
    "balloon": {
        "interval": 140,
        "frames": [
            " ",
            ".",
            "o",
            "O",
            "@",
            "*",
            " "
        ]
    },
    "balloon2": {
        "interval": 120,
        "frames": [
            ".",
            "o",
            "O",
            "°",
            "O",
            "o",
            "."
        ]
    },
    "noise": {
        "interval": 100,
        "frames": [
            "▓",
            "▒",
            "░"
        ]
    },
    "bounce": {
        "interval": 120,
        "frames": [
            "⠁",
            "⠂",
            "⠄",
            "⠂"
        ]
    },
    "boxBounce": {
        "interval": 120,
        "frames": [
            "▖",
            "▘",
            "▝",
            "▗"
        ]
    },
    "boxBounce2": {
        "interval": 100,
        "frames": [
            "▌",
            "▀",
            "▐",
            "▄"
        ]
    },
    "triangle": {
        "interval": 50,
        "frames": [
            "◢",
            "◣",
            "◤",
            "◥"
        ]
    },
    "binary": {
        "interval": 80,
        "frames": [
            "010010",
            "001100",
            "100101",
            "111010",
            "111101",
            "010111",
            "101011",
            "111000",
            "110011",
            "110101"
        ]
    },
    "arc": {
        "interval": 100,
        "frames": [
            "◜",
            "◠",
            "◝",
            "◞",
            "◡",
            "◟"
        ]
    },
    "circle": {
        "interval": 120,
        "frames": [
            "◡",
            "⊙",
            "◠"
        ]
    },
    "squareCorners": {
        "interval": 180,
        "frames": [
            "◰",
            "◳",
            "◲",
            "◱"
        ]
    },
    "circleQuarters": {
        "interval": 120,
        "frames": [
            "◴",
            "◷",
            "◶",
            "◵"
        ]
    },
    "circleHalves": {
        "interval": 50,
        "frames": [
            "◐",
            "◓",
            "◑",
            "◒"
        ]
    },
    "squish": {
        "interval": 100,
        "frames": [
            "╫",
            "╪"
        ]
    },
    "toggle": {
        "interval": 250,
        "frames": [
            "⊶",
            "⊷"
        ]
    },
    "toggle2": {
        "interval": 80,
        "frames": [
            "▫",
            "▪"
        ]
    },
    "toggle3": {
        "interval": 120,
        "frames": [
            "□",
            "■"
        ]
    },
    "toggle4": {
        "interval": 100,
        "frames": [
            "■",
            "□",
            "▪",
            "▫"
        ]
    },
    "toggle5": {
        "interval": 100,
        "frames": [
            "▮",
            "▯"
        ]
    },
    "toggle6": {
        "interval": 300,
        "frames": [
            "ဝ",
            "၀"
        ]
    },
    "toggle7": {
        "interval": 80,
        "frames": [
            "⦾",
            "⦿"
        ]
    },
    "toggle8": {
        "interval": 100,
        "frames": [
            "◍",
            "◌"
        ]
    },
    "toggle9": {
        "interval": 100,
        "frames": [
            "◉",
            "◎"
        ]
    },
    "toggle10": {
        "interval": 100,
        "frames": [
            "㊂",
            "㊀",
            "㊁"
        ]
    },
    "toggle11": {
        "interval": 50,
        "frames": [
            "⧇",
            "⧆"
        ]
    },
    "toggle12": {
        "interval": 120,
        "frames": [
            "☗",
            "☖"
        ]
    },
    "toggle13": {
        "interval": 80,
        "frames": [
            "=",
            "*",
            "-"
        ]
    },
    "arrow": {
        "interval": 100,
        "frames": [
            "←",
            "↖",
            "↑",
            "↗",
            "→",
            "↘",
            "↓",
            "↙"
        ]
    },
    "arrow2": {
        "interval": 80,
        "frames": [
            "⬆️ ",
            "↗️ ",
            "➡️ ",
            "↘️ ",
            "⬇️ ",
            "↙️ ",
            "⬅️ ",
            "↖️ "
        ]
    },
    "arrow3": {
        "interval": 120,
        "frames": [
            "▹▹▹▹▹",
            "▸▹▹▹▹",
            "▹▸▹▹▹",
            "▹▹▸▹▹",
            "▹▹▹▸▹",
            "▹▹▹▹▸"
        ]
    },
    "bouncingBar": {
        "interval": 80,
        "frames": [
            "[    ]",
            "[=   ]",
            "[==  ]",
            "[=== ]",
            "[====]",
            "[ ===]",
            "[  ==]",
            "[   =]",
            "[    ]",
            "[   =]",
            "[  ==]",
            "[ ===]",
            "[====]",
            "[=== ]",
            "[==  ]",
            "[=   ]"
        ]
    },
    "bouncingBall": {
        "interval": 80,
        "frames": [
            "( ●    )",
            "(  ●   )",
            "(   ●  )",
            "(    ● )",
            "(     ●)",
            "(    ● )",
            "(   ●  )",
            "(  ●   )",
            "( ●    )",
            "(●     )"
        ]
    },
    "smiley": {
        "interval": 200,
        "frames": [
            "😄 ",
            "😝 "
        ]
    },
    "monkey": {
        "interval": 300,
        "frames": [
            "🙈 ",
            "🙈 ",
            "🙉 ",
            "🙊 "
        ]
    },
    "hearts": {
        "interval": 100,
        "frames": [
            "💛 ",
            "💙 ",
            "💜 ",
            "💚 ",
            "❤️ "
        ]
    },
    "clock": {
        "interval": 100,
        "frames": [
            "🕛 ",
            "🕐 ",
            "🕑 ",
            "🕒 ",
            "🕓 ",
            "🕔 ",
            "🕕 ",
            "🕖 ",
            "🕗 ",
            "🕘 ",
            "🕙 ",
            "🕚 "
        ]
    },
    "earth": {
        "interval": 180,
        "frames": [
            "🌍 ",
            "🌎 ",
            "🌏 "
        ]
    },
    "material": {
        "interval": 17,
        "frames": [
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "██████████▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "█████████████▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁██████████████▁▁▁▁",
            "▁▁▁██████████████▁▁▁",
            "▁▁▁▁█████████████▁▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁▁█████████████▁▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁▁███████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁▁█████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁"
        ]
    },
    "moon": {
        "interval": 80,
        "frames": [
            "🌑 ",
            "🌒 ",
            "🌓 ",
            "🌔 ",
            "🌕 ",
            "🌖 ",
            "🌗 ",
            "🌘 "
        ]
    },
    "runner": {
        "interval": 140,
        "frames": [
            "🚶 ",
            "🏃 "
        ]
    },
    "pong": {
        "interval": 80,
        "frames": [
            "▐⠂       ▌",
            "▐⠈       ▌",
            "▐ ⠂      ▌",
            "▐ ⠠      ▌",
            "▐  ⡀     ▌",
            "▐  ⠠     ▌",
            "▐   ⠂    ▌",
            "▐   ⠈    ▌",
            "▐    ⠂   ▌",
            "▐    ⠠   ▌",
            "▐     ⡀  ▌",
            "▐     ⠠  ▌",
            "▐      ⠂ ▌",
            "▐      ⠈ ▌",
            "▐       ⠂▌",
            "▐       ⠠▌",
            "▐       ⡀▌",
            "▐      ⠠ ▌",
            "▐      ⠂ ▌",
            "▐     ⠈  ▌",
            "▐     ⠂  ▌",
            "▐    ⠠   ▌",
            "▐    ⡀   ▌",
            "▐   ⠠    ▌",
            "▐   ⠂    ▌",
            "▐  ⠈     ▌",
            "▐  ⠂     ▌",
            "▐ ⠠      ▌",
            "▐ ⡀      ▌",
            "▐⠠       ▌"
        ]
    },
    "shark": {
        "interval": 120,
        "frames": [
            "▐|\\____________▌",
            "▐_|\\___________▌",
            "▐__|\\__________▌",
            "▐___|\\_________▌",
            "▐____|\\________▌",
            "▐_____|\\_______▌",
            "▐______|\\______▌",
            "▐_______|\\_____▌",
            "▐________|\\____▌",
            "▐_________|\\___▌",
            "▐__________|\\__▌",
            "▐___________|\\_▌",
            "▐____________|\\▌",
            "▐____________/|▌",
            "▐___________/|_▌",
            "▐__________/|__▌",
            "▐_________/|___▌",
            "▐________/|____▌",
            "▐_______/|_____▌",
            "▐______/|______▌",
            "▐_____/|_______▌",
            "▐____/|________▌",
            "▐___/|_________▌",
            "▐__/|__________▌",
            "▐_/|___________▌",
            "▐/|____________▌"
        ]
    },
    "dqpb": {
        "interval": 100,
        "frames": [
            "d",
            "q",
            "p",
            "b"
        ]
    },
    "weather": {
        "interval": 100,
        "frames": [
            "☀️ ",
            "☀️ ",
            "☀️ ",
            "🌤 ",
            "⛅️ ",
            "🌥 ",
            "☁️ ",
            "🌧 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "⛈ ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "☁️ ",
            "🌥 ",
            "⛅️ ",
            "🌤 ",
            "☀️ ",
            "☀️ "
        ]
    },
    "christmas": {
        "interval": 400,
        "frames": [
            "🌲",
            "🎄"
        ]
    },
    "grenade": {
        "interval": 80,
        "frames": [
            "،  ",
            "′  ",
            " ´ ",
            " ‾ ",
            "  ⸌",
            "  ⸊",
            "  |",
            "  ⁎",
            "  ⁕",
            " ෴ ",
            "  ⁓",
            "   ",
            "   ",
            "   "
        ]
    },
    "point": {
        "interval": 125,
        "frames": [
            "∙∙∙",
            "●∙∙",
            "∙●∙",
            "∙∙●",
            "∙∙∙"
        ]
    },
    "layer": {
        "interval": 150,
        "frames": [
            "-",
            "=",
            "≡"
        ]
    },
    "betaWave": {
        "interval": 80,
        "frames": [
            "ρββββββ",
            "βρβββββ",
            "ββρββββ",
            "βββρβββ",
            "ββββρββ",
            "βββββρβ",
            "ββββββρ"
        ]
    },
    "fingerDance": {
        "interval": 160,
        "frames": [
            "🤘 ",
            "🤟 ",
            "🖖 ",
            "✋ ",
            "🤚 ",
            "👆 "
        ]
    },
    "fistBump": {
        "interval": 80,
        "frames": [
            "🤜\u3000\u3000\u3000\u3000🤛 ",
            "🤜\u3000\u3000\u3000\u3000🤛 ",
            "🤜\u3000\u3000\u3000\u3000🤛 ",
            "\u3000🤜\u3000\u3000🤛\u3000 ",
            "\u3000\u3000🤜🤛\u3000\u3000 ",
            "\u3000🤜✨🤛\u3000\u3000 ",
            "🤜\u3000✨\u3000🤛\u3000 "
        ]
    },
    "soccerHeader": {
        "interval": 80,
        "frames": [
            " 🧑⚽️       🧑 ",
            "🧑  ⚽️      🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑      ⚽️  🧑 ",
            "🧑       ⚽️🧑  ",
            "🧑      ⚽️  🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑  ⚽️      🧑 "
        ]
    },
    "mindblown": {
        "interval": 160,
        "frames": [
            "😐 ",
            "😐 ",
            "😮 ",
            "😮 ",
            "😦 ",
            "😦 ",
            "😧 ",
            "😧 ",
            "🤯 ",
            "💥 ",
            "✨ ",
            "\u3000 ",
            "\u3000 ",
            "\u3000 "
        ]
    },
    "speaker": {
        "interval": 160,
        "frames": [
            "🔈 ",
            "🔉 ",
            "🔊 ",
            "🔉 "
        ]
    },
    "orangePulse": {
        "interval": 100,
        "frames": [
            "🔸 ",
            "🔶 ",
            "🟠 ",
            "🟠 ",
            "🔶 "
        ]
    },
    "bluePulse": {
        "interval": 100,
        "frames": [
            "🔹 ",
            "🔷 ",
            "🔵 ",
            "🔵 ",
            "🔷 "
        ]
    },
    "orangeBluePulse": {
        "interval": 100,
        "frames": [
            "🔸 ",
            "🔶 ",
            "🟠 ",
            "🟠 ",
            "🔶 ",
            "🔹 ",
            "🔷 ",
            "🔵 ",
            "🔵 ",
            "🔷 "
        ]
    },
    "timeTravel": {
        "interval": 100,
        "frames": [
            "🕛 ",
            "🕚 ",
            "🕙 ",
            "🕘 ",
            "🕗 ",
            "🕖 ",
            "🕕 ",
            "🕔 ",
            "🕓 ",
            "🕒 ",
            "🕑 ",
            "🕐 "
        ]
    },
    "aesthetic": {
        "interval": 80,
        "frames": [
            "▰▱▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▰",
            "▰▱▱▱▱▱▱"
        ]
    },
    "dwarfFortress": {
        "interval": 80,
        "frames": [
            " ██████£££  ",
            "☺██████£££  ",
            "☺██████£££  ",
            "☺▓█████£££  ",
            "☺▓█████£££  ",
            "☺▒█████£££  ",
            "☺▒█████£££  ",
            "☺░█████£££  ",
            "☺░█████£££  ",
            "☺ █████£££  ",
            " ☺█████£££  ",
            " ☺█████£££  ",
            " ☺▓████£££  ",
            " ☺▓████£££  ",
            " ☺▒████£££  ",
            " ☺▒████£££  ",
            " ☺░████£££  ",
            " ☺░████£££  ",
            " ☺ ████£££  ",
            "  ☺████£££  ",
            "  ☺████£££  ",
            "  ☺▓███£££  ",
            "  ☺▓███£££  ",
            "  ☺▒███£££  ",
            "  ☺▒███£££  ",
            "  ☺░███£££  ",
            "  ☺░███£££  ",
            "  ☺ ███£££  ",
            "   ☺███£££  ",
            "   ☺███£££  ",
            "   ☺▓██£££  ",
            "   ☺▓██£££  ",
            "   ☺▒██£££  ",
            "   ☺▒██£££  ",
            "   ☺░██£££  ",
            "   ☺░██£££  ",
            "   ☺ ██£££  ",
            "    ☺██£££  ",
            "    ☺██£££  ",
            "    ☺▓█£££  ",
            "    ☺▓█£££  ",
            "    ☺▒█£££  ",
            "    ☺▒█£££  ",
            "    ☺░█£££  ",
            "    ☺░█£££  ",
            "    ☺ █£££  ",
            "     ☺█£££  ",
            "     ☺█£££  ",
            "     ☺▓£££  ",
            "     ☺▓£££  ",
            "     ☺▒£££  ",
            "     ☺▒£££  ",
            "     ☺░£££  ",
            "     ☺░£££  ",
            "     ☺ £££  ",
            "      ☺£££  ",
            "      ☺£££  ",
            "      ☺▓££  ",
            "      ☺▓££  ",
            "      ☺▒££  ",
            "      ☺▒££  ",
            "      ☺░££  ",
            "      ☺░££  ",
            "      ☺ ££  ",
            "       ☺££  ",
            "       ☺££  ",
            "       ☺▓£  ",
            "       ☺▓£  ",
            "       ☺▒£  ",
            "       ☺▒£  ",
            "       ☺░£  ",
            "       ☺░£  ",
            "       ☺ £  ",
            "        ☺£  ",
            "        ☺£  ",
            "        ☺▓  ",
            "        ☺▓  ",
            "        ☺▒  ",
            "        ☺▒  ",
            "        ☺░  ",
            "        ☺░  ",
            "        ☺   ",
            "        ☺  &",
            "        ☺ ☼&",
            "       ☺ ☼ &",
            "       ☺☼  &",
            "      ☺☼  & ",
            "      ‼   & ",
            "     ☺   &  ",
            "    ‼    &  ",
            "   ☺    &   ",
            "  ‼     &   ",
            " ☺     &    ",
            "‼      &    ",
            "      &     ",
            "      &     ",
            "     &   ░  ",
            "     &   ▒  ",
            "    &    ▓  ",
            "    &    £  ",
            "   &    ░£  ",
            "   &    ▒£  ",
            "  &     ▓£  ",
            "  &     ££  ",
            " &     ░££  ",
            " &     ▒££  ",
            "&      ▓££  ",
            "&      £££  ",
            "      ░£££  ",
            "      ▒£££  ",
            "      ▓£££  ",
            "      █£££  ",
            "     ░█£££  ",
            "     ▒█£££  ",
            "     ▓█£££  ",
            "     ██£££  ",
            "    ░██£££  ",
            "    ▒██£££  ",
            "    ▓██£££  ",
            "    ███£££  ",
            "   ░███£££  ",
            "   ▒███£££  ",
            "   ▓███£££  ",
            "   ████£££  ",
            "  ░████£££  ",
            "  ▒████£££  ",
            "  ▓████£££  ",
            "  █████£££  ",
            " ░█████£££  ",
            " ▒█████£££  ",
            " ▓█████£££  ",
            " ██████£££  ",
            " ██████£££  "
        ]
    }
}"#;
