use soloud::Wav;

struct AudioResource {
    res: Wav
}

impl AudioResource {
    pub fn new(res: Wav) -> AudioResource {
        AudioResource {
            res
        }
    }
}