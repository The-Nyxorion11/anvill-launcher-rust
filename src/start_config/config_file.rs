use std::fs;
use std::path::PathBuf; 

//create .minecraft folder if it doesn't exist
pub fn create_files() -> PathBuf {
    let Some(ruta_mc) = get_minecraft_path() else {
        eprintln!("Error path");
        return PathBuf::new();
    };

    fs::create_dir_all(ruta_mc.join("versions")).expect("Error creating versions folder");
    fs::create_dir_all(ruta_mc.join("libraries")).expect("Error creating libraries folder");
    fs::create_dir_all(ruta_mc.join("assets/indexes")).expect("Error creating assets folder");
    fs::create_dir_all(ruta_mc.join("assets/objects")).expect("Error creating assets folder");
    fs::create_dir_all(ruta_mc.join("mods")).expect("Error creating mods folder");
    return ruta_mc;
}

fn get_minecraft_path()-> Option<PathBuf>{
    #[cfg(target_os = "windows")]
    {
        // get C:\Users\<Usuario>\AppData\Roaming\.minecraft
        dirs::data_dir().map(|ruta| ruta.join(".minecraft"))
    }

    #[cfg(target_os = "macos")]
    {
        // get /Users/<Usuario>/Library/Application Support/minecraft
        dirs::data_dir().map(|ruta| ruta.join("minecraft"))
    }

    #[cfg(target_os = "linux")]
    {
        // get /home/<Usuario>/.minecraft
        dirs::home_dir().map(|ruta| ruta.join(".minecraft"))
    }

}