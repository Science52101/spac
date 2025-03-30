pub mod repos;

pub struct SPac
{
    repos: Vec::<String>
}

impl SPac
{
    pub fn init () -> Result<Self, Box::<dyn std::error::Error>>
    {
        for dir in ["spac_repos", "spac_set", "spac_tmp"]
        {
            if !std::path::Path::new(dir).exists()
            {
                if let Err(err) = std::fs::create_dir(dir)
                {
                    return Err(Box::new(err));
                }
            }
        }

        let repos : Vec::<String>;

        match std::fs::read_dir("spac_repos")
        {
        Ok(rd) =>
            {
                repos = rd.map(|x| String::from(x.unwrap().path().to_str().unwrap().trim_start_matches("spac_repos/"))).collect();
            }
        Err(err) =>
            return Err(Box::new(err))
        }

        Ok(Self { repos })
    }
}
