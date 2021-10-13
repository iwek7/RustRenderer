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
//
// struct AudioManager {
//
// }