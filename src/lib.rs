use file_info::FileInfo;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
pub mod file_info;

pub const JSON_CONF_FILE_NAME: &'static str = "bundle.json";
const DEFAULT_BUNDLE_ENV: &'static str = "DEFAULT_BUNDLE_PATH";

pub struct Bundle(PathBuf);

impl Bundle {
    pub fn create<P: AsRef<Path>>(bundle_folder: P) -> io::Result<Bundle> {
        let folder_nentries = fs::read_dir(&bundle_folder)?.into_iter().count();
        if folder_nentries != 0 {
            // TODO: Replace ErrorKind when <https://github.com/rust-lang/rust/issues/86442> is stabilized.
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "[DirectoryNotEmpty]: Init require an empty folder.",
            ));
        }
        write_sources(bundle_folder.as_ref(), "BUNDLE_PATH")?;
        let mut bundle_json = bundle_folder.as_ref().join(JSON_CONF_FILE_NAME);
        let mut config = fs::File::create(&bundle_json)?;
        write!(&mut config, "[]")?;
        bundle_json.pop();
        Ok(Bundle(bundle_json))
    }

    /// Use this one if you are shure that bundle_folder is empty and you have the permition to write in it.
    pub fn create_unchecked<P: AsRef<Path>>(bundle_folder: P) -> Bundle {
        write_sources(bundle_folder.as_ref(), "BUNDLE_PATH").unwrap();
        let mut bundle_json = bundle_folder.as_ref().join(JSON_CONF_FILE_NAME);
        let mut config = fs::File::create(&bundle_json).unwrap();
        write!(&mut config, "[]").unwrap();
        bundle_json.pop();
        Bundle(bundle_json)
    }

    pub fn open<P: AsRef<Path>>(bundle_folder: P) -> io::Result<Bundle> {
        let bundle_json = bundle_folder.as_ref().join(JSON_CONF_FILE_NAME);
        if !bundle_json.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} file not found in folder", JSON_CONF_FILE_NAME),
            ));
        }

        Ok(Bundle(PathBuf::from(bundle_folder.as_ref())))
    }

    pub fn get_info(&self) -> Vec<FileInfo> {
        let config = fs::File::open(self.0.join("bundle.json")).unwrap();
        let reader = io::BufReader::new(config);
        serde_json::from_reader(reader).unwrap()
    }

    fn update_info(&self, new_info: &Vec<FileInfo>) {
        let config = fs::File::create(self.0.join("bundle.json")).unwrap();
        let writer = io::BufWriter::new(config);
        serde_json::to_writer(writer, new_info).unwrap();
    }

    pub fn mkfile(&self, new_info: &FileInfo, ignore_existant: bool) -> Result<PathBuf, ()> {
        let mut bundle_info = self.get_info();
        if !ignore_existant {
            for info in bundle_info.iter() {
                if info == new_info {
                    return Err(());
                }
            }
        }
        bundle_info.push(new_info.clone());
        self.update_info(&bundle_info);
        let new_file = self
            .get_asolute()
            .unwrap()
            .join(format!("{}.{:#8X}", new_info.name, new_info.hash_id));
        fs::File::create(&new_file).unwrap();
        Ok(new_file)
    }

    pub fn set_default(&self) -> io::Result<()> {
        unimplemented!("ENV var not exported correctly.");
        #[allow(unreachable_code)]
        {
            env::set_var(DEFAULT_BUNDLE_ENV, self.get_asolute()?.as_os_str());
            Ok(())
        }
    }

    // TODO: May be overkile, a "check_abs" could be good.
    pub fn get_asolute(&self) -> io::Result<PathBuf> {
        fs::canonicalize(&self.0)
    }
}

fn write_sources<P: AsRef<Path>, S: AsRef<str>>(
    bundle_folder: P,
    sys_envvar_name: S,
) -> io::Result<()> {
    let file = bundle_folder.as_ref().join("source_me.sh");
    let mut bash_file = fs::File::create(&file)?;
    write!(
        &mut bash_file,
        concat!(
            "#!/usr/bin/env bash\n",
            "export {}=$(dirname \"$(realpath \"${{BASH_SOURCE[0]}}\")\")"
        ),
        sys_envvar_name.as_ref()
    )?;
    let file = file.with_extension("bat");
    let mut fish_file = fs::File::create(&file)?;
    // TODO: find someone on window
    write!(
        &mut fish_file,
        concat!("#!.bat\n", "SET {}=\"%~dp0\""),
        sys_envvar_name.as_ref()
    )?;
    Ok(())
}
