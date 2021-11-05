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

// this does not support multithreading!!!!
pub struct AudioManager {
    audio_engine: RefCell<Soloud>,
    currently_playing: RefCell<HashMap<String, Handle>>,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        AudioManager {
            audio_engine: RefCell::new(Soloud::default().unwrap()),
            currently_playing: RefCell::new(HashMap::new()),
        }
    }

    pub fn play(&self, audio: Rc<AudioResource>) {
        self.stop(audio.get_id().clone());

        let handle = self.audio_engine.borrow().play(audio.get_res());
        self.currently_playing.borrow_mut().insert(audio.get_id().clone(), handle);
    }

    pub fn load_paused(&self, audio: Rc<AudioResource>) {
        self.stop(audio.get_id().clone());

        let handle = self.audio_engine.borrow().play_ex(audio.get_res(), 1.0, 0.0, true, Handle::PRIMARY);
        self.currently_playing.borrow_mut().insert(audio.get_id().clone(), handle);
    }

    pub fn stop(&self, audio_id: String) {
        match self.currently_playing.borrow_mut().remove(&audio_id) {
            None => {}
            Some(handle) => {
                self.audio_engine.borrow().stop(handle);
            }
        }
    }

    pub fn pause(&self, audio_id: String) {
        match self.currently_playing.borrow_mut().get(&audio_id) {
            None => { panic!("Attempting to pause sound {:?} that is not playing", audio_id); }
            Some(handle) => {
                self.audio_engine.borrow_mut().set_pause(*handle, true);
            }
        }
    }

    pub fn unpause(&self, audio_id: String) {
        match self.currently_playing.borrow_mut().get(&audio_id) {
            None => { panic!("Attempting to unpause sound {:?} that is not playing", audio_id); }
            Some(handle) => {
                self.audio_engine.borrow_mut().set_pause(*handle, false);
            }
        }
    }
}