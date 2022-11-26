use crate::plugin_rack::PluginRack;
use std::io::Read;

pub struct StateHeadless {
    pub rack: PluginRack,
}

impl StateHeadless {
    pub fn new() -> Self {
        Self {
            rack: PluginRack::new(),
        }
    }

    pub fn load_project<P: AsRef<std::path::Path>>(&mut self, file: P) -> anyhow::Result<()> {
        let zip_file = std::fs::File::open(&file)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;
        let mut proj_file = archive.by_name("project.json")?;

        let mut proj_file_string = String::new();
        proj_file.read_to_string(&mut proj_file_string)?;
        let instacnes: Vec<crate::plugin_rack::PluginRackInstance> =
            serde_json::from_str(&proj_file_string)?;

        self.rack = PluginRack::new();

        self.rack.plugins.extend(instacnes);
        self.rack.load_uninitialzed_plugins()?;
        Ok(())
    }

    pub fn load_image<P: AsRef<std::path::Path>>(&mut self, file: P) -> anyhow::Result<()> {
        self.rack.load_image(file)?;
        Ok(())
    }

    pub fn process(&mut self) {}
}
