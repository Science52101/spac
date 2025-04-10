use crate::spac_api::SPac;

impl SPac
{
    pub fn inst (&mut self, name: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let None = self.repos.iter().find(|&x| x == name)
        { return Err(format!("Repository named {name} not found").into()) }

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

    pub fn run (&self, command: &str, args: &Vec::<String>) -> Result::<(), Box::<dyn std::error::Error>>
    {
        if let Some((name, _)) = self.set_up.iter().find(|x| x.1.as_str() == command)
        {
            let run = std::fs::read_to_string(format!("{}/spac_repos/{name}/.spac/run", self.spac_user_dir));

            if let Err(err) = run
            { return Err(Box::new(err)); }

            let mut run = run?;

            for arg in args
            {
                run.push(' ');
                run.push_str(arg);
            }

            match if cfg!(target_os = "windows")
                    { std::process::Command::new("cmd").arg("/C").arg(run).status() }
                    else
                    { std::process::Command::new("sh").arg("-c").arg(run).status() }
            {
            Ok(status) if status.success() => (),
            Ok(status) => return Err(format!("Run command(s) failed with status code {status}").into()),
            Err(err) => return Err(Box::new(err))
            }

            Ok(())
        }
        else
        { Err("Command not found for installed packages.".into()) }
    }

    pub fn listi (&self) -> Vec<String>
    {
        self.set_up.clone().iter().map(|x| x.0.clone()).collect()
    }
}
