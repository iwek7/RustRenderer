use std::rc::Rc;

use crate::engine::api::audio::AudioManager;
use crate::engine::api::resource_manager::ResourceManager;

pub struct EngineUtilities {
    resource_manager: Rc<dyn ResourceManager>,
    audio_manager: Rc<AudioManager>,
}

impl EngineUtilities {
    pub fn new(resource_manager: Rc<dyn ResourceManager>, audio_manager: Rc<AudioManager>) -> EngineUtilities {
        EngineUtilities {
            resource_manager,
            audio_manager,
        }
    }

    pub fn get_resource_manager(&self) -> Rc<dyn ResourceManager> {
        Rc::clone(&self.resource_manager)
    }

    pub fn get_audio_manager(&self) -> Rc<AudioManager> {
        Rc::clone(&self.audio_manager)
    }
}