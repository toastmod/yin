
use crate::errors::UnknownValueError;

#[derive(Clone, Debug)]
pub struct Yin32 {
    threshold: f32,
    tau_max: usize,
    tau_min: usize,
    sample_rate: usize,
}

impl Yin32 {
    pub fn init(threshold: f32, freq_min: f32, freq_max: f32, sample_rate: usize) -> Yin32 {
        let tau_max = sample_rate / freq_min as usize;
        let tau_min = sample_rate / freq_max as usize;
        let res = Yin32 {
            threshold,
            tau_max,
            tau_min,
            sample_rate,
        };
        res
    }

    pub fn estimate_freq(&self, audio_sample: &[f32]) -> Result<f32, Box<dyn std::error::Error>> {
        let sample_frequency = compute_sample_frequency(
            audio_sample,
            self.tau_min,
            self.tau_max,
            self.sample_rate,
            self.threshold,
        );

        if sample_frequency.is_infinite() {
            Err(Box::new(UnknownValueError {}))
        } else {
            Ok(sample_frequency)
        }
    }
}

fn diff_function(audio_sample: &[f32], tau_max: usize) -> Vec<f32> {
    let mut diff_function = vec![0.0; tau_max];
    let tau_max = std::cmp::min(audio_sample.len(), tau_max);
    for tau in 1..tau_max {
        for j in 0..(audio_sample.len() - tau_max) {
            let tmp = audio_sample[j] - audio_sample[j + tau];
            diff_function[tau] += tmp * tmp;
        }
    }
    diff_function
}

fn cmndf(raw_diff: &[f32]) -> Vec<f32> {
    let mut running_sum = 0.0;
    let mut cmndf_diff = vec![0.0];
    for index in 1..raw_diff.len() {
        running_sum += raw_diff[index];
        cmndf_diff.push(raw_diff[index] * index as f32 / running_sum);
    }

    cmndf_diff
}

fn compute_diff_min(diff_fn: &[f32], min_tau: usize, max_tau: usize, harm_threshold: f32) -> usize {
    let mut tau = min_tau;
    while tau < max_tau {
        if diff_fn[tau] < harm_threshold {
            while tau + 1 < max_tau && diff_fn[tau + 1] < diff_fn[tau] {
                tau += 1;
            }
            return tau;
        }
        tau += 1;
    }
    0
}

fn convert_to_frequency(
    diff_fn: &[f32],
    max_tau: usize,
    sample_period: usize,
    sample_rate: usize,
) -> f32 {
    let value: f32 = sample_rate as f32 / sample_period as f32;
    value
}

// should return a tau that gives the # of elements of offset in a given sample
pub fn compute_sample_frequency(
    audio_sample: &[f32],
    tau_min: usize,
    tau_max: usize,
    sample_rate: usize,
    threshold: f32,
) -> f32 {
    let diff_fn = diff_function(&audio_sample, tau_max);
    let cmndf = cmndf(&diff_fn);
    let sample_period = compute_diff_min(&cmndf, tau_min, tau_max, threshold);
    convert_to_frequency(&diff_fn, tau_max, sample_period, sample_rate)
}

