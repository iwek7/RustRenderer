use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use soloud::{Handle, Soloud, Wav};

pub struct AudioResource {
    id: String,
    res: Wav,
}

impl AudioResource {
    pub fn new(id: &str, res: Wav) -> AudioResource {
        AudioResource {
            id: String::from(id),
            res,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_res(&self) -> &Wav {
        &self.res
    }
}

pub struct AudioManager {
    audio_engine: Soloud,
    currently_playing: RefCell<HashMap<String, Handle>>,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        AudioManager {
            audio_engine: Soloud::default().unwrap(),
            currently_playing: RefCell::new(HashMap::new()),
        }
    }

    pub fn play(&self, audio: Rc<AudioResource>) {
        match self.currently_playing.borrow_mut().get(audio.get_id()) {
            Some(_) => { /*self.audio_engine.stop(audio.get_res())*/ }
            None => {}
        }

        let handle = self.audio_engine.play(audio.get_res());
        self.currently_playing.borrow_mut().insert(audio.get_id().clone(), handle);
    }

    pub fn stop(&self, audio_id: String) {
        match self.currently_playing.borrow_mut().remove(&audio_id) {
            None => {}
            Some(handle) => {
                self.audio_engine.stop(handle)
            }
        }

    }
}