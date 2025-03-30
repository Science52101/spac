use crate::spac_api::SPac;

pub fn spac_cli () -> Result<(), Box::<dyn std::error::Error>>
{
    /* SPac Command Line Interface */

    let spac = SPac::init();

    if let Err(err) = spac
    {
        return Err(err);
    }

    let mut spac = spac.unwrap();

    let mut args = std::env::args();

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
                spac.clone_repo(url.as_str())
            }
            else
            {
                println!("You must add an url as an argument for fetching a repository.");
                Err("Missing second argument for `fetch`.".into())
            }
        }
    "inst" =>
        {
            println!("`inst` is not implemented yet. This is a WIP.");
            Err(format!("WIP command").into())
        }
    "del" =>
        {
            println!("`del` is not implemented yet. This is a WIP.");
            Err(format!("WIP command").into())
        }
    arg =>
        {
            println!("{arg} is not a valid argument. Use `help` for a list of arguments.");
            Err(format!("Unmatched argument `{arg}`").into())
        }
    }
}
