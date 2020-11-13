// Write a function called repeat_str which repeats the given string src exactly count times.

// repeatStr(6, "I") // "IIIIII"
// repeatStr(5, "Hello") // "HelloHelloHelloHelloHello"

fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}