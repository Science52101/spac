use crate::spac_api::SPac;

impl SPac
{
    pub fn clone_repo (&mut self, url: &str) -> Result<(), Box::<dyn std::error::Error>>
    {
        let name = format!("spac_repos/{}/", url.split('/').last().unwrap_or("default").trim_end_matches(".git"));

        if let Err(err) = git2::Repository::clone(&url, &name)
        {
            println!("Error: Repository could not be cloned.");
            Err(Box::new(err))
        }
        else
        {
            println!("The repo `{name}` was cloned successfully!");
            self.repos.push(name);
            Ok(())
        }
    }
}
