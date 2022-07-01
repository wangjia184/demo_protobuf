use fpdec::Decimal;


include!("./_.rs");




impl From<&NotNullDecimal> for Decimal {
    fn from(raw: &NotNullDecimal) -> Self {
        let mut coeff: i128 = (raw.high.unwrap_or(0) as i128) << 64 | (raw.low.unwrap_or(0) as i128);
        let scale : u8 = ((raw.sign_scale.unwrap_or(0) & 0x01FE) >> 1) as u8;
        if raw.sign_scale.unwrap_or(0) & 0x0001 == 0x0001 {
            coeff = coeff * -1;
        }
        Decimal::new_raw( coeff, scale)
    }
}

impl From<NotNullDecimal> for Decimal {
    fn from(raw: NotNullDecimal) -> Self {
        Decimal::from(&raw)
    }
}


impl std::fmt::Display for NotNullDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Decimal::from(self))
    }
}






