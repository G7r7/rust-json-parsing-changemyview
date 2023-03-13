use std::{fs::File, io::BufReader, io::BufRead, str::FromStr};


pub fn count_lines(file: &File) -> usize {
    let iterator = BufReader::new(file).lines();
    iterator.count()
}

pub fn get_line_content(file: &File, n: usize) -> String {
    let mut iterator = BufReader::new(file).lines();
    if let Some(str) = iterator.nth(n - 1) {
        str.unwrap()
    } else {
        panic!("Nooo!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_count() {
        let file = File::open("./test/test1.txt").unwrap();
        assert_eq!(count_lines(&file), 3);
    }

    #[test]
    fn test_get_line_content() {
        let file = File::open("./test/test1.txt").unwrap();
        assert_eq!(get_line_content(&file, 1), "Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications de mise en page de texte, comme Aldus PageMaker.");
    }

    #[test]
    fn test_get_line_count_and_get_line_content() {
        let file = File::open("./test/test1.txt").unwrap();
        assert_eq!(count_lines(&file), 3);
        assert_eq!(get_line_content(&file, 1), "Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications de mise en page de texte, comme Aldus PageMaker.");
    }
}