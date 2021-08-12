use anyhow::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GigantesqueDecimal {
    chunk_size: usize,
    pub value: String,
}

impl GigantesqueDecimal {
    pub fn new() -> Result<Self> {
        Ok(Self {
            value: String::new(),
            chunk_size: 1000000000,
        })
    }

    pub fn from_string(value: String) -> Result<Self> {
        Ok(Self {
            value,
            chunk_size: 1000000000,
        })
    }

    pub fn add(&mut self, value: Self) -> Result<()> {
        let x = self.deserialize(value.value)?;
        let y = self.deserialize(self.value.clone())?;
        let mut result = (Vec::<u64>::new(), Vec::<u64>::new());

        let x_0_len = x.0.len();
        let y_0_len = y.0.len();
        let x_1_len = x.1.len();
        let y_1_len = y.1.len();

        match x_0_len.cmp(&y_0_len) {
            std::cmp::Ordering::Less => {
                for i in 0..y_0_len {
                    let x_value: u64 = if i > x_0_len || x_0_len == 0 {
                        0
                    } else {
                        x.0[i]
                    };
                    result.0.push(y.0[i] + x_value)
                }
            }
            std::cmp::Ordering::Equal => {
                for i in 0..x_0_len {
                    result.0.push(x.0[i] + y.0[i])
                }
            }
            std::cmp::Ordering::Greater => {
                for i in 0..x_0_len {
                    let y_value: u64 = if i > y_0_len || y_0_len == 0 {
                        0
                    } else {
                        y.0[i]
                    };
                    result.0.push(x.0[i] + y_value)
                }
            }
        }

        match x_1_len.cmp(&y_1_len) {
            std::cmp::Ordering::Less => {
                for i in 0..y_1_len {
                    let x_value: u64 = if i > x_1_len || x_1_len == 0 {
                        0
                    } else {
                        x.1[i]
                    };
                    result.1.push(y.1[i] + x_value)
                }
            }
            std::cmp::Ordering::Equal => {
                for i in 0..x_1_len {
                    result.1.push(x.1[i] + y.1[i])
                }
            }
            std::cmp::Ordering::Greater => {
                for i in 0..x_1_len {
                    let y_value: u64 = if i > y_1_len || y_1_len == 0 {
                        0
                    } else {
                        y.1[i]
                    };
                    result.1.push(x.1[i] + y_value)
                }
            }
        }

        self.value = self.serialize(result)?;

        Ok(())
    }

    fn serialize(&self, value: (Vec<u64>, Vec<u64>)) -> Result<String> {
        let mut x = Vec::<String>::new();
        let mut y = Vec::<String>::new();

        let integer = value.0;
        let decimal = value.1;

        for i in integer {
            x.push(format!("{}", i))
        }
        for i in decimal {
            y.push(format!("{}", i))
        }

        let mut result = String::new();
        for i in x {
            result = format!("{}{}", result, i)
        }
        result = format!("{}.", result);
        for i in y {
            result = format!("{}{}", result, i)
        }

        Ok(result)
    }

    fn deserialize(&self, value: String) -> Result<(Vec<u64>, Vec<u64>)> {
        let mut chunks = (Vec::<u64>::new(), Vec::<u64>::new());
        let split: Vec<&str> = value.split(".").collect();
        if split[0].len() > self.chunk_size {
            let slices = self.split_at_chunk_size(split[0])?;
            for i in slices.split("X") {
                let integer: u64 = i.parse()?;
                chunks.0.push(integer);
            }
        } else {
            let integer: u64 = split[0].parse()?;
            chunks.0.push(integer);
        }
        if split.len() >= 2 {
            if split[1].len() > self.chunk_size {
                let slices = self.split_at_chunk_size(split[1])?;
                for i in slices.split("X") {
                    let integer: u64 = i.parse()?;
                    chunks.1.push(integer);
                }
            } else {
                let integer: u64 = split[1].parse()?;
                chunks.1.push(integer);
            }
        }

        Ok(chunks)
    }

    fn split_at_chunk_size(&self, data: &str) -> Result<String> {
        Ok(data
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % self.chunk_size == 0 {
                    Some('X')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use crate::GigantesqueDecimal;

    #[test]
    fn addition() {
        let mut a = GigantesqueDecimal::from_string(String::from("552323423566654652354300.2366")).unwrap();
        let b = GigantesqueDecimal::from_string(String::from("1002354364576575675634500000.0")).unwrap();
        a.add(b).expect("Couldn't add");
        println!("{}", a.value);

        assert_eq!(a.value, String::from("700.0"));
    }
}
