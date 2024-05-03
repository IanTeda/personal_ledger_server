use crate::domain::ThingDescription;
use crate::domain::ThingName;

pub struct NewThing {
    // We are not using `String` anymore!
    pub description: ThingDescription,
    pub name: ThingName,
}