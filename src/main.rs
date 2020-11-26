use std::io;
use hex::encode;
use sha2::{Sha512, Digest};

fn main() {
    let mut hashes: Vec<String> = Vec::new();
    let mut current_block: String = String::new();
    let mut hashed_block: String = String::new();
    let mut blocks: Vec<String> = Vec::new();
    let mut message = String::new();
    let mut x = 0;


    loop {
        x = x + 1;

        io::stdin().read_line(&mut message).expect("Failed to read line");
        println!("{} transaction: ", x);

        let hash = hash_str(message.clone()); 
        hashes.push(hash);
        
        println!("All transactions {:?}:", hashes);
        
        for x in hashes.iter(){
            current_block = current_block + x;
        }

        let hash = hash_str(current_block.clone());
        blocks.push(hash.clone());

        println!("Gen Block transactions hash: {}", hash);
        println!("Gen Block transactions: {:?}", blocks);

        for x in blocks.iter(){
            hashed_block = hashed_block + x;
        }

        let hash = hash_str(hashed_block.clone());

        if x == 5{
            println!("Gen Block: {:?}", hash);
            break;
        }
        
    }
}

fn hash_str(msg: String) -> String {
    let mut hasher_512 = Sha512::new();
    hasher_512.update(msg.as_bytes());
    let result = hasher_512.finalize();
    let hex_result = encode(result);
    return hex_result;
}
