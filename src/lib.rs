
pub fn tokenize(text: &str) -> std::vec::Vec<&str> {
    text.split(" ").collect::<Vec<_>>()
}

pub fn count(text: &str, delimiter: &str) -> i32 {
    let result: i32 = text.split(delimiter).next().unwrap_or("0").parse().unwrap_or(0);
    result
}

#[cfg(test)]
mod tests {
    use crate::tokenize;

    #[test]
    fn test_tokenizer() {
        let t = "hello, this is some text";
        assert_eq!(tokenize(t), ["hello,", "this", "is", "some", "text"])
    }

    #[test]
    fn test_tokenizer_single_token() {
        let t = "hello";
        assert_eq!(tokenize(t), ["hello"])
    }

    #[test]
    fn test_tokenizer_empty() {

        let t = "";
        assert_eq!(tokenize(t), [""])
    }

    // #[test]
    // fn count_simple_text() {
    //     let t = "hello, this is some text";
    //     assert_eq!(count(t, " "), 5)
    // }

}
