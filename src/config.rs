use std::path::Path;


struct ConfigError

enum ErrorKind {
    NonexistantConfig
}

enum PathVisibility {
    ShortHand,
    LongHand,
    Alias(String)
}

struct Config {
    location: Path, 
}

impl Config {

    fn new(path: Path) -> Result<()> {
        
    }
}


