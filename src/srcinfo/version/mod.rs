use num_bigint::BigUint;
use num_traits::Zero;
use std::{cmp::Ordering, fmt::Write};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Version<PkgVer, PkgRel, Epoch>
where
    PkgVer: AsRef<str>,
    PkgRel: AsRef<str>,
    Epoch: AsRef<str>,
{
    pub pkgver: PkgVer,
    pub pkgrel: PkgRel,
    pub epoch: Epoch,
}

impl<PkgVer, PkgRel, Epoch> Version<PkgVer, PkgRel, Epoch>
where
    PkgVer: AsRef<str>,
    PkgRel: AsRef<str>,
    Epoch: AsRef<str>,
{
    pub fn new(pkgver: PkgVer, pkgrel: PkgRel, epoch: Epoch) -> Self {
        Version {
            pkgver,
            pkgrel,
            epoch,
        }
    }

    pub fn try_to_string(&self) -> Result<String, String> {
        let Version {
            pkgver,
            pkgrel,
            epoch,
        } = self;

        let epoch = epoch.as_ref();
        let mut result = if epoch.is_empty() {
            String::new()
        } else {
            match epoch.parse::<BigUint>() {
                Err(error) => return Err(format!("invalid epoch: {}", error.to_string())),
                Ok(value) => {
                    if value.is_zero() {
                        String::new()
                    } else {
                        format!("{}:", value)
                    }
                }
            }
        };

        write!(result, "{}-{}", pkgver.as_ref(), pkgrel.as_ref())
            .map_err(|error| format!("fail to write pkgver and pkgrel: {}", error.to_string()))?;

        Ok(result)
    }

    pub fn as_str(&self) -> Version<&str, &str, &str> {
        Version {
            pkgver: self.pkgver.as_ref(),
            pkgrel: self.pkgrel.as_ref(),
            epoch: self.epoch.as_ref(),
        }
    }
}

impl<PkgVer, PkgRel, Epoch> PartialOrd for Version<PkgVer, PkgRel, Epoch>
where
    PkgVer: AsRef<str> + PartialEq,
    PkgRel: AsRef<str> + PartialEq,
    Epoch: AsRef<str> + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (left, right) = match (self.try_to_string(), other.try_to_string()) {
            (Ok(left), Ok(right)) => (left, right),
            _ => return None,
        };
        Some(alpm::vercmp(left, right))
    }
}

#[cfg(test)]
mod tests;
