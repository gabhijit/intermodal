mod parser;

struct DockerReference {
    name: String,
    tag: String,
}

struct DockerRepo {
    domain: String,
    path: String,
}

#[cfg(test)]
mod tests;
