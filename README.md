# How it works ?
First binary takes your wordlist than binary hashes them with encryption algorithm. After that, the program compares the hash value you entered as an argument with the hash value of wordlist.

# Compile

You should compile the code with ```cargo build --release``` parameter. Then you can find the binary in ```release/sha1```.


# Usage

You should give a two argument. First one for wordlist. Second one for hash. 
>Usage: <wordlist.txt> <sha1_hash>
