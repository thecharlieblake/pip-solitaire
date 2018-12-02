pub mod string_ops {
    use std::cmp::max;

    pub fn append_horizontal(a: &str, b: &str) -> String {
        // Appends newlines to the strings to make each line end in one
        // (easier to reason with)
        let pad_a = format!("{}\n", a);
        let pad_b = format!("{}\n", b);

        let (a_width, a_height) = line_dimensions(&pad_a);
        let (b_width, b_height) = line_dimensions(&pad_b);
        let max_height = max(a_height, b_height);

        let pad_a = pad(&pad_a, a_width, a_height, max_height);
        let pad_b = pad(&pad_b, b_width, b_height, max_height);

        let mut res = concat_horizontal(&pad_a, &pad_b);
        res.pop(); // removes '\n'
        res
    }

    pub fn append_vertical(a: &str, b: &str) -> String {
        if a == "" {
            String::from(b)
        } else if b == "" {
            String::from(a)
        } else {
            let (a_width, _) = line_dimensions(a);
            let (b_width, _) = line_dimensions(b);
            let max_width = max(a_width, b_width);

            let pad_a = pad_h(a, max_width);
            let pad_b = pad_h(b, max_width);

            let mut res = format!("{}{}", pad_a, pad_b);
            res.pop();
            res
        }
    }

    fn repeat(c: char, n: usize) -> String {
        (0..n).map(|_| c).collect()
    }

    fn line_dimensions(s: &str) -> (usize, usize) {
        (
            s.lines().map(|l| l.len()).max().unwrap_or(0),
            s.lines().count()
        )
    }

    fn pad(s: &str, width: usize, cur_height: usize, height: usize) -> String {
        let s_pad = pad_h(s, width);
        pad_v(&s_pad, width, cur_height, height)
    }

    fn pad_h(s: &str, width: usize) -> String {
        s.lines()
            .map(|l| format!("{}{}\n", l, repeat(' ',width - l.len())))
            .collect::<Vec<String>>()
            .join("")
    }

    fn pad_v(s: &str, width: usize, cur_height: usize, height: usize) -> String {
        let pad = (0..height-cur_height)
            .map(|_| format!("{}\n", &repeat(' ', width)))
            .collect::<Vec<String>>()
            .join("");
        format!("{}{}", s, &pad)
    }

    fn concat_horizontal(a: &str, b: &str) -> String {
        a.lines()
            .zip(b.lines())
            .map(|(al, bl)| format!("{}{}\n", al, bl))
            .collect::<Vec<String>>()
            .join("")
    }
}

pub mod yaml {
    use regex::Regex;
    use serde::ser::Serialize;
    use serde_yaml;

    pub fn prettify(s: &str) -> String {
        let list_re = Regex::new(r"(?:\n {2}- - .*)(?:\n {2}  - .*)*").unwrap();
        let space_re = Regex::new(r"\n {4}-").unwrap();

        let mut out = format!("{}\n", s);

        for caps in list_re.captures_iter(&out.clone()) {
            let before = caps.get(0).unwrap().as_str();
            let after = format!(
                "{}]",
                space_re.replace_all(&before.replacen("- - ", "- [", 1), ",")
            );
            out = out.replacen(before, &after, 1);
        }
        format!("{}", out)
    }

    pub fn to_pretty_string<T: ?Sized>(value: &T) -> serde_yaml::Result<String>
        where T: Serialize,
    {
        Ok(prettify(&(serde_yaml::to_string(value)?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_horizontal(s: &str, app: &str, expected: &str) {
        let result = string_ops::append_horizontal(&s, app);
        assert_eq!(expected, result);
    }

    fn test_vertical(s: &str, app: &str, expected: &str) {
        let result = string_ops::append_vertical(&s, app);
        assert_eq!(expected, result);
    }

    #[test]
    fn add_nothing_horizontal() {
        test_horizontal("","", "");
    }

    #[test]
    fn add_one_char_horizontal() {
        test_horizontal("", "a", "a")
    }

    #[test]
    fn add_nothing_to_one_char_horizontal() {
        test_horizontal("a", "", "a")
    }

    #[test]
    fn add_one_line_to_one_horizontal() {
        test_horizontal("abc", "pqr", "abcpqr")
    }

    #[test]
    fn add_two_lines_to_one_horizontal() {
        test_horizontal("abc", "pqr\nstu", "abcpqr\n   stu")
    }

    #[test]
    fn add_one_line_to_two_horizontal() {
        test_horizontal("abc\ndef", "pqr", "abcpqr\ndef   ")
    }

    #[test]
    fn add_two_lines_to_two_horizontal() {
        test_horizontal("abc\ndef", "pqr\nstu", "abcpqr\ndefstu")
    }

    #[test]
    fn add_lines_of_different_lengths_horizontal() {
        test_horizontal("a\ndef", "pqr\nst", "a  pqr\ndefst ")
    }

    #[test]
    fn add_nothing_vertical() {
        test_vertical("","", "");
    }

    #[test]
    fn add_one_char_vertical() {
        test_vertical("", "a", "a")
    }

    #[test]
    fn add_nothing_to_one_char_vertical() {
        test_vertical("a", "", "a")
    }

    #[test]
    fn add_one_line_to_one_vertical() {
        test_vertical("abc", "pqr", "abc\npqr")
    }

    #[test]
    fn add_two_lines_to_one_vertical() {
        test_vertical("abc", "pqr\nstu", "abc\npqr\nstu")
    }

    #[test]
    fn add_one_line_to_two_vertical() {
        test_vertical("abc\ndef", "pqr", "abc\ndef\npqr")
    }

    #[test]
    fn top_line_longer_vertical() {
        test_vertical("a", "pqr", "a  \npqr")
    }

    #[test]
    fn bottom_line_longer_vertical() {
        test_vertical("abc", " r", "abc\n r ")
    }
}