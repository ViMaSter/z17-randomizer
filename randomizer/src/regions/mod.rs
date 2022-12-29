use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Subregion {
    name: &'static str,
    world: World,
    id: &'static str,
}

impl Subregion {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn world(&self) -> World {
        self.world
    }
}

impl Debug for Subregion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subregion")
            .field("name", &self.name)
            .field("world", &self.world)
            .field("id", &self.id)
            .finish()
    }
}

impl Eq for Subregion {}

impl PartialEq for Subregion {
    fn eq(&self, other: &Self) -> bool {
        self.world == other.world && self.name == other.name && self.id == other.id
    }
}

impl Hash for Subregion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.world.hash(state);
        self.name.hash(state);
        self.id.hash(state);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum World {
    Hyrule,
    Lorule,
    Dungeons,
}

macro_rules! regions {
    ($($world:ident($variant:ident) {
        $($region:ident;)+
    })+) => {
        use crate::patch::Patcher;

        $(pub(crate) mod $world {
            pub const WORLD: super::World = super::World::$variant;
            $(pub(crate) mod $region;)+
        })+

        pub(crate) fn patch(patcher: &mut Patcher, layout: &crate::Layout, settings: &$crate::Settings) -> crate::Result<()> {
            $($($world::$region::patch(patcher, layout, settings)?;)+)+
            Ok(())
        }
    };
}

