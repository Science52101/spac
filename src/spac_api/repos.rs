use crate::spac_api::SPac;

impl SPac
{
    pub fn fetch (&mut self, url: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        let name = url.split('/').last().unwrap_or("default").trim_end_matches(".git");

        if let Some(_) = self.repos.iter().find(|&x| x == name)
        { return Err(format!("Repository named {name} is already fetched").into()) }

        if name.chars().any(|c| c == ',' || c == ';')
        { return Err("Package is not compatible: name contains ',' or ';'.".into()) }

        if let Err(err) = git2::Repository::clone(&url, format!("{}/spac_repos/{name}/", self.spac_user_dir))
        {
            Err(Box::new(err))
        }
        else
        {
            self.repos.push(String::from(name));
            Ok(())
        }
    }

    pub fn del (&mut self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|&x| x == name)
        { return Err(format!("Repository named {name} not found").into()) }

        if let Err(err) = std::fs::remove_dir_all(format!("{}/spac_repos/{name}/", self.spac_user_dir))
        {
            Err(Box::new(err))
        }
        else
        {
            if !std::path::Path::new(&format!("{}/spac_repos/{name}/", self.spac_user_dir)).exists()
            {
                self.repos.retain(|x| x != name);
                self.set_up.retain(|x| x.0 != name);
            }
            Ok(())
        }
    }

    pub fn listf (&self) -> Vec<String>
    {
        self.repos.clone()
    }
}
