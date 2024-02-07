use solana_client::rpc_client::RpcClient;
use std::time::{Instant, Duration};
use std::env;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use rand::Rng;
use rand::seq::SliceRandom;
use std::thread;

const DEFAULT_URL: &str = "https://api.mainnet-beta.solana.com";
const DEFAULT_ITERATION: i32 = 10;
const DEFAULT_SLEEP: u64 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let iteration = args.get(1).map_or(DEFAULT_ITERATION.to_string(), |s| s.to_string());
    let sleep = args.get(2).map_or(DEFAULT_SLEEP.to_string(), |s| s.to_string());
    let url = args.get(3).map_or(DEFAULT_URL, |u| u.as_str());

    let key_pair = [
        Pubkey::from_str("7cVfgArCheMR6Cs4t6vz5rfnqd56vZq4ndaBrY5xkxXy").unwrap(),
        Pubkey::from_str("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg").unwrap()
    ];
    let mut rng = rand::thread_rng();
    let mut selected_keys: [Pubkey; 1] = Default::default();

    let iteration = iteration.parse::<i32>().unwrap_or(DEFAULT_ITERATION);
    let sleep = sleep.parse::<u64>().unwrap_or(DEFAULT_SLEEP);

    let sleep_duration = Duration::from_millis(sleep);

    let url = url.to_string();
    let rpc_client = RpcClient::new(url);
    let mut iteration_count = 0;
    let mut duration_list: Vec<u128> = Vec::new();

    while iteration_count < iteration {
        let random_index = rng.gen_range(0..key_pair.len());
        key_pair.choose_multiple(&mut rng, 1)
            .enumerate()
            .for_each(|(index, &selected_key)| {
                selected_keys[index] = selected_key;
            });

        let start_time = Instant::now();
        rpc_client.get_block_height();
        // rpc_client.get_account(&key_pair[random_index]);
        // rpc_client.get_cluster_nodes();
        // rpc_client.get_multiple_accounts(&selected_keys);
        let duration = start_time.elapsed().as_micros();

        duration_list.push(duration);
        println!("Request Time: {:?} microseconds", duration);

        thread::sleep(sleep_duration);
        iteration_count += 1;
    }

    let average_duration: f32 = duration_list.iter().sum::<u128>() as f32 / duration_list.len() as f32;
    println!("Average Request Time: {:?} milliseconds", average_duration / 1000.0);
}
