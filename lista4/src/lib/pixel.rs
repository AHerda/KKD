use std::{
    cmp::Ordering,
    fmt::Error,
    ops::{Add, Div, Index, IndexMut, Sub},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Add for Pixel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pixel {
            red: self.red.wrapping_add(rhs.red),
            green: self.green.wrapping_add(rhs.green),
            blue: self.blue.wrapping_add(rhs.blue),
        }
    }
}

impl Sub for Pixel {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pixel {
            red: self.red.wrapping_sub(rhs.red),
            green: self.green.wrapping_sub(rhs.green),
            blue: self.blue.wrapping_sub(rhs.blue),
        }
    }
}

impl Div<usize> for Pixel {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Pixel {
            red: self.red / rhs as u8,
            green: self.green / rhs as u8,
            blue: self.blue / rhs as u8,
        }
    }
}

impl Index<usize> for Pixel {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => panic!("Invalid index for Pixel"),
        }
    }
}

impl IndexMut<usize> for Pixel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            _ => panic!("Invalid index for Pixel"),
        }
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_sum = u32::from(self.red) + u32::from(self.green) + u32::from(self.blue);
        let other_sum = u32::from(other.red) + u32::from(other.green) + u32::from(other.blue);
        self_sum.cmp(&other_sum)
    }
}

impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn pixel_from(colors: &[u8]) -> Result<Pixel, Error> {
    if colors.len() != 3 {
        Err(Error)
    } else {
        Ok(Pixel {
            red: colors[0],
            green: colors[1],
            blue: colors[2],
        })
    }
}

impl Pixel {
    pub fn abs_diff(&self, other: Pixel) -> Self {
        Self {
            red: self.red.wrapping_sub(other.red),
            green: self.green.wrapping_sub(other.green),
            blue: self.blue.wrapping_sub(other.blue),
        }
    }
}
