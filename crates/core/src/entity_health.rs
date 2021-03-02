use super::entity::EntityKind;
pub struct Health {
    pub health: u32,
    pub max_health: u32,
}

pub enum EntityHealthConversionError {
    EntityCanNotHaveHealth,
    EntityNotFound,
}

/// From EntityKind to the default entity health struct.
impl std::convert::TryFrom<EntityKind> for Health {
    type Error = EntityHealthConversionError;
    fn try_from(entity_kind: EntityKind) -> Result<Self, Self::Error> {
        match entity_kind {
           EntityKind::Player => Ok(Health { health: 20, max_health: 20 }), // Player
           _ => Err(EntityHealthConversionError::EntityNotFound)
        }
    }
}
