use crate::spac_api::SPac;

pub fn spac_execute_args (spac: &mut SPac) -> Result::<(), Box::<dyn std::error::Error>>
{
    /* SPac CLI argument executor */

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
            println!("listf\tLists the fetched repositories");
            println!("listi\tLists the installed repositories");
            println!("sud\tSets the user directory");
            Ok(())
        }
    "fetch" =>
        {
            if let Some(url) = args.next()
            {
                if let Err(err) = spac.fetch(url.as_str())
                {
                    println!("Error: Repository could not be cloned.");
                    Err(err)
                }
                else
                {
                    println!("The repository was cloned successfully!");
                    Ok(())
                }
            }
            else
            {
                println!("You must add an url as an argument for fetching a repository.");
                Err("Missing second argument for `fetch`.".into())
            }
        }
    "inst" =>
        {
            if let Some(name) = args.next()
            {
                if let Err(err) = spac.inst(name.as_str())
                {
                    println!("Error: Repository could not be installed.");
                    Err(err)
                }
                else
                {
                    println!("The repository was installed successfully!");
                    Ok(())
                }
            }
            else
            {
                println!("You must add a fetched repository name as an argument for installing a repository.");
                Err("Missing second argument for `inst`.".into())
            }
        }
    "del" =>
        {
            if let Some(name) = args.next()
            {
                if let Err(err) = spac.del(name.as_str())
                {
                    println!("Error: Repository could not be deleted.");
                    Err(err)
                }
                else
                {
                    println!("The repository was deleted successfully!");
                    Ok(())
                }
            }
            else
            {
                println!("You must add a fetched repository name as an argument for deleting a repository.");
                Err("Missing second argument for `fetch`.".into())
            }
        }
    "listf" =>
        {
            for s in spac.listf()
            { println!("{s}") }

            Ok(())
        }
    "listi" =>
        {
            for s in spac.listi()
            { println!("{s}") }

            Ok(())
        }
    "sud" =>
        {
            if let Some(value) = args.next()
            {
                spac.set_user_dir(value.as_str());
                Ok(())
            }
            else
            {
                println!("You must add a directory name as an argument for setting the SPac user directory.");
                Err("Missing second argument for `sud`.".into())
            }
        }
    arg =>
        {
            println!("{arg} is not a valid argument. Use `help` for a list of arguments.");
            Err(format!("Unmatched argument `{arg}`").into())
        }
    }
}

pub fn spac_cli () -> Result::<(), Box::<dyn std::error::Error>>
{
    /* SPac Command Line Interface */

    let spac = SPac::init();

    if let Err(err) = spac
    {
        return Err(err);
    }

    let mut spac = spac.unwrap();

    if let Err(err) = spac_execute_args(&mut spac)
    { return Err(err); }

    spac.update_set()
}
