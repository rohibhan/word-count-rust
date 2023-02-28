# Create a program that reads a text file and counts the number of occurrences of each word in the file. 

1. The program should output the word count for each unique word in the file, sorted by frequency. 

2. The program should be able to handle large files and should use Rust's file I/O and string manipulation capabilities.


 
> By default data.txt file is set in arguments

## Usage
### To run the program default

```bash 
cargo run
```
### To search some specific word 

```bash 
cargo run -- --word the
```

### To search word in some specific file 

```bash 
cargo run -- --file data.txt --word the
```

### To get occurences of all the words


```bash 
cargo run -- --file data.txt
```









