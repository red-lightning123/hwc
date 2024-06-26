use crate::parse;

#[derive(Debug)]
pub struct File {
    minifiles : Vec<Minifile>
}

impl File {
    pub fn minifiles(&self) -> &Vec<Minifile> {
        &self.minifiles
    }
    
    fn new(minifiles : Vec<Minifile>) -> File {
        File {
            minifiles
        }
    }
}

impl TryFrom<parse::File> for File {
    type Error = String;
    fn try_from(file : parse::File) -> Result<Self, Self::Error> {
        let parse::File(parsed_minifiles) = file;
        let mut minifiles = vec![];
        for parsed_minifile in parsed_minifiles {
            minifiles.push(Minifile::try_from(parsed_minifile)?);
        }
        
        Ok(File::new(minifiles))
    }
}

#[derive(Debug)]
pub struct Minifile {
    name : String,
    content : MinifileContent
}

impl Minifile {
    pub fn name(&self) -> &String {
        &self.name
    }
    
    pub fn content(&self) -> &MinifileContent {
        &self.content
    }
    
    fn new(name : String, content : MinifileContent) -> Minifile {
        Minifile {
            name,
            content
        }
    }
}

impl TryFrom<parse::Minifile> for Minifile {
    type Error = String;
    fn try_from(minifile : parse::Minifile) -> Result<Self, Self::Error> {
        let parse::Minifile((name, content)) = minifile;
        let parse::MinifileName((_, parse::Text(name), _)) = name;
        let content = MinifileContent::try_from(content)?;
        Ok(Minifile::new(name, content))
    }
}

#[derive(Debug)]
pub struct MinifileContent {
    items : Vec<MinifileContentItem>
}

impl MinifileContent {
    pub fn items(&self) -> &Vec<MinifileContentItem> {
        &self.items
    }
    
    fn new(items : Vec<MinifileContentItem>) -> MinifileContent {
        MinifileContent {
            items
        }
    }
}

impl TryFrom<parse::MinifileContent> for MinifileContent {
    type Error = String;
    fn try_from(content : parse::MinifileContent) -> Result<Self, Self::Error> {
        let parse::MinifileContent((first, rep, last)) = content;
        let mut items = vec![];
        
        if let Some(first) = first {
            items.push(text_to_item(first));
        }
        
        for (include_cluster, text) in rep {
            items.append(&mut include_cluster_to_items(include_cluster));
            items.push(text_to_item(text));
        }
        
        if let Some(last) = last {
            items.append(&mut include_cluster_to_items(last));
        }
        
        Ok(MinifileContent::new(items))
    }
}

fn text_to_item(text : parse::Text) -> MinifileContentItem {
    let parse::Text(text) = text;
    MinifileContentItem::Text(text)
}

fn include_cluster_to_items(include_cluster : parse::IncludeCluster) -> Vec<MinifileContentItem> {
    let parse::IncludeCluster(includes) = include_cluster;
    let mut items = vec![];
    
    for include in includes {
        items.push(include_to_item(include));
    }
    
    items
}

fn include_to_item(include : parse::Include) -> MinifileContentItem {
    let parse::Include((_, parse::Text(text), _)) = include;
    MinifileContentItem::Include(text)
}

#[derive(Debug)]
pub enum MinifileContentItem {
    Include(String),
    Text(String)
}
