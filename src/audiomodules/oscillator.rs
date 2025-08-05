use crate::audio_modules::AudioModule;
use std::f32::consts::PI;

pub struct Oscillator {
    pub phase: f32,
    pub frequency: f32,
    pub sample_rate: f32,
}

impl Oscillator {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency,
            sample_rate,
        }
    }
}

impl AudioModule for Oscillator {
    fn process(&mut self, output: &mut [f32]) {
        let phase_increment = self.frequency / self.sample_rate; //phase_increment — это на сколько нужно сдвинуть фазу при каждом семпле, чтобы получить self.frequency  
        for sample in output.iter_mut() { //Проходим по всем элементам буфера output
            self.phase += phase_increment; //Фаза увеличивается на phase_increment
            if self.phase > 1.0 { // если выходит за границу то она вычитаеться что бы остаться в диапазоне
                self.phase -= 1.0;
            }
            *sample = (self.phase * 2.0 * PI).sin(); // sample. фазу умножаем на 2pi - переводим в радианы и в синус
        }
    }
}

struct Wave {               //синусойда: amplitude*sin((m*x*PI)/wavelength)                  
 amplitude: f32,   //пилообразная волна: (2*amlitude)/wavelength*mod(m*x,wavelength)-amlitude
 wavelength: f32,    //квадратная волна: amplitude*sgn(sin((m*PI*x)/wavelength))
}