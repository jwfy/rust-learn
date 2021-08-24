pub trait Encoder {
    type Target;
    fn encode(&self) -> Option<Vec<Self::Target>>;
}

pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data>
where
    Id: Encoder<Target = u8>,
    Data: Encoder<Target = u8>,
{
    pub fn new(id: Id, data: Data) -> Self {
        Event { id, data }
    }

    pub fn encode(&self) -> Option<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Some(result)
    }
}

impl Encoder for i32 {
    type Target = u8;
    // 给i32拓展实现encoder的特质功能
    fn encode(&self) -> Option<Vec<u8>> {
        Some(vec![1, 2, 3, 4, 5])
    }
}

impl Encoder for String {
    // 同上
    type Target = u8;
    fn encode(&self) -> Option<Vec<u8>> {
        Some(self.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test() {
        let event = Event::new(32, 56);
        let res = event.encode();
        println!("{:?}", res);

        let event1 = Event::new("hh".to_string(), 56);
        let res = event1.encode();
        println!("{:?}", res);
    }
}
