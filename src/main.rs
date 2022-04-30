use cfuzz::{FuzzTarget, Runner, TargetConfig};

fn main() {
    let runner = Runner::CargoFuzz {
        target: "fuzz_target_1".to_string(),
    };

    let config = TargetConfig::Git {
        repo: "git@github.com:Lol3rrr/cpiler.git".to_string(),
        folder: "semantic".to_string(),
    };

    let target = FuzzTarget::new("semantic_1", runner, config);

    let runnable = target.setup().unwrap();

    let handle = std::thread::spawn(|| runnable.run().unwrap());

    let result = handle.join().unwrap();

    dbg!(result);
}
