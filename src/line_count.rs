use std::{fs::File, io::BufRead, io::BufReader, path::Path};

fn get_reader(path: &Path) -> BufReader<File> {
    BufReader::new(File::open(path).unwrap())
}

pub fn count_lines(path: &Path) -> usize {
    let reader = get_reader(path);
    reader.lines().count()
}

pub fn get_line_content(path: &Path, n: usize) -> String {
    let reader = get_reader(path);
    let mut iterator = reader.lines();
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
        let path = Path::new("./test/test1.txt");
        assert_eq!(count_lines(&path), 3);
    }

    #[test]
    fn test_get_line_content() {
        let path = Path::new("./test/test1.txt");
        assert_eq!(get_line_content(&path, 1), "Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications de mise en page de texte, comme Aldus PageMaker.");
    }

    #[test]
    fn test_get_line_count_and_get_line_content() {
        let path = Path::new("./test/test1.txt");

        assert_eq!(count_lines(&path), 3);
        assert_eq!(get_line_content(&path, 1), "Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en soit modifié. Il a été popularisé dans les années 1960 grâce à la vente de feuilles Letraset contenant des passages du Lorem Ipsum, et, plus récemment, par son inclusion dans des applications de mise en page de texte, comme Aldus PageMaker.");
    }
}
