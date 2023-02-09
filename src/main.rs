// #![allow(dead_code)]
// #![allow(unused)]
// mod hash;
#![allow(unused_mut)]
mod big_int;
mod curve;
mod ec_pair;
mod hd_node;
mod hmac;
mod pbkdf2;
mod point;
mod sha512hash;

mod hmac32;
mod sha512hash32;

use hd_node::HDNode;
use profile::profile;
use sha256hash::Sha256Hash;
use tinyrand::{Rand, Seeded, StdRand};

use crate::bech32::{encode, to_words};
mod bech32;
mod profile_local;
mod rmd160hash;
mod sha256hash;
pub static SECP256K1: once_cell::sync::Lazy<curve::Curve> =
    once_cell::sync::Lazy::new(|| curve::Curve::new());
use colored::Colorize;
use std::{
    fmt::format,
    fs::{File, OpenOptions},
    hint::black_box,
    io::{BufWriter, Write},
    num::NonZeroU64,
    thread::{self, ThreadId},
};
#[allow(dead_code)]
pub static IS_PROFILING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[allow(dead_code)]
pub static IS_PROFILE_RECONCILING: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
#[allow(dead_code)]
pub static PROFILING_DEPTH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[allow(dead_code)]
pub static PROFILING_PATH: std::sync::atomic::AtomicPtr<Vec<String>> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
#[allow(dead_code)]

// [derive(Debug, Clone)]
pub struct Profile {
    pub count: u128,
    pub fn_name: String,
    pub speed: u128,
    pub depth: usize,
}

// A Map of all the profiled functions and their times
pub static PROFILING_MAP: std::sync::atomic::AtomicPtr<std::collections::HashMap<String, Profile>> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

#[profile()]
fn utf8_string_to_bytes(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for c in s.chars() {
        let mut buf = [0; 4];
        let s = c.encode_utf8(&mut buf);
        for b in s.bytes() {
            result.push(b);
        }
    }
    result
}

#[profile()]
fn to_seed(mnemonic: &str) -> Vec<u8> {
    let mnemonic_bits = utf8_string_to_bytes(mnemonic);
    let mnemonic_bits_u32 = mnemonic_bits
        .chunks(4)
        .map(|chunk| {
            let mut result = 0;
            for (i, byte) in chunk.iter().enumerate() {
                result |= (*byte as u32) << (8 * (3 - i));
            }
            result
        })
        .collect::<Vec<u32>>();
    let passphrase_bits = utf8_string_to_bytes("mnemonic");
    let passphrase_bits_u32 = passphrase_bits
        .chunks(4)
        .map(|chunk| {
            let mut result = 0;
            for (i, byte) in chunk.iter().enumerate() {
                result |= (*byte as u32) << (8 * (3 - i));
            }
            result
        })
        .collect::<Vec<u32>>();
    let output_u32 = pbkdf2::pbkdf2_32(mnemonic_bits_u32, passphrase_bits_u32, 2048, 512);
    let mut output = Vec::new();
    for word in output_u32 {
        for i in (0..4).rev() {
            output.push(((word >> (8 * i)) & 0xff) as u8);
        }
    }
    output
}
#[profile(no_sub)]
fn calc_bip32_extended_key(bip32_root_key: HDNode) -> HDNode {
    let mut extended_key = bip32_root_key;

    for i in [0, 0] {
        extended_key = extended_key.derive(i);
    }

    return extended_key;
}
#[profile()]
fn cosmos_buffer_to_address(pub_buf: Vec<u8>) -> String {
    let mut sha = Sha256Hash::new();
    sha.update(&pub_buf);
    let temp = sha.digest();

    let mut ripemd160 = rmd160hash::Rrd160Hash::new();
    ripemd160.update(&temp);
    let ripemd160 = ripemd160.digest();

    encode("cosmos", &to_words(&ripemd160))
}

#[profile()]
fn from_mnemonic_to_address(mnemonic: &str) -> String {
    let seed = to_seed(mnemonic);
    let hd = hd_node::HDNode::from_seed_buffer(&seed); //, "ed25519"
    let extended_key = calc_bip32_extended_key(hd);
    let pub_key = extended_key.key_pair.get_public_key_buffer();
    cosmos_buffer_to_address(pub_key)
}

