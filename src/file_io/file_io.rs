use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct User {
    name: String,
    age: i32,
    genger: Genger,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Genger {
    UNKNOW = 0,
    MAN = 1,
    WOMAN = 2,
}

impl User {
    pub fn new(name: String, age: i32, genger: Genger) -> Self {
        User { name, age, genger }
    }

    pub fn load(file_name: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(file_name)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user = serde_json::from_str(&data)?;
        Ok(user)
    }

    pub fn store(&self, file_name: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(file_name)?;
        // 这里是写入一个新的文件，所以需要使用create，他会默认创建一个文件
        let data = serde_json::to_string(&self)?;
        file.write_all(data.as_bytes()).unwrap();
        Ok(data.len())
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "张三".to_string(),
            age: 0,
            genger: Genger::MAN,
        }
    }
}

#[cfg(test)]
mod test {
    use super::User;

    #[test]
    fn test() {
        let user = User::default();
        let file_name = "/Users/junhong/learn/rust/rust-learn/user.txt";
        user.store(file_name).unwrap();

        let user1 = User::load(file_name).unwrap();
        assert_eq!(user, user1);
    }
}
