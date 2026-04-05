use std::io::{self, Read};
use rand::RngExt;
use rand::rngs::ThreadRng;

fn main() {
    k_bandit();
}

const K: usize = 5; // number of arms

fn k_bandit() {
    let epsilon = 0.1; // 10% exploration rate
    let mut rng = rand::rng();

    let mut q = vec![0.0_f64; K]; // Q(a): estimated value of each action
    let mut n = vec![0_u64; K];   // N(a): number of times each action was selected

    let mut step = 0_u64;
    loop {
        step += 1;
        let action = select_action(epsilon, &q, &mut rng);
        let reward = play(action);
        update_estimates(action, reward, &mut q, &mut n);

        println!("Step {step}:");
        println!("  Action: {action}");
        println!("  Reward: {reward}");
        println!("  Q(a): {:?}", q);
        println!("  N(a): {:?}", n);
        println!("Press any key to continue...");

        wait_for_keypress();
    }
}

fn wait_for_keypress() {
    let _ = io::stdin().read(&mut [0u8]);
}

fn play(action: usize) -> f64 {
    let rewards = vec![1.0, 0.0, 0.0, 0.0, 0.0];
    rewards[action]
}

/// Incremental update: Q(a) = Q(a) + (1/N(a)) * (reward - Q(a))
fn update_estimates(action: usize, reward: f64, q: &mut Vec<f64>, n: &mut Vec<u64>) {
    n[action] += 1;
    q[action] += (reward - q[action]) / n[action] as f64;
}

fn select_action(epsilon: f64, q: &[f64], rng: &mut ThreadRng) -> usize {
    if rng.random::<f64>() < epsilon {
        // Exploration: random action
        rng.random_range(0..q.len())
    } else {
        // Exploitation: greedy action — argmax_a Q(a)
        q.iter()
            .enumerate()// dress up with index (index, &value)
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()) // compare only values
            .map(|(idx, _)| idx)// undress to index only
            .unwrap() // extract value from Option.max_by
    }
}