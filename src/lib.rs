pub fn tokenize(text: &str) -> std::vec::Vec<&str> {
    text.split(" ").collect::<Vec<_>>()
}

pub fn token_count(text: &str) -> usize {
    let tokens = tokenize(text);

    if tokens == [""] {
        return 0
    }
    tokens.len()
}

#[cfg(test)]
mod tests {
    use crate::{tokenize, token_count};

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

    #[test]
    fn count_simple_text() {
        let t = "hello, this is some text";
        assert_eq!(token_count(t), 5)
    }

    #[test]
    fn count_single_token() {
        let t = "hello";
        assert_eq!(token_count(t), 1)
    }

    #[test]
    fn count_empty_string() {
        let t = "";
        assert_eq!(token_count(t), 0)
    }
}
