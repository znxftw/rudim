use std::fs::{self};

use bullet_lib::game::inputs::Chess768;
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

// WIP - commit in place for sample trained network
pub fn run(custom_dataset_path: Option<&str>, checkpoint_path: Option<&str>) {
    let dataset_path = custom_dataset_path.unwrap_or("data/self_play.binpack");

    let metadata = fs::metadata(dataset_path)
        .unwrap_or_else(|_| panic!("Failed to read dataset metadata for '{}'.", dataset_path));
    let file_size = metadata.len();
    let num_positions = (file_size / 8).max(1);
    println!(
        "Dataset contains approximately {} positions.",
        num_positions
    );

    let hl_size = 32;
    let initial_lr = 0.001;
    let final_lr = 0.00001;
    let wdl_proportion = 0.7; // 0.0 for pure value prediction

    let mut trainer = ValueTrainerBuilder::default()
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
        });

    if let Some(path) = checkpoint_path {
        println!("Loading checkpoint from {}...", path);
        trainer.load_from_checkpoint(path);
    }

    let schedule = TrainingSchedule {
        net_id: "1_simple".to_string(),
        eval_scale: 400.0,
        steps: TrainingSteps {
            batch_size: 16_384,
            batches_per_superbatch: 6104,
            start_superbatch: 1,
            end_superbatch: 40,
        },
        wdl_scheduler: wdl::ConstantWDL {
            value: wdl_proportion,
        },
        lr_scheduler: lr::CosineDecayLR {
            initial_lr,
            final_lr,
            final_superbatch: 40,
        },
        save_rate: 1,
    };

    let settings = LocalSettings {
        threads: 4,
        test_set: None,
        output_directory: "checkpoints",
        batch_queue_size: 4,
    };

    let filter = Filter::default();
    let dataloader = ViriBinpackLoader::new(dataset_path, 512, 4, filter);

    println!("Starting bullet training loop...");
    trainer.run(&schedule, &settings, &dataloader);
    println!("Bullet training completed successfully!");

    // Overwrite resources/nnue.bin with the newly trained weights
    let cp_dir = format!("checkpoints/1_simple-{}", 40);
    let cp_path = format!("{}/quantised.bin", cp_dir);
    println!("Copying weights from {} to resources/nnue.bin", cp_path);
    if let Err(e) = fs::copy(&cp_path, "resources/nnue.bin") {
        eprintln!("Error copying weights: {}", e);
    } else {
        println!("Successfully copied weights to resources/nnue.bin!");
    }
}
