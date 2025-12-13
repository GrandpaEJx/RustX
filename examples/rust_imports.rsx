use crate "rand" = "0.8"

rust {
    fn get_random() -> Result<Value, String> {
        let n: i64 = rand::random::<u8>() as i64;
        Ok(Value::Int(n))
    }
}

x = get_random()
print("Random number:", x)
