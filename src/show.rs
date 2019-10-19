pub trait PresentNum {
    fn as_dec(&self, simple: bool) -> String;
    fn as_hex(&self, simple: bool) -> String;
    fn as_bin(&self, simple: bool) -> (String, String);
    fn show_all(&self) -> String;
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
    fn as_dec(&self, simple: bool) -> String {
        let dec = format!("{}", self);
        if simple {
            dec
        } else {
            let mut s = Self::group_str(dec, 3).0;
            s.push_str("  ");
            s
        }
    }

    fn as_hex(&self, simple: bool) -> String {
        let hex = format!("{:x}", self);
        if simple {
            hex
        } else {
            let mut s = Self::group_str(hex, 3).0;
            s.push_str(" h");
            s
        }
    }

    fn as_bin(&self, simple: bool) -> (String, String) {
        let bin = format!("{:b}", self);
        if simple {
            (bin, String::from(""))
        } else {
            let mut s = Self::group_str(bin, 4);
            s.0.push_str(" b");
            s
        }
    }

    fn show_all(&self) -> String {
        let dec = self.as_dec(false);
        let hex = self.as_hex(false);
        let bin = self.as_bin(false);
        let ruler = bin
            .1
            .chars()
            .skip(bin.1.len() - bin.0.len())
            .collect::<String>();
        let max_len = [&dec, &hex, &bin.0].iter().map(|s| s.len()).max().unwrap();
        format!(
            "\x1B[36mDec   {1:>0$}\x1B[0m\n\
             \x1B[92mHex   {2:>0$}\x1B[0m\n\
             \x1B[91mBin   {3:>0$}\x1B[0m\n\
             \x1B[90m      {4}\x1B[0m",
            max_len, dec, hex, &bin.0, ruler
        )
    }
}

#[test]
fn test_show() {
    let i: i128 = 0xCAFEBABE;
    assert_eq!(i.as_dec(false), "3 405 691 582  ");
    assert_eq!(i.as_dec(true), "3405691582");
    assert_eq!(i.as_hex(false), "ca feb abe h");
    assert_eq!(i.as_hex(true), "cafebabe");
    assert_eq!(
        i.as_bin(false),
        (
            "1100 1010 1111 1110 1011 1010 1011 1110 b".to_string(),
            "---28---24---20---16---12----8----4----0 ".to_string()
        )
    );
    assert_eq!(
        i.as_bin(true),
        (
            "11001010111111101011101010111110".to_string(),
            "".to_string()
        )
    );
    assert_eq!(
        i.show_all(),
        "\u{1b}[36mDec                             3 405 691 582  \u{1b}[0m\n\
         \u{1b}[92mHex                                ca feb abe h\u{1b}[0m\n\
         \u{1b}[91mBin   1100 1010 1111 1110 1011 1010 1011 1110 b\u{1b}[0m\n\
         \u{1b}[90m      ---28---24---20---16---12----8----4----0 \u{1b}[0m"
    );
}