fn from_mnemonic_to_address_init(mnemonic: &str) -> String {
    let seed = to_seed(mnemonic);
    let hd = hd_node::HDNode::from_seed_buffer(&seed); //, "ed25519"
    let extended_key = calc_bip32_extended_key(hd);
    let pub_key = extended_key.key_pair.get_public_key_buffer();
    cosmos_buffer_to_address(pub_key)
}

fn profile_main() {
    IS_PROFILING.store(true, std::sync::atomic::Ordering::Relaxed);
    let address = from_mnemonic_to_address_init("surround miss nominee dream gap cross assault thank captain prosper drop duty group candy wealth weather scale put");
    if address != "cosmos19x6j6a99rpfjkgchakhclqpghxavq8c2dgdqvw" {
        panic!("Wrong address");
    }
    IS_PROFILING.store(false, std::sync::atomic::Ordering::Relaxed);

    let address = from_mnemonic_to_address("surround miss nominee dream gap cross assault thank captain prosper drop duty group candy wealth weather scale put");
    println!("{}", address);
    if address != "cosmos19x6j6a99rpfjkgchakhclqpghxavq8c2dgdqvw" {
        panic!("Wrong address");
    }

    IS_PROFILE_RECONCILING.store(true, std::sync::atomic::Ordering::Relaxed);
    from_mnemonic_to_address("surround miss nominee dream gap cross assault thank captain prosper drop duty group candy wealth weather scale put");
    IS_PROFILE_RECONCILING.store(false, std::sync::atomic::Ordering::Relaxed);
}

