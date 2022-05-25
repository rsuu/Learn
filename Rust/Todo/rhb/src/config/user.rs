#[derive(Debug)]
pub struct Profile {
    pub about: (String, String), // name + about
    pub state: State,
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub item: Item,
    //pub class: Class,
    pub buff: Buff,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub hp: usize,
    pub max_hp: usize,

    pub mp: usize,
    pub max_mp: usize,

    pub exp: usize,
}

#[derive(Debug, PartialEq)]
pub enum Class {
    Warrior,
    Mage,
    Healer,
    Rogue,
}

#[derive(Debug, PartialEq)]
pub struct Equipments {
    pub headgear: EquipmentMeta,
    pub body: EquipmentMeta,
    pub hand_main: EquipmentMeta,
    pub hand_off: EquipmentMeta,
}

#[derive(Debug, PartialEq)]
pub struct EquipmentMeta {
    pub name: String,
    pub gold: usize,
    pub lv: usize,
    pub buff: Buff,
}

#[derive(Debug, PartialEq)]
pub struct Pets {
    pub pet: PetMeta,
    pub lv: usize,
    pub buff: Buff,
}

#[derive(Debug, PartialEq)]
pub struct PetMeta {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Mounts {
    pub mount: MountMeta,
}

#[derive(Debug, PartialEq)]
pub struct MountMeta {
    pub name: String,
    pub lv: usize,
    pub buff: Buff,
}

#[derive(Debug, PartialEq)]
pub struct Buff {
    pub strength: BuffBonus,
    pub intelligence: BuffBonus,
    pub constitution: BuffBonus,
    pub perception: BuffBonus,
}

#[derive(Debug, PartialEq)]
pub struct BuffBonus(i8, i8); // gear + class

impl Profile {
    pub fn new(name: String, about: String, num: u8) -> Self {
        match rand_class(num) {
            Class::Healer => Profile {
                about: (name, about),
                state: State {
                    item: Item {
                        hp: 0,
                        max_hp: 0,

                        mp: 0,
                        max_mp: 0,

                        exp: 0,
                    },
                    buff: Buff {
                        strength: BuffBonus(0, 0),
                        intelligence: BuffBonus(0, 0),
                        constitution: BuffBonus(0, 0),
                        perception: BuffBonus(0, 0),
                    },
                },
            },
            Class::Mage => Profile {
                about: (name, about),
                state: State {
                    item: Item {
                        hp: 0,
                        max_hp: 0,

                        mp: 0,
                        max_mp: 0,

                        exp: 0,
                    },
                    buff: Buff {
                        strength: BuffBonus(0, 0),
                        intelligence: BuffBonus(0, 0),
                        constitution: BuffBonus(0, 0),
                        perception: BuffBonus(0, 0),
                    },
                },
            },
            Class::Rogue => Profile {
                about: (name, about),
                state: State {
                    item: Item {
                        hp: 0,
                        max_hp: 0,

                        mp: 0,
                        max_mp: 0,

                        exp: 0,
                    },
                    buff: Buff {
                        strength: BuffBonus(0, 0),
                        intelligence: BuffBonus(0, 0),
                        constitution: BuffBonus(0, 0),
                        perception: BuffBonus(0, 0),
                    },
                },
            },
            Class::Warrior => Profile {
                about: (name, about),
                state: State {
                    item: Item {
                        hp: 0,
                        max_hp: 0,

                        mp: 0,
                        max_mp: 0,

                        exp: 0,
                    },
                    buff: Buff {
                        strength: BuffBonus(0, 0),
                        intelligence: BuffBonus(0, 0),
                        constitution: BuffBonus(0, 0),
                        perception: BuffBonus(0, 0),
                    },
                },
            },
        }
    }
}

pub fn rand_class(num: u8) -> Class {
    match num {
        0 => Class::Healer,
        1 => Class::Mage,
        2 => Class::Rogue,
        3 => Class::Warrior,

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Profile::new("test_name".to_string(), "test_about".to_string(), 1);
        assert_eq!(&p.about.0, "test_name");
        assert_eq!(&p.about.1, "test_about");
    }
}
