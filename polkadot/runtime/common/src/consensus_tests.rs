#![cfg(test)]

use frame_support::{
    parameter_types,
    weights::constants::WEIGHT_REF_TIME_PER_SECOND,
};
use sp_runtime::Perbill;
use std::time::{Duration, Instant};

// Add Clone
#[derive(Clone)]
struct ConsensusConfig {
    name: &'static str,
    minimum_period: u64,
    epoch_duration: u64,
    block_weight_multiple: u32,
    finality_threshold: u32,
    authority_count: u32,
}

struct ConsensusMetrics {
    blocks_produced: u32,
    avg_block_time: Duration,
    finality_time: Duration,
    tps: f64,
    slot_efficiency: f64,
}

fn run_consensus_benchmark(config: ConsensusConfig) -> ConsensusMetrics {
    // Metrics to track
    let mut blocks_produced = 0;
    let mut finalized_blocks = 0;
    let mut block_times = Vec::new();
    let mut finality_times = Vec::new();
    let mut transactions = 0;
    let mut successful_slots = 0;
    let mut total_slots = 0;
    
    // Test parameters - fixed for simulation
    let block_time_ms = config.minimum_period;
    let sim_duration = Duration::from_secs(10);
    let start_time = Instant::now();
    let mut last_block_time = start_time;
    let mut pending_blocks = Vec::new();
    
    // Simulate blockchain
    while start_time.elapsed() < sim_duration {
        // Advance slot
        total_slots += 1;
        
        // 90% slot success rate
        if total_slots % 10 != 0 {
            successful_slots += 1;
            
            // Block produced
            blocks_produced += 1;
            
            // Simulate transactions
            let txs_in_block = 100 * config.block_weight_multiple;
            transactions += txs_in_block;
            
            // Calculate block time
            let now = Instant::now();
            let block_time = now.duration_since(last_block_time);
            block_times.push(block_time);
            last_block_time = now;
            
            // Add to pending finalization
            pending_blocks.push((blocks_produced, now));
        }
        
        // Check finalization
        if !pending_blocks.is_empty() && 
           pending_blocks[0].0 + config.finality_threshold <= blocks_produced {
            let (_, block_time) = pending_blocks.remove(0);
            finalized_blocks += 1;
            
            let finality_time = Instant::now().duration_since(block_time);
            finality_times.push(finality_time);
        }
        
        // Sleep to simulate slot time
        std::thread::sleep(Duration::from_millis(block_time_ms / 10));
    }
    
    // Calculate final metrics
    let total_time = start_time.elapsed();
    let avg_block_time = if !block_times.is_empty() {
        block_times.iter().sum::<Duration>() / block_times.len() as u32
    } else {
        Duration::from_secs(0)
    };
    
    let avg_finality_time = if !finality_times.is_empty() {
        finality_times.iter().sum::<Duration>() / finality_times.len() as u32
    } else {
        Duration::from_secs(0)
    };
    
    let tps = transactions as f64 / total_time.as_secs_f64();
    
    ConsensusMetrics {
        blocks_produced,
        avg_block_time,
        finality_time: avg_finality_time,
        tps,
        slot_efficiency: successful_slots as f64 / total_slots as f64,
    }
}

#[test]
fn benchmark_consensus_configurations() {
    // Define test configs
    let configs = vec![
        ConsensusConfig {
            name: "Baseline",
            minimum_period: 3000,
            epoch_duration: 200,
            block_weight_multiple: 1,
            finality_threshold: 10,
            authority_count: 4,
        },
        ConsensusConfig {
            name: "Fast Config 1",
            minimum_period: 1500,
            epoch_duration: 100,
            block_weight_multiple: 2,
            finality_threshold: 5,
            authority_count: 4,
        },
        ConsensusConfig {
            name: "Fast Config 2",
            minimum_period: 500,
            epoch_duration: 60,
            block_weight_multiple: 4,
            finality_threshold: 3,
            authority_count: 4,
        },
    ];
    
    println!("=== CONSENSUS BENCHMARK RESULTS ===");
    println!("{:<15} | {:<8} | {:<12} | {:<12} | {:<8} | {:<12}", 
             "Config", "Blocks", "Block Time", "Finality", "TPS", "Slot %");
    println!("---------------------------------------------------------------------");
    
    for config in configs.clone() {
        let metrics = run_consensus_benchmark(config.clone());
        
        println!("{:<15} | {:<8} | {:?} | {:?} | {:<8.2} | {:<12.1}%", 
                 config.name, 
                 metrics.blocks_produced,
                 metrics.avg_block_time,
                 metrics.finality_time,
                 metrics.tps,
                 metrics.slot_efficiency * 100.0);
    }
}