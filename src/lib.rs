use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_MAX_LENGTH: usize = 16;
pub const DEFAULT_ALPHABET: &str = "23456789BCDFGHJKMNPQRSTVWXYZ";

pub const DEFAULT_BANNED_SUBSTRINGS: &[&str] = &[
    "ANAL", "ARSE", "ASS", "BASTARD", "BLOW", "BULL", "CRAP", "CUNT", "DAMN", "DICK", "FUCK",
    "HELL", "JERK", "NIG", "PISS", "PORN", "PRICK", "PUSSY", "SHIT", "SLUT", "TWAT", "WANK",
];

#[derive(Clone, Debug)]
pub struct IdConfig {
    pub max_length: usize,
    pub alphabet: String,
    pub banned_substrings: Vec<String>,
}

impl Default for IdConfig {
    fn default() -> Self {
        Self {
            max_length: DEFAULT_MAX_LENGTH,
            alphabet: DEFAULT_ALPHABET.to_string(),
            banned_substrings: DEFAULT_BANNED_SUBSTRINGS
                .iter()
                .map(|item| item.to_string())
                .collect(),
        }
    }
}

impl IdConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.max_length == 0 {
            return Err("max_length must be positive".to_string());
        }
        let mut chars: Vec<char> = self.alphabet.chars().collect();
        chars.sort_unstable();
        chars.dedup();
        if chars.len() != self.alphabet.chars().count() {
            return Err("alphabet must not contain duplicates".to_string());
        }
        if !self
            .alphabet
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
        {
            return Err("alphabet must be alphanumeric uppercase characters only".to_string());
        }
        Ok(())
    }
}

pub struct IdSystem {
    config: IdConfig,
    rng: XorShift64,
}

impl IdSystem {
    pub fn new(config: IdConfig) -> Result<Self, String> {
        config.validate()?;
        Ok(Self {
            config,
            rng: XorShift64::from_system_time(),
        })
    }

    pub fn generate(&self, length: usize) -> Result<String, String> {
        if length == 0 || length > self.config.max_length {
            return Err(format!(
                "length must be 1..{}",
                self.config.max_length
            ));
        }

        let alphabet: Vec<char> = self.config.alphabet.chars().collect();
        let mut rng = self.rng.clone();

        loop {
            let candidate: String = (0..length)
                .map(|_| alphabet[rng.next_usize(alphabet.len())])
                .collect();
            if self.is_allowed(&candidate) {
                return Ok(candidate);
            }
        }
    }

    pub fn is_allowed(&self, candidate: &str) -> bool {
        let normalized = self.normalize(candidate);
        if normalized.is_empty() || normalized.len() > self.config.max_length {
            return false;
        }
        if !normalized
            .chars()
            .all(|ch| self.config.alphabet.contains(ch))
        {
            return false;
        }
        !self.contains_banned_substring(&normalized)
    }

    pub fn normalize(&self, candidate: &str) -> String {
        candidate.split_whitespace().collect::<String>().to_ascii_uppercase()
    }

    pub fn sanitize(&self, candidate: &str) -> String {
        let normalized = self.normalize(candidate);
        normalized
            .chars()
            .filter(|ch| self.config.alphabet.contains(*ch))
            .collect()
    }

    pub fn contains_banned_substring(&self, candidate: &str) -> bool {
        self.config
            .banned_substrings
            .iter()
            .any(|banned| candidate.contains(banned))
    }

    pub fn config(&self) -> &IdConfig {
        &self.config
    }
}

#[derive(Clone, Debug)]
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn from_system_time() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0);
        Self { state: nanos ^ 0x9E3779B97F4A7C15 }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_usize(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }
        (self.next_u64() % upper as u64) as usize
    }
}

pub fn build_custom_config(
    max_length: usize,
    alphabet: &str,
    banned_substrings: &[&str],
) -> IdConfig {
    IdConfig {
        max_length,
        alphabet: alphabet.to_ascii_uppercase(),
        banned_substrings: banned_substrings
            .iter()
            .map(|item| item.to_ascii_uppercase())
            .collect(),
    }
}
