use crate::model::word::object::WordId;
use anyhow::Result;
use fsrs::{FSRS, MemoryState};
use time::{Duration, OffsetDateTime};

pub const DESIRED_RETENTION: f32 = 0.9;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Grade {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub word_id: WordId,
    pub memory: Option<MemoryState>,
    pub lapses: u32,
    pub last_review: Option<OffsetDateTime>,
    pub next_review: Option<OffsetDateTime>,
}

#[derive(Debug, Clone)]
pub struct ReviewOutcome {
    pub card: Card,
    pub delta_t: u32,
    pub rating: u32,
}

impl Card {
    pub fn is_new(&self) -> bool {
        self.memory.is_none()
    }

    pub fn delta_t_days(&self, reviewed_at: OffsetDateTime) -> u32 {
        match self.last_review {
            Some(last) => (reviewed_at - last).whole_days().max(0) as u32,
            None => 0,
        }
    }

    pub fn review(
        &self,
        grade: Grade,
        reviewed_at: OffsetDateTime,
        fsrs: &FSRS,
    ) -> Result<ReviewOutcome> {
        let delta_t = self.delta_t_days(reviewed_at);
        let next = fsrs.next_states(self.memory, DESIRED_RETENTION, delta_t)?;
        let item = match grade {
            Grade::Again => next.again,
            Grade::Hard => next.hard,
            Grade::Good => next.good,
            Grade::Easy => next.easy,
        };

        let interval_days = item.interval.round().max(1.0) as i64;
        let lapses = if matches!(grade, Grade::Again) && !self.is_new() {
            self.lapses + 1
        } else {
            self.lapses
        };

        let card = Card {
            word_id: self.word_id.clone(),
            memory: Some(item.memory),
            lapses,
            last_review: Some(reviewed_at),
            next_review: Some(reviewed_at + Duration::days(interval_days)),
        };

        Ok(ReviewOutcome {
            card,
            delta_t,
            rating: grade as u32,
        })
    }
}
