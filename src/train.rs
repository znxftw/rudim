use std::fs::{self, File};
use std::io::{BufRead, BufReader, Result, Write};
use std::mem::size_of;
use std::path::Path;
use std::slice::from_raw_parts;
use std::str::FromStr;

use bullet_lib::game::inputs::Chess768;
use bullet_lib::{
    nn::optimiser::AdamW,
    trainer::{
        save::SavedFormat,
        schedule::{TrainingSchedule, TrainingSteps, lr, wdl},
        settings::LocalSettings,
    },
    value::{ValueTrainerBuilder, loader::DirectSequentialDataLoader},
};
use bulletformat::ChessBoard;

// WIP - commit in place for sample trained network
pub fn run(custom_dataset_path: Option<&str>, checkpoint_path: Option<&str>) {
    let dataset_path = custom_dataset_path.unwrap_or("data/self_play.data");

    let metadata = fs::metadata(dataset_path)
        .unwrap_or_else(|_| panic!("Failed to read dataset metadata for '{}'.", dataset_path));
    let file_size = metadata.len();
    let num_positions = file_size / size_of::<ChessBoard>() as u64;
    println!("Dataset contains {} positions.", num_positions);

    let batch_size = 16384.min(num_positions as usize).max(1);
    let batches_per_superbatch = (num_positions as usize / batch_size).max(1);
    let superbatches = 16;

    let mut start_superbatch = 1;
    let mut end_superbatch = superbatches;

    #[allow(clippy::collapsible_if)]
    if let Some(path) = checkpoint_path {
        if let Some(filename) = Path::new(path).file_name().and_then(|s| s.to_str()) {
            if let Some(pos) = filename.rfind('-') {
                if let Ok(num) = filename[pos + 1..].parse::<usize>() {
                    start_superbatch = num + 1;
                    end_superbatch = num + superbatches;
                }
            }
        }
    }

    // 2. Trainer Configuration
    let hl_size = 1024;
    let initial_lr = 0.001;
    let final_lr = 0.001;
    let wdl_proportion = 0.0; // 0.0 for pure value prediction

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
            batch_size,
            batches_per_superbatch,
            start_superbatch,
            end_superbatch,
        },
        wdl_scheduler: wdl::ConstantWDL {
            value: wdl_proportion,
        },
        lr_scheduler: lr::CosineDecayLR {
            initial_lr,
            final_lr,
            final_superbatch: end_superbatch,
        },
        save_rate: 1,
    };

    let settings = LocalSettings {
        threads: 4,
        test_set: None,
        output_directory: "checkpoints",
        batch_queue_size: 4,
    };

    let dataloader = DirectSequentialDataLoader::new(&[dataset_path]);

    println!("Starting bullet training loop...");
    trainer.run(&schedule, &settings, &dataloader);
    println!("Bullet training completed successfully!");

    // Overwrite resources/nnue.bin with the newly trained weights
    let cp_dir = format!("checkpoints/1_simple-{}", end_superbatch);
    let cp_path = format!("{}/quantised.bin", cp_dir);
    println!("Copying weights from {} to resources/nnue.bin", cp_path);
    if let Err(e) = fs::copy(&cp_path, "resources/nnue.bin") {
        eprintln!("Error copying weights: {}", e);
    } else {
        println!("Successfully copied weights to resources/nnue.bin!");
    }
}

pub fn convert_text_to_bin(input_path: &str, output_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(output_path)?;

    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(board) = ChessBoard::from_str(&line) {
            let bytes = unsafe {
                from_raw_parts(
                    &board as *const ChessBoard as *const u8,
                    size_of::<ChessBoard>(),
                )
            };
            output_file.write_all(bytes)?;
            count += 1;
            if count % 1_000_000 == 0 {
                println!("Converted {} positions...", count);
            }
        } else {
            eprintln!("Warning: Failed to parse line: {}", line);
        }
    }
    println!(
        "Successfully converted {} positions to binary format!",
        count
    );
    Ok(())
}
