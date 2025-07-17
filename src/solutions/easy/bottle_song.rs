pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        let current_bottles = start_bottles - i;
        let next_bottles = current_bottles - 1;

        let current_bottles_str = num_to_str(current_bottles);
        let next_bottles_str = num_to_str(next_bottles);

        // Формируем стих
        let verse = format!(
            "{} green bottle{} hanging on the wall,\n\
             {} green bottle{} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {} green bottle{} hanging on the wall.",
            current_bottles_str,
            plural_form(current_bottles),
            current_bottles_str,
            plural_form(current_bottles),
            next_bottles_str.to_lowercase(),
            plural_form(next_bottles),
        );

        verses.push(verse);
    }

    // Объединяем все стихи, разделяя их пустыми строками
    verses.join("\n\n")
}

fn num_to_str(num: u32) -> String {
    let num_str = match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => "no",
    };
    num_str.to_string()
}

fn plural_form(num: u32) -> String {
    let plural = match num {
        1 => "",
        _ => "s",
    };
    plural.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_generic_verse() {
        assert_eq!(
            recite(10, 1).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn last_generic_verse() {
        assert_eq!(
            recite(3, 1).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn verse_with_2_bottles() {
        assert_eq!(
            recite(2, 1).trim(),
            concat!(
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.",
            )
        );
    }
    #[test]
    fn verse_with_1_bottle() {
        assert_eq!(
            recite(1, 1).trim(),
            concat!(
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn first_two_verses() {
        assert_eq!(
            recite(10, 2).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn last_three_verses() {
        assert_eq!(
            recite(3, 3).trim(),
            concat!(
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
    #[test]
    fn all_verses() {
        assert_eq!(
            recite(10, 10).trim(),
            concat!(
                "Ten green bottles hanging on the wall,\n",
                "Ten green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be nine green bottles hanging on the wall.\n",
                "\n",
                "Nine green bottles hanging on the wall,\n",
                "Nine green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be eight green bottles hanging on the wall.\n",
                "\n",
                "Eight green bottles hanging on the wall,\n",
                "Eight green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be seven green bottles hanging on the wall.\n",
                "\n",
                "Seven green bottles hanging on the wall,\n",
                "Seven green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be six green bottles hanging on the wall.\n",
                "\n",
                "Six green bottles hanging on the wall,\n",
                "Six green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be five green bottles hanging on the wall.\n",
                "\n",
                "Five green bottles hanging on the wall,\n",
                "Five green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be four green bottles hanging on the wall.\n",
                "\n",
                "Four green bottles hanging on the wall,\n",
                "Four green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be three green bottles hanging on the wall.\n",
                "\n",
                "Three green bottles hanging on the wall,\n",
                "Three green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be two green bottles hanging on the wall.\n",
                "\n",
                "Two green bottles hanging on the wall,\n",
                "Two green bottles hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be one green bottle hanging on the wall.\n",
                "\n",
                "One green bottle hanging on the wall,\n",
                "One green bottle hanging on the wall,\n",
                "And if one green bottle should accidentally fall,\n",
                "There'll be no green bottles hanging on the wall.",
            )
        );
    }
}
