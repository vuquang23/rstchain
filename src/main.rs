use blockchainlib::*;
use std::time::SystemTime;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 50,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 30,
                    }
                ]
            }
        ],
        difficulty
    );

    genesis_block.mine();
    println!("Mined genesis block: {:?}", genesis_block);

    let mut last_hash = genesis_block.hash.clone();
    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Fail to add genesis block!");

    let mut block_1 = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Miner_Quang".to_owned(),
                        value: 1000
                    }
                ]
            },
            Transaction {
                inputs: vec![
                    blockchain.blocks[0].transactions[0].outputs[0].clone()
                ],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Christ".to_owned(),
                        value: 49,
                    },
                ]
            }
        ],
        difficulty
    );

    block_1.mine();
    println!("Mined block 1: {:?}", block_1);
    blockchain.update_with_block(block_1).expect("Fail to add block_1");
}
