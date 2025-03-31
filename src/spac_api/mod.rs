pub mod repos;
pub mod inst;

pub struct SPac
{
    repos: Vec::<String>,
    set_up: Vec::<String>
}

impl SPac
{
    pub fn init () -> Result::<Self, Box::<dyn std::error::Error>>
    {
        for dir in ["spac_repos", "spac_set", "spac_tmp"]
        {
            if !std::path::Path::new(dir).exists()
            {
                if let Err(err) = std::fs::create_dir(dir)
                { return Err(Box::new(err)) }
            }
        }

        let mut config = std::fs::read_to_string("spac_set/packs.csv");

        if let Err(_) = config
        {
            std::fs::File::create_new("spac_set/packs.csv")?;
            config = std::fs::read_to_string("spac_set/packs.csv");
        }

        let config = config?;

        let mut config: Vec::<Vec::<String>> = config
                                                .split('\n').map(|x| x.trim().split(',')
                                                                        .map(|y| String::from(y))
                                                                        .filter(|z| z.len() > 0)
                                                                        .collect::<Vec::<String>>()
                                                                ).collect();

        if config.len() != 2
        { config.resize(2, vec![]); }

        Ok(Self {repos: config[0].clone(), set_up: config[1].clone()})
    }

    pub fn update_set (&self) -> Result::<(), Box::<dyn std::error::Error>>
    {
        std::fs::File::create("spac_set/packs.csv")?;

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
            packs.push_str(&s);
            packs.push(',');
        }

        packs.pop();

        if let Err(err) = std::fs::write("spac_set/packs.csv", packs.into_bytes())
        { Err(Box::new(err)) }
        else
        { Ok(()) }
    }
}