const WORDS: [&str; 2048] = [
    "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd",
    "abuse", "access", "accident", "account", "accuse", "achieve", "acid", "acoustic", "acquire",
    "across", "act", "action", "actor", "actress", "actual", "adapt", "add", "addict", "address",
    "adjust", "admit", "adult", "advance", "advice", "aerobic", "affair", "afford", "afraid",
    "again", "age", "agent", "agree", "ahead", "aim", "air", "airport", "aisle", "alarm", "album",
    "alcohol", "alert", "alien", "all", "alley", "allow", "almost", "alone", "alpha", "already",
    "also", "alter", "always", "amateur", "amazing", "among", "amount", "amused", "analyst",
    "anchor", "ancient", "anger", "angle", "angry", "animal", "ankle", "announce", "annual",
    "another", "answer", "antenna", "antique", "anxiety", "any", "apart", "apology", "appear",
    "apple", "approve", "april", "arch", "arctic", "area", "arena", "argue", "arm", "armed",
    "armor", "army", "around", "arrange", "arrest", "arrive", "arrow", "art", "artefact", "artist",
    "artwork", "ask", "aspect", "assault", "asset", "assist", "assume", "asthma", "athlete",
    "atom", "attack", "attend", "attitude", "attract", "auction", "audit", "august", "aunt",
    "author", "auto", "autumn", "average", "avocado", "avoid", "awake", "aware", "away", "awesome",
    "awful", "awkward", "axis", "baby", "bachelor", "bacon", "badge", "bag", "balance", "balcony",
    "ball", "bamboo", "banana", "banner", "bar", "barely", "bargain", "barrel", "base", "basic",
    "basket", "battle", "beach", "bean", "beauty", "because", "become", "beef", "before", "begin",
    "behave", "behind", "believe", "below", "belt", "bench", "benefit", "best", "betray", "better",
    "between", "beyond", "bicycle", "bid", "bike", "bind", "biology", "bird", "birth", "bitter",
    "black", "blade", "blame", "blanket", "blast", "bleak", "bless", "blind", "blood", "blossom",
    "blouse", "blue", "blur", "blush", "board", "boat", "body", "boil", "bomb", "bone", "bonus",
    "book", "boost", "border", "boring", "borrow", "boss", "bottom", "bounce", "box", "boy",
    "bracket", "brain", "brand", "brass", "brave", "bread", "breeze", "brick", "bridge", "brief",
    "bright", "bring", "brisk", "broccoli", "broken", "bronze", "broom", "brother", "brown",
    "brush", "bubble", "buddy", "budget", "buffalo", "build", "bulb", "bulk", "bullet", "bundle",
    "bunker", "burden", "burger", "burst", "bus", "business", "busy", "butter", "buyer", "buzz",
    "cabbage", "cabin", "cable", "cactus", "cage", "cake", "call", "calm", "camera", "camp", "can",
    "canal", "cancel", "candy", "cannon", "canoe", "canvas", "canyon", "capable", "capital",
    "captain", "car", "carbon", "card", "cargo", "carpet", "carry", "cart", "case", "cash",
    "casino", "castle", "casual", "cat", "catalog", "catch", "category", "cattle", "caught",
    "cause", "caution", "cave", "ceiling", "celery", "cement", "census", "century", "cereal",
    "certain", "chair", "chalk", "champion", "change", "chaos", "chapter", "charge", "chase",
    "chat", "cheap", "check", "cheese", "chef", "cherry", "chest", "chicken", "chief", "child",
    "chimney", "choice", "choose", "chronic", "chuckle", "chunk", "churn", "cigar", "cinnamon",
    "circle", "citizen", "city", "civil", "claim", "clap", "clarify", "claw", "clay", "clean",
    "clerk", "clever", "click", "client", "cliff", "climb", "clinic", "clip", "clock", "clog",
    "close", "cloth", "cloud", "clown", "club", "clump", "cluster", "clutch", "coach", "coast",
    "coconut", "code", "coffee", "coil", "coin", "collect", "color", "column", "combine", "come",
    "comfort", "comic", "common", "company", "concert", "conduct", "confirm", "congress",
    "connect", "consider", "control", "convince", "cook", "cool", "copper", "copy", "coral",
    "core", "corn", "correct", "cost", "cotton", "couch", "country", "couple", "course", "cousin",
    "cover", "coyote", "crack", "cradle", "craft", "cram", "crane", "crash", "crater", "crawl",
    "crazy", "cream", "credit", "creek", "crew", "cricket", "crime", "crisp", "critic", "crop",
    "cross", "crouch", "crowd", "crucial", "cruel", "cruise", "crumble", "crunch", "crush", "cry",
    "crystal", "cube", "culture", "cup", "cupboard", "curious", "current", "curtain", "curve",
    "cushion", "custom", "cute", "cycle", "dad", "damage", "damp", "dance", "danger", "daring",
    "dash", "daughter", "dawn", "day", "deal", "debate", "debris", "decade", "december", "decide",
    "decline", "decorate", "decrease", "deer", "defense", "define", "defy", "degree", "delay",
    "deliver", "demand", "demise", "denial", "dentist", "deny", "depart", "depend", "deposit",
    "depth", "deputy", "derive", "describe", "desert", "design", "desk", "despair", "destroy",
    "detail", "detect", "develop", "device", "devote", "diagram", "dial", "diamond", "diary",
    "dice", "diesel", "diet", "differ", "digital", "dignity", "dilemma", "dinner", "dinosaur",
    "direct", "dirt", "disagree", "discover", "disease", "dish", "dismiss", "disorder", "display",
    "distance", "divert", "divide", "divorce", "dizzy", "doctor", "document", "dog", "doll",
    "dolphin", "domain", "donate", "donkey", "donor", "door", "dose", "double", "dove", "draft",
    "dragon", "drama", "drastic", "draw", "dream", "dress", "drift", "drill", "drink", "drip",
    "drive", "drop", "drum", "dry", "duck", "dumb", "dune", "during", "dust", "dutch", "duty",
    "dwarf", "dynamic", "eager", "eagle", "early", "earn", "earth", "easily", "east", "easy",
    "echo", "ecology", "economy", "edge", "edit", "educate", "effort", "egg", "eight", "either",
    "elbow", "elder", "electric", "elegant", "element", "elephant", "elevator", "elite", "else",
    "embark", "embody", "embrace", "emerge", "emotion", "employ", "empower", "empty", "enable",
    "enact", "end", "endless", "endorse", "enemy", "energy", "enforce", "engage", "engine",
    "enhance", "enjoy", "enlist", "enough", "enrich", "enroll", "ensure", "enter", "entire",
    "entry", "envelope", "episode", "equal", "equip", "era", "erase", "erode", "erosion", "error",
    "erupt", "escape", "essay", "essence", "estate", "eternal", "ethics", "evidence", "evil",
    "evoke", "evolve", "exact", "example", "excess", "exchange", "excite", "exclude", "excuse",
    "execute", "exercise", "exhaust", "exhibit", "exile", "exist", "exit", "exotic", "expand",
    "expect", "expire", "explain", "expose", "express", "extend", "extra", "eye", "eyebrow",
    "fabric", "face", "faculty", "fade", "faint", "faith", "fall", "false", "fame", "family",
    "famous", "fan", "fancy", "fantasy", "farm", "fashion", "fat", "fatal", "father", "fatigue",
    "fault", "favorite", "feature", "february", "federal", "fee", "feed", "feel", "female",
    "fence", "festival", "fetch", "fever", "few", "fiber", "fiction", "field", "figure", "file",
    "film", "filter", "final", "find", "fine", "finger", "finish", "fire", "firm", "first",
    "fiscal", "fish", "fit", "fitness", "fix", "flag", "flame", "flash", "flat", "flavor", "flee",
    "flight", "flip", "float", "flock", "floor", "flower", "fluid", "flush", "fly", "foam",
    "focus", "fog", "foil", "fold", "follow", "food", "foot", "force", "forest", "forget", "fork",
    "fortune", "forum", "forward", "fossil", "foster", "found", "fox", "fragile", "frame",
    "frequent", "fresh", "friend", "fringe", "frog", "front", "frost", "frown", "frozen", "fruit",
    "fuel", "fun", "funny", "furnace", "fury", "future", "gadget", "gain", "galaxy", "gallery",
    "game", "gap", "garage", "garbage", "garden", "garlic", "garment", "gas", "gasp", "gate",
    "gather", "gauge", "gaze", "general", "genius", "genre", "gentle", "genuine", "gesture",
    "ghost", "giant", "gift", "giggle", "ginger", "giraffe", "girl", "give", "glad", "glance",
    "glare", "glass", "glide", "glimpse", "globe", "gloom", "glory", "glove", "glow", "glue",
    "goat", "goddess", "gold", "good", "goose", "gorilla", "gospel", "gossip", "govern", "gown",
    "grab", "grace", "grain", "grant", "grape", "grass", "gravity", "great", "green", "grid",
    "grief", "grit", "grocery", "group", "grow", "grunt", "guard", "guess", "guide", "guilt",
    "guitar", "gun", "gym", "habit", "hair", "half", "hammer", "hamster", "hand", "happy",
    "harbor", "hard", "harsh", "harvest", "hat", "have", "hawk", "hazard", "head", "health",
    "heart", "heavy", "hedgehog", "height", "hello", "helmet", "help", "hen", "hero", "hidden",
    "high", "hill", "hint", "hip", "hire", "history", "hobby", "hockey", "hold", "hole", "holiday",
    "hollow", "home", "honey", "hood", "hope", "horn", "horror", "horse", "hospital", "host",
    "hotel", "hour", "hover", "hub", "huge", "human", "humble", "humor", "hundred", "hungry",
    "hunt", "hurdle", "hurry", "hurt", "husband", "hybrid", "ice", "icon", "idea", "identify",
    "idle", "ignore", "ill", "illegal", "illness", "image", "imitate", "immense", "immune",
    "impact", "impose", "improve", "impulse", "inch", "include", "income", "increase", "index",
    "indicate", "indoor", "industry", "infant", "inflict", "inform", "inhale", "inherit",
    "initial", "inject", "injury", "inmate", "inner", "innocent", "input", "inquiry", "insane",
    "insect", "inside", "inspire", "install", "intact", "interest", "into", "invest", "invite",
    "involve", "iron", "island", "isolate", "issue", "item", "ivory", "jacket", "jaguar", "jar",
    "jazz", "jealous", "jeans", "jelly", "jewel", "job", "join", "joke", "journey", "joy", "judge",
    "juice", "jump", "jungle", "junior", "junk", "just", "kangaroo", "keen", "keep", "ketchup",
    "key", "kick", "kid", "kidney", "kind", "kingdom", "kiss", "kit", "kitchen", "kite", "kitten",
    "kiwi", "knee", "knife", "knock", "know", "lab", "label", "labor", "ladder", "lady", "lake",
    "lamp", "language", "laptop", "large", "later", "latin", "laugh", "laundry", "lava", "law",
    "lawn", "lawsuit", "layer", "lazy", "leader", "leaf", "learn", "leave", "lecture", "left",
    "leg", "legal", "legend", "leisure", "lemon", "lend", "length", "lens", "leopard", "lesson",
    "letter", "level", "liar", "liberty", "library", "license", "life", "lift", "light", "like",
    "limb", "limit", "link", "lion", "liquid", "list", "little", "live", "lizard", "load", "loan",
    "lobster", "local", "lock", "logic", "lonely", "long", "loop", "lottery", "loud", "lounge",
    "love", "loyal", "lucky", "luggage", "lumber", "lunar", "lunch", "luxury", "lyrics", "machine",
    "mad", "magic", "magnet", "maid", "mail", "main", "major", "make", "mammal", "man", "manage",
    "mandate", "mango", "mansion", "manual", "maple", "marble", "march", "margin", "marine",
    "market", "marriage", "mask", "mass", "master", "match", "material", "math", "matrix",
    "matter", "maximum", "maze", "meadow", "mean", "measure", "meat", "mechanic", "medal", "media",
    "melody", "melt", "member", "memory", "mention", "menu", "mercy", "merge", "merit", "merry",
    "mesh", "message", "metal", "method", "middle", "midnight", "milk", "million", "mimic", "mind",
    "minimum", "minor", "minute", "miracle", "mirror", "misery", "miss", "mistake", "mix", "mixed",
    "mixture", "mobile", "model", "modify", "mom", "moment", "monitor", "monkey", "monster",
    "month", "moon", "moral", "more", "morning", "mosquito", "mother", "motion", "motor",
    "mountain", "mouse", "move", "movie", "much", "muffin", "mule", "multiply", "muscle", "museum",
    "mushroom", "music", "must", "mutual", "myself", "mystery", "myth", "naive", "name", "napkin",
    "narrow", "nasty", "nation", "nature", "near", "neck", "need", "negative", "neglect",
    "neither", "nephew", "nerve", "nest", "net", "network", "neutral", "never", "news", "next",
    "nice", "night", "noble", "noise", "nominee", "noodle", "normal", "north", "nose", "notable",
    "note", "nothing", "notice", "novel", "now", "nuclear", "number", "nurse", "nut", "oak",
    "obey", "object", "oblige", "obscure", "observe", "obtain", "obvious", "occur", "ocean",
    "october", "odor", "off", "offer", "office", "often", "oil", "okay", "old", "olive", "olympic",
    "omit", "once", "one", "onion", "online", "only", "open", "opera", "opinion", "oppose",
    "option", "orange", "orbit", "orchard", "order", "ordinary", "organ", "orient", "original",
    "orphan", "ostrich", "other", "outdoor", "outer", "output", "outside", "oval", "oven", "over",
    "own", "owner", "oxygen", "oyster", "ozone", "pact", "paddle", "page", "pair", "palace",
    "palm", "panda", "panel", "panic", "panther", "paper", "parade", "parent", "park", "parrot",
    "party", "pass", "patch", "path", "patient", "patrol", "pattern", "pause", "pave", "payment",
    "peace", "peanut", "pear", "peasant", "pelican", "pen", "penalty", "pencil", "people",
    "pepper", "perfect", "permit", "person", "pet", "phone", "photo", "phrase", "physical",
    "piano", "picnic", "picture", "piece", "pig", "pigeon", "pill", "pilot", "pink", "pioneer",
    "pipe", "pistol", "pitch", "pizza", "place", "planet", "plastic", "plate", "play", "please",
    "pledge", "pluck", "plug", "plunge", "poem", "poet", "point", "polar", "pole", "police",
    "pond", "pony", "pool", "popular", "portion", "position", "possible", "post", "potato",
    "pottery", "poverty", "powder", "power", "practice", "praise", "predict", "prefer", "prepare",
    "present", "pretty", "prevent", "price", "pride", "primary", "print", "priority", "prison",
    "private", "prize", "problem", "process", "produce", "profit", "program", "project", "promote",
    "proof", "property", "prosper", "protect", "proud", "provide", "public", "pudding", "pull",
    "pulp", "pulse", "pumpkin", "punch", "pupil", "puppy", "purchase", "purity", "purpose",
    "purse", "push", "put", "puzzle", "pyramid", "quality", "quantum", "quarter", "question",
    "quick", "quit", "quiz", "quote", "rabbit", "raccoon", "race", "rack", "radar", "radio",
    "rail", "rain", "raise", "rally", "ramp", "ranch", "random", "range", "rapid", "rare", "rate",
    "rather", "raven", "raw", "razor", "ready", "real", "reason", "rebel", "rebuild", "recall",
    "receive", "recipe", "record", "recycle", "reduce", "reflect", "reform", "refuse", "region",
    "regret", "regular", "reject", "relax", "release", "relief", "rely", "remain", "remember",
    "remind", "remove", "render", "renew", "rent", "reopen", "repair", "repeat", "replace",
    "report", "require", "rescue", "resemble", "resist", "resource", "response", "result",
    "retire", "retreat", "return", "reunion", "reveal", "review", "reward", "rhythm", "rib",
    "ribbon", "rice", "rich", "ride", "ridge", "rifle", "right", "rigid", "ring", "riot", "ripple",
    "risk", "ritual", "rival", "river", "road", "roast", "robot", "robust", "rocket", "romance",
    "roof", "rookie", "room", "rose", "rotate", "rough", "round", "route", "royal", "rubber",
    "rude", "rug", "rule", "run", "runway", "rural", "sad", "saddle", "sadness", "safe", "sail",
    "salad", "salmon", "salon", "salt", "salute", "same", "sample", "sand", "satisfy", "satoshi",
    "sauce", "sausage", "save", "say", "scale", "scan", "scare", "scatter", "scene", "scheme",
    "school", "science", "scissors", "scorpion", "scout", "scrap", "screen", "script", "scrub",
    "sea", "search", "season", "seat", "second", "secret", "section", "security", "seed", "seek",
    "segment", "select", "sell", "seminar", "senior", "sense", "sentence", "series", "service",
    "session", "settle", "setup", "seven", "shadow", "shaft", "shallow", "share", "shed", "shell",
    "sheriff", "shield", "shift", "shine", "ship", "shiver", "shock", "shoe", "shoot", "shop",
    "short", "shoulder", "shove", "shrimp", "shrug", "shuffle", "shy", "sibling", "sick", "side",
    "siege", "sight", "sign", "silent", "silk", "silly", "silver", "similar", "simple", "since",
    "sing", "siren", "sister", "situate", "six", "size", "skate", "sketch", "ski", "skill", "skin",
    "skirt", "skull", "slab", "slam", "sleep", "slender", "slice", "slide", "slight", "slim",
    "slogan", "slot", "slow", "slush", "small", "smart", "smile", "smoke", "smooth", "snack",
    "snake", "snap", "sniff", "snow", "soap", "soccer", "social", "sock", "soda", "soft", "solar",
    "soldier", "solid", "solution", "solve", "someone", "song", "soon", "sorry", "sort", "soul",
    "sound", "soup", "source", "south", "space", "spare", "spatial", "spawn", "speak", "special",
    "speed", "spell", "spend", "sphere", "spice", "spider", "spike", "spin", "spirit", "split",
    "spoil", "sponsor", "spoon", "sport", "spot", "spray", "spread", "spring", "spy", "square",
    "squeeze", "squirrel", "stable", "stadium", "staff", "stage", "stairs", "stamp", "stand",
    "start", "state", "stay", "steak", "steel", "stem", "step", "stereo", "stick", "still",
    "sting", "stock", "stomach", "stone", "stool", "story", "stove", "strategy", "street",
    "strike", "strong", "struggle", "student", "stuff", "stumble", "style", "subject", "submit",
    "subway", "success", "such", "sudden", "suffer", "sugar", "suggest", "suit", "summer", "sun",
    "sunny", "sunset", "super", "supply", "supreme", "sure", "surface", "surge", "surprise",
    "surround", "survey", "suspect", "sustain", "swallow", "swamp", "swap", "swarm", "swear",
    "sweet", "swift", "swim", "swing", "switch", "sword", "symbol", "symptom", "syrup", "system",
    "table", "tackle", "tag", "tail", "talent", "talk", "tank", "tape", "target", "task", "taste",
    "tattoo", "taxi", "teach", "team", "tell", "ten", "tenant", "tennis", "tent", "term", "test",
    "text", "thank", "that", "theme", "then", "theory", "there", "they", "thing", "this",
    "thought", "three", "thrive", "throw", "thumb", "thunder", "ticket", "tide", "tiger", "tilt",
    "timber", "time", "tiny", "tip", "tired", "tissue", "title", "toast", "tobacco", "today",
    "toddler", "toe", "together", "toilet", "token", "tomato", "tomorrow", "tone", "tongue",
    "tonight", "tool", "tooth", "top", "topic", "topple", "torch", "tornado", "tortoise", "toss",
    "total", "tourist", "toward", "tower", "town", "toy", "track", "trade", "traffic", "tragic",
    "train", "transfer", "trap", "trash", "travel", "tray", "treat", "tree", "trend", "trial",
    "tribe", "trick", "trigger", "trim", "trip", "trophy", "trouble", "truck", "true", "truly",
    "trumpet", "trust", "truth", "try", "tube", "tuition", "tumble", "tuna", "tunnel", "turkey",
    "turn", "turtle", "twelve", "twenty", "twice", "twin", "twist", "two", "type", "typical",
    "ugly", "umbrella", "unable", "unaware", "uncle", "uncover", "under", "undo", "unfair",
    "unfold", "unhappy", "uniform", "unique", "unit", "universe", "unknown", "unlock", "until",
    "unusual", "unveil", "update", "upgrade", "uphold", "upon", "upper", "upset", "urban", "urge",
    "usage", "use", "used", "useful", "useless", "usual", "utility", "vacant", "vacuum", "vague",
    "valid", "valley", "valve", "van", "vanish", "vapor", "various", "vast", "vault", "vehicle",
    "velvet", "vendor", "venture", "venue", "verb", "verify", "version", "very", "vessel",
    "veteran", "viable", "vibrant", "vicious", "victory", "video", "view", "village", "vintage",
    "violin", "virtual", "virus", "visa", "visit", "visual", "vital", "vivid", "vocal", "voice",
    "void", "volcano", "volume", "vote", "voyage", "wage", "wagon", "wait", "walk", "wall",
    "walnut", "want", "warfare", "warm", "warrior", "wash", "wasp", "waste", "water", "wave",
    "way", "wealth", "weapon", "wear", "weasel", "weather", "web", "wedding", "weekend", "weird",
    "welcome", "west", "wet", "whale", "what", "wheat", "wheel", "when", "where", "whip",
    "whisper", "wide", "width", "wife", "wild", "will", "win", "window", "wine", "wing", "wink",
    "winner", "winter", "wire", "wisdom", "wise", "wish", "witness", "wolf", "woman", "wonder",
    "wood", "wool", "word", "work", "world", "worry", "worth", "wrap", "wreck", "wrestle", "wrist",
    "write", "wrong", "yard", "year", "yellow", "you", "young", "youth", "zebra", "zero", "zone",
    "zoo",
];

