use crate::spac_api::SPac;

impl SPac
{
    pub fn inst (&mut self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|&x| x == name)
        { return Err(format!("Repository named {name} not found").into()) }

        let inst_c = format!("cd ./spac_repos/{name}/ && . ./.spac/inst_{}", std::env::consts::OS);

        println!("{inst_c}");

        match if cfg!(target_os = "windows")
                            { std::process::Command::new("cmd").arg("/C").arg(inst_c).status() }
                            else
                            { std::process::Command::new("sh").arg("-c").arg(inst_c).status() }
        {
        Ok(status) if status.success() => (),
        Ok(status) => return Err(format!("Instalation commands failed with status code {status}").into()),
        Err(err) => return Err(Box::new(err))
        }

        self.set_up.push((String::from(name), String::from("")));

        Ok(())
    }

    pub fn listi (&self) -> Vec<String>
    {
        self.set_up.clone().iter().map(|x| x.0.clone()).collect()
    }
}
