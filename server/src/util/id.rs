use sonyflake::Sonyflake;
pub fn next_id(sf: &mut Sonyflake) -> i64 {
    let next_id = sf.next_id().unwrap();
    next_id as i64
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn test_next_id() {
        let mut sf = Sonyflake::new().unwrap();

        let mut id_map = HashMap::new();
        // loop
        for _ in 0..100 {
            let id = next_id(&mut sf);
            println!("id: {}", id);
            if id_map.contains_key(&id) {
                panic!("id is repeated");
            }
            id_map.insert(id, true);
            assert!(id > 0);
        }
    }
}
