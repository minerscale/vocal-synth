mod data;

use rand::random;
use rodio::Source;
use std::f32::consts::PI;
use std::time::Duration;
use simple_eq::Equalizer;
use simple_eq::design::Curve;

const NUM_OSCILLATORS: usize = 50;
const SAMPLE_RATE: f32 = 48000.0;

#[derive(Clone, Debug)]
pub struct VocalSynth {
    freq: f32,
    position: f32,
    time: f32,
    prev_time: f32,
    eq: Equalizer<f32>,
    gain: f32,
}

const EQ_FREQ: [u32; 31] = [20, 40, 103, 207, 352, 539, 768, 1038, 1350, 1704, 2099, 2535, 3013, 3533, 4095, 4697, 5342, 6028, 6756, 7525, 8336, 9188, 10082, 11018, 11995, 13014, 14074, 15176, 16320, 17505, 18731];

const EQ_GAIN: [[f32; 32]; 5] = [[1.7, 2.3, 2.6, -1.3, -1.5, 2.6, -5.4, -12.5, 19.3, -3.1, -10.2, 18.1, -18.5, 16.8, -15.0, 12.9, -16.1, 19.4, -11.9, 10.2, -6.6, 16.3, -7.9, 18.6, -18.5, 16.1, -17.8, 14.7, 7.6, -19.2, 8.4, -3.9],
[6.2, 5.6, 4.2, -0.1, -0.5, -3.0, 5.3, -17.8, 13.2, -4.1, 1.5, 6.8, -10.7, 11.0, -15.4, 18.9, -15.9, 16.3, -13.8, 16.7, -8.7, 16.1, -11.2, 18.4, -19.2, 15.7, -13.1, 14.1, 9.1, 6.6, -19.0, -8.3],
[3.9, -1.5, -4.6, -1.5, -4.2, -5.5, 4.0, -7.5, -4.0, 11.4, -8.4, -4.3, 12.6, -14.4, 11.8, -15.7, 17.1, 2.3, 11.6, -10.8, 19.5, -6.1, 18.2, -9.5, 16.8, -11.8, 16.7, -14.0, 16.9, -16.2, 17.5, -19.0],
[1.8, 0.5, 0.5, -1.5, -1.8, -3.8, -1.6, -8.7, 0.2, -0.5, -7.7, 19.3, 1.7, 9.7, -15.0, 7.1, -7.7, 19.2, -9.3, 13.4, -8.6, 16.0, -12.3, 17.3, -16.3, 16.7, -18.2, 16.5, -19.7, 16.7, -16.0, -15.0],
[5.1, 4.4, 5.4, -3.8, -10.6, 7.1, 19.1, -18.0, 17.7, -17.8, 8.5, 0.2, 0.4, 14.2, -20.0, -3.8, -2.3, 6.8, 3.2, 6.8, -13.3, 16.6, -6.4, -8.1, 17.2, -13.2, -14.5, 16.3, -16.1, 16.4, -16.3, -12.3]];

impl VocalSynth {
    fn db_to_voltage(db: f32) -> f32{
        10.0_f32.powf(db/20.0)
    }

    #[inline]
    pub fn new(freq: f32) -> VocalSynth {
        
        let mut eq = Equalizer::new(SAMPLE_RATE);

        for x in 0..31 {
            eq.set(x, Curve::Peak, EQ_FREQ[x] as f32, 1.0, EQ_GAIN[0][x])
        }

        eq.set(31, Curve::Highshelf, 10000.0, 0.5, -36.0);

        VocalSynth {
            freq,
            position: 0.0,
            time: 0.0,
            prev_time: 0.0,
            eq,
            gain: Self::db_to_voltage(EQ_GAIN[0][31])
        }
    }

    fn change_fricative(&mut self, idx: usize) {
        for x in 0..31 {
            self.eq.set(x, Curve::Peak, EQ_FREQ[x] as f32, 1.0, EQ_GAIN[idx][x]);
            self.gain = Self::db_to_voltage(EQ_GAIN[idx][31]);
        }
    }
}

impl Iterator for VocalSynth {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.position += self.freq / SAMPLE_RATE;
        self.position %= 1.0;

        self.time += 1.0 / 48000.0;
        self.freq = 146.0 + 1.0 * (self.time * 2.0 * PI * 5.0).sin() + (146.0*(9.0/8.0) - 146.0)*(((self.time + 1.0/16.0) * 2.0) % 3.0).floor();

        if (self.time % 1.0) > (self.prev_time % 1.0) {
            self.change_fricative(self.time as usize % 5)
        }

        let tijd = self.time * 16.0;

        let total = (0..NUM_OSCILLATORS)
            .map(|x| {
                ((((tijd % 8.0) - 7.0).max(0.0))
                    * data::VOWELS[(tijd / 8.0).floor() as usize + 1][x]
                    + (1.0 - (((tijd % 8.0) - 7.0).max(0.0)))
                        * data::VOWELS[(tijd / 8.0).floor() as usize][x])
                    * (2.0 * PI * ((x + 1) as f32) * self.position).sin()
            })
            .sum::<f32>() / (NUM_OSCILLATORS as f32);


        Some(self.eq.process(random::<f32>() * self.gain * 0.01) + 5.0*total)
    }
}

impl Source for VocalSynth {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