const TARGET_STARTERS: [&str; 4] = ["test", "tskr", "tasker", "alex"];

const TARGET_ENDERS: [&str; 1] = ["wurts"];

// #[profile()]
fn main() {
    let mut handles = vec![];
    for _ in 0..16 {
        // Spawn a thread and pass it the thread id
        handles.push(std::thread::spawn(move || {
            brute_force(thread::current().id());
        }));
    }

    // If a thread panics just start a new one
    loop {
        let mut new_handles = vec![];
        for handle in handles.into_iter() {
            if handle.is_finished() {
                let thread_id = handle.thread().id();
                println!("Thread {:?} finished", thread_id);
                match handle.join() {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Thread {:?} panicked", thread_id);
                    }
                }
                new_handles.push(std::thread::spawn(move || {
                    brute_force(thread_id);
                }));
            } else {
                new_handles.push(handle);
            }
        }
        handles = new_handles;
        // Sleep for a bit to avoid spinning
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn brute_force(thread_id: ThreadId) {
    loop {
        let mut i = 0;
        let mut rng = StdRand::seed(
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        );
        // Open a file to stream the results into. If it exists already, append to it
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("thread_{:?}.txt", thread_id))
            .expect("Unable to create file");

        let mut stream = BufWriter::new(file);

        loop {
            let mut twelve_random_words = generate_mnemonic(
                [
                    rng.next_u32(),
                    rng.next_u32(),
                    rng.next_u32(),
                    rng.next_u32(),
                ]
                .as_ref(),
            );

            match stream.write_all(format!("{:<100}", twelve_random_words).as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "Error writing mnemonic to file. thread {:?}, {}",
                        thread_id, e
                    );
                }
            }

            println!("{} : {}", i, twelve_random_words);

            match std::panic::catch_unwind(|| {
                from_mnemonic_to_address(twelve_random_words.as_str())
            }) {
                Ok(address) => {
                    match stream.write_all(format!(" : {}\n", address).as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!(
                                "Error writing address to file. thread {:?} [{}]",
                                thread_id, e
                            );
                        }
                    }

                    let address_after = &address[7..];

                    // If the address starts with one of the target starters, print the mnemonic and store the result into a file titled the address
                    for target_starter in TARGET_STARTERS.iter() {
                        if address_after.starts_with(*target_starter) {
                            println!("{} : {}", address, twelve_random_words);
                            let mut file = File::create(&address).unwrap();
                            file.write_all(twelve_random_words.as_bytes()).unwrap();
                            break;
                        }
                    }

                    for target_ender in TARGET_ENDERS.iter() {
                        if address_after.ends_with(*target_ender) {
                            println!("{} : {}", address, twelve_random_words);
                            let mut file = File::create(&address).unwrap();
                            file.write_all(twelve_random_words.as_bytes()).unwrap();
                            break;
                        }
                    }

                    if i % 1000 == 0 {
                        // Dump the current state to a file
                        match stream.flush() {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error flushing file. thread {:?}, {}", thread_id, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Thread {:?} panicked: {:?}", thread_id, e);

                    match stream.write_all(format!(" PANIC:{:?} {:?}\n", thread_id, e).as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!(
                                "Error writing panic to stream. thread {:?} [{}]",
                                thread_id, e
                            );
                        }
                    }

                    // Dump the current state to a file
                    match stream.flush() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error flushing file. thread {:?}, {}", thread_id, e);
                        }
                    }
                }
            }

            i += 1;
        }
    }
}

