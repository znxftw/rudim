use std::fs;

use bullet_lib::game::inputs::Chess768;
use bullet_lib::nn::optimiser::AdamWOptimiser;
use bullet_lib::value::{NoOutputBuckets, ValueTrainer};
use bullet_lib::{
    nn::optimiser::AdamW,
    trainer::{
        save::SavedFormat,
        schedule::{TrainingSchedule, TrainingSteps, lr, wdl},
        settings::LocalSettings,
    },
    value::{
        ValueTrainerBuilder,
        loader::{ViriBinpackLoader, viribinpack::Filter},
    },
};

pub fn run(custom_dataset_path: Option<&str>) {
    let dataset_path = custom_dataset_path.unwrap_or(DEFAULT_DATASET_PATH);

    let mut trainer = build_trainer(HL_SIZE);

    let schedule = build_schedule();
    let settings = build_settings();
    let dataloader = build_dataloader(dataset_path);

    println!("Starting bullet training loop...");
    trainer.run(&schedule, &settings, &dataloader);
    println!("Bullet training completed successfully!");

    copy_trained_weights();
}

// Tunable Hyperparameters and Configurations
const DEFAULT_DATASET_PATH: &str = "data/v1_gen3_1m_d7.binpack";
const OUTPUT_DIRECTORY: &str = "checkpoints";
const TARGET_WEIGHTS_PATH: &str = "resources/nnue.bin";

const HL_SIZE: usize = 256;

const INITIAL_LR: f32 = 0.001;
const FINAL_LR: f32 = 0.00001;
const WDL_START: f32 = 0.2;
const WDL_END: f32 = 0.7;
const EVAL_SCALE: f32 = 400.0;

const NET_ID: &str = "rudim-256";
const BATCH_SIZE: usize = 16_384;
const BATCHES_PER_SUPERBATCH: usize = 6104;
const START_SUPERBATCH: usize = 1;
const END_SUPERBATCH: usize = 40;
const SAVE_RATE: usize = 5;

const THREADS: usize = 4;
const BATCH_QUEUE_SIZE: usize = 4;
const DATALOADER_PER_THREAD_BUFFERS: usize = 512;

fn build_trainer(hl_size: usize) -> ValueTrainer<AdamWOptimiser, Chess768, NoOutputBuckets> {
    ValueTrainerBuilder::default()
        .dual_perspective()
        .optimiser(AdamW)
        .inputs(Chess768)
        .save_format(&[
            SavedFormat::id("l0w").round().quantise::<i16>(255),
            SavedFormat::id("l0b").round().quantise::<i16>(255),
            SavedFormat::id("l1w").round().quantise::<i16>(64),
            SavedFormat::id("l1b").round().quantise::<i16>(255 * 64),
        ])
        .loss_fn(|output, target| output.sigmoid().squared_error(target))
        .build(|builder, stm_inputs, ntm_inputs| {
            let l0 = builder.new_affine("l0", 768, hl_size);
            let l1 = builder.new_affine("l1", 2 * hl_size, 1);

            let stm_hidden = l0.forward(stm_inputs).screlu();
            let ntm_hidden = l0.forward(ntm_inputs).screlu();
            let hidden_layer = stm_hidden.concat(ntm_hidden);
            l1.forward(hidden_layer)
        })
}

fn build_schedule() -> TrainingSchedule<lr::CosineDecayLR, wdl::LinearWDL> {
    TrainingSchedule {
        net_id: NET_ID.to_string(),
        eval_scale: EVAL_SCALE,
        steps: TrainingSteps {
            batch_size: BATCH_SIZE,
            batches_per_superbatch: BATCHES_PER_SUPERBATCH,
            start_superbatch: START_SUPERBATCH,
            end_superbatch: END_SUPERBATCH,
        },
        wdl_scheduler: wdl::LinearWDL {
            start: WDL_START,
            end: WDL_END,
        },
        lr_scheduler: lr::CosineDecayLR {
            initial_lr: INITIAL_LR,
            final_lr: FINAL_LR,
            final_superbatch: END_SUPERBATCH,
        },
        save_rate: SAVE_RATE,
    }
}

fn build_settings() -> LocalSettings<'static> {
    LocalSettings {
        threads: THREADS,
        test_set: None,
        output_directory: OUTPUT_DIRECTORY,
        batch_queue_size: BATCH_QUEUE_SIZE,
    }
}

fn build_dataloader(dataset_path: &str) -> ViriBinpackLoader {
    let filter = Filter::default();
    ViriBinpackLoader::new(dataset_path, DATALOADER_PER_THREAD_BUFFERS, THREADS, filter)
}

fn copy_trained_weights() {
    let cp_dir = format!("{}/{}-{}", OUTPUT_DIRECTORY, NET_ID, END_SUPERBATCH);
    let cp_path = format!("{}/quantised.bin", cp_dir);
    println!(
        "Copying weights from {} to {}",
        cp_path, TARGET_WEIGHTS_PATH
    );
    if let Err(e) = fs::copy(&cp_path, TARGET_WEIGHTS_PATH) {
        eprintln!("Error copying weights: {}", e);
    } else {
        println!("Successfully copied weights to {}!", TARGET_WEIGHTS_PATH);
    }
}
