use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

pub struct AtomicEffects {
    volume: Arc<AtomicU8>,
}

// Represents effects that should be handled atomically
// Examples: Master volume, distortion, gain, overdrive, etc
impl AtomicEffects {
    pub fn new() -> Self {
        Self {
            volume: Arc::new(AtomicU8::new(0)), // should be applied on output online
        }
    }

    pub fn set_volume(&self, vol: u32) {
        let new_volume: u8 = vol as u8;
        self.volume.store(new_volume, Ordering::Relaxed);
        println!("{vol} {new_volume}")
    }

    pub fn get_volume(&self) -> f32 {
        let vol = self.volume.load(Ordering::Relaxed);
        return (vol as f32 / 100.0).clamp(0., 1.);
    }
}