fn generate_mnemonic(data: &[u32]) -> String {
    //h = hashlib.sha256(data).hexdigest()

    let mut sha = sha256hash::Sha256Hash::new();

    let mut u8Array: Vec<u8> = Vec::new();
    for i in data {
        // Data is u32, but we need u8 so split it into 4 u8s
        u8Array.push(((i >> 24) & 0xFF) as u8);
        u8Array.push((i >> 16 & 0xFF) as u8);
        u8Array.push((i >> 8 & 0xFF) as u8);
        u8Array.push((i & 0xFF) as u8);
    }

    sha.update(&u8Array);
    let hash = sha.digest();

    let mut hash_hex = String::new();
    for i in 0..16 {
        hash_hex.push_str(format!("{:02x}", hash[i]).as_str());
    }

    // let a = byteArrayToBinaryString(data);
    let mut origional_data = String::new();
    for i in data {
        // i is u32
        origional_data.push_str(format!("{:032b}", i).as_str());
    }

    let mut hash_bits = String::new();
    for i in 0..32 {
        hash_bits.push_str(format!("{:08b}", hash_hex.as_bytes()[i]).as_str());
    }

    let mut checksum = format!("{:08b}", hash[0]);

    let mut total = format!("{}{}", origional_data, &checksum);

    let mut result: Vec<String> = Vec::new();

    for i in 0..12 {
        let part1 = i * 11;
        let part2 = (i + 1) * 11;
        let slice = &total[part1..part2];
        let idx = u32::from_str_radix(slice, 2).unwrap();

        result.push(WORDS[idx as usize].to_string());
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_seed_test() {
        let mnemonic = "surround miss nominee dream gap cross assault thank captain prosper drop duty group candy wealth weather scale put";
        let result = to_seed(mnemonic);
        assert_eq!(result, vec![]);
    }
}
