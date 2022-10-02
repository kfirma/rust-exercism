#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const FULL_PINS: u16 = 10;
const NUMBER_OF_FRAMES: usize = 10;
const TWO_THROWS: usize = 2;

pub struct BowlingGame {
    throws: Vec<u16>,
    first_throw: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            throws: Vec::new(),
            first_throw: true,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > FULL_PINS
            || (!self.first_throw && (self.throws.last().unwrap() + pins) > FULL_PINS)
        {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.score().is_some() {
            return Err(Error::GameComplete);
        }
        self.throws.push(pins);
        if pins == FULL_PINS {
            self.first_throw = true;
        } else {
            self.first_throw = !self.first_throw;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score = 0;
        let mut throws_index = 0;
        // repeat NUMBER_OF_FRAMES times
        for _frame in 0..NUMBER_OF_FRAMES {
            match (
                &self.throws.get(throws_index),
                &self.throws.get(throws_index + 1),
            ) {
                (Some(&first_throw), Some(&second_throw)) => {
                    score += first_throw + second_throw;
                    // strike or spare
                    if first_throw == FULL_PINS || first_throw + second_throw == FULL_PINS {
                        match &self.throws.get(throws_index + TWO_THROWS) {
                            Some(&next_throw) => {
                                score += next_throw;
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                }
                _ => {
                    return None;
                }
            }
            // if strike add just 1 index
            if *self.throws.get(throws_index).unwrap() == FULL_PINS {
                throws_index += 1;
            } else {
                throws_index += TWO_THROWS;
            }
        }
        Some(score)
    }
}
