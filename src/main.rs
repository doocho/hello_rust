use hello_rust::line;
use hello_rust::tx;

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
    let result = line::longer(a, b);
    println!("Result: {:?}", result);
}