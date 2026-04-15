use embassy_stm32::gpio::{AnyPin, Output};
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

// 1. Nos variables partagées (accessibles de partout)
pub static BARGRAPH_LEVEL: AtomicU32 = AtomicU32::new(0);
pub static BARGRAPH_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

// 2. La fonction "statique" pour envoyer une nouvelle valeur
// (Elle n'a pas besoin de &self, on peut l'appeler de n'importe où !)
pub fn update_value(new_value: u32) {
    BARGRAPH_LEVEL.store(new_value, Ordering::Relaxed); // On sauvegarde la valeur
    BARGRAPH_SIGNAL.signal(());                         // On "sonne" la cloche !
}

pub struct BarGraph<'d, const N: usize> {
    leds: [Output<'d, AnyPin>; N],
}

impl<'d, const N: usize> BarGraph<'d, N> {
    pub fn new(leds: [Output<'d, AnyPin>; N]) -> Self {
        Self { leds }
    }

    pub fn set_level(&mut self, level: usize) {
        for (i, led) in self.leds.iter_mut().enumerate() {
            if i < level {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }

    // 3. La méthode asynchrone qui attend la cloche
    pub async fn wait_and_update(&mut self) {
        // Le programme se met en pause ici jusqu'à ce que update_value() soit appelée
        BARGRAPH_SIGNAL.wait().await;
        
        // On lit la nouvelle valeur
        let level = BARGRAPH_LEVEL.load(Ordering::Relaxed) as usize;
        
        // On met à jour les LEDs physiquement
        self.set_level(level);
    }
}