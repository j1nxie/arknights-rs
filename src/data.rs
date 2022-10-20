use std::fmt;

enum Mission {
    CargoEscort,
    TacticalDrill,
    ResourceSearch,
    ToughSiege,
    AerialThreat,
    SolidDefense,
    FierceAttack,
    UnstoppableCharge,
    FearlessProtection,
}

lazy_static! {
    static ref WEEKDAYS: Vec<Vec<Mission>> = vec![
        vec![
            Mission::TacticalDrill,
            Mission::ResourceSearch,
            Mission::ToughSiege,
            Mission::SolidDefense,
            Mission::FierceAttack,
        ],
        vec![
            Mission::TacticalDrill,
            Mission::CargoEscort,
            Mission::AerialThreat,
            Mission::FierceAttack,
            Mission::FearlessProtection,
        ],
        vec![
            Mission::TacticalDrill,
            Mission::ResourceSearch,
            Mission::AerialThreat,
            Mission::UnstoppableCharge,
            Mission::FearlessProtection,
        ],
        vec![
            Mission::TacticalDrill,
            Mission::CargoEscort,
            Mission::ToughSiege,
            Mission::SolidDefense,
            Mission::UnstoppableCharge,
        ],
        vec![
            Mission::TacticalDrill,
            Mission::ResourceSearch,
            Mission::AerialThreat,
            Mission::SolidDefense,
            Mission::FierceAttack,
        ],
        vec![
            Mission::TacticalDrill,
            Mission::CargoEscort,
            Mission::ResourceSearch,
            Mission::ToughSiege,
            Mission::SolidDefense,
            Mission::FierceAttack,
            Mission::UnstoppableCharge,
            Mission::FearlessProtection,
        ],
        vec![
            Mission::TacticalDrill,
            Mission::CargoEscort,
            Mission::ToughSiege,
            Mission::AerialThreat,
            Mission::UnstoppableCharge,
            Mission::FearlessProtection,
        ],
    ];
}

impl fmt::Display for Mission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mission::CargoEscort => write!(f, "Cargo Escort"),
            Mission::TacticalDrill => write!(f, "Tactical Drill"),
            Mission::ResourceSearch => write!(f, "Resource Search"),
            Mission::ToughSiege => write!(f, "Tough Siege"),
            Mission::AerialThreat => write!(f, "Aerial Threat"),
            Mission::SolidDefense => write!(f, "Solid Defense"),
            Mission::FierceAttack => write!(f, "Fierce Attack"),
            Mission::UnstoppableCharge => write!(f, "Unstoppable Charge"),
            Mission::FearlessProtection => write!(f, "Fearless Protection"),
        }
    }
}
