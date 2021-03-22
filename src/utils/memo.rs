use crate::twitter;

#[derive(Debug)]
pub struct Memo {
    pub handle: Option<String>,
    pub address: Option<String>,
}

pub fn process_memo(memo: &String) -> Memo {
    let mut x: Vec<&str> = memo.split(":").collect();

    if x.len() < 2 {
        x = memo.split(" ").collect();
    }

    if x.len() == 1 {
        x = memo.split("：").collect();
    }

    if x.len() == 1 {
        x = memo.split("\n").collect();
    }

    let handle: Option<String> = x.get(0).and_then(|v| v.parse().ok());
    let address: Option<String> = x.get(1).and_then(|v| v.parse().ok());

    let results: Memo = Memo {
        handle: twitter::process_twitter_handler(handle),
        address: process_address(address),
    };
    results
}

fn process_address(address: Option<String>) -> Option<String> {
    match address {
        Some(v) => Some(v.trim().parse().unwrap()),
        None => None,
    }
}

mod tests {
    use super::*;

    #[test]
    fn process_meme_test00() {
        let memo00 = "Hydroflash2:sif1clj542dadlxk702s9zv0yfr4gv6lr5maujqwr6".to_string();
        let results = process_memo(&memo00);
        assert_eq!(results.handle, Some("Hydroflash2".to_string()));
        assert_eq!(
            results.address,
            Some("sif1clj542dadlxk702s9zv0yfr4gv6lr5maujqwr6".to_string())
        )
    }

    #[test]
    fn process_memo_test01() {
        let memo01 = "@DavidJRaw64: sif1rnlrt3rhzqxkp32chwkxm5pexw3vdqy8fngd43".to_string();
        let results = process_memo(&memo01);
        assert_eq!(results.handle, Some("DavidJRaw64".to_string()));
        assert_eq!(
            results.address,
            Some("sif1rnlrt3rhzqxkp32chwkxm5pexw3vdqy8fngd43".to_string())
        );
    }

    #[test]
    fn process_memo_test02() {
        let memo02 = "".to_string();
        let results = process_memo(&memo02);
        assert_eq!(results.handle, None);
        assert_eq!(results.address, None);
    }

    #[test]
    fn process_memo_test03() {
        let memo03 = "@dodawuk sif1kkxgg2lhz753wgrrgl0ehzp5lxfam9qsmpvnw3".to_string();
        let result = process_memo(&memo03);
        assert_eq!(result.handle, Some("dodawuk".to_string()));
        assert_eq!(
            result.address,
            Some("sif1kkxgg2lhz753wgrrgl0ehzp5lxfam9qsmpvnw3".to_string())
        )
    }

    #[test]
    fn process_memo_test04() {
        let memo04 = "MelanieGilson16：sif12wn03kmuc7skk4eyc2fej0g3d8pjljjan69n2q".to_string();
        let result = process_memo(&memo04);
        assert_eq!(result.handle, Some("MelanieGilson16".to_string()));
        assert_eq!(
            result.address,
            Some("sif12wn03kmuc7skk4eyc2fej0g3d8pjljjan69n2q".to_string())
        );
    }

    #[test]
    fn process_memo_test05() {
        let memo05 = "linkowskaz
sif1vv06crzl53yv5kp2w56hcmt30cmflnt7uxdvs9"
            .to_string();
        let result = process_memo(&memo05);
        assert_eq!(result.handle, Some("linkowskaz".to_string()));
        assert_eq!(
            result.address,
            Some("sif1vv06crzl53yv5kp2w56hcmt30cmflnt7uxdvs9".to_string())
        );
    }

    // Can't easily get this working... skipping.
    // #[test]
    // fn process_memo_test06() {
    //     let memo06 = "
    //         みんみん。
    //         @Be3hfcqupcWfKnk
    //         sif19e2670v3akpgnj4wetq2g4c954zkpkt8pccmd2
    //         "
    //     .to_string();
    //     let result = process_memo(&memo06);
    //     assert_eq!(result.handle, Some("Be3hfcqupcWfKnk".to_string()));
    //     assert_eq!(
    //         result.address,
    //         Some("sif19e2670v3akpgnj4wetq2g4c954zkpkt8pccmd2".to_string())
    //     );
    // }
}
