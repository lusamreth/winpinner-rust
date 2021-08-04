use super::AsyncFnPtr;

pub fn time(closure:impl FnOnce() -> ()) -> std::time::Duration {
    use std::time::Instant;
    let now = Instant::now();
    closure();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    return elapsed
}

#[allow(dead_code)]
pub fn avg_run(eachrun:impl Fn(&mut Vec<f64>) -> (),runtime:i32) -> f64 {
    let mut recorder:Vec<f64> = Vec::new();
    (0..runtime).into_iter().for_each(|_| eachrun(&mut recorder));
    let sum:f64 = Iterator::sum(recorder.iter());
    return sum / runtime as f64
}

pub async fn benchmark_async(fnptr:AsyncFnPtr<()>) -> std::time::Duration {
    use std::time::Instant;
    let now = Instant::now();

    println!("output {:#?}",fnptr.run().await);
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    return elapsed
}
