use bevy::prelude::*;
use frame::ClientSet;

use crate::class_store::{
    ClassStoreFile, load_class_store, save_class_store, sync_host_class_loadouts,
};

#[derive(Resource, Clone, Debug, Default)]
pub struct MenuMapList(pub Vec<assets::MapPack>);

impl MenuMapList {
    pub fn maps(&self) -> impl Iterator<Item = &String> {
        self.0.iter().flat_map(|pack| &pack.maps)
    }

    pub fn contains(&self, map: &str) -> bool {
        self.maps().any(|installed| installed == map)
    }

    pub fn pack_of(&self, map: &str) -> Option<usize> {
        self.0
            .iter()
            .position(|pack| pack.maps.iter().any(|installed| installed == map))
    }
}

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMapList>()
            .init_resource::<crate::ClassLoadoutCatalog>()
            .init_resource::<frame::GameSettings>()
            .init_resource::<crate::BindingView>()
            .init_resource::<crate::SessionClassStore>()
            .init_resource::<ClassStoreFile>()
            .init_resource::<frame::HostClassLoadouts>()
            .init_resource::<assets::MenuCatalog>()
            .init_resource::<assets::LocalizeCatalog>()
            .add_systems(
                Update,
                (
                    crate::options::apply_window_settings,
                    load_class_store,
                    sync_host_class_loadouts,
                    save_class_store,
                )
                    .chain()
                    .in_set(ClientSet::Ui),
            );
    }
}

pub fn install_frontend_menus(catalog: &mut assets::MenuCatalog) -> Result<(), String> {
    catalog.load_definitions(include_str!("../menus/frontend.json"))?;
    catalog.load_definitions(include_str!("../menus/classes.json"))?;
    catalog.load_definitions(include_str!("../menus/settings.json"))
}
