use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Tx {
    from: String,
    to: String,
    amount: u64,
}

impl FromStr for Tx {
    type Err = ParseTxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_tx(s)
    }
}

#[derive(Debug)]
struct Payment {
    id: u64,
    from: String,
    to: String,
    amount: u64,
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
enum ParseTxError {
    NotEnoughParts,
    InvalidAmount(ParseIntError),
}

impl From<ParseIntError> for ParseTxError {
    fn from(error: ParseIntError) -> Self {
        ParseTxError::InvalidAmount(error)
    }
}

trait HasId {
    fn id(&self) -> u64;
}

impl HasId for Payment {
    fn id(&self) -> u64 {
        self.id
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

impl HasAmount for Payment {
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

fn main() {
    // let mut txs = vec![
    //     Tx {
    //         from: "Alice".to_string(),
    //         to: "Bob".to_string(),
    //         amount: 100,
    //     },
    //     Tx {
    //         from: "Bob".to_string(),
    //         to: "Charlie".to_string(),
    //         amount: 200,
    //     },
    //     Tx {
    //         from: "Charlie".to_string(),
    //         to: "Alice".to_string(),
    //         amount: 300,
    //     },
    // ];

    // txs.iter_mut().for_each(|tx| {
    //     if tx.amount >= 150 {
    //         tx.amount -= 100;
    //         println!("Tx: {:?}", tx);
    //     }
    // });

    // let large_txs = filter_large_txs(&txs, 150);
    // println!("Large Txs: {:?}", large_txs);

    // let total = total_amount(&txs);
    // println!("Total Amount: {:?}", total);
    // let tx = parse_tx("Alice,Bob,abs").unwrap_or_else(|e| {
    //     println!("Error: {:?}", e);
    //     Tx {
    //         from: "".to_string(),
    //         to: "".to_string(),
    //         amount: 0,
    //     }
    // });
    // println!("Tx: {:?}", tx);
    // let txs_string = "Alice,Bob,100\nBob,Charlie,200\nCharlie,Alice,300";
    // let txs = parse_txs(txs_string).unwrap();
    // println!(
    //     "Txs count: {:?}, total amount: {:?}",
    //     txs.len(),
    //     total_amount_with_print_items(&txs)
    // );

    // let payments = vec![Payment {
    //     id: 1,
    //     from: "Alice".to_string(),
    //     to: "Bob".to_string(),
    //     amount: 100,
    // }];
    // println!(
    //     "Payments count: {:?}, total amount: {:?}",
    //     payments.len(),
    //     total_amount_with_print_items(&payments)
    // );

    // let tx1 = Tx::from_str("Alice,Bob,100").unwrap();
    // println!("Tx1: {:?}", tx1);
    // let tx2 = "Alice,Bob,100".parse::<Tx>().unwrap();
    // println!("Tx2: {:?}", tx2);
    let a = "Hello";
    let b = "World";
    let result = longer(a, b);
    println!("Result: {:?}", result);
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

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct LineRef<'a> {
    line: &'a str,
}

impl<'a> LineRef<'a> {
    fn new(line: &'a str) -> Self {
        Self { line }
    }
}