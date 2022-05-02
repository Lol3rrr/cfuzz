import type { FuzzResult } from "./results";

const base = "http://192.168.178.22:8080/api";

export class Project {
    name: String;
    source: Source;
    targets: Array<ProjectTarget>
}

export class Source {
    Git: {
        repo: String;
    };
}

export class ProjectTarget {
    name: String;
    folder: String;
    target: {
        CargoFuzz: {
            name: String;
        };
    };
}

export async function loadResults(project: String): Promise<Array<FuzzResult>> {
    return fetch(base + "/results?pname=" + project).then((response) => response.json());
}

export async function loadRunning(): Promise<Array<String>> {
    return fetch(base + "/targets").then((response) => response.json());
}

export async function run(project_name: String, name: String, fuzz_target: String, repo: String, folder: String) {
    let config = {
        "pname": project_name,
        "name": name,
        "runner": {
            "CargoFuzz": {
                "target": fuzz_target
            }
        },
        "config": {
            "Git": {
                "repo": repo,
                "folder": folder
            }
        },
        "repeating": false
    };

    return fetch(base + "/run", {
        method: 'POST',
        body: JSON.stringify(config),
        headers: {
            "content-type": "application/json",
        },
    });
}

export async function load_projects(): Promise<Array<Project>> {
    return fetch(base + "/projects/list").then((response) => response.json());
}

export async function addProject(name: String, repo: String) {
    let config = {
        "name": name,
        "source": {
            "Git": {
                "repo": repo,
            }
        },
        "targets": []
    };

    fetch(base + "/projects/update", {
        method: "POST",
        body: JSON.stringify(config),
        headers: {
            "content-type": "application/json"
        }
    });
}

export async function removeProject(name: String) {
    fetch(base + "/projects/remove?pname=" + name, {
        method: "POST"
    });
}

export async function addProjectTarget(pname: String, tname: String, folder: String, fuzz_target: String) {
    let config = {
        "name": tname,
        "folder": folder,
        "target": {
            "CargoFuzz": {
                "name": fuzz_target
            }
        }
    };

    fetch(base + "/projects/targets/add?pname=" + pname, {
        method: "POST",
        body: JSON.stringify(config),
        headers: {
            "content-type": "application/json"
        }
    });
}