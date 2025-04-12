use crate::spac_api::SPac;

impl SPac
{
    pub fn inst (&mut self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|&x| x == name)
        { return Err(format!("Repository named {name} not found").into()) }

        if let Some(_) = self.set_up.iter().find(|&x| x.1 == name)
        { return Err(format!("Repository named {name} is already installed").into()) }

        let inst_c = format!("cd {}/spac_repos/{name}/ && . ./.spac/inst_{}", self.spac_user_dir, std::env::consts::OS);

        match if cfg!(target_os = "windows")
                { std::process::Command::new("cmd").arg("/C").arg(inst_c).status() }
                else
                { std::process::Command::new("sh").arg("-c").arg(inst_c).status() }
        {
        Ok(status) if status.success() => (),
        Ok(status) => return Err(format!("Instalation commands failed with status code {status}").into()),
        Err(err) => return Err(Box::new(err))
        }

        let run_command = std::fs::read_to_string(format!("{}/spac_repos/{name}/.spac/run_command", self.spac_user_dir));

        let run_command = if let Err(_) = run_command { String::from(name) } else { run_command? };

        if run_command.chars().any(|c| c == ',' || c == ';')
        { return Err("Package run command is not compatible: contains ',' or ';'.".into()) }

        self.set_up.push((String::from(name), run_command));

        Ok(())
    }

    pub fn upd (&mut self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if !self.repos.iter().any(|x| x == name)
        { return Err(format!("Repository named {name} not found").into()) }

        let url = git2::Repository::open(format!("{}/spac_repos/{name}", self.spac_user_dir))?;
        let url = url.find_remote("origin")?;
        let url = url.url().unwrap();

        let installed = self.set_up.iter().any(|x| x.1 == name);

        for res in [
                        self.del(name),
                        self.fetch(url),
                    ]
        { if let Err(err) = res { return Err(err) } }

        if installed
        { self.inst(name) }
        else
        { Ok(()) }
    }

    pub fn run (&self, command: &str, args: &Vec::<String>) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let Some((name, _)) = self.set_up.iter().find(|x| x.1.as_str() == command)
        {
            let run_plat = std::fs::read_to_string(format!("{}/spac_repos/{name}/.spac/run_{}", self.spac_user_dir, std::env::consts::OS));

            if let Err(err) = run_plat
            { return Err(Box::new(err)); }

            let mut run_plat = run_plat?.trim()
                                            .replace("<REPO>", format!("{}/spac_repos/{name}", self.spac_user_dir).as_str())
                                            .replace("<SPAC_REPOS>", format!("{}/spac_repos/", self.spac_user_dir).as_str())
                                            .replace("<SPAC_TMP>", format!("{}/spac_tmp/", self.spac_user_dir).as_str())
                                                .to_string();

            for arg in args
            {
                run_plat.push(' ');
                run_plat.push_str(arg);
            }

            println!("{run_plat}");

            std::fs::File::create(format!("{}/spac_tmp/run", self.spac_user_dir))?;

            if let Err(err) = std::fs::write(format!("{}/spac_tmp/run", self.spac_user_dir), run_plat.into_bytes())
            { return Err(Box::new(err)) }
 
            let run_c = format!(". {}/spac_tmp/run", self.spac_user_dir);

            match if cfg!(target_os = "windows")
                    { std::process::Command::new("cmd").arg("/C").arg(run_c).stdout(std::process::Stdio::inherit()).stderr(std::process::Stdio::inherit()).stdin(std::process::Stdio::inherit()).spawn()?.wait() }
                    else
                    { std::process::Command::new("sh").arg("-c").arg(run_c).stdout(std::process::Stdio::inherit()).stderr(std::process::Stdio::inherit()).stdin(std::process::Stdio::inherit()).spawn()?.wait() }
            {
            Ok(_) => (),
            Err(err) => return Err(Box::new(err))
            }

            std::fs::File::create(format!("{}/spac_tmp/run", self.spac_user_dir))?;

            Ok(())
        }
        else
        { Err("Command not found for installed packages.".into()) }
    }

    pub fn listi (&self) -> Vec<(String, String)>
    {
        self.set_up.clone()
    }
}
