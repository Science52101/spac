pub mod repos;
pub mod inst;

pub struct SPac
{
    spac_user_dir: String,
    repos: Vec::<String>,
    set_up: Vec::<(String, String)>
}

impl SPac
{
    pub fn init () -> Result::<Self, Box::<dyn std::error::Error>>
    {
        let sud_path = format!("{}/.spac_user_dir", if cfg!(target_os = "windows") { std::env::var("APPDATA") }
                                                     else { std::env::var("HOME") }? );

        let mut spac_user_dir = std::fs::read_to_string(&sud_path);

        if let Err(_) = spac_user_dir
        {
            std::fs::File::create_new(&sud_path)?;
            std::fs::write(&sud_path, b".")?;
            spac_user_dir = std::fs::read_to_string(&sud_path);
        }

        let spac_user_dir = spac_user_dir?.trim().to_string();

        for dir in ["", "spac_repos", "spac_set", "spac_tmp"]
        {
            if !std::path::Path::new(&format!("{spac_user_dir}/{dir}")).exists()
            {
                if let Err(err) = std::fs::create_dir(format!("{spac_user_dir}/{dir}"))
                { return Err(Box::new(err)) }
            }
        }

        let mut config = std::fs::read_to_string(format!("{spac_user_dir}/spac_set/packs.csv"));

        if let Err(_) = config
        {
            std::fs::File::create_new(format!("{spac_user_dir}/spac_set/packs.csv"))?;
            config = std::fs::read_to_string(format!("{spac_user_dir}/spac_set/packs.csv"));
        }

        let config = config?;

        let mut config: Vec::<Vec::<String>> = config.split('\n').map(|x| x
                                                                        .trim().split(',')
                                                                        .map(|y| String::from(y))
                                                                        .filter(|z| z.len() > 0)
                                                                        .collect::<Vec::<String>>()
                                                                    ).collect();

        if config.len() != 2
        { config.resize(2, vec![]); }

        Ok(Self {
                spac_user_dir,
                repos: config[0].clone(),
                set_up: config[1].clone().iter()
                                    .map(|x| (String::from(x.split_once(';').unwrap().0), String::from(x.split_once(';').unwrap().1)))
                                    .collect()
            })
    }

    pub fn update_set (&self) -> Result::<(), Box::<dyn std::error::Error>>
    {
        std::fs::File::create(format!("{}/spac_set/packs.csv", self.spac_user_dir))?;

        let mut packs = String::new();

        for s in self.repos.iter()
        {
            packs.push_str(&s);
            packs.push(',');
        }

        packs.pop();
        packs.push('\n');

        for s in self.set_up.iter()
        {
            packs.push_str(&s.0);
            packs.push(';');
            packs.push_str(&s.1);
            packs.push(',');
        }

        packs.pop();

        if let Err(err) = std::fs::write(format!("{}/spac_set/packs.csv", self.spac_user_dir), packs.into_bytes())
        { Err(Box::new(err)) }
        else
        { Ok(()) }
    }

    pub fn set_user_dir (&mut self, value: &str) -> Result::<(), Box::<dyn std::error::Error>>
    {
        let sud_path = std::fs::read_to_string(format!("{}/.spac_user_dir", if cfg!(target_os = "windows") { std::env::var("APPDATA") }
                                                                             else { std::env::var("HOME") }? ))?;

        std::fs::File::create(&sud_path)?;

        if let Err(err) = std::fs::write(sud_path, value.to_string().into_bytes())
        { Err(Box::new(err)) }
        else
        { Ok(()) }
    }
}
