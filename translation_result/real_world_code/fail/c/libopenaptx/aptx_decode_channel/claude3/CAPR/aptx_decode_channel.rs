
const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;
const NB_SUBBANDS: usize = 4;

const APTX_QMF_OUTER_COEFFS: [[i32; FILTER_TAPS]; NB_FILTERS] = [[0; FILTER_TAPS]; NB_FILTERS];
const APTX_QMF_INNER_COEFFS: [[i32; FILTER_TAPS]; NB_FILTERS] = [[0; FILTER_TAPS]; NB_FILTERS];

struct AptxQMFAnalysis {
    inner_filter_signal: [[[i32; FILTER_TAPS]; NB_FILTERS]; 2],
    outer_filter_signal: [[i32; FILTER_TAPS]; NB_FILTERS],
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [[i32; FILTER_TAPS]; NB_FILTERS],
    coeffs: &[[i32; FILTER_TAPS]; NB_FILTERS],
    shift: u32,
    low_subband_input: i32,
    high_subband_input: i32,
) -> [i32; NB_FILTERS] {
    let mut subbands = [0i32; NB_FILTERS];
    subbands[0] = low_subband_input.wrapping_add(high_subband_input);
    subbands[1] = low_subband_input.wrapping_sub(high_subband_input);

    let mut samples = [0i32; NB_FILTERS];
    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], subbands[1 - i]);
        samples[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
    samples
}

fn aptx_qmf_tree_synthesis(qmf: &mut AptxQMFAnalysis, subband_samples: &[i32; NB_SUBBANDS], samples: &mut [i32; 4]) {
    let mut intermediate_samples = [0i32; 4];

    for i in 0..2 {
        let inner_samples = aptx_qmf_polyphase_synthesis(
            &mut qmf.inner_filter_signal[i],
            &APTX_QMF_INNER_COEFFS,
            22,
            subband_samples[2 * i],
            subband_samples[2 * i + 1],
        );
        intermediate_samples[2 * i] = inner_samples[0];
        intermediate_samples[2 * i + 1] = inner_samples[1];
    }

    for i in 0..2 {
        let outer_samples = aptx_qmf_polyphase_synthesis(
            &mut qmf.outer_filter_signal,
            &APTX_QMF_OUTER_COEFFS,
            21,
            intermediate_samples[i],
            intermediate_samples[i + 2],
        );
        samples[2 * i] = outer_samples[0];
        samples[2 * i + 1] = outer_samples[1];
    }
}

fn aptx_qmf_filter_signal_push(signal: &mut [i32; FILTER_TAPS], sample: i32) {
    for i in (1..FILTER_TAPS).rev() {
        signal[i] = signal[i - 1];
    }
    signal[0] = sample;
}

fn aptx_qmf_convolution(signal: &[i32; FILTER_TAPS], coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let mut sum = 0i64;
    for i in 0..FILTER_TAPS {
        sum = sum.wrapping_add((signal[i] as i64) * (coeffs[i] as i64));
    }
    (sum >> shift) as i32
}
