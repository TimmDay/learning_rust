
pub fn char_ngrams(seq: &[char], n_min: usize, n_max: usize) -> Vec<&[char]> {

    let mut result : Vec<&[char]> = Vec::new();
    let mut greaterSlice = &seq[..];

    while greaterSlice.len() >= n_min {

        let mut n_slice;
        if n_max <= greaterSlice.len() {
            n_slice = &greaterSlice[..n_max];
        } else {
            n_slice = &greaterSlice[..];
        }

        while n_slice.len() >= n_min {
            result.push(&greaterSlice[0..n_slice.len()]);
            n_slice = &n_slice[0..n_slice.len()-1];
        }

        greaterSlice = &greaterSlice[1..];
    }
    result
}

// ?: line 17 - 21
// question: is there a syntactic sugar way to get if/else to one line in rust? like the ternary operator?
// n_slice = (n_max <= greaterSlice.len()) ? &greaterSlice[0..n_max] : &greaterSlice[0..];


#[cfg(test)]
mod tests {
    use super::char_ngrams;

    #[test]
    fn ngrams_test() {
        let hello_chars: Vec<_> = "hello world".chars().collect();
        let mut hello_check: Vec<&[char]> = vec![
            &['h'],
            &['h', 'e'],
            &['h', 'e', 'l'],
            &['e'],
            &['e', 'l'],
            &['e', 'l', 'l'],
            &['l'],
            &['l', 'l'],
            &['l', 'l', 'o'],
            &['l'],
            &['l', 'o'],
            &['l', 'o', ' '],
            &['o'],
            &['o', ' '],
            &['o', ' ', 'w'],
            &[' '],
            &[' ', 'w'],
            &[' ', 'w', 'o'],
            &['w'],
            &['w', 'o'],
            &['w', 'o', 'r'],
            &['o'],
            &['o', 'r'],
            &['o', 'r', 'l'],
            &['r'],
            &['r', 'l'],
            &['r', 'l', 'd'],
            &['l'],
            &['l', 'd'],
            &['d'],
        ];
        hello_check.sort();

        let mut hello_ngrams = char_ngrams(&hello_chars, 1, 3);
        hello_ngrams.sort();

        assert_eq!(hello_check, hello_ngrams);
    }

    #[test]
    fn ngrams_23_test() {
        let hello_chars: Vec<_> = "hello world".chars().collect();
        let mut hello_check: Vec<&[char]> = vec![
            &['h', 'e'],
            &['h', 'e', 'l'],
            &['e', 'l'],
            &['e', 'l', 'l'],
            &['l', 'l'],
            &['l', 'l', 'o'],
            &['l', 'o'],
            &['l', 'o', ' '],
            &['o', ' '],
            &['o', ' ', 'w'],
            &[' ', 'w'],
            &[' ', 'w', 'o'],
            &['w', 'o'],
            &['w', 'o', 'r'],
            &['o', 'r'],
            &['o', 'r', 'l'],
            &['r', 'l'],
            &['r', 'l', 'd'],
            &['l', 'd'],
        ];
        hello_check.sort();

        let mut hello_ngrams = char_ngrams(&hello_chars, 2, 3);
        hello_ngrams.sort();

        assert_eq!(hello_check, hello_ngrams);
    }

    #[test]
    fn empty_ngram_test() {
        let check: &[&[char]] = &[];
        assert_eq!(char_ngrams(&[], 1, 3), check);
    }
}
