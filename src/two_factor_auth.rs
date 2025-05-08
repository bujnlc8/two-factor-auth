#[derive(Debug)]
pub struct TwoFactorAuth {
    pub secret: String,
    pub timestamp: u64,
}

impl TwoFactorAuth {
    pub fn new(secret: &str, timestamp: u64) -> Self {
        TwoFactorAuth {
            secret: secret.to_string(),
            timestamp,
        }
    }

    fn get_hmac(&self) -> Vec<u8> {
        if let Some(key) =
            base32::decode(base32::Alphabet::Rfc4648 { padding: (true) }, &self.secret)
        {
            return hmac_sha1::hmac_sha1(
                key.as_slice(),
                self.to_bytes(self.timestamp / 30).as_slice(),
            )
            .to_vec();
        } else {
            eprintln!("Secret {} decode error!", self.secret);
            std::process::exit(1);
        }
    }

    pub fn calc(&self) -> u32 {
        let mut hmac = self.get_hmac();
        let start: usize = usize::from(hmac.last().unwrap() & 0x0F);
        let four_bytes = &mut hmac[start..start + 4];
        let mut num = 0;
        for (k, v) in four_bytes.iter_mut().enumerate() {
            if k == 0 {
                *v &= 0x7F;
            }
            num += (*v as u32) << (24 - k * 8);
        }
        num % 1000000
    }

    fn to_bytes(&self, val: u64) -> Vec<u8> {
        let mut res = Vec::new();
        for shift in [56, 48, 40, 32, 24, 16, 8, 0] {
            res.push(((val >> shift) & 0xFF) as u8);
        }
        res
    }
}
