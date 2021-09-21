use bincode;
use glob::glob;
use landon;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn convert_blender_files(blender_files: &[String]) -> Result<(), String> {
    let blender_files = blender_files.iter().map(PathBuf::from).collect::<Vec<_>>();
    let blender_stdout = match landon::export_blender_data(&blender_files) {
        Ok(s) => s,
        Err(err) => {
            panic!("Failed exporting blender data: {}", err);
        }
    };

    // Meshes
    let meshes_by_file = blender_mesh::parse_meshes_from_blender_stdout(&blender_stdout)
        .map_err(|e| format!("{:?}", e))?;
    let flattened_meshes =
        blender_mesh::flatten_exported_meshes(&meshes_by_file).map_err(|e| format!("{}", e))?;
    let flattened_meshes = bincode::serialize(&flattened_meshes).map_err(|e| format!("{:?}", e))?;

    let mut f = File::create("./models/meshes.bin").map_err(|e| format!("{}", e))?;
    f.write_all(&flattened_meshes[..])
        .map_err(|e| format!("{}", e))?;

    // Armatures
    let armatures_by_file =
        blender_armature::parse_armatures_from_blender_stdout(&blender_stdout).unwrap();

    let flattened_armatures =
        blender_armature::flatten_exported_armatures(&armatures_by_file).unwrap();

    let flattened_armatures = bincode::serialize(&flattened_armatures).unwrap();

    let mut f = File::create("./models/armatures.bin").unwrap();
    f.write_all(&flattened_armatures[..]).unwrap();

    Ok(())
}

fn main() {
    let blender_paths = glob("./blender/**/*.blend")
        .expect("Failed to read glob pattern")
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid glob file");
    let blender_files = blender_paths
        .into_iter()
        .map(|f| format!("{}", f.display()))
        .collect::<Vec<_>>();
    for file in blender_files.iter() {
        println!("cargo:rerun-if-changed={}", file);
        // println!("cargo:warning={}", file);
    }

    convert_blender_files(&blender_files).unwrap();
}
