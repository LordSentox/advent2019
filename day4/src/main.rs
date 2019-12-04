use std::cmp::Ordering;

const NUM_DIGITS: usize = 7;

const MIN_VAL: u32 = 402328;
const MAX_VAL: u32 = 864247;

#[derive(Debug, Eq, PartialEq)]
struct Digits([u8; NUM_DIGITS]);

fn into_digits(mut val: u32) -> Digits {
    let mut digits = [0; NUM_DIGITS];

    for i in 0..NUM_DIGITS {
        digits[i] = (val % 10) as u8;
        val /= 10;
    }

    Digits(digits)
}

impl Digits {
    fn increment_to_next_ascending(&mut self) {
        // Increment to the next higher number
        let mut digit_inc = 0;
        loop {
            self.0[digit_inc] += 1;
            if self.0[digit_inc] >= 10 {
                self.0[digit_inc] = 0;
                digit_inc += 1;
            }
            else {
                break;
            }
        }

        // Make sure it is sorted like requested
        for i in 0..NUM_DIGITS - 1 {
            if self.0[i] < self.0[i + 1] {
                for j in 0..=i {
                    self.0[j] = self.0[i + 1];
                }
            }
        }
    }

    pub fn to_next_password(&mut self) {
        self.increment_to_next_ascending();

        // There has to be at least one digit doubled, otherwise the next number should
        // be taken
        for i in 0..NUM_DIGITS - 1 {
            if self.0[i] == self.0[i + 1] {
                return;
            }
        }

        // No double digit was found
        self.to_next_password();
    }

    pub fn to_next_password_no_triples(&mut self) {
        self.increment_to_next_ascending();

        // Try to find a doubled digit, that has no further neighbours with the same
        // value
        for i in 0..NUM_DIGITS - 1 {
            if self.0[i] == self.0[i + 1] {
                if i != 0 && self.0[i - 1] == self.0[i] {
                    continue;
                }
                if i + 1 < NUM_DIGITS - 1 && self.0[i + 2] == self.0[i] {
                    continue;
                }

                return;
            }
        }

        // No such pair could be found
        self.to_next_password_no_triples();
    }
}

impl PartialOrd for Digits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(&other)) }
}

impl Ord for Digits {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..NUM_DIGITS {
            let i = NUM_DIGITS - i - 1;

            match self.0[i].cmp(&other.0[i]) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => {}
            }
        }

        Ordering::Equal
    }
}

fn main() {
    let min = into_digits(MIN_VAL);
    let max = into_digits(MAX_VAL);
    assert!(min <= max);

    println!("Running through passwords from {} to {}", MIN_VAL, MAX_VAL);

    // Start value is one below the minimal value, since the minimum could already
    // be a valid password.
    let mut val = into_digits(MIN_VAL - 1);
    let mut num_vals = 0;
    while {
        val.to_next_password();
        val < max
    } {
        println!("{:?} ", val);
        num_vals += 1;
    }

    println!("Total number of matches for the first part: {}", num_vals);

    // Start value is one below the minimal value, since the minimum could already
    // be a valid password.
    let mut val = into_digits(MIN_VAL - 1);
    let mut num_vals = 0;
    while {
        val.to_next_password_no_triples();
        val < max
    } {
        println!("{:?} ", val);
        num_vals += 1;
    }

    println!("Total number of matches for the second part: {}", num_vals);
}
