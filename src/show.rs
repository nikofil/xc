pub trait PresentNum {
    fn as_dec(&self) -> String;
    fn as_hex(&self) -> String;
    fn as_bin(&self) -> String;
    fn show_all(&self);
    fn group_str(s: String) -> String {
        let mut v: Vec<char> = Vec::new();
        for (i, c) in s.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                v.push(' ');
            }
            v.push(c);
        }
        v.iter().rev().collect::<String>()
    }
}

impl PresentNum for i128 {
    fn as_dec(&self) -> String {
        let mut s = Self::group_str(format!("{}", self));
        s.push_str("  ");
        s
    }

    fn as_hex(&self) -> String {
        let mut s = Self::group_str(format!("{:x}", self));
        s.push_str(" h");
        s
    }

    fn as_bin(&self) -> String {
        let mut s = Self::group_str(format!("{:b}", self));
        s.push_str(" b");
        s
    }

    fn show_all(&self) {
        let strs = vec![self.as_dec(), self.as_hex(), self.as_bin()];
        let max_len = strs.iter().map(|s| s.len()).max().unwrap();
        for i in &strs {
            println!("{:>1$}", i, max_len);
        }
    }
}
