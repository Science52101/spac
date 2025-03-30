
fn main () -> Result<(), Box<dyn std::error::Error>>
{
    let mut args = std::env::args();

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

    let _ = args.next();
    
    match
        match args.next()
        {
        Some(arg) => arg,
        None =>
            {
                println!("You must add an argument to use spac. Use `help` for a list of commands.");
                String::new()
            }
        }
            .as_str()
    {
    "help" =>
        {
            println!("Please check the available commands:");
            println!("help\tOutputs this list of commands");
            println!("fetch\tFetches a repository");
            println!("inst\tInstalls a software of a repository");
            println!("del\tDeletes a repository");
            Ok(())
        }
    "fetch" =>
        {
            if let Some(url) = args.next()
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
                    Ok(())
                }
            }
            else
            {
                println!("You must add an url as an argument for fetching a repository.");
                Err("Missing second argument for `fetch`.".into())
            }
        }
    arg =>
        {
            println!("{arg} is not a valid argument. Use `help` for a list of arguments.");
            Err(format!("Unmatched argument `{arg}`").into())
        }
    }

}
