use std::path::PathBuf;
use dirs;

/// OS specific path to the application cache directory.
pub fn cache(app_name: &str) -> Option<PathBuf>
{
   if app_name.is_empty() {
      return None;
   }

   cache_home().map(|mut dir| { dir.push(app_name); dir })
}

/// OS specific path for caches.
pub fn cache_home() -> Option<PathBuf>
{
   #[cfg(unix)]
   fn _cache_home() -> Option<PathBuf>
   {
      dirs::home_dir().map(|mut dir| { dir.push(".cache"); dir })
   }

   #[cfg(windows)]
   fn _cache_home() -> Option<PathBuf>
   {
      dirs::home_dir().map(|mut dir| {
         dir.push("Local Settings");
         dir.push("Cache");
         dir
      })
   }

   _cache_home()
}

#[test]
#[cfg(test)]
fn tests()
{
   #[cfg(unix)]
   fn _tests()
   {
      match dirs::home_dir() {
         Some(mut dir) => {
            dir.push(".cache");
            dir.push("blub");
            let dir_str = format!("{}", dir.display());
            let cache_str = format!("{}", cache("blub").unwrap().display());
            assert_eq!(dir_str, cache_str);
         }

         None => assert!(false, "Couldn't get homedir!")
      }
   }

   _tests()
}
