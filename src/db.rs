use sqlite::Sqlite;

impl Database for Sqlite {
    fn connect(&self) -> String {
        "sqlite".to_string()
    }
}

pub trait Database {
    fn connect(&self) -> String;
}

pub fn connect(db: impl Database) -> String {
    db.connect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let db = Sqlite;
        assert_eq!(connect(db), "sqlite");
    }
}
