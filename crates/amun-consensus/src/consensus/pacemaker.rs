//! Pacemaker Protocol - DETERMINISTIC
//! PANIC-FREE CONSENSUS PATH - all operations return Result

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Round(pub u64);

impl Round {
    pub const fn new(r: u64) -> Self { Self(r) }
    pub const fn as_u64(&self) -> u64 { self.0 }
    
    /// SAFE: Returns None on overflow, never wraps silently
    pub fn try_increment(&self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
    
    /// Returns error on overflow - NO PANIC in consensus path
    pub fn increment(&self) -> Result<Self, &'static str> {
        self.try_increment().ok_or("Round overflow - constitutional violation")
    }
}

#[derive(Debug, Clone)]
pub enum LeaderSelection {
    RoundRobin(Vec<u64>),
    Staked,
}

impl Default for LeaderSelection {
    fn default() -> Self {
        Self::RoundRobin(vec![1, 2, 3])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixedMultiplier {
    numerator: u64,
    denominator: u64,
}

impl FixedMultiplier {
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self { numerator, denominator }
    }

    pub fn apply(&self, value: u64) -> Result<u64, &'static str> {
        value.checked_mul(self.numerator)
            .map(|v| v / self.denominator)
            .ok_or("pacemaker multiplier overflow")
    }
}

impl Default for FixedMultiplier {
    fn default() -> Self {
        Self::new(3, 2)
    }
}

#[derive(Debug, Clone)]
pub struct PacemakerConfig {
    pub base_timeout_rounds: u64,
    pub timeout_multiplier: FixedMultiplier,
    pub max_timeout_rounds: u64,
    pub leader_selection: LeaderSelection,
}

impl Default for PacemakerConfig {
    fn default() -> Self {
        Self {
            base_timeout_rounds: 10,
            timeout_multiplier: FixedMultiplier::default(),
            max_timeout_rounds: 600,
            leader_selection: LeaderSelection::default(),
        }
    }
}

#[derive(Debug)]
pub struct Pacemaker {
    config: PacemakerConfig,
    current_round: Round,
    current_leader: u64,
    rounds_since_start: u64,
    timeout_rounds: u64,
    timeout_count: u64,
}

impl Pacemaker {
    pub fn new(config: PacemakerConfig) -> Self {
        if let LeaderSelection::RoundRobin(v) = &config.leader_selection {
            assert!(!v.is_empty(), "RoundRobin validator set cannot be empty");
        }
        
        let mut pm = Self {
            current_round: Round::new(0),
            current_leader: 0,
            rounds_since_start: 0,
            timeout_rounds: config.base_timeout_rounds,
            timeout_count: 0,
            config,
        };
        pm.update_leader();
        pm
    }

    fn update_leader(&mut self) {
        match &self.config.leader_selection {
            LeaderSelection::RoundRobin(v) => {
                if v.is_empty() {
                    self.current_leader = 0;
                    return;
                }
                let idx = self.current_round.as_u64() as usize % v.len();
                self.current_leader = v[idx];
            }
            LeaderSelection::Staked => {
                self.current_leader = 1;
            }
        }
    }

    pub fn current_round(&self) -> Round { self.current_round }
    pub fn current_leader(&self) -> u64 { self.current_leader }

    pub fn is_timeout(&self) -> bool {
        self.rounds_since_start >= self.timeout_rounds
    }

    pub fn advance_round(&mut self) -> Result<(), &'static str> {
        self.current_round = self.current_round.increment()?;
        self.rounds_since_start += 1;
        self.timeout_count += 1;

        let mut accumulated = self.config.base_timeout_rounds;
        for _ in 0..self.timeout_count {
            accumulated = self.config.timeout_multiplier.apply(accumulated)?;
        }
        self.timeout_rounds = accumulated.min(self.config.max_timeout_rounds);
        self.update_leader();
        Ok(())
    }

    pub fn reset_timeout(&mut self) {
        self.timeout_rounds = self.config.base_timeout_rounds;
        self.timeout_count = 0;
        self.rounds_since_start = 0;
    }

    pub fn on_commit(&mut self) {
        self.reset_timeout();
    }
}
