#![allow(unused)]
///////////final-update
use std::fs;
use std::env;
use std::thread;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;


fn search_name_file_or_folder(path:&str, target: &str, max_depth_line : usize , mut max_depth_folder : usize , num_threads: usize, include_line_number: usize, reverse_pattern : usize, search_in_file: usize , search_name_file: usize ,print_contain_line:usize) -> Option<String> {
  
    //if max_depth_folder == 0 { return None;}
    let pool = ThreadPool::new(num_threads);
    let queue_first: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut number_of_string_appear = 0;
    //agar linux bod
    //let directory_num = path.matches('/').count();
    
    //agar windows bod
    let mut count = 0;
    let path_pathform = std::path::PathBuf::from(&path);
    for component in path_pathform.components() {
        if let std::path::Component::Normal(_) = component {
            count+=1;
        }
    }
    
    let directory_num = count;

    let p = path.to_string();

    {
    let mut queue = queue_first.lock().unwrap();
    queue.push_back(p);
    }
    
    let target = target.to_string();

    loop{

        let queue_clone = Arc::clone(&queue_first);
        let target_clone = target.clone();
        let entry = {
            let mut queue_clone = queue_clone.lock().unwrap();
            queue_clone.pop_front()
        };
        
        if let Some(entry) = entry {
            
            pool.execute(move || {
            let path = std::path::PathBuf::from(&entry);
            //if path.to_str().matches('/').count()> :{}
            //ba esme file check mikone
            if search_name_file == 1 {
                //ba esme file check mikone
                if reverse_pattern == 0 && path.file_name().unwrap().to_str().unwrap().contains(&target_clone) {
                    
                    println!("Y Esm file {:?} kalame {} ke search kardi ro dare! ",path , target_clone);
                }else if reverse_pattern == 1 && !path.file_name().unwrap().to_str().unwrap().contains(&target_clone){
                    println!("N Esm file {:?} kalame {} ke search kardi ro ndare! ",path , target_clone);
                }
            }


            //ba toe file chec mikone
            
            if path.is_file() {  if search_in_file == 1{
                //println!("hereee");
                let mut search_in_if = 0;
                let path_clone = path.clone();
                let target_copy = target_clone.clone();
                let contents = std::fs::read_to_string(&path_clone);
                let contents = match contents {
                    Ok(contents) => contents,
                    Err(error) =>"".to_string() ,
                };
                if reverse_pattern == 1  &&  !contents.contains(&target_copy){
                    if include_line_number == 0 && print_contain_line==0{
                        println!("N File {:?} reshte {} ro nadare.", path,target_copy); 
                        search_in_if = 1;
                    }
                }
                //let mut count_many_revers = 0;
                //let mut count_loop = 0;
                if search_in_if == 0{
                for (i, line) in contents.lines().enumerate() {
                    //count_loop+=1;
                    if i >= max_depth_line {
                        /* 
                        if count_many_revers == i{
                            println!("N File {:?} reshte {} ro nadare.", path,target_copy); 
                        }
                         
                        */

                        break;
                    }
                    /*
                    if reverse_pattern == 1  &&  !line.contains(&target_copy){
                        if include_line_number == 0 && print_contain_line==0{
                            count_many_revers+=1;
                            
                        }
                    }
                
                     */
                    if (reverse_pattern == 0  && line.contains(&target_copy))   {
                        if include_line_number == 1{
                            println!("File: {:?} reshteh {} ro to khate  {} dere.", path, target_copy ,i + 1);
                            number_of_string_appear += 1;
                            if print_contain_line == 1{
                                println!("LINE: {:?}",line);
                            }
                        }else{
                            println!("File: {:?} reshteh {} ro  dere.", path, target_copy);
                            number_of_string_appear += 1;
                            if print_contain_line == 1{
                                println!("LINE: {:?}",line);
                            }
                        }
                    }else if reverse_pattern == 1  &&  !line.contains(&target_copy)  { 
                        if include_line_number == 1{
                        
                            println!("N File {:?} dar khate {}  nadare", path, i + 1); 
                            if print_contain_line == 1{
                                println!("LINE: {:?}",line);
                            }
                        }else if print_contain_line== 1{
                            println!("LINE: {:?}",line);
                            println!("dar File{:?} reshte ro nadare.",path);
                        }
                    }   
                }
                /* 
                if count_loop == count_many_revers{
                    println!("N File {:?} reshte {} ro nadare.", path,target_copy); 
                }
                */

            }

            }}else{
                for dir_entry in path.read_dir().unwrap() {
                    
                    let dir_entry = dir_entry.unwrap();
                    let entry_path = dir_entry.path();
                    let p = entry_path.to_str().unwrap().to_string();
                    
                    let mut count_file = 0;
                    for component in entry_path.components() {
                        if let std::path::Component::Normal(_) = component {
                            count_file += 1;
                        }
                    }
                    //let num_directory_folder = p.matches('/').count() - directory_num;
                    let  num_directory_folder = count_file - directory_num;
                    if num_directory_folder <= max_depth_folder{
                        let mut gueue = queue_clone.lock().unwrap();
                        gueue.push_back(p);
                    }
                }
            }
            
        
        });

        }else{
            pool.join();  
            let entry = {
                let mut queue_clone = queue_clone.lock().unwrap();
                queue_clone.pop_front()
            };
            if let Some(entry) = entry {
                let mut queue_clone = queue_clone.lock().unwrap();
                queue_clone.push_back(entry);
            } else {
                break;
            }

        
        }
     
        
    }

    return None
}


