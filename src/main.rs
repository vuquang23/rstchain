use blockchainlib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        difficulty
    );

    genesis_block.mine();
    println!("{:?}", genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let chain = Blockchain {
        blocks: vec![genesis_block]
    };

    for i in 1..=10 {
        let mut new_block = Block::new(
            i,
            0,
            last_hash,
            0,
            "Not genesis block!".to_owned(),
            difficulty
        );
        new_block.mine();
        println!("{:?}", new_block);

        last_hash = new_block.hash.clone();
    }
    println!("{}", chain.verify());
}
