use serde;
use csv::Writer;
use std::io::Write;

pub fn write_to_csv<T:serde::Serialize, W:Write>(writer: &mut Writer<W>, data: &Vec<T>) -> Result<(), csv::Error>{
    for record in data {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}