regions! {
    dungeons(Dungeons) {
        eastern;
        house;
        tower;
        graveyard;
        dark;
        swamp;
        skull;
        thieves;
        ice;
        desert;
        turtle;
        castle;
    }
    hyrule(Hyrule) {
        field;
        lost;
        death;
        sanctuary;
        kakariko;
        zoras;
        eastern;
        southern;
        lake;
        maiamai;
    }
    lorule(Lorule) {
        field;
        skull;
        death;
        graveyard;
        dark;
        misery;
        lake;
        maiamai;
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! region {
    (
        course: $course:ident,
        name: $name:literal,
        $start:ident $start_props:tt,
        $($id:ident $props:tt,)*
    ) => {

        #[inline]
        pub fn patch(patcher: &mut crate::patch::Patcher, layout: &crate::Layout, settings: &$crate::Settings) -> crate::Result<()> {
            $start::patch(patcher, layout, settings)?;
            $($id::patch(patcher, layout, settings)?;)*
            Ok(())
        }

        crate::subregion!($start $start_props);
        $(crate::subregion!($id $props);)*

        #[allow(unused)]
        pub(crate) fn start() -> &'static crate::regions::Subregion {
            $start::SUBREGION
        }

        pub const NAME: &str = $name;
        pub const COURSE: albw::course::Id = albw::course::Id::$course;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! subregion {
    ($id:ident {
        $(locations: [
            $($key:literal: $item:ident @$variant:ident $props:tt $(:- $condition:tt)?
                $(where $settings:ident: $where:expr)?,)*
        ],)?
        $(paths: [
            $($path:ident$(::$path_rest:ident)* $(:- $pcondition:tt)?,)*
        ],)?
        $(quest: $kind:ident$(::$qvariant:ident)?,)?
    }) => {
        pub(crate) mod $id {

            use crate::{patch::Patcher, regions::Subregion};

            pub use super::COURSE;

            pub(crate) const SUBREGION: &Subregion = &Subregion {
                name: super::NAME,
                world: super::super::WORLD,
                id: stringify!($id),
            };

            #[allow(unused)]
            #[inline]
            pub fn add(graph: &mut dyn $crate::graph::Graph) {
                $($(if $crate::settings_check!($($settings $where)?)(graph.settings()) {
                    let edge = $crate::edge!($($condition)?);
                    let location = $crate::LocationInfo::new(SUBREGION, $key);
                    if graph.check(edge) {
                        graph.add(location.into());
                    } else {
                        graph.add_edge(edge, location.into());
                    }
                })*)?
                $($(let edge = $crate::edge!($($pcondition)?);
                let path = $crate::path!($path$(::$path_rest)*);
                if graph.check(edge) {
                    graph.add(path.into());
                } else {
                    graph.add_edge(edge, path.into());
                })*)?
            }

            #[allow(unused)]
            #[inline]
            pub fn patch(patcher: &mut Patcher, layout: &crate::Layout, settings: &$crate::Settings) -> crate::Result<()> {
                $(use crate::patch::Patch;
                $(if $crate::settings_check!($($settings $where)?)(settings) {
                    crate::patch!($variant $props).apply(
                        patcher,
                        layout
                            .get(&crate::LocationInfo::new(SUBREGION, $key))
                            .unwrap_or_else(|| unreachable!(stringify!($key))),
                        settings,
                    )?;
                })*)?
                Ok(())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! settings_check {
    () => {
        |_: &$crate::settings::Settings| true
    };
    ($settings:ident $check:expr) => {
        |$settings: &$crate::settings::Settings| $check
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! edge {
    () => {
        |_: &$crate::state::State| true
    };
    ($method:ident) => {
        |state: &$crate::state::State| state.$method()
    };
    ({|$state:ident| $fn:expr}) => {
        |$state: &$crate::state::State| $fn
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! path {
    ($subarea:ident) => {
        super::$subarea::SUBREGION
    };
    ($region:ident::$node:ident) => {
        super::super::$region::$node::SUBREGION
    };
    ($world:ident::$region:ident::$node:ident) => {
        crate::regions::$world::$region::$node::SUBREGION
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! quest {
    () => {
        None
    };
    ($variant:ident) => {
        Some(crate::Quest::$variant)
    };
    ($variant:ident::$subvariant:ident) => {
        (Some(crate::Quest::$variant(crate::$variant::$subvariant)))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! patch {
    (Chest($course:ident $stage:literal[$unq:literal])) => {
        Patch::Chest { course: albw::course::Id::$course, stage: $stage - 1, unq: $unq }
    };
    (Chest($stage:literal[$unq:literal])) => {
        Patch::Chest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (BigChest($course:ident $stage:literal[$unq:literal])) => {
        Patch::BigChest { course: albw::course::Id::$course, stage: $stage - 1, unq: $unq }
    };
    (BigChest($stage:literal[$unq:literal])) => {
        Patch::BigChest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Event($name:ident[$index:literal])) => {
        Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index }
    };
    (Event(Boot/$name:ident[$index:literal])) => {
        Patch::Event { course: None, name: stringify!($name), index: $index }
    };
    (Event($course:ident/$name:ident[$index:literal])) => {
        Patch::Event { course: Some(albw::course::Id::$course), name: stringify!($name), index: $index }
    };
    (Event[$($name:ident[$index:literal],)+]) => {
        Patch::Multi(vec![
            $(
                Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index },
            )+
        ])
    };
    (Heart($course:ident $scene:literal[$unq:literal])) => {
        Patch::Heart { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Heart($scene:literal[$unq:literal])) => {
        Patch::Heart { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Key($course:ident $scene:literal[$unq:literal])) => {
        Patch::Key { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Key($scene:literal[$unq:literal])) => {
        Patch::Key { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($course:ident $scene:literal[$unq:literal])) => {
        Patch::Maiamai { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($scene:literal[$unq:literal])) => {
        Patch::Maiamai { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($course:ident $scene:literal[$unq:literal])) => {
        Patch::SilverRupee { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($scene:literal[$unq:literal])) => {
        Patch::SilverRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($course:ident $scene:literal[$unq:literal])) => {
        Patch::GoldRupee { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($scene:literal[$unq:literal])) => {
        Patch::GoldRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Shop($variant:ident$($args:tt)?)) => {
        Patch::Shop(crate::patch::Shop::$variant $($args)?)
    };
    (None()) => {
        Patch::None
    }
}
