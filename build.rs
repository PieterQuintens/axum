fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/payments.proto")?;

    return Ok(()); // Can also be 'Ok(())' without the semicolon and return
}