fn search_exactly_name_file_or_folder(path:&str, target: &str, max_depth_line : usize , mut max_depth_folder : usize , num_threads: usize, include_line_number: usize, reverse_pattern : usize, search_in_file: usize , search_name_file: usize ,print_contain_line:usize) -> Option<String> {
  
    //if max_depth_folder == 0 { return None;}
    let pool = ThreadPool::new(num_threads);
    let queue_first: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut number_of_string_appear = 0;
    //agar linux bod
    //let directory_num = path.matches('/').count();
    
    //agar windows bod
    let mut count = 0;
    let path_pathform = std::path::PathBuf::from(&path);
    for component in path_pathform.components() {
        if let std::path::Component::Normal(_) = component {
            count+=1;
        }
    }
    
    let directory_num = count;

    let p = path.to_string();

    {
    let mut queue = queue_first.lock().unwrap();
    queue.push_back(p);
    }
    
    let target = target.to_string();

    loop{

        let queue_clone = Arc::clone(&queue_first);
        let target_clone = target.clone();
        let entry = {
            let mut queue_clone = queue_clone.lock().unwrap();
            queue_clone.pop_front()
        };
        
        if let Some(entry) = entry {
            
            pool.execute(move || {
            let path = std::path::PathBuf::from(&entry);
            //if path.to_str().matches('/').count()> :{}
            //ba esme file check mikone
            if search_name_file == 1 {
                //ba esme file check mikone
                if reverse_pattern == 0 && path.file_name().unwrap().to_str().unwrap() == target_clone {
                    
                    println!("Y Esm file {:?} kalame {} ke search kardi ro dare! ",path , target_clone);
                }else if reverse_pattern == 1 && path.file_name().unwrap().to_str().unwrap() != target_clone{
                    println!("N Esm file {:?} kalame {} ke search kardi ro ndare! ",path , target_clone);
                }
            }


            //ba toe file chec mikone
            
            if path.is_file() {  if search_in_file == 1{
                //println!("hereee");
                let mut search_in_if = 0;
                let path_clone = path.clone();
                let target_copy = target_clone.clone();
                let contents = std::fs::read_to_string(&path_clone);
                let contents = match contents {
                    Ok(contents) => contents,
                    Err(error) =>"".to_string() ,
                };
                if reverse_pattern == 1  &&  contents != target_copy{
                    if include_line_number == 0 && print_contain_line==0{
                        println!("N File {:?} reshte {} ro nadare.", path,target_copy); 
                        search_in_if = 1;
                    }
                }
                //let mut count_many_revers = 0;
                //let mut count_loop = 0;
                if search_in_if == 0{
                for (i, line) in contents.lines().enumerate() {
                    //count_loop+=1;
                    if i >= max_depth_line {
                        /* 
                        if count_many_revers == i{
                            println!("N File {:?} reshte {} ro nadare.", path,target_copy); 
                        }
                         
                        */

                        break;
                    }
                    /*
                    if reverse_pattern == 1  &&  !line.contains(&target_copy){
                        if include_line_number == 0 && print_contain_line==0{
                            count_many_revers+=1;
                            
                        }
                    }
                
                     */
                    if (reverse_pattern == 0  && line == target_copy)   {
                        if include_line_number == 1{
                            println!("File: {:?} reshteh {} ro to khate  {} dere.", path, target_copy ,i + 1);
                            number_of_string_appear += 1;
                            if print_contain_line == 1{
                                println!("LINE: {:?}",line);
                            }
                        }else{
                            println!("File: {:?} reshteh {} ro  dere.", path, target_copy);
                            number_of_string_appear += 1;
                            if print_contain_line == 1{
                                println!("LINE: {:?}",line);
                            }
                        }
                    }else if reverse_pattern == 1  &&  line != target_copy  { 
                        if include_line_number == 1{
                        
                            println!("N File {:?} dar khate {}  nadare", path, i + 1); 
                            if print_contain_line == 1{
                                println!("LINE: {:?}",line);
                            }
                        }else if print_contain_line== 1{
                            println!("LINE: {:?}",line);
                            println!("dar File{:?} reshte ro nadare.",path);
                        }
                    }   
                }
                /* 
                if count_loop == count_many_revers{
                    println!("N File {:?} reshte {} ro nadare.", path,target_copy); 
                }
                */

            }

            }}else{
                for dir_entry in path.read_dir().unwrap() {
                    
                    let dir_entry = dir_entry.unwrap();
                    let entry_path = dir_entry.path();
                    let p = entry_path.to_str().unwrap().to_string();
                    
                    let mut count_file = 0;
                    for component in entry_path.components() {
                        if let std::path::Component::Normal(_) = component {
                            count_file += 1;
                        }
                    }
                    //let num_directory_folder = p.matches('/').count() - directory_num;
                    let  num_directory_folder = count_file - directory_num;
                    if num_directory_folder <= max_depth_folder{
                        let mut gueue = queue_clone.lock().unwrap();
                        gueue.push_back(p);
                    }
                }
            }
            
        
        });

        }else{
            pool.join();  
            let entry = {
                let mut queue_clone = queue_clone.lock().unwrap();
                queue_clone.pop_front()
            };
            if let Some(entry) = entry {
                let mut queue_clone = queue_clone.lock().unwrap();
                queue_clone.push_back(entry);
            } else {
                break;
            }

        
        }
     
        
    }

    return None
}



    fn main() {
        let args: Vec<String> = env::args().collect();
    
        let path = &args[1];
        let pattern = &args[2];

        let max_depth_folder: usize = args.iter().position(|arg| arg == "--max-depth-folder").and_then(|index| args.get(index + 1)).and_then(|depth| depth.parse().ok()).unwrap_or(usize::MAX);
        let max_depth_file: usize = args.iter().position(|arg| arg == "--max-depth-file").and_then(|index| args.get(index + 1)).and_then(|depth| depth.parse().ok()).unwrap_or(usize::MAX);
        let num_threads = args.iter().position(|arg| arg == "--num-threads").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(2);
        let include_line_number = args.iter().position(|arg| arg == "--include-line-number").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(0);
        let reverse_pattern = args.iter().position(|arg| arg == "--reverse").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(0);
        let search_name_file = args.iter().position(|arg| arg == "--search-name-file").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(1);
        let search_in_file = args.iter().position(|arg| arg == "--search-in-file").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(1);
        let print_contain_line  = args.iter().position(|arg| arg == "--print-contain-line").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(0);
        let exactly_check  = args.iter().position(|arg| arg == "--exactly-check").and_then(|index| args.get(index + 1)).and_then(|threads| threads.parse().ok()).unwrap_or(0);

        println!("path: {}", path);
        println!("pattern: {}", pattern);
        println!("include_line_number: {}", include_line_number);
        println!("max_depth_folder: {}", max_depth_folder);
        println!("max_depth_file: {}", max_depth_file);
        println!("num_threads: {}", num_threads);
        println!("reverse_pattern: {}", reverse_pattern);
        println!("search_in_file: {}", search_in_file);
        println!("search_name_file: {}", search_name_file);
        println!("print_contain_line: {}", print_contain_line);
        println!("exactly_check: {}", exactly_check);


        println!("-----------------------");
        if (exactly_check == 0){
            search_name_file_or_folder(path,pattern, max_depth_file, max_depth_folder, num_threads,include_line_number,reverse_pattern, search_in_file, search_name_file,print_contain_line);
        }else{
            search_exactly_name_file_or_folder(path,pattern, max_depth_file, max_depth_folder, num_threads,include_line_number,reverse_pattern, search_in_file, search_name_file,print_contain_line);

        }
        println!("------------------------");
    
    }


