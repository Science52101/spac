use crate::spac_api::SPac;

impl SPac
{
    pub fn inst (&self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|x| x.as_str() == name)
        {
            return Err(format!("Repository named {name} not found").into());
        }

        // TODO: Instalation proccess...

        Ok(())
    }

    pub fn listi (&self) -> Vec<String>
    {
        self.set_up.clone()
    }
}
