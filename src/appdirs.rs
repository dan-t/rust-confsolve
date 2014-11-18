use std::os::homedir;

/// OS specific path to the application cache directory.
pub fn cache(app_name: &str) -> Option<Path>
{
   if app_name.is_empty() {
      return None;
   }

   match cache_home() {
      Some(mut dir) => {
         dir.push(app_name);
         Some(dir)
      }

      None => None
   }
}

/// OS specific path for caches.
pub fn cache_home() -> Option<Path>
{
   #[cfg(unix)]
   fn _cache_home() -> Option<Path>
   {
      match homedir() {
         Some(mut dir) => {
            dir.push(".cache");
            Some(dir)
         }

         None => None
      }
   }

   #[cfg(windows)]
   fn _cache_home() -> Option<Path>
   {
      match homedir() {
         Some(mut dir) => {
            dir.push("Local Settings");
            dir.push("Cache");
            Some(dir)
         }

         None => None
      }
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
      match homedir() {
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
