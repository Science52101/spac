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
            println!("inst\tInstalls a repository");
            println!("del\tDeletes a repository");
            println!("upd\tUpdates a repository: del + fetch + inst (if installed)");
            println!("listf\tLists the fetched repositories");
            println!("listi\tLists the installed repositories and their commands");
            println!("sud\tGets (or sets if there is an argument) the user directory");
            println!("run\t(aka. run) Runs an installed package by its command");
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
                Err("Missing second argument for `del`.".into())
            }
        }
    "upd" =>
        {
            if let Some(name) = args.next()
            {
                if let Err(err) = spac.upd(name.as_str())
                {
                    println!("Error: Repository could not be updated.");
                    Err(err)
                }
                else
                {
                    println!("The repository was updated successfully!");
                    Ok(())
                }
            }
            else
            {
                println!("You must add a fetched repository name as an argument for updating a repository.");
                Err("Missing second argument for `upd`.".into())
            }
        }
    "run" | "r" =>
        {
            if let Some(command) = args.next()
            {
                if let Err(err) = spac.run(command.as_str(), &args.collect())
                {
                    println!("Error: The run command could not be executed.");
                    Err(err)
                }
                else
                {
                    Ok(())
                }
            }
            else
            {
                println!("You must add an installed repository name as an argument for running a repository.");
                Err("Missing second argument for `run`.".into())
            }
        }

    "listf" =>
        {
            println!("Fetched repos: (name)");
            for s in spac.listf()
            { println!("{s}") }

            Ok(())
        }
    "listi" =>
        {
            println!("Installed repos: (name, command)");
            for s in spac.listi()
            { println!("{}, {}", s.0, s.1) }

            Ok(())
        }
    "sud" =>
        {
            if let Some(value) = args.next()
            {
                if let Err(err) = spac.set_user_dir(value.as_str())
                {
                    println!("Error: The user directory could not be changed.");
                    Err(err)
                }
                else
                {
                    println!("The user directory was changed successfully!");
                    Ok(())
                }
            }
            else
            {
                println!("{}", spac.get_user_dir());
                Ok(())
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
