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
                    println!("Error: Directory `{dir}` could not be created.");
                    return Err(Box::new(err));
                }
            }
        }

        Ok(Self { repos: vec![] })
    }
}
