use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc_2020::lib::util::get_file;

#[derive(Debug)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl EyeColor {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "amb" => Some(Self::Amb),
            "blu" => Some(Self::Blu),
            "brn" => Some(Self::Brn),
            "gry" => Some(Self::Gry),
            "grn" => Some(Self::Grn),
            "hzl" => Some(Self::Hzl),
            "oth" => Some(Self::Oth),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct HairColor {
    pub value: String,
}

impl HairColor {
    pub fn from(value: String) -> Option<Self> {
        if !value.starts_with("#") {
            return None;
        }

        Some(HairColor {
            value: value[1..].to_string(),
        })
    }

    pub fn is_valid(&self) -> bool {
        self.value
            .chars()
            .into_iter()
            .all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
    }
}

#[derive(Debug, Copy, Clone)]
enum Height {
    Cm(usize),
    In(usize),
}

impl Height {
    pub fn from(value: String) -> Option<Self> {
        let (val, unit) = value.split_at(value.len() - 2);
        match unit {
            "cm" => Some(Height::Cm(val.parse::<usize>().unwrap())),
            "in" => Some(Height::In(val.parse::<usize>().unwrap())),
            _ => {
                println!("Invalid unit for height");
                None
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match &self {
            Self::Cm(val) => val >= &(150 as usize) && val <= &(193 as usize),
            Self::In(val) => val >= &(59 as usize) && val <= &(76 as usize),
        }
    }
}

#[derive(Debug)]
struct Passport {
    pub byr: Option<usize>,
    pub iyr: Option<usize>,
    pub eyr: Option<usize>,
    pub hgt: Option<Height>,
    pub hcl: Option<HairColor>,
    pub ecl: Option<EyeColor>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.ecl.is_some()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        self.byr.is_some() && self.byr.unwrap() >= 1920 && self.byr.unwrap() <= 2002
    }

    fn is_iyr_valid(&self) -> bool {
        self.iyr.is_some() && self.iyr.unwrap() >= 2010 && self.iyr.unwrap() <= 2020
    }

    fn is_eyr_valid(&self) -> bool {
        self.eyr.is_some() && self.eyr.unwrap() >= 2020 && self.eyr.unwrap() <= 2030
    }

    fn is_hgt_valid(&self) -> bool {
        self.hgt.is_some() && self.hgt.unwrap().is_valid()
    }

    fn is_hcl_valid(&self) -> bool {
        self.hcl.is_some() && self.hcl.as_ref().unwrap().is_valid()
    }

    fn is_pid_valid(&self) -> bool {
        self.pid.is_some()
            && self.pid.as_ref().unwrap().chars().count() == 9
            && self
                .pid
                .as_ref()
                .unwrap()
                .chars()
                .all(|c| c >= '0' && c <= '9')
    }
}

impl From<String> for Passport {
    fn from(value: String) -> Self {
        let mut passport: Passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        for field_raw in value.split(" ").into_iter() {
            let mut parts = field_raw.split(":");
            match parts.nth(0).unwrap() {
                "byr" => passport.byr = parts.nth(0).unwrap().parse::<usize>().ok(),
                "iyr" => passport.iyr = parts.nth(0).unwrap().parse::<usize>().ok(),
                "eyr" => passport.eyr = parts.nth(0).unwrap().parse::<usize>().ok(),
                "hgt" => passport.hgt = Height::from(parts.nth(0).unwrap().to_string()),
                "hcl" => passport.hcl = HairColor::from(parts.nth(0).unwrap().to_string()),
                "ecl" => passport.ecl = EyeColor::from(parts.nth(0).unwrap()),
                "pid" => passport.pid = Some(parts.nth(0).unwrap().to_string()),
                "cid" => passport.cid = Some(parts.nth(0).unwrap().to_string()),
                _ => {}
            }
        }
        passport
    }
}

fn main() {
    let file: File = get_file().unwrap();

    let raw_lines: Vec<String> = BufReader::new(&file).lines().flatten().collect();
    let passport_lines: Vec<Vec<String>> = raw_lines
        .split(|s| s.is_empty())
        .map(|group| group.to_vec())
        .collect();

    let passports: Vec<Passport> = passport_lines
        .iter()
        .map(|lines| lines.join(" ").into())
        .collect();

    println!("passports: {:?}", passports);

    let valid_count = passports.iter().filter(|p| p.is_valid()).count();
    println!("Valid count: {}", valid_count);
}
