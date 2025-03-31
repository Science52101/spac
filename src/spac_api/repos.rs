use crate::spac_api::SPac;

impl SPac
{
    pub fn fetch (&mut self, url: &str) -> Result<(), Box::<dyn std::error::Error>>
    {
        let name = url.split('/').last().unwrap_or("default").trim_end_matches(".git");

        if let Err(err) = git2::Repository::clone(&url, format!("spac_repos/{name}/"))
        {
            Err(Box::new(err))
        }
        else
        {
            self.repos.push(String::from(name));
            Ok(())
        }
    }

    pub fn del (&mut self, name: &str) -> Result<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|x| x.as_str() == name)
        {
            return Err(format!("Repository named {name} not found").into());
        }

        if let Err(err) = std::fs::remove_dir_all(format!("spac_repos/{name}/"))
        {
            Err(Box::new(err))
        }
        else
        {
            if !std::path::Path::new("spac_repos/{name}/").exists()
            { self.repos.retain(|x| x != name); }
            Ok(())
        }
    }

    pub fn listf (&self) -> Vec<String>
    {
        self.repos.clone()
    }
}
