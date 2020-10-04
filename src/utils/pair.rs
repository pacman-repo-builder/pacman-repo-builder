#[derive(Debug, Copy, Clone)]
pub struct Pair<Primary, Secondary> {
    primary: Primary,
    secondary: Secondary,
}

impl<Primary, Secondary> Pair<Primary, Secondary> {
    pub fn new(primary: Primary, secondary: Secondary) -> Self {
        Pair { primary, secondary }
    }

    pub fn from_tuple((primary, secondary): (Primary, Secondary)) -> Self {
        Pair::new(primary, secondary)
    }

    pub fn into_tuple(self) -> (Primary, Secondary) {
        (self.primary, self.secondary)
    }

    pub fn to_ref(&self) -> Pair<&Primary, &Secondary> {
        Pair::new(&self.primary, &self.secondary)
    }

    pub fn swap_role(self) -> Pair<Secondary, Primary> {
        Pair::new(self.secondary, self.primary)
    }

    pub fn map<Return>(self, f: impl FnOnce(Primary) -> Return) -> Pair<Return, Secondary> {
        Pair::new(f(self.primary), self.secondary)
    }
}
