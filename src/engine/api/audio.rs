use soloud::Wav;

pub struct AudioResource {
    id: String,
    pub res: Wav
}

impl AudioResource {
    pub fn new(id: &str, res: Wav) -> AudioResource {
        AudioResource {
            id: String::from(id),
            res
        }
    }
}

pub struct AudioManager {

}

impl AudioManager {
    pub fn new() -> AudioManager {
        AudioManager{}
    }
}