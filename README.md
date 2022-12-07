# How it works ?
First binary takes your wordlist than binary hashes them with encryption algorithm. After that, the program compares the hash value you entered as an argument with the hash value of wordlist. If the result of the comparison is correct, the unhashed value is displayed on the screen.

![image](https://user-images.githubusercontent.com/54737933/206111985-6c0fb9b2-b332-4a1a-8de4-61c4b7608bbe.png)


# Compile

You should compile the code with ```cargo build --release``` parameter. Then you can find the binary in ```release/sha1```.


# Usage

You should give a two argument. First one for wordlist. Second one for hash. 
>Usage: <wordlist.txt> <sha1_hash>
![image](https://user-images.githubusercontent.com/54737933/206112115-f091caa9-d16c-45b2-8eb2-042aa0571657.png)
