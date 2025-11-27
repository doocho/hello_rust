use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Tx {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

impl FromStr for Tx {
    type Err = ParseTxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_tx(s)
    }
}

struct AmountList<T> {
    items: Vec<T>,
}

impl<T> AmountList<T>
where
    T: HasAmount,
{
    fn total(&self) -> u64 {
        self.items.iter().map(|item| item.amount()).sum()
    }
}

impl<T> AmountList<T> {
    fn push(&mut self, item: T) {}

    fn len(&self) -> usize {
        self.items.len()
    }

    fn print_all(&self)
    where
        T: Debug,
    {
    }
}

#[derive(Debug)]
pub enum ParseTxError {
    NotEnoughParts,
    InvalidAmount(ParseIntError),
}

impl From<ParseIntError> for ParseTxError {
    fn from(error: ParseIntError) -> Self {
        ParseTxError::InvalidAmount(error)
    }
}

trait HasAmount {
    fn amount(&self) -> u64;
}

impl HasAmount for Tx {
    fn amount(&self) -> u64 {
        self.amount
    }
}

fn total_amount_with_print_items<T>(items: &[T]) -> u64
where
    T: HasAmount + Debug,
{
    let mut sum = 0;
    for item in items {
        println!("{:?}", item);
        sum += item.amount();
    }
    sum
}

fn total_amount<T: HasAmount>(items: &[T]) -> u64 {
    let mut sum = 0;
    for item in items {
        sum += item.amount();
    }
    sum
}

fn filter_large_txs<'a>(txs: &'a [Tx], min: u64) -> Vec<&'a Tx> {
    txs.iter().filter(|tx| tx.amount >= min).collect()
}

fn parse_tx(line: &str) -> Result<Tx, ParseTxError> {
    let parts: Vec<&str> = line.split(',').collect();
    let from = parts
        .get(0)
        .ok_or(ParseTxError::NotEnoughParts)?
        .to_string();
    let to = parts
        .get(1)
        .ok_or(ParseTxError::NotEnoughParts)?
        .to_string();
    let amount = parts
        .get(2)
        .ok_or(ParseTxError::NotEnoughParts)?
        .parse::<u64>()?;
    Ok(Tx { from, to, amount })
}

fn parse_txs(input: &str) -> Result<Vec<Tx>, ParseTxError> {
    let mut txs = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let tx = parse_tx(line)?;
        txs.push(tx);
    }
    Ok(txs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_line() {
        let line = "Alice,Bob,100";
        let tx = parse_tx(line).unwrap();
        assert_eq!(tx.from, "Alice");
        assert_eq!(tx.to, "Bob");
        assert_eq!(tx.amount, 100);
    }

    #[test]
    fn parse_not_enough_parts() {
        let line = "Alice,Bob";
        let result = parse_tx(line);
        assert!(result.is_err());
        assert_eq!(matches!(result.unwrap_err(), ParseTxError::NotEnoughParts), true);
    }

    #[test]
    fn parse_invalid_amount() {
        let line = "Alice,Bob,abc";
        let result = parse_tx(line);
        assert!(result.is_err());
        assert_eq!(matches!(result.unwrap_err(), ParseTxError::InvalidAmount(_)), true);
    }
}