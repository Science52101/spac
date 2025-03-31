use crate::spac_api::SPac;

impl SPac
{
    pub fn inst (&self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|x| x.as_str() == name)
        { return Err(format!("Repository named {name} not found").into()) }

        let inst_f = std::fs::read_to_string(format!("spac_repos/{name}/.spac/inst_{}", std::env::consts::OS));

        if let Err(err) = inst_f
        { return Err(Box::new(err)) }

        let inst_f = inst_f?;

        for l in inst_f.split('\n')
        {
            match if cfg!(target_os = "windows")
                                { std::process::Command::new("cmd").arg("/C").arg(&l).status() }
                                else
                                { std::process::Command::new("sh").arg("-c").arg(&l).status() }
            {
            Ok(status) if status.success() => (),
            Ok(status) => return Err(format!("Instalation command {l} failed with status code {status}").into()),
            Err(err) => return Err(Box::new(err))
            }
        }

        Ok(())
    }

    pub fn listi (&self) -> Vec<String>
    {
        self.set_up.clone()
    }
}
