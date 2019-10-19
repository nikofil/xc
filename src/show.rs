pub trait PresentNum {
    fn as_dec(&self) -> String;
    fn as_hex(&self) -> String;
    fn as_bin(&self) -> (String, String);
    fn show_all(&self);
    fn group_str(s: String, every: usize) -> (String, String) {
        let mut v: Vec<char> = Vec::new();
        let mut ruler = String::from("----0 ");
        for (i, c) in s.chars().rev().enumerate() {
            if i > 0 && i % every == 0 {
                v.push(' ');
                ruler.insert_str(0, format!("{0:->1$}", i, every + 1).as_str());
            }
            v.push(c);
        }
        (v.iter().rev().collect::<String>(), ruler)
    }
}

impl PresentNum for i128 {
    fn as_dec(&self) -> String {
        let mut s = Self::group_str(format!("{}", self), 3).0;
        s.push_str("  ");
        s
    }

    fn as_hex(&self) -> String {
        let mut s = Self::group_str(format!("{:x}", self), 3).0;
        s.push_str(" h");
        s
    }

    fn as_bin(&self) -> (String, String) {
        let mut s = Self::group_str(format!("{:b}", self), 4);
        s.0.push_str(" b");
        s
    }

    fn show_all(&self) {
        let dec = self.as_dec();
        let hex = self.as_hex();
        let bin = self.as_bin();
        let ruler = bin
            .1
            .chars()
            .skip(bin.1.len() - bin.0.len())
            .collect::<String>();
        let max_len = [&dec, &hex, &bin.0].iter().map(|s| s.len()).max().unwrap();
        println!("\x1B[36mDec   {:>1$}\x1B[0m", dec, max_len);
        println!("\x1B[92mHex   {:>1$}\x1B[0m", hex, max_len);
        println!("\x1B[91mBin   {:>1$}\x1B[0m", &bin.0, max_len);
        println!("\x1B[90m      {}\x1B[0m", ruler);
    }
}
