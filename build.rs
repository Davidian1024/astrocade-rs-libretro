use vergen::{Emitter, BuildBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
let build = BuildBuilder::all_build()?;

Emitter::default()
    .add_instructions(&build)?
    .emit()?;       
    Ok(())
}