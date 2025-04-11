use crate::model::{Comment, Position};

pub fn scan_comments(content: &str) -> Vec<Comment> {
    let mut comments = Vec::new();

    for (line_number, line) in content.lines().enumerate() {
        if let Some(index) = line.find("//") {
            comments.push(Comment::new(
                Position::new(line_number + 1, index),
                &line[index..],
            ));
        }
    }

    comments
}
