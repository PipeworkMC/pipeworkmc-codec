#[derive(Clone, Copy)]
pub struct Protocol {
    id    : u32,
    names : &'static [&'static str]
}

impl Protocol {

    pub const LATEST : Self = Self::by_id(772).unwrap();

    pub const fn by_id(id : u32) -> Option<Self> {
        let mut i = 0;
        while (i < Self::LOOKUP.len()) {
            let (checking_id, names,) = Self::LOOKUP[i];
            if (id == checking_id) {
                return Some(Self { id, names });
            }
            i += 1;
        }
        None
    }

    pub const fn by_name(name : &'static str) -> Option<Self> {
        let mut i = 0;
        while (i < Self::LOOKUP.len()) {
            let (id, checking_names,) = Self::LOOKUP[i];
            let mut j = 0;
            while (j < checking_names.len()) {
                let checking_name = checking_names[j];
                if (name.len() == checking_name.len()) {
                    let mut k = 0;
                    while (k < name.len()) {
                        if (name.as_bytes()[k] == checking_name.as_bytes()[k]) {
                            return Some(Self { id, names : checking_names });
                        }
                        k += 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        None
    }

}

impl Protocol {

    #[inline(always)]
    pub const fn id(self) -> u32 { self.id }

    #[inline(always)]
    pub const fn names(self) -> &'static [&'static str] { self.names }

    #[inline(always)]
    pub const fn earliest_name(self) -> &'static str { self.names[0] }

    #[inline(always)]
    pub const fn latest_name(self) -> &'static str { self.names.last().unwrap() }

}

include!("out.rs");
