use std::cmp::Ordering;
use std::collections::VecDeque;

pub trait Aggregator<A> {
    fn push(self, value: Option<A>) -> Self;
    fn value(&self) -> Option<A>;
    fn reset(self) -> Self;
}

#[derive(Clone, Debug, Copy)]
pub struct ExponentialMovingAverage {
    value: f64,
    alpha: f64,
}

impl ExponentialMovingAverage {
    pub fn new(alpha: f64) -> ExponentialMovingAverage {
        ExponentialMovingAverage {
            alpha,
            value: 0_f64,
        }
    }
}

impl Aggregator<f64> for ExponentialMovingAverage {
    fn push(self, value: Option<f64>) -> Self {
        match value {
            Some(v) => ExponentialMovingAverage {
                value: if self.value == 0_f64 {
                    v
                } else {
                    self.alpha * v + (1_f64 - self.alpha) * self.value
                },
                alpha: self.alpha,
            },
            None => self,
        }
    }

    fn value(&self) -> Option<f64> {
        Some(self.value)
    }

    fn reset(self) -> Self {
        ExponentialMovingAverage {
            value: 0_f64,
            alpha: self.alpha,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MovingAverage {
    values: VecDeque<Option<f64>>,
    over: usize,
}

impl MovingAverage {
    pub fn new(over: usize) -> MovingAverage {
        MovingAverage {
            values: VecDeque::new(),
            over,
        }
    }
}

impl Aggregator<f64> for MovingAverage {
    fn push(self, value: Option<f64>) -> Self {
        let mut values = self.values;
        values.push_front(value);
        if values.len() > self.over {
            values.pop_back();
        }
        MovingAverage { values, ..self }
    }

    fn value(&self) -> Option<f64> {
        let mut sum = 0_f64;
        let mut count = 0_usize;
        for v in self.values.iter() {
            if let Some(value) = v {
                sum += *value;
                count += 1;
            }
        }
        if count < (self.over / 4) || count < 1 {
            None
        } else {
            Some(sum / (count as f64))
        }
    }

    fn reset(self) -> Self {
        let mut values = self.values;
        values.clear();
        MovingAverage { values, ..self }
    }
}

#[derive(Clone, Debug)]
pub struct MedianOverWindow {
    values: VecDeque<Option<f64>>,
    window_size: usize,
}

impl MedianOverWindow {
    pub fn new(over: usize) -> MedianOverWindow {
        MedianOverWindow {
            values: VecDeque::new(),
            window_size: over,
        }
    }
}

impl Aggregator<f64> for MedianOverWindow {
    fn push(self, value: Option<f64>) -> Self {
        let mut values = self.values;
        values.push_front(value);
        if values.len() > self.window_size {
            values.pop_back();
        }
        MedianOverWindow { values, ..self }
    }

    fn value(&self) -> Option<f64> {
        let count = self.values.iter().filter(|x| x.is_some()).count();
        if count < (self.window_size / 4) || count < 1 {
            return None;
        }

        let mut values: Vec<f64> = self.values.iter().flat_map(|x| x.clone()).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));

        let mid = values.len() / 2;
        if values.len() % 2 == 0 {
            Some((values[mid - 1] + values[mid]) / 2_f64)
        } else {
            Some(values[mid])
        }
    }

    fn reset(self) -> Self {
        let mut values = self.values;
        values.clear();
        MedianOverWindow { values, ..self }
    }
}
