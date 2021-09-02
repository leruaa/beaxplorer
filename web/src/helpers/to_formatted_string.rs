use std::ops::{Div, Mul};

pub trait ToFormattedString {
    fn to_formatted_string(&self, decimals: u32) -> String;
}

pub trait ToEther: ToFormattedString {
    fn to_ether_value(&self, decimals: u32) -> String;
}

pub trait ToPercentage: ToFormattedString {
    fn to_percentage(&self) -> String;
}

impl ToFormattedString for i64 {
    fn to_formatted_string(&self, _: u32) -> String {
        self.to_string()
            .chars()
            .rev()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % 3 == 0 {
                    Some(',')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>()
            .chars()
            .rev()
            .collect()
    }
}

impl ToEther for i64 {
    fn to_ether_value(&self, decimals: u32) -> String {
        let f = *self as f64;
        f.div(1_000_000_000f64).to_formatted_string(decimals)
    }
}

impl ToFormattedString for f64 {
    fn to_formatted_string(&self, decimals: u32) -> String {
        let int_value = self.trunc() as i64;
        let frac_value = self.fract().mul(10i32.pow(decimals) as f64).round();

        format!("{}.{}", int_value.to_formatted_string(0), frac_value)
    }
}

impl ToPercentage for f64 {
    fn to_percentage(&self) -> String {
        self.mul(100f64).to_formatted_string(0)
    }
}

impl<T: ToFormattedString> ToFormattedString for Option<T> {
    fn to_formatted_string(&self, _: u32) -> String {
        match self {
            None => 0.to_string(),
            Some(n) => n.to_formatted_string(0),
        }
    }
}

impl<T: ToEther> ToEther for Option<T> {
    fn to_ether_value(&self, decimals: u32) -> String {
        match self {
            None => 0.to_string(),
            Some(n) => n.to_ether_value(decimals),
        }
    }
}

impl<T: ToPercentage> ToPercentage for Option<T> {
    fn to_percentage(&self) -> String {
        match self {
            None => 0.to_string(),
            Some(n) => n.to_percentage(),
        }
    }
}
