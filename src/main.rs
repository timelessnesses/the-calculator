use std::io::Write;
use std::str::FromStr;

fn input(prompt: Option<&str>) -> String {
    let mut input = String::new();
    if let Some(prompt) = prompt {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
    }
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let a = input(None);
    let i = a.split(" ").collect::<Vec<&str>>();
    let a = FloatInfinite::from_str(i[0]).unwrap();
    let b = FloatInfinite::from_str(i[1]).unwrap();
    let c = FloatInfinite::from_str(i[2]).unwrap();
    if a + b == c {
        println!("Correct");
    } else {
        println!("Wrong");
    }
}

/// Because rust doesn't have f128 stabilized, we are making our own! Fuck you!
struct FloatInfinite {
    floats: Vec<u64>,
    front: u64,
}

impl std::fmt::Display for FloatInfinite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.", self.front)?;

        let floats = self.floats.clone();
        for i in 0..floats.len() {
            let digit = floats.get(i).copied().unwrap_or(0);
            write!(f, "{}", digit)?;
        }

        Ok(())
    }
}

impl FromStr for FloatInfinite {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut floats = Vec::new();
        let parts: Vec<&str> = s.split('.').collect();

        let front = if let Some(int_part) = parts.get(0) {
            int_part.parse::<u64>()?
        } else {
            0
        };

        if let Some(fraction_part) = parts.get(1) {
            for digit in fraction_part.chars() {
                floats.push(digit.to_digit(10).ok_or("Invalid digit")? as u64);
            }
        }

        Ok(Self { floats, front })
    }
}

impl std::ops::Add for FloatInfinite {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut floats = Vec::new();
        let mut carry = 0;

        // Find the maximum fractional length
        let max_len = self.floats.len().max(rhs.floats.len());

        // Pad the fractional parts with zeros to make them the same length
        let mut self_floats = self.floats.clone();
        let mut rhs_floats = rhs.floats.clone();
        self_floats.resize(max_len, 0);
        rhs_floats.resize(max_len, 0);

        // Add fractional parts
        for (a, b) in self_floats.into_iter().rev().zip(rhs_floats.into_iter().rev()) {
            let sum = a + b + carry;
            floats.push(sum % 10);
            carry = sum / 10; // Carry overflows to the next digit
        }
        floats.reverse();

        // Add the integer parts with the carry
        let new_front = self.front + rhs.front + carry;

        FloatInfinite {
            floats,
            front: new_front,
        }
    }
}

impl PartialEq for FloatInfinite {
    fn eq(&self, other: &Self) -> bool {
        self.floats == other.floats && self.front == other.front
    }
}