use sha1::Digest;
use std::{
    env::{self},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HASH_LEN: usize = 40;
fn main()-> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }
    
    let crack_hash = args[2].trim();
    if crack_hash.len() != SHA1_HASH_LEN {
        return Err("Wrong Format".into());
    }    
    
    let reader = File::open(&args[1])?;
    let worlist_read = BufReader::new(&reader);
    for line in worlist_read.lines(){
        let line = line?;
        let common_passwd = line.trim();
        if crack_hash == &hex::encode(sha1::Sha1::digest(common_passwd.as_bytes())) {
            println!("Hash: {:?}", common_passwd);
            return Ok(());
        }
    }
    println!("Not Found");
    Ok(())
}